#![allow(dead_code, unused_imports, non_camel_case_types)]
#![allow(missing_docs, rustdoc::missing_crate_level_docs)]
#![allow(clippy::unnecessary_cast)]
#![doc = include_str!("readme.md")]

mod parse_ast;
mod parse_cst;

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
    TextElements,
    TEMPLATE_E,
    TEXT_SPACE,
    TEXT_WORD,
    TEMPLATE_L,
    TEMPLATE_R,
    SPACE_CONTROL,
    KW_END,
    TemplateExport,
    ExportItem,
    KW_EXPORT,
    KW_CLASS,
    KW_TRAIT,
    KW_TO,
    KW_BY,
    TemplateIf,
    IfBegin,
    IfElse,
    IfElseIf,
    IfEnd,
    KW_IF,
    KW_ELSE,
    Atomic,
    String,
    Number,
    NamepathFree,
    Namepath,
    Identifier,
    Boolean,
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
            Self::TextElements => "",
            Self::TEMPLATE_E => "",
            Self::TEXT_SPACE => "",
            Self::TEXT_WORD => "",
            Self::TEMPLATE_L => "",
            Self::TEMPLATE_R => "",
            Self::SPACE_CONTROL => "",
            Self::KW_END => "",
            Self::TemplateExport => "",
            Self::ExportItem => "",
            Self::KW_EXPORT => "",
            Self::KW_CLASS => "",
            Self::KW_TRAIT => "",
            Self::KW_TO => "",
            Self::KW_BY => "",
            Self::TemplateIf => "",
            Self::IfBegin => "",
            Self::IfElse => "",
            Self::IfElseIf => "",
            Self::IfEnd => "",
            Self::KW_IF => "",
            Self::KW_ELSE => "",
            Self::Atomic => "",
            Self::String => "",
            Self::Number => "",
            Self::NamepathFree => "",
            Self::Namepath => "",
            Self::Identifier => "",
            Self::Boolean => "",
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
    TemplateExport(TemplateExportNode),
    TemplateIf(TemplateIfNode),
    TextElements(TextElementsNode),
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
pub struct TemplateLNode {
    pub space_control: SpaceControlNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateRNode {
    pub space_control: SpaceControlNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpaceControlNode {
    SpaceControl0,
    SpaceControl1,
    SpaceControl2,
    SpaceControl3,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwEndNode {
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateExportNode {
    pub export_item: Vec<ExportItemNode>,
    pub template_l: TemplateLNode,
    pub template_r: TemplateRNode,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExportItemNode {
    pub identifier: IdentifierNode,
    pub kw_by: Vec<KwByNode>,
    pub kw_class: KwClassNode,
    pub kw_export: KwExportNode,
    pub kw_to: KwToNode,
    pub kw_trait: KwTraitNode,
    pub namepath_free: Option<NamepathFreeNode>,
    pub class: NamepathFreeNode,
    pub language: IdentifierNode,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwExportNode {
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwClassNode {
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwTraitNode {
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwToNode {
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwByNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateIfNode {
    pub if_begin: IfBeginNode,
    pub if_else: Option<IfElseNode>,
    pub if_else_if: Vec<IfElseIfNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfBeginNode {
    pub atomic: AtomicNode,
    pub kw_if: KwIfNode,
    pub template_l: TemplateLNode,
    pub template_r: TemplateRNode,
    pub text_elements: Vec<TextElementsNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfElseNode {
    pub kw_else: KwElseNode,
    pub template_l: TemplateLNode,
    pub template_r: TemplateRNode,
    pub text_elements: Vec<TextElementsNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfElseIfNode {
    pub kw_else: KwElseNode,
    pub kw_if: KwIfNode,
    pub template_l: TemplateLNode,
    pub template_r: TemplateRNode,
    pub text_elements: Vec<TextElementsNode>,
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
pub struct KwIfNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwElseNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AtomicNode {
    Boolean(BooleanNode),
    Identifier(IdentifierNode),
    Number(NumberNode),
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StringNode {
    String0,
    String1,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberNode {
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamepathFreeNode {
    pub identifier: Vec<IdentifierNode>,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamepathNode {
    pub identifier: Vec<IdentifierNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BooleanNode {
    Boolean0,
    Boolean1,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhiteSpaceNode {
    pub span: Range<u32>,
}
