use crate::{Config, Error};
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JointType {
  SpringFree = 0,
  Free = 1,
  P2P = 2,
  ConeTwist = 3,
  Slider = 4,
  Hinge = 5,
}

impl Display for JointType {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    match self {
      JointType::SpringFree => write!(f, "free (spring)"),
      JointType::Free => write!(f, "free"),
      JointType::P2P => write!(f, "p2p"),
      JointType::ConeTwist => write!(f, "cone twist"),
      JointType::Slider => write!(f, "slider"),
      JointType::Hinge => write!(f, "hinge"),
    }
  }
}

impl TryFrom<u8> for JointType {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    Ok(match value {
      0 => JointType::SpringFree,
      1 => JointType::Free,
      2 => JointType::P2P,
      3 => JointType::ConeTwist,
      4 => JointType::Slider,
      5 => JointType::Hinge,
      e => return Err(Error::InvalidJointType(e)),
    })
  }
}

pub struct Joint<C: Config> {
  pub local_name: String,
  pub universal_name: String,
  pub joint_type: JointType,
  pub rigid_body_a: C::RigidbodyIndex,
  pub rigid_body_b: C::RigidbodyIndex,
  pub position: C::Vec3,
  pub rotation: C::Vec3,
  pub position_min: C::Vec3,
  pub position_max: C::Vec3,
  pub rotation_min: C::Vec3,
  pub rotation_max: C::Vec3,
  pub position_spring: C::Vec3,
  pub rotation_spring: C::Vec3,
}

impl<C: Config> Display for Joint<C>
where
  C::RigidbodyIndex: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(
      f,
      r"local name: {}, universal name: {},
type: {}, rigid body a: {}, rigid body b: {},
position: {:?}, rotation: {:?},
position min: {:?}, position max: {:?},
rotation min: {:?}, rotation max: {:?},
position spring: {:?}, rotation spring: {:?}",
      self.local_name,
      self.universal_name,
      self.joint_type,
      self.rigid_body_a,
      self.rigid_body_b,
      self.position,
      self.rotation,
      self.position_min,
      self.position_max,
      self.rotation_min,
      self.rotation_max,
      self.position_spring,
      self.rotation_spring,
    )
  }
}
