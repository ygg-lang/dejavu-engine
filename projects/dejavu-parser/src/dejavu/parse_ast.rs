#![allow(unused_variables)]
use super::*;
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RootNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Root)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::Root
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> RootNode<'i> {
    pub fn element(&self) -> Vec<ElementNode<'i>> {
        self.pair.take_tagged_items("element").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ElementNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Element)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Err(YggdrasilError::invalid_node(DejavuRule::Element, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::Element
    }

    fn get_str(&self) -> &'i str {
        match self {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {}
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TextManyNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TextMany)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TextMany
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> TextManyNode<'i> {
    pub fn text_element(&self) -> Vec<TextElementNode<'i>> {
        self.pair.take_tagged_items("text_element").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TextElementNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TextElement)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Err(YggdrasilError::invalid_node(DejavuRule::TextElement, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TextElement
    }

    fn get_str(&self) -> &'i str {
        match self {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {}
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TemplateENode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEMPLATE_E)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TEMPLATE_E
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> TemplateENode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TextSpaceNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEXT_SPACE)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TEXT_SPACE
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> TextSpaceNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TextWordNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEXT_WORD)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TEXT_WORD
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> TextWordNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TemplateLNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEMPLATE_L)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TEMPLATE_L
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> TemplateLNode<'i> {
    pub fn space_control(&self) -> Option<SpaceControlNode<'i>> {
        self.pair.take_tagged_option("space_control")
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TemplateRNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEMPLATE_R)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TEMPLATE_R
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> TemplateRNode<'i> {
    pub fn space_control(&self) -> Option<SpaceControlNode<'i>> {
        self.pair.take_tagged_option("space_control")
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for SpaceControlNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::SpaceControl)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Err(YggdrasilError::invalid_node(DejavuRule::SpaceControl, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::SpaceControl
    }

    fn get_str(&self) -> &'i str {
        match self {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {}
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwEndNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_END)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::KW_END
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> KwEndNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TemplateExportNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TemplateExport)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TemplateExport
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> TemplateExportNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ExportItemNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::ExportItem)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::ExportItem
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> ExportItemNode<'i> {
    pub fn class(&self) -> Option<NamepathFreeNode<'i>> {
        self.pair.take_tagged_option("class")
    }
    pub fn language(&self) -> Option<IdentifierNode<'i>> {
        self.pair.take_tagged_option("language")
    }
    pub fn name(&self) -> IdentifierNode<'i> {
        self.pair.take_tagged_one("name").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwExportNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_EXPORT)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::KW_EXPORT
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> KwExportNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwClassNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_CLASS)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::KW_CLASS
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> KwClassNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwTraitNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_TRAIT)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::KW_TRAIT
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> KwTraitNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwToNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_TO)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::KW_TO
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> KwToNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwByNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_BY)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::KW_BY
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> KwByNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TemplateIfNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TemplateIf)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TemplateIf
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> TemplateIfNode<'i> {
    pub fn if_begin(&self) -> IfBeginNode<'i> {
        self.pair.take_tagged_one("if_begin").unwrap()
    }
    pub fn if_else(&self) -> Option<IfElseNode<'i>> {
        self.pair.take_tagged_option("if_else")
    }
    pub fn if_else_if(&self) -> Vec<IfElseIfNode<'i>> {
        self.pair.take_tagged_items("if_else_if").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn if_end(&self) -> IfEndNode<'i> {
        self.pair.take_tagged_one("if_end").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for IfBeginNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::IfBegin)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::IfBegin
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> IfBeginNode<'i> {
    pub fn element(&self) -> Vec<ElementNode<'i>> {
        self.pair.take_tagged_items("element").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn expression(&self) -> ExpressionNode<'i> {
        self.pair.take_tagged_one("expression").unwrap()
    }
    pub fn template_l(&self) -> TemplateLNode<'i> {
        self.pair.take_tagged_one("template_l").unwrap()
    }
    pub fn template_r(&self) -> TemplateRNode<'i> {
        self.pair.take_tagged_one("template_r").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for IfElseNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::IfElse)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::IfElse
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> IfElseNode<'i> {
    pub fn element(&self) -> Vec<ElementNode<'i>> {
        self.pair.take_tagged_items("element").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn template_l(&self) -> TemplateLNode<'i> {
        self.pair.take_tagged_one("template_l").unwrap()
    }
    pub fn template_r(&self) -> TemplateRNode<'i> {
        self.pair.take_tagged_one("template_r").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for IfElseIfNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::IfElseIf)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::IfElseIf
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> IfElseIfNode<'i> {
    pub fn element(&self) -> Vec<ElementNode<'i>> {
        self.pair.take_tagged_items("element").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn expression(&self) -> ExpressionNode<'i> {
        self.pair.take_tagged_one("expression").unwrap()
    }
    pub fn template_l(&self) -> TemplateLNode<'i> {
        self.pair.take_tagged_one("template_l").unwrap()
    }
    pub fn template_r(&self) -> TemplateRNode<'i> {
        self.pair.take_tagged_one("template_r").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for IfEndNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::IfEnd)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::IfEnd
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> IfEndNode<'i> {
    pub fn template_l(&self) -> TemplateLNode<'i> {
        self.pair.take_tagged_one("template_l").unwrap()
    }
    pub fn template_r(&self) -> TemplateRNode<'i> {
        self.pair.take_tagged_one("template_r").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwIfNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_IF)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::KW_IF
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> KwIfNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwElseNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_ELSE)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::KW_ELSE
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> KwElseNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TemplateForNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TemplateFor)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TemplateFor
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> TemplateForNode<'i> {
    pub fn for_begin(&self) -> ForBeginNode<'i> {
        self.pair.take_tagged_one("for_begin").unwrap()
    }
    pub fn for_else(&self) -> Option<ForElseNode<'i>> {
        self.pair.take_tagged_option("for_else")
    }
    pub fn for_end(&self) -> ForEndNode<'i> {
        self.pair.take_tagged_one("for_end").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ForBeginNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::ForBegin)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::ForBegin
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> ForBeginNode<'i> {
    pub fn element(&self) -> Vec<ElementNode<'i>> {
        self.pair.take_tagged_items("element").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn kw_in(&self) -> Vec<KwInNode<'i>> {
        self.pair.take_tagged_items("kw_in").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn pattern(&self) -> PatternNode<'i> {
        self.pair.take_tagged_one("pattern").unwrap()
    }
    pub fn template_l(&self) -> TemplateLNode<'i> {
        self.pair.take_tagged_one("template_l").unwrap()
    }
    pub fn template_r(&self) -> TemplateRNode<'i> {
        self.pair.take_tagged_one("template_r").unwrap()
    }
    pub fn condition(&self) -> Option<ExpressionNode<'i>> {
        self.pair.take_tagged_option("condition")
    }
    pub fn iterator(&self) -> ExpressionNode<'i> {
        self.pair.take_tagged_one("iterator").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ForElseNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::ForElse)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::ForElse
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> ForElseNode<'i> {
    pub fn element(&self) -> Vec<ElementNode<'i>> {
        self.pair.take_tagged_items("element").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn template_l(&self) -> TemplateLNode<'i> {
        self.pair.take_tagged_one("template_l").unwrap()
    }
    pub fn template_r(&self) -> TemplateRNode<'i> {
        self.pair.take_tagged_one("template_r").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ForEndNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::ForEnd)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::ForEnd
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> ForEndNode<'i> {
    pub fn template_l(&self) -> TemplateLNode<'i> {
        self.pair.take_tagged_one("template_l").unwrap()
    }
    pub fn template_r(&self) -> TemplateRNode<'i> {
        self.pair.take_tagged_one("template_r").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwForNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_FOR)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::KW_FOR
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> KwForNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwInNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_IN)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::KW_IN
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> KwInNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for PatternNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Pattern)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Err(YggdrasilError::invalid_node(DejavuRule::Pattern, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::Pattern
    }

    fn get_str(&self) -> &'i str {
        match self {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {}
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for BarePatternNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::BarePattern)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::BarePattern
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> BarePatternNode<'i> {
    pub fn identifier(&self) -> Vec<IdentifierNode<'i>> {
        self.pair.take_tagged_items("identifier").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ExpressionNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Expression)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::Expression
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> ExpressionNode<'i> {
    pub fn expression_rest(&self) -> Vec<ExpressionRestNode<'i>> {
        self.pair.take_tagged_items("expression_rest").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn term(&self) -> TermNode<'i> {
        self.pair.take_tagged_one("term").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ExpressionRestNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::ExpressionRest)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::ExpressionRest
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> ExpressionRestNode<'i> {
    pub fn infix(&self) -> InfixNode<'i> {
        self.pair.take_tagged_one("infix").unwrap()
    }
    pub fn term(&self) -> TermNode<'i> {
        self.pair.take_tagged_one("term").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for InfixNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Infix)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Err(YggdrasilError::invalid_node(DejavuRule::Infix, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::Infix
    }

    fn get_str(&self) -> &'i str {
        match self {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {}
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TermNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Term)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::Term
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> TermNode<'i> {
    pub fn atomic(&self) -> AtomicNode<'i> {
        self.pair.take_tagged_one("atomic").unwrap()
    }
    pub fn prefix(&self) -> Vec<PrefixNode<'i>> {
        self.pair.take_tagged_items("prefix").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn suffix(&self) -> Vec<SuffixNode<'i>> {
        self.pair.take_tagged_items("suffix").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for PrefixNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Prefix)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Err(YggdrasilError::invalid_node(DejavuRule::Prefix, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::Prefix
    }

    fn get_str(&self) -> &'i str {
        match self {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {}
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for SuffixNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Suffix)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Err(YggdrasilError::invalid_node(DejavuRule::Suffix, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::Suffix
    }

    fn get_str(&self) -> &'i str {
        match self {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {}
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for AtomicNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Atomic)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Err(YggdrasilError::invalid_node(DejavuRule::Atomic, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::Atomic
    }

    fn get_str(&self) -> &'i str {
        match self {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {}
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StringNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::String)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Err(YggdrasilError::invalid_node(DejavuRule::String, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::String
    }

    fn get_str(&self) -> &'i str {
        match self {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {}
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for NumberNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Number)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Err(YggdrasilError::invalid_node(DejavuRule::Number, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::Number
    }

    fn get_str(&self) -> &'i str {
        match self {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {}
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for DigitsNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Digits)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::Digits
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> DigitsNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for UnitNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Unit)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::Unit
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> UnitNode<'i> {
    pub fn identifier(&self) -> IdentifierNode<'i> {
        self.pair.take_tagged_one("identifier").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for BinNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::BIN)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::BIN
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> BinNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for OctNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::OCT)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::OCT
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> OctNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for HexNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::HEX)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::HEX
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> HexNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for NamepathFreeNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::NamepathFree)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::NamepathFree
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> NamepathFreeNode<'i> {
    pub fn identifier(&self) -> Vec<IdentifierNode<'i>> {
        self.pair.take_tagged_items("identifier").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for NamepathNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Namepath)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::Namepath
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> NamepathNode<'i> {
    pub fn identifier(&self) -> Vec<IdentifierNode<'i>> {
        self.pair.take_tagged_items("identifier").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for IdentifierNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Identifier)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::Identifier
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> IdentifierNode<'i> {
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for BooleanNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Boolean)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Err(YggdrasilError::invalid_node(DejavuRule::Boolean, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::Boolean
    }

    fn get_str(&self) -> &'i str {
        match self {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {}
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for WhiteSpaceNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::WhiteSpace)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::WhiteSpace
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}

impl<'i> WhiteSpaceNode<'i> {
}
