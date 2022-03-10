use crate::{
  pmx::rigid_body::*,
  reader::{helpers::ReadHelpers, DisplayReader},
  Config, DefaultConfig, Result, Settings,
};
use byteorder::{ReadBytesExt, LE};
use std::convert::TryFrom;
use std::io::Read;
use std::marker::PhantomData;

pub struct RigidBodyReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R,
  pub(crate) poison: bool,
}

impl<R: Read> RigidBodyReader<R> {
  pub fn new(mut d: DisplayReader<R>) -> Result<RigidBodyReader<R>> {
    assert!(!d.poison);
    while d.remaining > 0 {
      d.next::<DefaultConfig>()?;
    }
    let count = d.read.read_i32::<LE>()?;

    Ok(RigidBodyReader {
      settings: d.settings,
      count,
      remaining: count,
      read: d.read,
      poison: false,
    })
  }

  pub fn next<C: Config>(&mut self) -> Result<Option<RigidBody<C>>> {
    assert!(!self.poison);
    let result = self.next_impl::<C>();
    if result.is_err() {
      self.poison = true;
    }
    result
  }

  fn next_impl<C: Config>(&mut self) -> Result<Option<RigidBody<C>>> {
    if self.remaining <= 0 {
      return Ok(None);
    }

    self.remaining -= 1;

    Ok(Some(RigidBody {
      local_name: self.read.read_text(self.settings.text_encoding)?,
      universal_name: self.read.read_text(self.settings.text_encoding)?,
      bone_index: self.read.read_index(self.settings.bone_index_size)?,
      group_id: self.read.read_u8()?,
      non_collision_mask: self.read.read_u16::<LE>()?,
      shape: ShapeType::try_from(self.read.read_u8()?)?,
      shape_size: self.read.read_vec3::<C>()?,
      shape_position: self.read.read_vec3::<C>()?,
      shape_rotation: self.read.read_vec3::<C>()?,
      mass: self.read.read_f32::<LE>()?,
      move_attenuation: self.read.read_f32::<LE>()?,
      rotation_damping: self.read.read_f32::<LE>()?,
      repulsion: self.read.read_f32::<LE>()?,
      fiction: self.read.read_f32::<LE>()?,
      physics_mode: PhysicsMode::try_from(self.read.read_u8()?)?,
    }))
  }

  pub fn iter<C>(&mut self) -> RigidBodyIterator<R, C> {
    RigidBodyIterator {
      reader: self,
      phantom: PhantomData,
    }
  }
}

pub struct RigidBodyIterator<'a, R, C> {
  reader: &'a mut RigidBodyReader<R>,
  phantom: PhantomData<C>,
}

impl<R: Read, C: Config> Iterator for RigidBodyIterator<'_, R, C> {
  type Item = Result<RigidBody<C>>;

  fn next(&mut self) -> Option<Self::Item> {
    self
      .reader
      .next()
      .map_or_else(|e| Some(Err(e)), |v| v.map(Ok))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (
      self.reader.remaining as usize,
      Some(self.reader.remaining as usize),
    )
  }
}

impl<R: Read, C: Config> ExactSizeIterator for RigidBodyIterator<'_, R, C> {
  fn len(&self) -> usize {
    self.reader.remaining as usize
  }
}
