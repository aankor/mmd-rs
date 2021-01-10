use enumflags2::BitFlags;
use std::fmt::{Display, Formatter, Debug};
use itertools::Itertools;

#[derive(BitFlags, Copy, Clone, PartialEq, Debug)]
#[repr(u16)]
pub enum BoneFlags {
  Connection              = 0b0000_0000_0000_0001,
  Rotatable               = 0b0000_0000_0000_0010,
  Movable                 = 0b0000_0000_0000_0100,
  Display                 = 0b0000_0000_0000_1000,
  CanOperate              = 0b0000_0000_0001_0000,
  InverseKinematics       = 0b0000_0000_0010_0000,
  Unknown6                = 0b0000_0000_0100_0000,
  AddLocalDeform          = 0b0000_0000_1000_0000,
  AddRotation             = 0b0000_0001_0000_0000,
  AddMovement             = 0b0000_0010_0000_0000,
  FixedAxis               = 0b0000_0100_0000_0000,
  LocalAxis               = 0b0000_1000_0000_0000,
  PhysicalTransform       = 0b0001_0000_0000_0000,
  ExternalParentTransform = 0b0010_0000_0000_0000
}

struct BoneFlagsFmt(BitFlags<BoneFlags>);

impl Display for BoneFlagsFmt {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "{}", self.0.iter().map(|v| format!("{:?}", v)).join("|"))
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Connection<I = i32> {
  Index(I),
  Position([f32; 3]),
}

impl<I: Display> Display for Connection<I> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    match self {
      Connection::Index(t) => write!(f, "index({})", t),
      Connection::Position(i) => write!(f, "offset({:?})", i)
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Additional<I = i32> {
  pub parent: I,
  pub rate: f32,
}

impl<I: Display> Display for Additional<I> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "{} at rate {}", self.parent, self.rate)
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LocalAxis {
  pub x: [f32; 3],
  pub z: [f32; 3],
}

impl Display for LocalAxis {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "x: {:?} z: {:?}", self.x, self.z)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InverseKinematics<I = i32> {
  pub ik_bone: I,
  pub iterations: u32,
  pub limit_angle: f32,
  pub links: Vec<IKLink>
}

impl<I: Display> Display for InverseKinematics<I> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(f,
           "index: {} iterations: {} limit angle: {}\n{}",
           self.ik_bone, self.iterations, self.limit_angle,
           self.links.iter().map(ToString::to_string).join("\n"))
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IKLink<I = i32> {
  pub ik_bone: I,
  pub limits: Option<([f32; 3], [f32; 3])>,
}

impl Display for IKLink {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "link: {} limits: {:?}", self.ik_bone, self.limits)
  }
}

pub struct Bone<I = i32> {
  pub local_name: String,
  pub universal_name: String,
  pub position: [f32; 3],
  pub parent: I,
  pub transform_level: i32,
  pub bone_flags: BitFlags<BoneFlags>,
  pub connection: Connection<I>,
  pub additional: Option<Additional<I>>,
  pub fixed_axis: Option<[f32; 3]>,
  pub local_axis: Option<LocalAxis>,
  pub external_parent_transform: Option<i32>,
  pub inverse_kinematics: Option<InverseKinematics<I>>,
}

impl<I: Display> Display for Bone<I> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f,
           r"local name: {}, universal name: {},
position: {:?}, parent: {}, transform level: {},
flags: {},
connection: {}, additional: {}, fixed axis: {:?}, local axis {}, parent transform: {},
inverse kinematics: {}",
           self.local_name, self.universal_name,
           self.position, self.parent, self.transform_level,
           BoneFlagsFmt(self.bone_flags),
           self.connection, print_option(&self.additional), self.fixed_axis, print_option(&self.local_axis), print_option(&self.external_parent_transform),
           print_option(&self.inverse_kinematics))
  }
}

fn print_option<T: Display>(val: &Option<T>) -> String {
  val.as_ref().map(ToString::to_string).unwrap_or_else(|| "None".to_string())
}
