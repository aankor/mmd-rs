use crate::{pmx::types::*, Error, Result};
use byteorder::{ReadBytesExt, LE};
use encoding::all::{UTF_16LE, UTF_8};
use encoding::{DecoderTrap, Encoding};
use std::convert::TryFrom;
use std::io::Read;

pub(crate) trait ReadHelpers: Read {
  fn read_text(&mut self, encoding: TextEncoding) -> Result<String> {
    let size = self.read_i32::<LE>()?;
    let mut buf = Vec::with_capacity(size as usize);
    buf.resize(size as usize, 0u8);
    self.read_exact(&mut buf)?;
    (match encoding {
      TextEncoding::UTF8 => UTF_8.decode(&buf, DecoderTrap::Strict),
      TextEncoding::UTF16LE => UTF_16LE.decode(&buf, DecoderTrap::Strict),
    })
    .map_err(|e| Error::DecodeText(e))
  }

  fn read_vec2(&mut self) -> Result<[f32; 2]> {
    Ok([self.read_f32::<LE>()?, self.read_f32::<LE>()?])
  }

  fn read_vec3(&mut self) -> Result<[f32; 3]> {
    Ok([
      self.read_f32::<LE>()?,
      self.read_f32::<LE>()?,
      self.read_f32::<LE>()?,
    ])
  }

  fn read_vec4(&mut self) -> Result<[f32; 4]> {
    Ok([
      self.read_f32::<LE>()?,
      self.read_f32::<LE>()?,
      self.read_f32::<LE>()?,
      self.read_f32::<LE>()?,
    ])
  }

  fn read_index<I: Index>(&mut self, size: IndexSize) -> Result<I> {
    match size {
      IndexSize::I8 => I::try_from(self.read_i8()?).map_err(|_| Error::IndexOverflow),
      IndexSize::I16 => I::try_from(self.read_i16::<LE>()?).map_err(|_| Error::IndexOverflow),
      IndexSize::I32 => I::try_from(self.read_i32::<LE>()?).map_err(|_| Error::IndexOverflow),
    }
  }

  fn read_vertex_index<I: VertexIndex>(&mut self, size: IndexSize) -> Result<I> {
    match size {
      IndexSize::I8 => I::try_from(self.read_u8()?).map_err(|_| Error::IndexOverflow),
      IndexSize::I16 => I::try_from(self.read_u16::<LE>()?).map_err(|_| Error::IndexOverflow),
      IndexSize::I32 => I::try_from(self.read_i32::<LE>()?).map_err(|_| Error::IndexOverflow),
    }
  }
}

impl<R: Read> ReadHelpers for R {}
