use crate::pmx::WeightDeform;

pub struct Vertex<I> {
  pub position: [f32; 3],
  pub normal: [f32; 3],
  pub uv: [f32; 2],
  pub additional: [[f32; 4]; 4],
  pub weight_deform: WeightDeform<I>,
  pub edge_scale: f32
}
