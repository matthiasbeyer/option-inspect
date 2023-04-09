//! This crate adds the missing Option::inspect function via an extension trait
//!

/// Extension trait for adding Option::inspect
pub trait OptionInspect<F, T>
where
    F: FnOnce(&T),
    T: Sized,
{
    /// Inspect the Option
    ///
    /// Either call `f` on the value in `Some` or do nothing if this Option is a None.
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
        if let Some(o) = self.as_ref() {
            (f)(o);
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
            (f)(o);
        }
    }
}

/// Extension trait for adding Option::inspect_none
pub trait OptionInspectNone<F>
where
    F: FnOnce(),
{
    /// Call `f` if the Option this is called on is a None
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
        if self.is_none() {
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
        if self.is_none() {
            (f)();
        }
    }
}
