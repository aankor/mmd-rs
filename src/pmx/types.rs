use crate::pmx::Error;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[repr(u8)]
pub enum TextEncoding {
  UTF16LE = 0,
  UTF8 = 1
}

impl Display for TextEncoding {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    match self {
      TextEncoding::UTF16LE => write!(f, "utf16 le"),
      TextEncoding::UTF8 => write!(f, "utf8")
    }
  }
}

impl TryFrom<u8> for TextEncoding {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(TextEncoding::UTF16LE),
      1 => Ok(TextEncoding::UTF8),
      e => Err(Error::UnknownTextEncoding(e))
    }
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[repr(u8)]
pub enum IndexSize {
  I8 = 1,
  I16 = 2,
  I32 = 4
}

impl Display for IndexSize {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    match self {
      IndexSize::I8 => write!(f, "8-bit"),
      IndexSize::I16 => write!(f, "16-bit"),
      IndexSize::I32 => write!(f, "32-bit")
    }
  }
}

impl TryFrom<u8> for IndexSize {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      1 => Ok(IndexSize::I8),
      2 => Ok(IndexSize::I16),
      4 => Ok(IndexSize::I32),
      e => Err(Error::UnknownIndexSize(e))
    }
  }
}