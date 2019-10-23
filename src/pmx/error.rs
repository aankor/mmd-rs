use std::borrow::Cow;

#[derive(Debug)]
pub enum Error {
  Io(std::io::Error),
  WrongSignature([u8; 4]),
  GlobalsCountLessThan8(u8),
  UnknownIndexSize(u8),
  UnknownTextEncoding(u8),
  DecodeText(Cow<'static, str>),
  UnknownWeightType(u8),
  IndexOverflow,
  InvalidEnvironmentBlendMode(u8),
  InvalidToonReference(u8)
}

impl From<std::io::Error> for Error {
  fn from(e: std::io::Error) -> Self {
    Self::Io(e)
  }
}
