use crate::{
  pmx::material::*, reader::helpers::ReadHelpers, reader::TextureReader, Error, Result, Settings,
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
}

impl<R: Read> MaterialReader<R> {
  pub fn new(mut v: TextureReader<R>) -> Result<MaterialReader<R>> {
    while v.remaining > 0 {
      v.next()?;
    }
    let count = v.read.read_i32::<LE>()?;

    Ok(MaterialReader {
      settings: v.settings,
      count,
      remaining: count,
      read: v.read,
    })
  }

  pub fn next<I: TryFrom<i8> + TryFrom<i16> + TryFrom<i32>>(
    &mut self,
  ) -> Result<Option<Material<I>>> {
    if self.remaining <= 0 {
      return Ok(None);
    }

    self.remaining -= 1;

    Ok(Some(Material {
      local_name: self.read.read_text(self.settings.text_encoding)?,
      universal_name: self.read.read_text(self.settings.text_encoding)?,
      diffuse_color: self.read.read_vec4()?,
      specular_color: self.read.read_vec3()?,
      specular_strength: self.read.read_f32::<LE>()?,
      ambient_color: self.read.read_vec3()?,
      draw_flags: BitFlags::from_bits(self.read.read_u8()?).unwrap(),
      edge_color: self.read.read_vec4()?,
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

  pub fn iter<I>(&mut self) -> MaterialIterator<R, I> {
    MaterialIterator {
      reader: self,
      phantom: PhantomData,
    }
  }
}

pub struct MaterialIterator<'a, R, I = i32> {
  reader: &'a mut MaterialReader<R>,
  phantom: PhantomData<I>,
}

impl<R: Read, I: TryFrom<i8> + TryFrom<i16> + TryFrom<i32>> Iterator
  for MaterialIterator<'_, R, I>
{
  type Item = Result<Material<I>>;

  fn next(&mut self) -> Option<Self::Item> {
    self.reader.next().map_or(None, |v| v.map(Ok))
  }
}
