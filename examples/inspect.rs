use std::env;
use mmd::Error;
use std::io::BufReader;
use std::fs::File;
use fallible_iterator::FallibleIterator;

fn main() -> Result<(), Error> {
  let filename = env::args().skip(1).next().unwrap();

  use mmd::pmx::reader::*;

  let header = HeaderReader::new(
    BufReader::new(File::open(filename)?))?;

  println!("{}", header);

  let vertices = VertexReader::<_>::new(header)?;
  println!("\nVertex count: {}", vertices.count);

  let surfaces = SurfaceReader::<_>::new(vertices)?;
  println!("\nSurface count: {}", surfaces.count);

  let mut textures = TextureReader::new(surfaces)?;
  println!("\nTextures:");
  textures.iter().enumerate().for_each(|(i, t)| {
    println!("{}) {}", i, t);
    Ok(())
  })?;

  let mut materials = MaterialReader::<_>::new(textures)?;
  println!("\nMaterials:");
  materials.iter::<i32>().enumerate().for_each(|(i, m)| {
    println!("\n{}) {}", i, m);
    Ok(())
  })?;
  
  let mut bones = BoneReader::<_>::new(materials)?;
  println!("\n\nBones:");
  bones.iter::<i32>().enumerate().for_each(|(i, b)| {
    println!("\n{}) {}", i, b);
    Ok(())
  })?;


  Ok(())
}