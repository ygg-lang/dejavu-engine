use super::*;

impl core::fmt::Display for HelloTemplate {
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
        f.write_str("\n\n# nested if statement\n")?;
        if true {
            f.write_str("\n    term 1\n    ")?;
            if true {
                f.write_str("\n     term 2\n    ")?;
            }
            f.write_str("\n term 3\n")?;
        }
        Ok(())
    }
}
