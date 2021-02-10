use crate::{
  pmx::material::*,
  reader::{helpers::ReadHelpers, TextureReader},
  Config, DefaultConfig, Error, Result, Settings,
};
use byteorder::{ReadBytesExt, LE};
use enumflags2::BitFlags;
use std::convert::TryFrom;
use std::io::Read;
use std::marker::PhantomData;

pub struct MaterialReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R,
  pub(crate) poison: bool,
}

impl<R: Read> MaterialReader<R> {
  pub fn new(mut t: TextureReader<R>) -> Result<MaterialReader<R>> {
    assert!(!t.poison);
    while t.remaining > 0 {
      t.next()?;
    }
    let count = t.read.read_i32::<LE>()?;

    Ok(MaterialReader {
      settings: t.settings,
      count,
      remaining: count,
      read: t.read,
      poison: false,
    })
  }

  pub fn next<C: Config>(&mut self) -> Result<Option<Material<C>>> {
    assert!(!self.poison);
    let result = self.next_impl::<C>();
    if result.is_err() {
      self.poison = true;
    }
    result
  }

  fn next_impl<C: Config>(&mut self) -> Result<Option<Material<C>>> {
    if self.remaining <= 0 {
      return Ok(None);
    }

    self.remaining -= 1;

    Ok(Some(Material {
      local_name: self.read.read_text(self.settings.text_encoding)?,
      universal_name: self.read.read_text(self.settings.text_encoding)?,
      diffuse_color: self.read.read_vec4::<C>()?,
      specular_color: self.read.read_vec3::<C>()?,
      specular_strength: self.read.read_f32::<LE>()?,
      ambient_color: self.read.read_vec3::<C>()?,
      draw_flags: BitFlags::from_bits(self.read.read_u8()?).unwrap(),
      edge_color: self.read.read_vec4::<C>()?,
      edge_scale: self.read.read_f32::<LE>()?,
      texture_index: self.read.read_index(self.settings.texture_index_size)?,
      environment_index: self.read.read_index(self.settings.texture_index_size)?,
      environment_blend_mode: EnvironmentBlendMode::try_from(self.read.read_u8()?)?,
      toon: match self.read.read_u8()? {
        0 => Toon::Texture(self.read.read_index(self.settings.texture_index_size)?),
        1 => Toon::Internal(self.read.read_u8()?),
        e => return Err(Error::InvalidToonReference(e)),
      },
      metadata: self.read.read_text(self.settings.text_encoding)?,
      surface_count: self.read.read_i32::<LE>()?,
    }))
  }

  pub fn iter<C>(&mut self) -> MaterialIterator<R, C> {
    MaterialIterator {
      reader: self,
      phantom: PhantomData,
    }
  }
}

pub struct MaterialIterator<'a, R, C = DefaultConfig> {
  reader: &'a mut MaterialReader<R>,
  phantom: PhantomData<C>,
}

impl<R: Read, C: Config> Iterator for MaterialIterator<'_, R, C> {
  type Item = Result<Material<C>>;

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

impl<R: Read, C: Config> ExactSizeIterator for MaterialIterator<'_, R, C> {
  fn len(&self) -> usize {
    self.reader.remaining as usize
  }
}
