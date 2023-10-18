use super::*;

impl core::fmt::Display for HelloTemplate {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("# for one pattern\n")?;
        Ok(())
    }
}