use crate::{Config, WeightDeform};

pub struct Vertex<C: Config> {
  pub position: C::Vec3,
  pub normal: C::Vec3,
  pub uv: C::Vec2,
  pub additional: C::AdditionalVec4s,
  pub weight_deform: WeightDeform<C>,
  pub edge_scale: f32,
}
