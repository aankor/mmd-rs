use crate::pmx::types::*;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub struct Settings {
  pub text_encoding: TextEncoding,
  pub additional_vec4_count: u8,
  pub vertex_index_size: IndexSize,
  pub texture_index_size: IndexSize,
  pub material_index_size: IndexSize,
  pub bone_index_size: IndexSize,
  pub morph_index_size: IndexSize,
  pub rigidbody_index_size: IndexSize,
}

impl Display for Settings {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    writeln!(
      f,
      r"encoding: {}, additional vec4s: {}, vertex index: {}, texture index: {},
material index: {}, bone index: {}, morph index: {}, rigidbody index: {}",
      self.text_encoding,
      self.additional_vec4_count,
      self.vertex_index_size,
      self.texture_index_size,
      self.material_index_size,
      self.bone_index_size,
      self.morph_index_size,
      self.rigidbody_index_size
    )
  }
}
