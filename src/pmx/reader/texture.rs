use crate::{
  reader::{helpers::ReadHelpers, SurfaceReader},
  DefaultConfig, Result, Settings,
};
use byteorder::{ReadBytesExt, LE};
use std::io::Read;

pub struct TextureReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R,
  pub(crate) poison: bool,
}

impl<R: Read> TextureReader<R> {
  pub fn new(mut s: SurfaceReader<R>) -> Result<TextureReader<R>> {
    assert!(!s.poison);
    while s.remaining > 0 {
      s.next::<DefaultConfig>()?;
    }
    let count = s.read.read_i32::<LE>()?;

    Ok(TextureReader {
      settings: s.settings,
      count,
      remaining: count,
      read: s.read,
      poison: false,
    })
  }

  pub fn next(&mut self) -> Result<Option<String>> {
    assert!(!self.poison);
    let result = self.next_impl();
    if result.is_err() {
      self.poison = true;
    }
    result
  }

  fn next_impl(&mut self) -> Result<Option<String>> {
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
    self
      .reader
      .next()
      .map_or_else(|e| Some(Err(e)), |v| v.map(Ok))
  }
}
