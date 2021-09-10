use crate::{Config, Error};
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ShapeType {
  Sphere = 0,
  Box = 1,
  Capsule = 2,
}

impl Display for ShapeType {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    match self {
      ShapeType::Sphere => write!(f, "sphere"),
      ShapeType::Box => write!(f, "box"),
      ShapeType::Capsule => write!(f, "capsule"),
    }
  }
}

impl TryFrom<u8> for ShapeType {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    Ok(match value {
      0 => ShapeType::Sphere,
      1 => ShapeType::Box,
      2 => ShapeType::Capsule,
      e => return Err(Error::InvalidShapeType(e)),
    })
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PhysicsMode {
  Static = 0,
  Dynamic = 1,
  DynamicPivoted = 2,
}

impl Display for PhysicsMode {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    match self {
      PhysicsMode::Static => write!(f, "static"),
      PhysicsMode::Dynamic => write!(f, "dynamic"),
      PhysicsMode::DynamicPivoted => write!(f, "dynamic pivoted"),
    }
  }
}

impl TryFrom<u8> for PhysicsMode {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    Ok(match value {
      0 => PhysicsMode::Static,
      1 => PhysicsMode::Dynamic,
      2 => PhysicsMode::DynamicPivoted,
      e => return Err(Error::InvalidPhysicsMode(e)),
    })
  }
}

pub struct RigidBody<C: Config> {
  pub local_name: String,
  pub universal_name: String,
  pub bone_index: C::BoneIndex,
  pub group_id: u8,
  pub non_collision_mask: u16,
  pub shape: ShapeType,
  pub shape_size: C::Vec3,
  pub shape_position: C::Vec3,
  pub shape_rotation: C::Vec3,
  pub mass: f32,
  pub move_attenuation: f32,
  pub rotation_damping: f32,
  pub repulsion: f32,
  pub fiction: f32,
  pub physics_mode: PhysicsMode,
}

impl<C: Config> Display for RigidBody<C>
where
  C::BoneIndex: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(
      f,
      r"local name: {}, universal name: {},
bone index: {}, group id: {}, non_collision_mask: 0b{:b},
shape: {}, size: {:?},
pos: {:?}, rot: {:?},
mass: {}, move attenuation: {}, rotaton damping: {}
repulsion: {}, fiction: {}, physics: {}",
      self.local_name,
      self.universal_name,
      self.bone_index,
      self.group_id,
      self.non_collision_mask,
      self.shape,
      self.shape_size,
      self.shape_position,
      self.shape_rotation,
      self.mass,
      self.move_attenuation,
      self.rotation_damping,
      self.repulsion,
      self.fiction,
      self.physics_mode,
    )
  }
}
