#![allow(dead_code, unused_imports, non_camel_case_types)]
#![allow(missing_docs, rustdoc::missing_crate_level_docs)]
#![allow(clippy::unnecessary_cast)]
#![doc = include_str!("readme.md")]

mod parse_ast;
mod parse_cst;

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
    ARGUEMENT,
    ATOMIC,
    BIN,
    BARE_PATTERN,
    COLON,
    COMMA,
    DEC,
    DOT,
    DOT_CALL,
    ELEMENT,
    EXPORT_ITEM,
    EXPRESSION,
    EXPRESSION_REST,
    FOR_BLOCK,
    FUNCTION_CALL,
    GROUP_EXPRESSION,
    HEX,
    IDENTIFIER,
    IF_BLOCK,
    INFIX,
    INFIX0,
    INFIX1,
    KW_BY,
    KW_CLASS,
    KW_ELSE,
    KW_END,
    KW_EXPORT,
    KW_FOR,
    KW_IF,
    KW_IN,
    KW_TO,
    KW_TRAIT,
    KEY,
    NAMEPATH,
    NAMEPATH_FREE,
    NUMBER,
    OCT,
    PATTERN,
    PREFIX,
    PREFIX0,
    ROOT,
    SPACE_CONTROL,
    SPACE_CONTROL0,
    SPACE_CONTROL1,
    SPACE_CONTROL2,
    SPACE_CONTROL3,
    SPACE_CONTROL4,
    SPECIAL,
    SPECIAL0,
    SPECIAL1,
    SPECIAL2,
    STRING,
    STRING0,
    STRING1,
    SUFFIX,
    SUFFIX0,
    TEMPLATE_L,
    TEMPLATE_R,
    TEXT_SPACE,
    TEXT_WORD,
    TEMPLATE_ELSE,
    TEMPLATE_ELSE_IF,
    TEMPLATE_END,
    TEMPLATE_EXPORT,
    TEMPLATE_FOR,
    TEMPLATE_IF,
    TERM,
    TEXT_ELEMENT,
    TEXT_MANY,
    TUPLE,
    UNIT,
    WHITE_SPACE,
    /// Label for unnamed text literal
    HiddenText,
}

impl YggdrasilRule for DejavuRule {
    fn is_ignore(&self) -> bool {
        matches!(self, Self::HiddenText | Self::WHITE_SPACE)
    }

    fn get_style(&self) -> &'static str {
        match self {
            Self::ROOT => "",
            Self::ELEMENT => "",
            Self::TEXT_MANY => "",
            Self::TEXT_ELEMENT => "",
            Self::TEMPLATE_EXPORT => "",
            Self::EXPORT_ITEM => "",
            Self::IF_BLOCK => "",
            Self::TEMPLATE_IF => "",
            Self::TEMPLATE_ELSE_IF => "",
            Self::TEMPLATE_ELSE => "",
            Self::FOR_BLOCK => "",
            Self::TEMPLATE_FOR => "",
            Self::TEMPLATE_END => "",
            Self::PATTERN => "",
            Self::BARE_PATTERN => "",
            Self::EXPRESSION => "",
            Self::EXPRESSION_REST => "",
            Self::INFIX => "",
            Self::TERM => "",
            Self::PREFIX => "",
            Self::SUFFIX => "",
            Self::FUNCTION_CALL => "",
            Self::DOT_CALL => "",
            Self::TUPLE => "",
            Self::ARGUEMENT => "",
            Self::KEY => "",
            Self::ATOMIC => "",
            Self::GROUP_EXPRESSION => "",
            Self::SPECIAL => "",
            Self::NUMBER => "",
            Self::DEC => "",
            Self::BIN => "",
            Self::OCT => "",
            Self::HEX => "",
            Self::UNIT => "",
            Self::STRING => "",
            Self::NAMEPATH_FREE => "",
            Self::NAMEPATH => "",
            Self::IDENTIFIER => "",
            Self::KW_EXPORT => "",
            Self::KW_CLASS => "",
            Self::KW_TRAIT => "",
            Self::KW_TO => "",
            Self::KW_BY => "",
            Self::KW_FOR => "",
            Self::KW_IN => "",
            Self::KW_IF => "",
            Self::KW_ELSE => "",
            Self::KW_END => "",
            Self::TEMPLATE_L => "",
            Self::TEMPLATE_R => "",
            Self::SPACE_CONTROL => "",
            Self::DOT => "",
            Self::COMMA => "",
            Self::COLON => "",
            Self::TEXT_SPACE => "",
            Self::TEXT_WORD => "",
            Self::WHITE_SPACE => "",
            Self::INFIX0 => "",
            Self::INFIX1 => "",
            Self::PREFIX0 => "",
            Self::SUFFIX0 => "",
            Self::SPECIAL0 => "",
            Self::SPECIAL1 => "",
            Self::SPECIAL2 => "",
            Self::STRING0 => "",
            Self::STRING1 => "",
            Self::SPACE_CONTROL0 => "",
            Self::SPACE_CONTROL1 => "",
            Self::SPACE_CONTROL2 => "",
            Self::SPACE_CONTROL3 => "",
            Self::SPACE_CONTROL4 => "",
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
    TextMany(TextManyNode<'i>),
    TemplateExport(TemplateExportNode<'i>),
    IfBlock(IfBlockNode<'i>),
    ForBlock(ForBlockNode<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextManyNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextElementNode<'i> {
    TextSpace(TextSpaceNode<'i>),
    TextWord(TextWordNode<'i>),
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
pub struct IfBlockNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateIfNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateElseIfNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateElseNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForBlockNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateForNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateEndNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PatternNode<'i> {
    BarePattern(BarePatternNode<'i>),
    Identifier(IdentifierNode<'i>),
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
    Add(Infix0Node<'i>),
    Mul(Infix1Node<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TermNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrefixNode<'i> {
    Not(Prefix0Node<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SuffixNode<'i> {
    Null(Suffix0Node<'i>),
    DotCall(DotCallNode<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionCallNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DotCallNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TupleNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArguementNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KeyNode<'i> {
    Identifier(IdentifierNode<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AtomicNode<'i> {
    GroupExpression(GroupExpressionNode<'i>),
    Tuple(TupleNode<'i>),
    Special(SpecialNode<'i>),
    FunctionCall(FunctionCallNode<'i>),
    Identifier(IdentifierNode<'i>),
    Number(NumberNode<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupExpressionNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpecialNode<'i> {
    Null(Special0Node<'i>),
    True(Special1Node<'i>),
    False(Special2Node<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NumberNode<'i> {
    Dec(DecNode<'i>),
    Bin(BinNode<'i>),
    Hex(HexNode<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecNode<'i> {
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
pub struct UnitNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StringNode<'i> {
    DoubleQuote(String0Node<'i>),
    SingleQuote(String1Node<'i>),
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
pub struct KwEndNode<'i> {
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
    Nothing(SpaceControl0Node<'i>),
    Break0(SpaceControl1Node<'i>),
    Break1(SpaceControl2Node<'i>),
    Delete0(SpaceControl3Node<'i>),
    Delete1(SpaceControl4Node<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DotNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommaNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColonNode<'i> {
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
pub struct WhiteSpaceNode<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Infix0Node<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Infix1Node<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prefix0Node<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Suffix0Node<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Special0Node<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Special1Node<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Special2Node<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct String0Node<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct String1Node<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpaceControl0Node<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpaceControl1Node<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpaceControl2Node<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpaceControl3Node<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpaceControl4Node<'i> {
    pair: TokenPair<'i, DejavuRule>,
}
