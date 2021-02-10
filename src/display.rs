use std::fmt::Display;

#[repr(transparent)]
pub(crate) struct DisplayOption<'a, T>(Option<&'a T>);

impl<'a, T> DisplayOption<'a, T> {
  pub(crate) fn new(inner: &'a Option<T>) -> Self {
    Self(inner.as_ref())
  }
}

impl<'a, T: Display> Display for DisplayOption<'a, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(t) = self.0 {
      T::fmt(t, f)
    } else {
      write!(f, "None")
    }
  }
}
