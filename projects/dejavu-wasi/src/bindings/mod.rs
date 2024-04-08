use crate::{
    exports::dejavu::core::types::{GuestUrl, SyntaxError, TextRange},
    DejavuError, DejavuHost,
};
use std::str::FromStr;
use url::ParseError;

mod backends;

impl crate::exports::dejavu::core::types::Guest for DejavuHost {
    type Url = UrlNative;
}

pub struct UrlNative {
    repr: url::Url,
}

impl GuestUrl for UrlNative {}

impl FromStr for UrlNative {
    type Err = DejavuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { repr: url::Url::from_str(s)? })
    }
}

impl From<ParseError> for DejavuError {
    fn from(value: ParseError) -> Self {
        DejavuError::Syntax(SyntaxError {
            reason: value.to_string(),
            file: None,
            range: TextRange { head_offset: 0, tail_offset: 0 },
        })
    }
}
