use crate::{ParserConfig, Result, Value};

///
#[cfg(feature = "json")]
pub fn parse_json(json: &str) -> Result<Value> {
    let data = serde_json::from_str::<serde_json::Value>(json);
    Ok(data?.into())
}

///
#[cfg(feature = "toml")]
pub fn parse_toml(toml: &str) -> Result<Value> {
    let data = toml::from_str::<toml::Value>(toml);
    Ok(data?.into())
}

///
#[cfg(feature = "yaml")]
pub fn parse_yaml(yaml: &str) -> Result<Value> {
    let out = yaml_rust::YamlLoader::load_from_str(&yaml)?;
    let value = match out.len() {
        1 => Value::from(out[0].clone()),
        _ => Value::from(out),
    };
    Ok(value)
}

///
pub fn parse_arc(arc: &str) -> Result<Value> {
    let cfg = ParserConfig::default();
    cfg.parse(arc).map(|e| Value::from(e))
}
