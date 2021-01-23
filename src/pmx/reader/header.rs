use crate::{pmx::types::*, reader::helpers::ReadHelpers, Error, Settings};
use byteorder::{ReadBytesExt, LE};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::io::Read;

pub struct HeaderReader<R> {
  pub version: f32,
  pub settings: Settings,
  pub model_local_name: String,
  pub model_universal_name: String,
  pub local_comments: String,
  pub universal_comments: String,
  pub(crate) read: R,
}

impl<R: Read> HeaderReader<R> {
  pub fn new(mut read: R) -> Result<HeaderReader<R>, Error> {
    let mut magic = [0u8; 4];
    read.read_exact(&mut magic)?;
    if magic != [0x50, 0x4D, 0x58, 0x20] {
      return Err(Error::WrongSignature(magic));
    }

    let version = read.read_f32::<LE>()?;
    let globals_count = read.read_u8()?;
    if globals_count < 8 {
      return Err(Error::GlobalsCountLessThan8(globals_count));
    }

    let mut globals = Vec::with_capacity(globals_count as usize);
    globals.resize(globals_count as usize, 0u8);
    read.read_exact(&mut globals)?;

    let settings = Settings {
      text_encoding: TextEncoding::try_from(globals[0])?,
      additional_vec4_count: globals[1],
      vertex_index_size: IndexSize::try_from(globals[2])?,
      texture_index_size: IndexSize::try_from(globals[3])?,
      material_index_size: IndexSize::try_from(globals[4])?,
      bone_index_size: IndexSize::try_from(globals[5])?,
      morph_index_size: IndexSize::try_from(globals[6])?,
      rigidbody_index_size: IndexSize::try_from(globals[7])?,
    };

    Ok(HeaderReader::<R> {
      version,
      settings,
      model_local_name: read.read_text(settings.text_encoding)?,
      model_universal_name: read.read_text(settings.text_encoding)?,
      local_comments: read.read_text(settings.text_encoding)?,
      universal_comments: read.read_text(settings.text_encoding)?,
      read,
    })
  }
}

impl<R> Display for HeaderReader<R> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "version: {}, ", self.version)?;
    self.settings.fmt(f)?;
    writeln!(
      f,
      "local name: {}, universal name: {}\nLocal comments\n{}\nUniversal comments\n{}",
      self.model_local_name,
      self.model_universal_name,
      self.local_comments,
      self.universal_comments
    )
  }
}
