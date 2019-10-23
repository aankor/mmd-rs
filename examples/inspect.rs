use std::env;
use mmd::pmx::Error;
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
  println!("Vertex count: {}", vertices.count);

  let surfaces = SurfaceReader::<_>::new(vertices)?;
  println!("Surface count: {}", surfaces.count);

  let mut textures = TextureReader::new(surfaces)?;
  println!("Textures:");
  textures.iter().enumerate().for_each(|(i, t)| {
    println!("{}) {}", i, t);
    Ok(())
  })?;

  let mut materials = MaterialReader::<_>::new(textures)?;
  println!("Materials:");
  materials.iter::<i32>().enumerate().for_each(|(i, m)| {
    println!("{}) {}", i, m);
    Ok(())
  })?;


  Ok(())
}