use super::*;

#[derive(Serialize, Deserialize)]
pub struct SahaRender {}

impl SahaCompiler {
    pub fn as_render(&self) -> SahaRender {
        SahaRender {}
    }
}
