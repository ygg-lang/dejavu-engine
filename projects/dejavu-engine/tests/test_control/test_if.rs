struct HelloTemplate<'a> {
    name: &'a str,
}

impl<'a> core::fmt::Display for HelloTemplate<'a> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("# simple if elements\n")?;

        if true {
            f.write_str("\n if text\n")?;
        }

        f.write_str("\n\n# if-else element\n")?;

        if true {
            f.write_str("\n if text\n")?;
        }
        else {
            f.write_str("\n else text\n")?;
        }

        f.write_str("\n\n\n# if-else-if element\n")?;

        if true {
            f.write_str("\n if text\n")?;
        }
        else if true {
            f.write_str("\n else if text\n")?;
        }

        f.write_str("\n\n# if-else-if element\n")?;

        if true {
            f.write_str("\n if text\n")?;
        }
        else if true {
            f.write_str("\n else if text\n")?;
        }
        else {
            f.write_str("\n else text\n")?;
        }

        Ok(())
    }
}
