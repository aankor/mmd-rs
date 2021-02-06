use crate::{
  reader::{helpers::ReadHelpers, SurfaceReader},
  Result, Settings,
};
use byteorder::{ReadBytesExt, LE};
use std::io::Read;

pub struct TextureReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R,
}

impl<R: Read> TextureReader<R> {
  pub fn new(mut v: SurfaceReader<R>) -> Result<TextureReader<R>> {
    while v.remaining > 0 {
      v.next_surface::<i32>()?;
    }
    let count = v.read.read_i32::<LE>()?;

    Ok(TextureReader {
      settings: v.settings,
      count,
      remaining: count,
      read: v.read,
    })
  }

  pub fn next(&mut self) -> Result<Option<String>> {
    if self.remaining <= 0 {
      return Ok(None);
    }

    self.remaining -= 1;

    self.read.read_text(self.settings.text_encoding).map(Some)
  }

  pub fn iter(&mut self) -> TextureIterator<R> {
    TextureIterator { reader: self }
  }
}

pub struct TextureIterator<'a, R> {
  reader: &'a mut TextureReader<R>,
}

impl<R: Read> Iterator for TextureIterator<'_, R> {
  type Item = Result<String>;

  fn next(&mut self) -> Option<Self::Item> {
    self.reader.next().map_or(None, |v| v.map(Ok))
  }
}
