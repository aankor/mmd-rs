use err_derive::Error;
use std::borrow::Cow;

#[derive(Debug, Error)]
pub enum Error {
  #[error(display = "{}", _0)]
  Io(#[error(source)] std::io::Error),
  #[error(display = "Wrong signature {:?}", _0)]
  WrongSignature([u8; 4]),
  #[error(display = "Globals count less than 8 {}", _0)]
  GlobalsCountLessThan8(u8),
  #[error(display = "Unknown index size {}", _0)]
  UnknownIndexSize(u8),
  #[error(display = "Unknown text encoding {}", _0)]
  UnknownTextEncoding(u8),
  #[error(display = "Decode text {}", _0)]
  DecodeText(Cow<'static, str>),
  #[error(display = "Unknown weigh type {}", _0)]
  UnknownWeightType(u8),
  #[error(display = "Index overflow {}", _0)]
  IndexOverflow(i64),
  #[error(display = "Invalid environment blendMode {}", _0)]
  InvalidEnvironmentBlendMode(u8),
  #[error(display = "Invalid toon reference {}", _0)]
  InvalidToonReference(u8),
  #[error(display = "Invalid morph type {}", _0)]
  InvalidMorphType(u8),
  #[error(display = "Invalid material offset method {}", _0)]
  InvalidMaterialOffsetMethod(u8),
  #[error(display = "Invalid display frame type {}", _0)]
  InvalidFrameType(u8),
}

pub type Result<T> = std::result::Result<T, Error>;
