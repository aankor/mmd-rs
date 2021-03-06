use crate::{
  reader::{helpers::ReadHelpers, VertexReader},
  Config, DefaultConfig, Result, Settings,
};
use byteorder::{ReadBytesExt, LE};
use std::io::Read;
use std::marker::PhantomData;

pub struct SurfaceReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R,
  pub(crate) poison: bool,
}

impl<R: Read> SurfaceReader<R> {
  pub fn new(mut v: VertexReader<R>) -> Result<SurfaceReader<R>> {
    assert!(!v.poison);
    while v.remaining > 0 {
      v.next::<DefaultConfig>()?;
    }
    let count = v.read.read_i32::<LE>()?;

    Ok(SurfaceReader {
      settings: v.settings,
      count,
      remaining: count,
      read: v.read,
      poison: false,
    })
  }

  pub fn next<C: Config>(&mut self) -> Result<Option<[C::VertexIndex; 3]>> {
    assert!(!self.poison);
    let result = self.next_impl::<C>();
    if result.is_err() {
      self.poison = true;
    }
    result
  }

  fn next_impl<C: Config>(&mut self) -> Result<Option<[C::VertexIndex; 3]>> {
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

pub struct SurfaceIterator<'a, R, C = DefaultConfig> {
  reader: &'a mut SurfaceReader<R>,
  phantom: PhantomData<C>,
}

impl<R: Read, C: Config> Iterator for SurfaceIterator<'_, R, C> {
  type Item = Result<[C::VertexIndex; 3]>;

  fn next(&mut self) -> Option<Self::Item> {
    self
      .reader
      .next::<C>()
      .map_or_else(|e| Some(Err(e)), |v| v.map(Ok))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (
      self.reader.remaining as usize,
      Some(self.reader.remaining as usize),
    )
  }
}

impl<R: Read, C: Config> ExactSizeIterator for SurfaceIterator<'_, R, C> {
  fn len(&self) -> usize {
    self.reader.remaining as usize
  }
}
