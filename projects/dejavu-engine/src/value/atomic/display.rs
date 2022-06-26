use super::*;

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, id) in self.path.iter().enumerate() {
            if idx != 0 {
                f.write_str("::")?
            }
            f.write_str(&id.name)?;
        }
        Ok(())
    }
}

impl Display for DjvPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, symbol) in self.symbols.iter().enumerate() {
            if i != 0 {
                f.write_char(',')?;
            }
            f.write_str(&symbol.name)?
        }
        Ok(())
    }
}
