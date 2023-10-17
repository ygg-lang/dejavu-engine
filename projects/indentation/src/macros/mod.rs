/// A macro to create a `String` from a format string.
#[macro_export]
macro_rules! indent_fmt {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args!($($arg)*))
    };
}

/// A macro to write to a `Formatter` from a format string.
#[macro_export]
macro_rules! indent_write {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args!($($arg)*))
    };
}

/// Wrap for [`core::fmt::Display`]
#[macro_export]
macro_rules! display_wrap {
    ($($t:ty),*) => {
        $(
            impl Display for $t {
                fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
                    self.fmt_indent(IndentFormatter::new(f, "    "))
                }
            }
        )*
    };
}
