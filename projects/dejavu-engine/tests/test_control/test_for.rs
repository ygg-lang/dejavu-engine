use super::*;

impl core::fmt::Display for HelloTemplate {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("# for one pattern\n")?;
        for i in j {
            f.write_str("\n if text\n")?;
        }

        f.write_str("# for two pattern\n")?;
        for i in j {
            f.write_str("\n if text\n")?;
        }

        f.write_str("# for if pattern\n")?;
        for i in j {
            if !(true) {
                continue;
            }
            f.write_str("\n if text\n")?;
        }

        f.write_str("# for else pattern\n")?;
        let mut _looped = false;
        for i in j {
            _looped = true;
            f.write_str("\n if text\n")?;
        }
        if !_looped {
            f.write_str("\n else text\n")?;
        }

        Ok(())
    }
}
