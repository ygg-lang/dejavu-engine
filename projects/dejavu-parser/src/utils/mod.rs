use dejavu_ir::DejavuError;

#[derive(Default)]
pub struct TemplateState {}

pub(crate) trait ToHir<'i> {
    type Ir;
    fn to_hir(&'i self, state: &mut TemplateState) -> Result<Self::Ir, DejavuError>;
}
