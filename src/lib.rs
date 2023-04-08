//! This crate adds the missing Option::inspect function via an extension trait
//!

pub trait OptionInspect<F, T>
where
    F: FnOnce(&T),
    T: Sized,
{
    fn inspect(self, f: F) -> Self;
}

pub trait OptionInspectRef<F, T>
where
    F: FnOnce(&T),
    T: Sized,
{
    fn inspect(&self, f: F);
}

impl<F, T> OptionInspect<F, T> for Option<T>
where
    F: FnOnce(&T),
    T: Sized,
{
    fn inspect(self, f: F) -> Self {
        if let Some(ref o) = self.as_ref() {
            (f)(&o);
        }

        self
    }
}

impl<F, T> OptionInspectRef<F, T> for Option<T>
where
    F: FnOnce(&T),
    T: Sized,
{
    fn inspect(&self, f: F) {
        if let Some(ref o) = self {
            (f)(&o);
        }
    }
}

pub trait OptionInspectNone<F>
where
    F: FnOnce(),
{
    fn inspect_none(self, f: F) -> Self;
}

pub trait OptionInspectNoneRef<F>
where
    F: FnOnce(),
{
    fn inspect_none(&self, f: F);
}

impl<F, T> OptionInspectNone<F> for Option<T>
where
    F: FnOnce(),
{
    fn inspect_none(self, f: F) -> Self {
        if let None = self.as_ref() {
            (f)();
        }

        self
    }
}

impl<F, T> OptionInspectNoneRef<F> for Option<T>
where
    F: FnOnce(),
{
    fn inspect_none(&self, f: F) {
        if let None = self {
            (f)();
        }
    }
}
