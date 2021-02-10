use crate::{
  pmx::weight_deform::*,
  reader::{helpers::ReadHelpers, HeaderReader},
  Config, DefaultConfig, Error, Result, Settings, Vertex,
};
use byteorder::{ReadBytesExt, LE};
use std::io::Read;
use std::marker::PhantomData;

pub struct VertexReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R,
  pub(crate) poison: bool,
}

impl<R: Read> VertexReader<R> {
  pub fn new(mut header: HeaderReader<R>) -> Result<VertexReader<R>> {
    let count = header.read.read_i32::<LE>()?;
    Ok(VertexReader {
      settings: header.settings,
      count,
      remaining: count,
      read: header.read,
      poison: false,
    })
  }

  pub fn next<C: Config>(&mut self) -> Result<Option<Vertex<C>>> {
    assert!(!self.poison);
    let result = self.next_impl::<C>();
    if result.is_err() {
      self.poison = true;
    }
    result
  }

  fn next_impl<C: Config>(&mut self) -> Result<Option<Vertex<C>>> {
    if self.remaining == 0 {
      return Ok(None);
    }
    let position = self.read.read_vec3::<C>()?;
    let normal = self.read.read_vec3::<C>()?;
    let uv = self.read.read_vec2::<C>()?;
    let additional = (0..self.settings.additional_vec4_count)
      .map(|_| self.read.read_vec4::<C>())
      .collect::<Result<C::AdditionalVec4s>>()?;

    let weight_deform = match self.read.read_u8()? {
      0u8 => WeightDeform::Bdef1(Bdef1 {
        bone_index: self.read.read_index(self.settings.bone_index_size)?,
      }),
      1u8 => WeightDeform::Bdef2(Bdef2 {
        bone_1_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_2_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_1_weight: self.read.read_f32::<LE>()?,
      }),
      2u8 => WeightDeform::Bdef4(Bdef4 {
        bone_1_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_2_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_3_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_4_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_1_weight: self.read.read_f32::<LE>()?,
        bone_2_weight: self.read.read_f32::<LE>()?,
        bone_3_weight: self.read.read_f32::<LE>()?,
        bone_4_weight: self.read.read_f32::<LE>()?,
      }),
      3u8 => WeightDeform::Sdef(Sdef {
        bone_1_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_2_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_1_weight: self.read.read_f32::<LE>()?,
        c: self.read.read_vec3::<C>()?,
        r0: self.read.read_vec3::<C>()?,
        r1: self.read.read_vec3::<C>()?,
      }),
      4u8 => WeightDeform::Qdef(Qdef {
        bone_1_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_2_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_3_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_4_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_1_weight: self.read.read_f32::<LE>()?,
        bone_2_weight: self.read.read_f32::<LE>()?,
        bone_3_weight: self.read.read_f32::<LE>()?,
        bone_4_weight: self.read.read_f32::<LE>()?,
      }),
      e => return Err(Error::UnknownWeightType(e)),
    };

    self.remaining -= 1;
    Ok(Some(Vertex {
      position,
      normal,
      uv,
      additional,
      weight_deform,
      edge_scale: self.read.read_f32::<LE>()?,
    }))
  }

  pub fn iter<C>(&mut self) -> VertexIterator<R, C> {
    VertexIterator {
      reader: self,
      phantom: PhantomData,
    }
  }
}

pub struct VertexIterator<'a, R, C = DefaultConfig> {
  reader: &'a mut VertexReader<R>,
  phantom: PhantomData<C>,
}

impl<'a, R: Read, C: Config> Iterator for VertexIterator<'a, R, C> {
  type Item = Result<Vertex<C>>;

  fn next(&mut self) -> Option<Self::Item> {
    self
      .reader
      .next()
      .map_or_else(|e| Some(Err(e)), |v| v.map(Ok))
  }
}
