use crate::Error;
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
pub struct MorphOffset<I> {
  pub morph: I,
  pub rate: f32,
}

impl<I: Display> Display for MorphOffset<I> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "{} at rate {}", self.morph, self.rate)
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VertexOffset<VI> {
  pub vertex: VI,
  pub offset: [f32; 3],
}

impl<VI: Display> Display for VertexOffset<VI> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "{} offset {:?}", self.vertex, self.offset)
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BoneOffset<BI> {
  pub bone: BI,
  pub translation: [f32; 3],
  pub rotation: [f32; 4],
}

impl<BI: Display> Display for BoneOffset<BI> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(
      f,
      "{} move {:?} rotate {:?}",
      self.bone, self.translation, self.rotation
    )
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UVOffset<VI> {
  pub vertex: VI,
  pub offset: [f32; 4],
}

impl<VI: Display> Display for UVOffset<VI> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "{} offset {:?}", self.vertex, self.offset)
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
pub struct MaterialOffset<MI> {
  pub material: MI,
  pub method: OffsetMethod,
  pub diffuse_color: [f32; 4],
  pub specular_color: [f32; 3],
  pub specular_strength: f32,
  pub ambient_color: [f32; 3],
  pub edge_color: [f32; 4],
  pub edge_scale: f32,
  pub texture_tint: [f32; 4],
  pub environment_tint: [f32; 4],
  pub toon_tint: [f32; 4],
}

impl<MI: Display> Display for MaterialOffset<MI> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(
      f,
      r"diffuse: {:?}, specular: {:?}/{}, ambient: {:?}, edge: {:?}/{},
texture tint: {:?}, environment tint: {:?}, toon tint: {:?}",
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
pub struct ImpulseOffset<RBI> {
  pub rigid_body: RBI,
  pub local: bool,
  pub velocity: [f32; 3],
  pub torque: [f32; 3],
}

impl<RBI: Display> Display for ImpulseOffset<RBI> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(
      f,
      "{} {} velocity: {:?} torque: {:?}",
      self.rigid_body,
      if self.local { "local" } else { "global" },
      self.velocity,
      self.torque,
    )
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Offsets<I, VI, BI, MI, RBI> {
  Group(Vec<MorphOffset<I>>),
  Vertex(Vec<VertexOffset<VI>>),
  Bone(Vec<BoneOffset<BI>>),
  UV(Vec<UVOffset<VI>>),
  AdditionalUV1(Vec<UVOffset<VI>>),
  AdditionalUV2(Vec<UVOffset<VI>>),
  AdditionalUV3(Vec<UVOffset<VI>>),
  AdditionalUV4(Vec<UVOffset<VI>>),
  Material(Vec<MaterialOffset<MI>>),
  Flip(Vec<MorphOffset<I>>),
  Impulse(Vec<ImpulseOffset<RBI>>),
}

impl<I, VI, BI, MI, RBI> Display for Offsets<I, VI, BI, MI, RBI>
where
  I: Display,
  VI: Display,
  BI: Display,
  MI: Display,
  RBI: Display,
{
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

pub struct Morph<I, VI, BI, MI, RBI> {
  pub local_name: String,
  pub universal_name: String,
  pub panel: Panel,
  pub offsets: Offsets<I, VI, BI, MI, RBI>,
}

impl<I, VI, BI, MI, RBI> Display for Morph<I, VI, BI, MI, RBI>
where
  Offsets<I, VI, BI, MI, RBI>: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(
      f,
      r"local name: {}, universal name: {},
panel: {:?}, offsets: {}",
      self.local_name, self.universal_name, self.panel, self.offsets,
    )
  }
}
