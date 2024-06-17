use crate::{
    dejavu::{
        DejavuParser, DejavuRule, ElementNode, ExpressionNode, ForBlockNode, IfBlockNode, RootNode, TemplateLNode,
        TemplateRNode, TextElementNode, TextManyNode,
    },
    utils::{TemplateState, ToHir},
};
use dejavu_ir::{
    hir::{DejavuRoot, DejavuSequence, DejavuStatement},
    DejavuError, Result,
};
use yggdrasil_rt::{YggdrasilNode, YggdrasilParser};

mod control_flow;
mod looper;

impl<'i> TextElementNode<'i> {
    pub fn pure_space(&self) -> bool {
        matches!(self, Self::TextSpace { .. })
    }
    pub fn write_buffer(&self, w: &mut String) {
        match self {
            TextElementNode::TextSpace(s) => w.push_str(s.get_str()),
            TextElementNode::TextWord(s) => w.push_str(s.get_str()),
        }
    }
}

pub fn parse_dejavu(input: &str) -> Result<DejavuRoot> {
    let cst = DejavuParser::parse_cst(input, DejavuRule::ROOT).unwrap();
    let root = RootNode::from_cst(cst).unwrap();
    let mut state = TemplateState::default();
    root.to_hir(&mut state)
}

impl<'i> ToHir<'i> for RootNode<'i> {
    type Ir = DejavuRoot;

    fn to_hir(&'i self, state: &mut TemplateState) -> std::result::Result<Self::Ir, DejavuError> {
        let elements = self.element();
        let mut statements = Vec::with_capacity(elements.len());
        for x in elements {
            match x.to_hir(state) {
                Ok(o) => statements.push(o),
                Err(_) => {}
            }
        }
        Ok(DejavuRoot { body: DejavuSequence { statements } })
    }
}

impl<'i> ToHir<'i> for ElementNode<'i> {
    type Ir = DejavuStatement;

    fn to_hir(&'i self, state: &mut TemplateState) -> std::result::Result<Self::Ir, DejavuError> {
        match self {
            ElementNode::TextMany(text) => text.to_hir(state),
            ElementNode::TemplateExport(_) => {
                todo!()
            }
            ElementNode::IfBlock(block) => block.to_hir(state),
            ElementNode::ForBlock(block) => block.to_hir(state),
        }
    }
}

impl<'i> ToHir<'i> for TextManyNode<'i> {
    type Ir = DejavuStatement;

    fn to_hir(&'i self, state: &mut TemplateState) -> std::result::Result<Self::Ir, DejavuError> {
        todo!()
    }
}

impl<'i> ToHir<'i> for IfBlockNode<'i> {
    type Ir = DejavuStatement;

    fn to_hir(&'i self, state: &mut TemplateState) -> std::result::Result<Self::Ir, DejavuError> {
        todo!()
    }
}

impl<'i> ToHir<'i> for ForBlockNode<'i> {
    type Ir = DejavuStatement;

    fn to_hir(&'i self, state: &mut TemplateState) -> std::result::Result<Self::Ir, DejavuError> {
        todo!()
    }
}
