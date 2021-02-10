use arrayvec::ArrayVec;

use crate::Error;
use std::fmt::{Display, Formatter};
use std::{convert::TryFrom, fmt::Debug, iter::FromIterator};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[repr(u8)]
pub enum TextEncoding {
  UTF16LE = 0,
  UTF8 = 1,
}

impl Display for TextEncoding {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    match self {
      TextEncoding::UTF16LE => write!(f, "utf16 le"),
      TextEncoding::UTF8 => write!(f, "utf8"),
    }
  }
}

impl TryFrom<u8> for TextEncoding {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(TextEncoding::UTF16LE),
      1 => Ok(TextEncoding::UTF8),
      e => Err(Error::UnknownTextEncoding(e)),
    }
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[repr(u8)]
pub enum IndexSize {
  I8 = 1,
  I16 = 2,
  I32 = 4,
}

impl Display for IndexSize {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    match self {
      IndexSize::I8 => write!(f, "8-bit"),
      IndexSize::I16 => write!(f, "16-bit"),
      IndexSize::I32 => write!(f, "32-bit"),
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
      e => Err(Error::UnknownIndexSize(e)),
    }
  }
}

pub trait Index: TryFrom<i8> + TryFrom<i16> + TryFrom<i32> + Clone + Debug + Eq {}
impl<I: TryFrom<i8> + TryFrom<i16> + TryFrom<i32> + Clone + Debug + Eq> Index for I {}

pub trait VertexIndex: TryFrom<u8> + TryFrom<u16> + TryFrom<i32> + Clone + Debug + Eq {}
impl<I: TryFrom<u8> + TryFrom<u16> + TryFrom<i32> + Clone + Debug + Eq> VertexIndex for I {}

pub trait Config {
  type VertexIndex: VertexIndex;
  type TextureIndex: Index;
  type MaterialIndex: Index;
  type BoneIndex: Index;
  type MorphIndex: Index;
  type RigidbodyIndex: Index;

  type Vec2: From<[f32; 2]> + Clone + Debug + PartialEq;
  type Vec3: From<[f32; 3]> + Clone + Debug + PartialEq;
  type Vec4: From<[f32; 4]> + Clone + Debug + PartialEq;
  type AdditionalVec4s: FromIterator<Self::Vec4> + Clone + Debug + PartialEq;
}

pub struct DefaultConfig;

impl Config for DefaultConfig {
  type VertexIndex = i32;
  type TextureIndex = i32;
  type MaterialIndex = i32;
  type BoneIndex = i32;
  type MorphIndex = i32;
  type RigidbodyIndex = i32;

  #[cfg(feature = "vek")]
  type Vec2 = vek::Vec2<f32>;
  #[cfg(not(feature = "vek"))]
  type Vec2 = [f32; 2];

  #[cfg(feature = "vek")]
  type Vec3 = vek::Vec3<f32>;
  #[cfg(not(feature = "vek"))]
  type Vec3 = [f32; 3];

  #[cfg(feature = "vek")]
  type Vec4 = vek::Vec4<f32>;
  #[cfg(not(feature = "vek"))]
  type Vec4 = [f32; 4];

  #[cfg(feature = "arrayvec")]
  type AdditionalVec4s = ArrayVec<[Self::Vec4; 4]>;
  #[cfg(not(feature = "arrayvec"))]
  type AdditionalVec4s = Vec<Self::Vec4>;
}
