use crate::{pmx::types::*, Error, Result};
use byteorder::{ReadBytesExt, LE};
use encoding::all::{UTF_16LE, UTF_8};
use encoding::{DecoderTrap, Encoding};
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

  fn read_vec2<C: Config>(&mut self) -> Result<C::Vec2> {
    Ok([self.read_f32::<LE>()?, self.read_f32::<LE>()?].into())
  }

  fn read_vec3<C: Config>(&mut self) -> Result<C::Vec3> {
    Ok(
      [
        self.read_f32::<LE>()?,
        self.read_f32::<LE>()?,
        self.read_f32::<LE>()?,
      ]
      .into(),
    )
  }

  fn read_vec4<C: Config>(&mut self) -> Result<C::Vec4> {
    Ok(
      [
        self.read_f32::<LE>()?,
        self.read_f32::<LE>()?,
        self.read_f32::<LE>()?,
        self.read_f32::<LE>()?,
      ]
      .into(),
    )
  }

  fn read_index<I: Index>(&mut self, size: IndexSize) -> Result<I> {
    match size {
      IndexSize::I8 => {
        let v = self.read_i8()?;
        I::try_from(v).map_err(|_| Error::IndexOverflow(v.into()))
      }
      IndexSize::I16 => {
        let v = self.read_i16::<LE>()?;
        I::try_from(v).map_err(|_| Error::IndexOverflow(v.into()))
      }
      IndexSize::I32 => {
        let v = self.read_i32::<LE>()?;
        I::try_from(v).map_err(|_| Error::IndexOverflow(v.into()))
      }
    }
  }

  fn read_vertex_index<I: VertexIndex>(&mut self, size: IndexSize) -> Result<I> {
    match size {
      IndexSize::I8 => {
        let v = self.read_u8()?;
        I::try_from(v).map_err(|_| Error::IndexOverflow(v.into()))
      }
      IndexSize::I16 => {
        let v = self.read_u16::<LE>()?;
        I::try_from(v).map_err(|_| Error::IndexOverflow(v.into()))
      }
      IndexSize::I32 => {
        let v = self.read_i32::<LE>()?;
        I::try_from(v).map_err(|_| Error::IndexOverflow(v.into()))
      }
    }
  }
}

impl<R: Read> ReadHelpers for R {}
