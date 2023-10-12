#![allow(dead_code, unused_imports, non_camel_case_types)]
#![allow(missing_docs, rustdoc::missing_crate_level_docs)]
#![allow(clippy::unnecessary_cast)]
#![doc = include_str!("readme.md")]

mod parse_cst;
mod parse_ast;

use std::{borrow::Cow, ops::Range, sync::OnceLock};
use yggdrasil_rt::*;

type Input<'i> = Box<State<'i, DejavuRule>>;
type Output<'i> = Result<Box<State<'i, DejavuRule>>, Box<State<'i, DejavuRule>>>;

#[doc = include_str!("railway.min.svg")]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DejavuParser {}

impl YggdrasilParser for DejavuParser {
    type Rule = DejavuRule;
    fn parse_cst(input: &str, rule: Self::Rule) -> OutputResult<DejavuRule> {
        self::parse_cst::parse_cst(input, rule)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DejavuRule {
    Root,
    Element,
    TemplateIf,
    IfBegin,
    IfElse,
    IfElseIf,
    IfEnd,
    KW_END,
    KW_IF,
    TEMPLATE_L,
    TEMPLATE_R,
    TextElements,
    TEMPLATE_E,
    TEXT_SPACE,
    TEXT_WORD,
    WhiteSpace,
    /// Label for text literal
    IgnoreText,
    /// Label for regex literal
    IgnoreRegex,
}

impl YggdrasilRule for DejavuRule {
    fn is_ignore(&self) -> bool {
        matches!(self, Self::IgnoreText | Self::IgnoreRegex | Self::WhiteSpace)
    }

    fn get_style(&self) -> &'static str {
        match self {
            Self::Root => "",
            Self::Element => "",
            Self::TemplateIf => "",
            Self::IfBegin => "",
            Self::IfElse => "",
            Self::IfElseIf => "",
            Self::IfEnd => "",
            Self::KW_END => "",
            Self::KW_IF => "",
            Self::TEMPLATE_L => "",
            Self::TEMPLATE_R => "",
            Self::TextElements => "",
            Self::TEMPLATE_E => "",
            Self::TEXT_SPACE => "",
            Self::TEXT_WORD => "",
            Self::WhiteSpace => "",
            _ => "",
        }
    }
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RootNode {
    pub element: Vec<ElementNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ElementNode {
    TemplateIf(TemplateIfNode),
    TextElements(TextElementsNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateIfNode {
    pub if_begin: IfBeginNode,
    pub if_end: IfEndNode,
    pub text_elements: Vec<TextElementsNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfBeginNode {
    pub kw_end: KwEndNode,
    pub kw_if: Option<KwIfNode>,
    pub template_l: TemplateLNode,
    pub template_r: TemplateRNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfElseNode {
    pub kw_end: KwEndNode,
    pub kw_if: Option<KwIfNode>,
    pub template_l: TemplateLNode,
    pub template_r: TemplateRNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfElseIfNode {
    pub kw_end: KwEndNode,
    pub kw_if: Option<KwIfNode>,
    pub template_l: TemplateLNode,
    pub template_r: TemplateRNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfEndNode {
    pub kw_end: KwEndNode,
    pub kw_if: Option<KwIfNode>,
    pub template_l: TemplateLNode,
    pub template_r: TemplateRNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwEndNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwIfNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateLNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateRNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextElementsNode {
    TemplateE(TemplateENode),
    TextSpace(TextSpaceNode),
    TextWord(TextWordNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateENode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextSpaceNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextWordNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhiteSpaceNode {
    pub span: Range<u32>,
}
