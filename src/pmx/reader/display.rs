use crate::{
  pmx::display::*,
  reader::{helpers::ReadHelpers, MorphReader},
  Config, DefaultConfig, Error, Result, Settings,
};
use byteorder::{ReadBytesExt, LE};
use std::io::Read;
use std::marker::PhantomData;

pub struct DisplayReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R,
  pub(crate) poison: bool,
}

impl<R: Read> DisplayReader<R> {
  pub fn new(mut m: MorphReader<R>) -> Result<DisplayReader<R>> {
    assert!(!m.poison);
    while m.remaining > 0 {
      m.next::<DefaultConfig>()?;
    }
    let count = m.read.read_i32::<LE>()?;

    Ok(DisplayReader {
      settings: m.settings,
      count,
      remaining: count,
      read: m.read,
      poison: false,
    })
  }

  pub fn next<C: Config>(&mut self) -> Result<Option<DisplayFrame<C>>> {
    assert!(!self.poison);
    let result = self.next_impl::<C>();
    if result.is_err() {
      self.poison = true;
    }
    result
  }

  fn next_impl<C: Config>(&mut self) -> Result<Option<DisplayFrame<C>>> {
    if self.remaining <= 0 {
      return Ok(None);
    }

    self.remaining -= 1;

    let local_name = self.read.read_text(self.settings.text_encoding)?;
    let universal_name = self.read.read_text(self.settings.text_encoding)?;
    let special_flag = self.read.read_u8()? != 0;
    let frame_count = self.read.read_u32::<LE>()?;
    let mut frames = Vec::with_capacity(frame_count as usize);

    for _ in 0..frame_count {
      let frame = match self.read.read_u8()? {
        0 => Frame::Bone(self.read.read_index(self.settings.bone_index_size)?),
        1 => Frame::Morph(self.read.read_index(self.settings.morph_index_size)?),
        e => return Err(Error::InvalidFrameType(e)),
      };

      frames.push(frame);
    }

    Ok(Some(DisplayFrame {
      local_name,
      universal_name,
      special_flag,
      frames,
    }))
  }

  pub fn iter<C>(&mut self) -> DisplayIterator<R, C> {
    DisplayIterator {
      reader: self,
      phantom: PhantomData,
    }
  }
}

pub struct DisplayIterator<'a, R, C> {
  reader: &'a mut DisplayReader<R>,
  phantom: PhantomData<C>,
}

impl<R: Read, C: Config> Iterator for DisplayIterator<'_, R, C> {
  type Item = Result<DisplayFrame<C>>;

  fn next(&mut self) -> Option<Self::Item> {
    self
      .reader
      .next()
      .map_or_else(|e| Some(Err(e)), |v| v.map(Ok))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (
      self.reader.remaining as usize,
      Some(self.reader.remaining as usize),
    )
  }
}

impl<R: Read, C: Config> ExactSizeIterator for DisplayIterator<'_, R, C> {
  fn len(&self) -> usize {
    self.reader.remaining as usize
  }
}
