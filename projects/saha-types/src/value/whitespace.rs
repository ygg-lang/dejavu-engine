use super::*;

impl Display for Destroyer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Destroyer::Everything => {}
            Destroyer::NewlineAll => {}
            Destroyer::NewlineOne => {}
            Destroyer::OneLine => {}
        }
    }
}

impl From<Option<char>> for Destroyer {
    fn from(value: Option<char>) -> Self {
        match value {
            None => {}
            Some(_) => {}
        }
    }
}

impl From<char> for Destroyer {
    fn from(value: char) -> Self {
        match value {
            '_' => {}
        }
    }
}
