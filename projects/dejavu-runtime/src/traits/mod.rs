use alloc::string::String;
use core::fmt::Write;

pub trait Template {
    const EXTENSION: &'static str;
    const MIME_TYPE: &'static str;
    const SIZE_HINT: usize;

    fn write_fmt<W>(&self, w: &mut W) -> core::fmt::Result
    where
        W: Write + ?Sized;

    fn render(&self) -> String {
        let mut out = String::with_capacity(Self::SIZE_HINT);
        self.write_fmt(&mut out).unwrap();
        out
    }
}
