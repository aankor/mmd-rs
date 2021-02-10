use enumflags2::BitFlags;
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter};

use crate::{display::DisplayOption, Config};

#[derive(BitFlags, Copy, Clone, PartialEq, Debug)]
#[repr(u16)]
pub enum BoneFlags {
  Connection = 0b0000_0000_0000_0001,
  Rotatable = 0b0000_0000_0000_0010,
  Movable = 0b0000_0000_0000_0100,
  Display = 0b0000_0000_0000_1000,
  CanOperate = 0b0000_0000_0001_0000,
  InverseKinematics = 0b0000_0000_0010_0000,
  Unknown6 = 0b0000_0000_0100_0000,
  AddLocalDeform = 0b0000_0000_1000_0000,
  AddRotation = 0b0000_0001_0000_0000,
  AddMovement = 0b0000_0010_0000_0000,
  FixedAxis = 0b0000_0100_0000_0000,
  LocalAxis = 0b0000_1000_0000_0000,
  PhysicalTransform = 0b0001_0000_0000_0000,
  ExternalParentTransform = 0b0010_0000_0000_0000,
}

struct BoneFlagsFmt(BitFlags<BoneFlags>);

impl Display for BoneFlagsFmt {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "{}", self.0.iter().map(|v| format!("{:?}", v)).join("|"))
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Connection<C: Config> {
  Index(C::BoneIndex),
  Position(C::Vec3),
}

impl<C: Config> Display for Connection<C>
where
  C::BoneIndex: Display,
  C::Vec3: Display,
{
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    match self {
      Connection::Index(t) => write!(f, "index({})", t),
      Connection::Position(i) => write!(f, "offset({})", i),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Additional<C: Config> {
  pub parent: C::BoneIndex,
  pub rate: f32,
}

impl<C: Config> Display for Additional<C>
where
  C::BoneIndex: Display,
{
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "{} at rate {}", self.parent, self.rate)
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LocalAxis<C: Config> {
  pub x: C::Vec3,
  pub z: C::Vec3,
}

impl<C: Config> Display for LocalAxis<C>
where
  C::Vec3: Display,
{
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "x: {} z: {}", self.x, self.z)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InverseKinematics<C: Config> {
  pub ik_bone: C::BoneIndex,
  pub iterations: u32,
  pub limit_angle: f32,
  pub links: Vec<IKLink<C>>,
}

impl<C: Config> Display for InverseKinematics<C>
where
  C::BoneIndex: Display,
  C::Vec3: Display,
{
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(
      f,
      "index: {} iterations: {} limit angle: {}\n{}",
      self.ik_bone,
      self.iterations,
      self.limit_angle,
      self.links.iter().map(ToString::to_string).join("\n")
    )
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IKLink<C: Config> {
  pub ik_bone: C::BoneIndex,
  pub limits: Option<(C::Vec3, C::Vec3)>,
}

impl<C: Config> Display for IKLink<C>
where
  C::BoneIndex: Display,
  C::Vec3: Display,
{
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "link: {} ", self.ik_bone,)?;
    if let Some((ref low, ref high)) = self.limits {
      write!(f, "limits: [{} - {}]", low, high)
    } else {
      write!(f, "unlimited")
    }
  }
}

pub struct Bone<C: Config> {
  pub local_name: String,
  pub universal_name: String,
  pub position: C::Vec3,
  pub parent: C::BoneIndex,
  pub transform_level: i32,
  pub bone_flags: BitFlags<BoneFlags>,
  pub connection: Connection<C>,
  pub additional: Option<Additional<C>>,
  pub fixed_axis: Option<C::Vec3>,
  pub local_axis: Option<LocalAxis<C>>,
  pub external_parent_transform: Option<i32>,
  pub inverse_kinematics: Option<InverseKinematics<C>>,
}

impl<C: Config> Display for Bone<C>
where
  C::BoneIndex: Display,
  C::Vec3: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(
      f,
      r"local name: {}, universal name: {},
position: {}, parent: {}, transform level: {},
flags: {},
connection: {}, additional: {}, fixed axis: {}, local axis {}, parent transform: {},
inverse kinematics: {}",
      self.local_name,
      self.universal_name,
      self.position,
      self.parent,
      self.transform_level,
      BoneFlagsFmt(self.bone_flags),
      self.connection,
      DisplayOption::new(&self.additional),
      DisplayOption::new(&self.fixed_axis),
      DisplayOption::new(&self.local_axis),
      DisplayOption::new(&self.external_parent_transform),
      DisplayOption::new(&self.inverse_kinematics)
    )
  }
}
