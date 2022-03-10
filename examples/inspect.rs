use mmd::{DefaultConfig, Error};
use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Error> {
  let filename = env::args().skip(1).next().unwrap();
  println!("Inspect file: {}", filename);

  use mmd::pmx::reader::*;

  let header = HeaderReader::new(BufReader::new(File::open(filename)?))?;

  println!("{}", header);

  let vertices = VertexReader::<_>::new(header)?;
  println!("\nVertex count: {}", vertices.count);

  let surfaces = SurfaceReader::<_>::new(vertices)?;
  println!("\nSurface count: {}", surfaces.count);

  let mut textures = TextureReader::new(surfaces)?;
  println!("\nTextures:");
  for (i, t) in textures.iter().enumerate() {
    println!("{}) {}", i, t?);
  }

  let mut materials = MaterialReader::<_>::new(textures)?;
  println!("\nMaterials:");
  for (i, m) in materials.iter::<DefaultConfig>().enumerate() {
    println!("\n{}) {}", i, m?);
  }

  let mut bones = BoneReader::<_>::new(materials)?;
  println!("\n\nBones:");
  for (i, b) in bones.iter::<DefaultConfig>().enumerate() {
    println!("\n{}) {}", i, b?);
  }

  let mut morphs = MorphReader::<_>::new(bones)?;
  println!("\n\nMorphs:");
  for (i, m) in morphs.iter::<DefaultConfig>().enumerate() {
    println!("\n{}) {}", i, m?);
  }

  let mut displays = DisplayReader::<_>::new(morphs)?;
  println!("\n\nDisplay Frames:");
  for (i, d) in displays.iter::<DefaultConfig>().enumerate() {
    println!("\n{}) {}", i, d?);
  }

  let mut rigid_bodies = RigidBodyReader::<_>::new(displays)?;
  println!("\n\nRigid Bodies:");
  for (i, r) in rigid_bodies.iter::<DefaultConfig>().enumerate() {
    println!("\n{}) {}", i, r?);
  }

  let mut joints = JointReader::<_>::new(rigid_bodies)?;
  println!("\n\nJoints:");
  for (i, j) in joints.iter::<DefaultConfig>().enumerate() {
    println!("\n{}) {}", i, j?);
  }

  Ok(())
}
