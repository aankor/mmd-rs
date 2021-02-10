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
  for (i, b) in morphs.iter::<DefaultConfig>().enumerate() {
    println!("\n{}) {}", i, b?);
  }

  Ok(())
}
