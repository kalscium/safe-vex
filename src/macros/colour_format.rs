/// Used to format text with colour
/// # Examples
/// ```rust
/// use soulog::*;
/// colour_format![pink("["), none("Logger"), pink("] "), none("Example Log")];
/// // outputs: [Logger] Example Log
/// // but with colour
/// ```
#[macro_export]
macro_rules! colour_format { // Verbose ugly stuff I can't read
    ($(
        $(none($none:expr))?
        $(blue($blue:expr))?
        $(pink($pink:expr))?
        $(white($white:expr))?
        $(green($green:expr))?
        $(cyan($cyan:expr))?
        $(red($red:expr))?
        $(black($black:expr))?
        $(yellow($yellow:expr))?
        $(raw($raw:expr))?
    ),*) => {{
        let mut string = alloc::string::String::new();
        $(
            $(string.push_str("\x1b[0m"); string.push_str($none);)?
            $(string.push_str("\x1b[34m"); string.push_str($blue);)?
            $(string.push_str("\x1b[35m"); string.push_str($pink);)?
            $(string.push_str("\x1b[37m"); string.push_str($white);)?
            $(string.push_str("\x1b[32m"); string.push_str($green);)?
            $(string.push_str("\x1b[36m"); string.push_str($cyan);)?
            $(string.push_str("\x1b[31m"); string.push_str($red);)?
            $(string.push_str("\x1b[30m"); string.push_str($black);)?
            $(string.push_str("\x1b[33m"); string.push_str($yellow);)?
            $(string.push_str($raw);)?
        )* string.push_str("\x1b[0m");
        string
    }}
}