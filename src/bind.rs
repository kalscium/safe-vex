pub trait Bind {
    type Input;
    type Output;
    fn bind(&mut self, f: &'static mut impl FnMut(&Self::Input) -> Self::Output);
}

#[macro_export]
macro_rules! bind {
    () => {
        compile_error!("binds must have at least one condition");
    };

    ($head:expr, $($tail:expr),* => $func:expr) => {
        $crate::bind!($head, $tail => () $func)
    };
    
    ($head:expr, $($tail:expr),* => ($($extra:expr),*) $func:expr) => {
        $head.bind(&|x| $crate::bind!($($tail),* => ($($extra,)* x) $func))
    };

    ($tail:expr => ($extra:expr) $func:expr) => {
        $tail.bind(&|x| $func(($($extra,)* x)))
    };
}