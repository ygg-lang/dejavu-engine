pub extern crate anyhow;

pub use anyhow::{Error, Result};

pub use self::{
    escaper::{
        utils::{escape, TemplateDisplay},
        Escaper, PlainText, XmlEscaper,
    },
    looper::{ForLooper, Looper},
    traits::Template,
};

mod escaper;
mod for_3rd;
mod looper;
mod traits;
