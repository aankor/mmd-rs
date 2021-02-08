use crate::{
  reader::{helpers::ReadHelpers, VertexReader},
  Result, Settings, VertexIndex,
};
use byteorder::{ReadBytesExt, LE};
use std::io::Read;
use std::marker::PhantomData;

pub struct SurfaceReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R,
}

impl<R: Read> SurfaceReader<R> {
  pub fn new(mut v: VertexReader<R>) -> Result<SurfaceReader<R>> {
    while v.remaining > 0 {
      v.next_vertex::<i32>()?;
    }
    let count = v.read.read_i32::<LE>()?;

    Ok(SurfaceReader {
      settings: v.settings,
      count,
      remaining: count,
      read: v.read,
    })
  }

  pub fn next_surface<I: VertexIndex>(&mut self) -> Result<Option<[I; 3]>> {
    if self.remaining <= 0 {
      return Ok(None);
    }

    self.remaining -= 3;
    Ok(Some([
      self
        .read
        .read_vertex_index(self.settings.vertex_index_size)?,
      self
        .read
        .read_vertex_index(self.settings.vertex_index_size)?,
      self
        .read
        .read_vertex_index(self.settings.vertex_index_size)?,
    ]))
  }

  pub fn iter<I>(&mut self) -> SurfaceIterator<R, I> {
    SurfaceIterator {
      reader: self,
      phantom: PhantomData,
    }
  }
}

pub struct SurfaceIterator<'a, R, I = i32> {
  reader: &'a mut SurfaceReader<R>,
  phantom: PhantomData<I>,
}

impl<R: Read, I: VertexIndex> Iterator for SurfaceIterator<'_, R, I> {
  type Item = Result<[I; 3]>;

  fn next(&mut self) -> Option<Self::Item> {
    self.reader.next_surface().map_or(None, |v| v.map(Ok))
  }
}
