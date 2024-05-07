use alloc::boxed::Box;

/// A struct that may or may not exist (like `Option`)
///
/// On an attempt to get the value, it either returns it (Some) or returns None and tries to build it again
pub struct Maybe<T> {
    /// internal option
    value: Option<T>,
    /// a function that tries to build / create the value
    build: Box<dyn Fn() -> Option<T>>,
}

impl<T> Maybe<T> {
    /// Creates a new optional version of `T`
    #[inline]
    pub fn new(f: Box<dyn Fn() -> Option<T>>) -> Self {
        Self {
            value: f(),
            build: f,
        }
    }

    /// Tries to get the internal value, it either returns it (Some) or returns None and tries to build it again
    #[inline]
    pub fn get(&mut self) -> Option<&mut T> {
        if let Some(ref mut x) = self.value {
            return Some(x);
        };

        self.value = (self.build)();
        self.value.as_mut()
    }
}
