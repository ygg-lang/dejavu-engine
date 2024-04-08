#![allow(dead_code, unused_imports, non_camel_case_types)]
#![allow(missing_docs, rustdoc::missing_crate_level_docs)]
#![allow(clippy::unnecessary_cast)]
#![doc = include_str!("readme.md")]

mod parse_cst;
mod parse_ast;

use core::str::FromStr;
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
    TextMany,
    TextElement,
    TEMPLATE_E,
    TEXT_SPACE,
    TEXT_WORD,
    TEMPLATE_L,
    TEMPLATE_R,
    SpaceControl,
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
    TemplateFor,
    ForBegin,
    ForElse,
    ForEnd,
    KW_FOR,
    KW_IN,
    Pattern,
    BarePattern,
    Expression,
    ExpressionRest,
    Infix,
    Term,
    Prefix,
    Suffix,
    Atomic,
    String,
    Number,
    Digits,
    Unit,
    BIN,
    OCT,
    HEX,
    NamepathFree,
    Namepath,
    Identifier,
    Boolean,
    WhiteSpace,
    /// Label for unnamed text literal
    HiddenText,
}

impl YggdrasilRule for DejavuRule {
    fn is_ignore(&self) -> bool {
        matches!(self, Self::HiddenText)
    }

    fn get_style(&self) -> &'static str {
        match self {
            Self::Root => "",
            Self::Element => "",
            Self::TextMany => "",
            Self::TextElement => "",
            Self::TEMPLATE_E => "",
            Self::TEXT_SPACE => "",
            Self::TEXT_WORD => "",
            Self::TEMPLATE_L => "",
            Self::TEMPLATE_R => "",
            Self::SpaceControl => "",
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
            Self::TemplateFor => "",
            Self::ForBegin => "",
            Self::ForElse => "",
            Self::ForEnd => "",
            Self::KW_FOR => "",
            Self::KW_IN => "",
            Self::Pattern => "",
            Self::BarePattern => "",
            Self::Expression => "",
            Self::ExpressionRest => "",
            Self::Infix => "",
            Self::Term => "",
            Self::Prefix => "",
            Self::Suffix => "",
            Self::Atomic => "",
            Self::String => "",
            Self::Number => "",
            Self::Digits => "",
            Self::Unit => "",
            Self::BIN => "",
            Self::OCT => "",
            Self::HEX => "",
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
pub struct RootNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ElementNode<'i> {
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextManyNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextElementNode<'i> {
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateENode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextSpaceNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextWordNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateLNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateRNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpaceControlNode<'i> {
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwEndNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateExportNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExportItemNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwExportNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwClassNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwTraitNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwToNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwByNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateIfNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfBeginNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfElseNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfElseIfNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfEndNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwIfNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwElseNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateForNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForBeginNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForElseNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForEndNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwForNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwInNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PatternNode<'i> {
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BarePatternNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionRestNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InfixNode<'i> {
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TermNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrefixNode<'i> {
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SuffixNode<'i> {
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AtomicNode<'i> {
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StringNode<'i> {
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NumberNode<'i> {
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DigitsNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnitNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OctNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HexNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamepathFreeNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamepathNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BooleanNode<'i> {
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhiteSpaceNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
