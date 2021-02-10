use crate::{Config, Error};
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Panel {
  Hidden,
  Eyebrows,
  Eyes,
  Mouth,
  Other,
  Unknown(u8),
}

impl Display for Panel {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    match self {
      Panel::Hidden => write!(f, "hidden"),
      Panel::Eyebrows => write!(f, "eyebrows"),
      Panel::Eyes => write!(f, "eyes"),
      Panel::Mouth => write!(f, "mouth"),
      Panel::Other => write!(f, "other"),
      Panel::Unknown(panel) => write!(f, "unknown({})", panel),
    }
  }
}

impl From<u8> for Panel {
  fn from(value: u8) -> Self {
    match value {
      0 => Panel::Hidden,
      1 => Panel::Eyebrows,
      2 => Panel::Eyes,
      3 => Panel::Mouth,
      4 => Panel::Other,
      panel => Panel::Unknown(panel),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GroupOffset<C: Config> {
  pub morph: C::MorphIndex,
  pub influence: f32,
}

impl<C: Config> Display for GroupOffset<C>
where
  C::MorphIndex: Display,
{
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "{} at rate {}", self.morph, self.influence)
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VertexOffset<C: Config> {
  pub vertex: C::VertexIndex,
  pub offset: C::Vec3,
}

impl<C: Config> Display for VertexOffset<C>
where
  C::VertexIndex: Display,
  C::Vec3: Display,
{
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "{} offset {}", self.vertex, self.offset)
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BoneOffset<C: Config> {
  pub bone: C::BoneIndex,
  pub translation: C::Vec3,
  pub rotation: C::Vec4,
}

impl<C: Config> Display for BoneOffset<C>
where
  C::BoneIndex: Display,
  C::Vec3: Display,
  C::Vec4: Display,
{
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(
      f,
      "{} move {} rotate {}",
      self.bone, self.translation, self.rotation
    )
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UVOffset<C: Config> {
  pub vertex: C::VertexIndex,
  pub offset: C::Vec4,
}

impl<C: Config> Display for UVOffset<C>
where
  C::VertexIndex: Display,
  C::Vec4: Display,
{
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "{} offset {}", self.vertex, self.offset)
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum OffsetMethod {
  Multiply = 0,
  Additive = 1,
}

impl Display for OffsetMethod {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    match self {
      OffsetMethod::Multiply => write!(f, "multiply"),
      OffsetMethod::Additive => write!(f, "additive"),
    }
  }
}

impl TryFrom<u8> for OffsetMethod {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    Ok(match value {
      0 => OffsetMethod::Multiply,
      1 => OffsetMethod::Additive,
      e => return Err(Error::InvalidMaterialOffsetMethod(e)),
    })
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MaterialOffset<C: Config> {
  pub material: C::MaterialIndex,
  pub method: OffsetMethod,
  pub diffuse_color: C::Vec4,
  pub specular_color: C::Vec3,
  pub specular_strength: f32,
  pub ambient_color: C::Vec3,
  pub edge_color: C::Vec4,
  pub edge_scale: f32,
  pub texture_tint: C::Vec4,
  pub environment_tint: C::Vec4,
  pub toon_tint: C::Vec4,
}

impl<C: Config> Display for MaterialOffset<C>
where
  C::Vec3: Display,
  C::Vec4: Display,
{
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(
      f,
      r"diffuse: {}, specular: {}/{}, ambient: {}, edge: {}/{},
texture tint: {}, environment tint: {}, toon tint: {}",
      self.diffuse_color,
      self.specular_color,
      self.specular_strength,
      self.ambient_color,
      self.edge_color,
      self.edge_scale,
      self.texture_tint,
      self.environment_tint,
      self.toon_tint,
    )
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ImpulseOffset<C: Config> {
  pub rigid_body: C::RigidbodyIndex,
  pub local: bool,
  pub velocity: C::Vec3,
  pub torque: C::Vec3,
}

impl<C: Config> Display for ImpulseOffset<C>
where
  C::RigidbodyIndex: Display,
  C::Vec3: Display,
{
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(
      f,
      "{} {} velocity: {} torque: {}",
      self.rigid_body,
      if self.local { "local" } else { "global" },
      self.velocity,
      self.torque,
    )
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Offsets<C: Config> {
  Group(Vec<GroupOffset<C>>),
  Vertex(Vec<VertexOffset<C>>),
  Bone(Vec<BoneOffset<C>>),
  UV(Vec<UVOffset<C>>),
  AdditionalUV1(Vec<UVOffset<C>>),
  AdditionalUV2(Vec<UVOffset<C>>),
  AdditionalUV3(Vec<UVOffset<C>>),
  AdditionalUV4(Vec<UVOffset<C>>),
  Material(Vec<MaterialOffset<C>>),
  Flip(Vec<GroupOffset<C>>),
  Impulse(Vec<ImpulseOffset<C>>),
}

impl<C: Config> Display for Offsets<C> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    match self {
      Offsets::Group(offsets) => write!(f, "group ({})", offsets.len()),
      Offsets::Vertex(offsets) => write!(f, "vertices ({})", offsets.len()),
      Offsets::Bone(offsets) => write!(f, "bones ({})", offsets.len()),
      Offsets::UV(offsets) => write!(f, "UVs ({})", offsets.len()),
      Offsets::AdditionalUV1(offsets) => write!(f, "additional UVs 1 ({})", offsets.len()),
      Offsets::AdditionalUV2(offsets) => write!(f, "additional UVs 2 ({})", offsets.len()),
      Offsets::AdditionalUV3(offsets) => write!(f, "additional UVs 3 ({})", offsets.len()),
      Offsets::AdditionalUV4(offsets) => write!(f, "additional UVs 4 ({})", offsets.len()),
      Offsets::Material(offsets) => write!(f, "materials  ({})", offsets.len()),
      Offsets::Flip(offsets) => write!(f, "flip ({})", offsets.len()),
      Offsets::Impulse(offsets) => write!(f, "impulses ({})", offsets.len()),
    }
  }
}

pub struct Morph<C: Config> {
  pub local_name: String,
  pub universal_name: String,
  pub panel: Panel,
  pub offsets: Offsets<C>,
}

impl<C: Config> Display for Morph<C>
where
  Offsets<C>: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(
      f,
      r"local name: {}, universal name: {},
panel: {}, offsets: {}",
      self.local_name, self.universal_name, self.panel, self.offsets,
    )
  }
}
