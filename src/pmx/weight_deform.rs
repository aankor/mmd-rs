use crate::Config;

pub struct Bdef1<C: Config> {
  pub bone_index: C::BoneIndex,
}

pub struct Bdef2<C: Config> {
  pub bone_1_index: C::BoneIndex,
  pub bone_2_index: C::BoneIndex,
  pub bone_1_weight: f32,
}

pub struct Bdef4<C: Config> {
  pub bone_1_index: C::BoneIndex,
  pub bone_2_index: C::BoneIndex,
  pub bone_3_index: C::BoneIndex,
  pub bone_4_index: C::BoneIndex,
  pub bone_1_weight: f32,
  pub bone_2_weight: f32,
  pub bone_3_weight: f32,
  pub bone_4_weight: f32,
}

pub struct Sdef<C: Config> {
  pub bone_1_index: C::BoneIndex,
  pub bone_2_index: C::BoneIndex,
  pub bone_1_weight: f32,
  pub c: C::Vec3,
  pub r0: C::Vec3,
  pub r1: C::Vec3,
}

pub struct Qdef<C: Config> {
  pub bone_1_index: C::BoneIndex,
  pub bone_2_index: C::BoneIndex,
  pub bone_3_index: C::BoneIndex,
  pub bone_4_index: C::BoneIndex,
  pub bone_1_weight: f32,
  pub bone_2_weight: f32,
  pub bone_3_weight: f32,
  pub bone_4_weight: f32,
}

pub enum WeightDeform<C: Config> {
  Bdef1(Bdef1<C>),
  Bdef2(Bdef2<C>),
  Bdef4(Bdef4<C>),
  Sdef(Sdef<C>),
  Qdef(Qdef<C>),
}
