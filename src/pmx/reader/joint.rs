use crate::{
  pmx::joint::*,
  reader::{helpers::ReadHelpers, RigidBodyReader},
  Config, DefaultConfig, Result, Settings,
};
use byteorder::{ReadBytesExt, LE};
use std::convert::TryFrom;
use std::io::Read;
use std::marker::PhantomData;

pub struct JointReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R,
  pub(crate) poison: bool,
}

impl<R: Read> JointReader<R> {
  pub fn new(mut r: RigidBodyReader<R>) -> Result<JointReader<R>> {
    assert!(!r.poison);
    while r.remaining > 0 {
      r.next::<DefaultConfig>()?;
    }
    let count = r.read.read_i32::<LE>()?;

    Ok(JointReader {
      settings: r.settings,
      count,
      remaining: count,
      read: r.read,
      poison: false,
    })
  }

  pub fn next<C: Config>(&mut self) -> Result<Option<Joint<C>>> {
    assert!(!self.poison);
    let result = self.next_impl::<C>();
    if result.is_err() {
      self.poison = true;
    }
    result
  }

  fn next_impl<C: Config>(&mut self) -> Result<Option<Joint<C>>> {
    if self.remaining <= 0 {
      return Ok(None);
    }

    self.remaining -= 1;

    Ok(Some(Joint {
      local_name: self.read.read_text(self.settings.text_encoding)?,
      universal_name: self.read.read_text(self.settings.text_encoding)?,
      joint_type: JointType::try_from(self.read.read_u8()?)?,
      rigid_body_a: self.read.read_index(self.settings.rigidbody_index_size)?,
      rigid_body_b: self.read.read_index(self.settings.rigidbody_index_size)?,
      position: self.read.read_vec3::<C>()?,
      rotation: self.read.read_vec3::<C>()?,
      position_min: self.read.read_vec3::<C>()?,
      position_max: self.read.read_vec3::<C>()?,
      rotation_min: self.read.read_vec3::<C>()?,
      rotation_max: self.read.read_vec3::<C>()?,
      position_spring: self.read.read_vec3::<C>()?,
      rotation_spring: self.read.read_vec3::<C>()?,
    }))
  }

  pub fn iter<C>(&mut self) -> JointIterator<R, C> {
    JointIterator {
      reader: self,
      phantom: PhantomData,
    }
  }
}

pub struct JointIterator<'a, R, C> {
  reader: &'a mut JointReader<R>,
  phantom: PhantomData<C>,
}

impl<R: Read, C: Config> Iterator for JointIterator<'_, R, C> {
  type Item = Result<Joint<C>>;

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

impl<R: Read, C: Config> ExactSizeIterator for JointIterator<'_, R, C> {
  fn len(&self) -> usize {
    self.reader.remaining as usize
  }
}
