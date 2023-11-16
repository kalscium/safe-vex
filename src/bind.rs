pub trait Bind {
    type Input;
    type Output;
    /// Safely accesses some data that may not exist
    fn bind(&mut self, f: impl FnMut(&mut Self::Input) -> Self::Output);
}