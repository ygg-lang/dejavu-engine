use std::fmt::Formatter;

use serde::{
    de::{IgnoredAny, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use super::*;

pub struct ConfigVisitor<'w> {
    value: &'w mut WorkspaceConfig,
}

const DEFAULT_INCLUDE: &str = "
*.dj
*.djv
*.dejavu
";

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self { root: Default::default(), include: DEFAULT_INCLUDE.to_string() }
    }
}

impl<'de> Deserialize<'de> for WorkspaceConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut out = Default::default();
        let v = ConfigVisitor { value: &mut out };
        deserializer.deserialize_map(v)?;
        Ok(out)
    }

    fn deserialize_in_place<'w, D>(deserializer: D, place: &'w mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = ConfigVisitor { value: place };
        deserializer.deserialize_map(v)
    }
}

#[allow(unused_variables)]
impl<'de, 'w> Visitor<'de> for ConfigVisitor<'w> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Expect type `WorkspaceConfig`")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "include" => self.value.include = map.next_value::<String>()?,
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        Ok(())
    }
}
