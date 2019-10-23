use byteorder::{ReadBytesExt, LE};
use encoding::all::{UTF_8, UTF_16LE};
use encoding::{Encoding, DecoderTrap};
use crate::pmx::types::*;
use crate::pmx::Error;
use std::io::Read;
use std::convert::TryFrom;

pub(crate) trait ReadHelpers: Read {
  fn read_text(&mut self, encoding: TextEncoding) -> Result<String, Error> {
    let size = self.read_i32::<LE>()?;
    let mut buf = Vec::with_capacity(size as usize);
    buf.resize(size as usize, 0u8);
    self.read_exact(&mut buf)?;
    (match encoding {
      TextEncoding::UTF8 => UTF_8.decode(&buf, DecoderTrap::Strict),
      TextEncoding::UTF16LE => UTF_16LE.decode(&buf, DecoderTrap::Strict)
    }).map_err(|e| Error::DecodeText(e))
  }

  fn read_vec2(&mut self) -> Result<[f32; 2], Error> {
    Ok([self.read_f32::<LE>()?, self.read_f32::<LE>()?])
  }

  fn read_vec3(&mut self) -> Result<[f32; 3], Error> {
    Ok([self.read_f32::<LE>()?, self.read_f32::<LE>()?, self.read_f32::<LE>()?])
  }

  fn read_vec4(&mut self) -> Result<[f32; 4], Error> {
    Ok([self.read_f32::<LE>()?, self.read_f32::<LE>()?, self.read_f32::<LE>()?, self.read_f32::<LE>()?])
  }

  fn read_index<I: TryFrom<i8> + TryFrom<i16> + TryFrom<i32>>
  (&mut self, size: IndexSize) -> Result<I, Error>
  {
    match size {
      IndexSize::I8 => I::try_from(self.read_i8()?).map_err(|_| Error::IndexOverflow),
      IndexSize::I16 => I::try_from(self.read_i16::<LE>()?).map_err(|_| Error::IndexOverflow),
      IndexSize::I32 => I::try_from(self.read_i32::<LE>()?).map_err(|_| Error::IndexOverflow)
    }
  }

  fn read_vertex_index<I: TryFrom<u8> + TryFrom<u16> + TryFrom<i32>>
  (&mut self, size: IndexSize) -> Result<I, Error> {
    match size {
      IndexSize::I8 => I::try_from(self.read_u8()?).map_err(|_| Error::IndexOverflow),
      IndexSize::I16 => I::try_from(self.read_u16::<LE>()?).map_err(|_| Error::IndexOverflow),
      IndexSize::I32 => I::try_from(self.read_i32::<LE>()?).map_err(|_| Error::IndexOverflow)
    }
  }
}

impl<R: Read> ReadHelpers for R {}