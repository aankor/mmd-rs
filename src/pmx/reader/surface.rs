use crate::{Error, Settings, reader::VertexReader, reader::helpers::ReadHelpers};
use std::io::Read;
use fallible_iterator::FallibleIterator;
use byteorder::{ReadBytesExt, LE};
use std::convert::TryFrom;
use std::marker::PhantomData;

pub struct SurfaceReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R,
}

impl<R: Read> SurfaceReader<R> {
  pub fn new(mut v: VertexReader<R>) -> Result<SurfaceReader<R>, Error> {
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

  pub fn next_surface<I: TryFrom<i8> + TryFrom<i16> + TryFrom<i32>>(&mut self) -> Result<Option<[I; 3]>, Error> {
    if self.remaining <= 0 {
      return Ok(None)
    }

    self.remaining -= 3;
    Ok(Some([
      self.read.read_index(self.settings.bone_index_size)?,
      self.read.read_index(self.settings.bone_index_size)?,
      self.read.read_index(self.settings.bone_index_size)?]))
  }

  pub fn iter<I>(&mut self) -> SurfaceIterator<R, I> {
    SurfaceIterator {
      reader: self,
      phantom: PhantomData
    }
  }
}

pub struct SurfaceIterator<'a, R, I = i32> {
  reader: &'a mut SurfaceReader<R>,
  phantom: PhantomData<I>
}

impl<R: Read, I: TryFrom<i8> + TryFrom<i16> + TryFrom<i32>> FallibleIterator for SurfaceIterator<'_, R, I> {
  type Item = [I; 3];
  type Error = Error;

  fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
    self.reader.next_surface()
  }
}