pub struct Bdef1<I> {
  pub bone_index: I,
}

pub struct Bdef2<I> {
  pub bone_1_index: I,
  pub bone_2_index: I,
  pub bone_1_weight: f32,
}

pub struct Bdef4<I> {
  pub bone_1_index: I,
  pub bone_2_index: I,
  pub bone_3_index: I,
  pub bone_4_index: I,
  pub bone_1_weight: f32,
  pub bone_2_weight: f32,
  pub bone_3_weight: f32,
  pub bone_4_weight: f32,
}

pub struct Sdef<I> {
  pub bone_1_index: I,
  pub bone_2_index: I,
  pub bone_1_weight: f32,
  pub c: [f32; 3],
  pub r0: [f32; 3],
  pub r1: [f32; 3],
}

pub struct Qdef<I> {
  pub bone_1_index: I,
  pub bone_2_index: I,
  pub bone_3_index: I,
  pub bone_4_index: I,
  pub bone_1_weight: f32,
  pub bone_2_weight: f32,
  pub bone_3_weight: f32,
  pub bone_4_weight: f32,
}

pub enum WeightDeform<I> {
  Bdef1(Bdef1<I>),
  Bdef2(Bdef2<I>),
  Bdef4(Bdef4<I>),
  Sdef(Sdef<I>),
  Qdef(Qdef<I>),
}
