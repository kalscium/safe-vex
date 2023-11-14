pub trait Bind {
    type Input;
    fn bind(&self, f: &'static impl FnMut(Self::Input));
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