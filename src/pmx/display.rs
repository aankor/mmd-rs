use crate::Config;
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Frame<C: Config> {
  Bone(C::BoneIndex),
  Morph(C::MorphIndex),
}

impl<C: Config> Display for Frame<C>
where
  C::BoneIndex: Display,
  C::MorphIndex: Display,
{
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    match self {
      Frame::Bone(id) => write!(f, "bone {}", id),
      Frame::Morph(id) => write!(f, "morph {}", id),
    }
  }
}

pub struct DisplayFrame<C: Config> {
  pub local_name: String,
  pub universal_name: String,
  pub special_flag: bool,
  pub frames: Vec<Frame<C>>,
}

impl<C: Config> Display for DisplayFrame<C>
where
  Frame<C>: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(
      f,
      r"local name: {}, universal name: {},
flag: {}, frames: {}",
      self.local_name,
      self.universal_name,
      if self.special_flag {
        "special"
      } else {
        "normal"
      },
      self.frames.iter().map(ToString::to_string).join(", "),
    )
  }
}
