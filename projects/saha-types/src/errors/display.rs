use diagnostic_quick::Diagnostic;

use super::*;

impl Debug for SahaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Display for SahaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for SahaErrorKind {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{file}{start}..{end}", file = self.file, start = self.range.start, end = self.range.end)
    }
}

impl SahaError {
    pub fn as_diagnostic(&self) -> Diagnostic {
        let mut out = Diagnostic::new(self.level);
        match &*self.kind {
            SahaErrorKind::IoError { message, file: _ } => out.message = message.to_string(),
            SahaErrorKind::SyntaxError { message, span } => out = out.with_primary(&span.file, span.range.clone(), message),
            SahaErrorKind::RuntimeError { message } => out.message = message.to_string(),
        }
        out
    }
}
