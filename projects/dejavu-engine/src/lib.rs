pub extern crate anyhow;

pub use self::{
    escaper::{
        utils::{escape, TemplateDisplay},
        Escaper, PlainText, XmlEscaper,
    },
    traits::Template,
};

mod escaper;
mod for_3rd;
mod traits;
