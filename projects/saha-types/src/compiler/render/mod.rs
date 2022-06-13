use serde::__private::de::Content;
use std::collections::BTreeMap;

use crate::SahaNode;

use super::*;

#[derive(Serialize, Deserialize)]
pub struct SahaRenderer {
    defines: BTreeMap<String, Vec<SahaNode>>,
    functions: BTreeMap<String, Vec<SahaNode>>,
}

impl SahaVM {
    pub fn as_renderer(&self) -> SahaRenderer {
        SahaRenderer { defines: Default::default(), functions: Default::default() }
    }
}

impl SahaRenderer {
    pub fn try_render<V>(&self, input: V)
    where
        V: Serialize,
    {
        input.serialize(Content::None)?

        let data = Content::deserialize(input)?;
    }
}
