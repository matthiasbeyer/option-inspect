//! This crate adds the missing Option::inspect function via an extension trait
//!

pub trait OptionInspect<F, T>
    where F: FnOnce(&T),
          T: Sized
{
    fn inspect(self, f: F) -> Self;
}

pub trait OptionInspectRef<F, T>
    where F: FnOnce(&T),
          T: Sized
{
    fn inspect(&self, f: F);
}

impl<F, T> OptionInspect<F, T> for Option<T>
    where F: FnOnce(&T),
          T: Sized
{
    fn inspect(self, f: F) -> Self {
        if let Some(ref o) = self.as_ref() {
            (f)(&o);
        }

        self
    }
}

impl<F, T> OptionInspectRef<F, T> for Option<T>
    where F: FnOnce(&T),
          T: Sized
{
    fn inspect(&self, f: F) {
        if let Some(ref o) = self {
            (f)(&o);
        }
    }
}
