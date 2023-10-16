struct HelloTemplate<'a> {
    name: &'a str,
}

impl<'a> core::fmt::Display for HelloTemplate<'a> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_fmt(format_args!(f.write_str()
        ))


        Ok(())
    }
}
