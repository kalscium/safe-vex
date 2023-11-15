pub trait Bind {
    type Input;
    type Output;
    /// Safely accesses some data that may not exist
    fn bind(&mut self, f: &'static mut impl FnMut(&Self::Input) -> Self::Output);
}