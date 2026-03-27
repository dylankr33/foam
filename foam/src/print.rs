use foam_proc_macro::{cfg_modern, cfg_retro};

#[cfg_retro]
#[macro_export]
macro_rules! lprint {
    ($($arg:tt)*) => (psp::dprint!($($arg)*));
}

#[cfg_modern]
#[macro_export]
macro_rules! lprint {
    ($($arg:tt)*) => (std::print!($($arg)*));
}

#[macro_export]
macro_rules! lprintln {
    () => ($crate::lprint!("\n"));
    ($($arg:tt)*) => {
        $crate::lprint!("{}\n", format_args!($($arg)*))
    };
}
