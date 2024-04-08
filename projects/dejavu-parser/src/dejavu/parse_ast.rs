#![allow(unused_variables)]
use super::*;
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RootNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::ROOT)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::ROOT
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
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::ELEMENT)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("text_many") {
            return Ok(Self::TextMany(s));
        }
        if let Ok(s) = pair.take_tagged_one("template_export") {
            return Ok(Self::TemplateExport(s));
        }
        if let Ok(s) = pair.take_tagged_one("if_block") {
            return Ok(Self::IfBlock(s));
        }
        if let Ok(s) = pair.take_tagged_one("for_block") {
            return Ok(Self::ForBlock(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::ELEMENT, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::ELEMENT
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::TextMany(s) => s.get_str(),
            Self::TemplateExport(s) => s.get_str(),
            Self::IfBlock(s) => s.get_str(),
            Self::ForBlock(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::TextMany(s) => s.get_range(),
            Self::TemplateExport(s) => s.get_range(),
            Self::IfBlock(s) => s.get_range(),
            Self::ForBlock(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TextManyNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEXT_MANY)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TEXT_MANY
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
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEXT_ELEMENT)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("text_space") {
            return Ok(Self::TextSpace(s));
        }
        if let Ok(s) = pair.take_tagged_one("text_word") {
            return Ok(Self::TextWord(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::TEXT_ELEMENT, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TEXT_ELEMENT
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::TextSpace(s) => s.get_str(),
            Self::TextWord(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::TextSpace(s) => s.get_range(),
            Self::TextWord(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TemplateExportNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEMPLATE_EXPORT)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TEMPLATE_EXPORT
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> TemplateExportNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ExportItemNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::EXPORT_ITEM)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::EXPORT_ITEM
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
impl<'i> YggdrasilNode<'i> for IfBlockNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::IF_BLOCK)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::IF_BLOCK
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> IfBlockNode<'i> {
    pub fn template_else(&self) -> Option<TemplateElseNode<'i>> {
        self.pair.take_tagged_option("template_else")
    }
    pub fn template_else_if(&self) -> Vec<TemplateElseIfNode<'i>> {
        self.pair.take_tagged_items("template_else_if").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn template_end(&self) -> TemplateEndNode<'i> {
        self.pair.take_tagged_one("template_end").unwrap()
    }
    pub fn template_if(&self) -> TemplateIfNode<'i> {
        self.pair.take_tagged_one("template_if").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TemplateIfNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEMPLATE_IF)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TEMPLATE_IF
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> TemplateIfNode<'i> {
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
impl<'i> YggdrasilNode<'i> for TemplateElseIfNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEMPLATE_ELSE_IF)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TEMPLATE_ELSE_IF
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> TemplateElseIfNode<'i> {
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
impl<'i> YggdrasilNode<'i> for TemplateElseNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEMPLATE_ELSE)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TEMPLATE_ELSE
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> TemplateElseNode<'i> {
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
impl<'i> YggdrasilNode<'i> for ForBlockNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::FOR_BLOCK)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::FOR_BLOCK
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ForBlockNode<'i> {
    pub fn template_else(&self) -> Option<TemplateElseNode<'i>> {
        self.pair.take_tagged_option("template_else")
    }
    pub fn template_end(&self) -> TemplateEndNode<'i> {
        self.pair.take_tagged_one("template_end").unwrap()
    }
    pub fn template_for(&self) -> TemplateForNode<'i> {
        self.pair.take_tagged_one("template_for").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TemplateForNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEMPLATE_FOR)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TEMPLATE_FOR
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> TemplateForNode<'i> {
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
impl<'i> YggdrasilNode<'i> for TemplateEndNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEMPLATE_END)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TEMPLATE_END
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> TemplateEndNode<'i> {
    pub fn identifier(&self) -> Option<IdentifierNode<'i>> {
        self.pair.take_tagged_option("identifier")
    }
    pub fn template_l(&self) -> TemplateLNode<'i> {
        self.pair.take_tagged_one("template_l").unwrap()
    }
    pub fn template_r(&self) -> TemplateRNode<'i> {
        self.pair.take_tagged_one("template_r").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for PatternNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::PATTERN)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("bare_pattern") {
            return Ok(Self::BarePattern(s));
        }
        if let Ok(s) = pair.take_tagged_one("identifier") {
            return Ok(Self::Identifier(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::PATTERN, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::PATTERN
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::BarePattern(s) => s.get_str(),
            Self::Identifier(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::BarePattern(s) => s.get_range(),
            Self::Identifier(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for BarePatternNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::BARE_PATTERN)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::BARE_PATTERN
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> BarePatternNode<'i> {
    pub fn comma(&self) -> Vec<CommaNode<'i>> {
        self.pair.take_tagged_items("comma").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn pattern(&self) -> Vec<PatternNode<'i>> {
        self.pair.take_tagged_items("pattern").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ExpressionNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::EXPRESSION)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::EXPRESSION
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
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::EXPRESSION_REST)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::EXPRESSION_REST
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
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::INFIX)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("add") {
            return Ok(Self::Add(s));
        }
        if let Ok(s) = pair.take_tagged_one("mul") {
            return Ok(Self::Mul(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::INFIX, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::INFIX
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::Add(s) => s.get_str(),
            Self::Mul(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Add(s) => s.get_range(),
            Self::Mul(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TermNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TERM)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TERM
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
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::PREFIX)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("not") {
            return Ok(Self::Not(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::PREFIX, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::PREFIX
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::Not(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Not(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for SuffixNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::SUFFIX)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("null") {
            return Ok(Self::Null(s));
        }
        if let Ok(s) = pair.take_tagged_one("dot_call") {
            return Ok(Self::DotCall(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::SUFFIX, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::SUFFIX
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::Null(s) => s.get_str(),
            Self::DotCall(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Null(s) => s.get_range(),
            Self::DotCall(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for FunctionCallNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::FUNCTION_CALL)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::FUNCTION_CALL
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> FunctionCallNode<'i> {
    pub fn namepath(&self) -> NamepathNode<'i> {
        self.pair.take_tagged_one("namepath").unwrap()
    }
    pub fn tuple(&self) -> Option<TupleNode<'i>> {
        self.pair.take_tagged_option("tuple")
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for DotCallNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::DOT_CALL)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::DOT_CALL
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> DotCallNode<'i> {
    pub fn identifier(&self) -> IdentifierNode<'i> {
        self.pair.take_tagged_one("identifier").unwrap()
    }
    pub fn tuple(&self) -> Option<TupleNode<'i>> {
        self.pair.take_tagged_option("tuple")
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TupleNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TUPLE)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::TUPLE
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> TupleNode<'i> {
    pub fn arguement(&self) -> Option<ArguementNode<'i>> {
        self.pair.take_tagged_option("arguement")
    }
    pub fn comma(&self) -> Vec<CommaNode<'i>> {
        self.pair.take_tagged_items("comma").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn expression(&self) -> Vec<ExpressionNode<'i>> {
        self.pair.take_tagged_items("expression").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ArguementNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::ARGUEMENT)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::ARGUEMENT
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ArguementNode<'i> {
    pub fn colon(&self) -> ColonNode<'i> {
        self.pair.take_tagged_one("colon").unwrap()
    }
    pub fn expression(&self) -> ExpressionNode<'i> {
        self.pair.take_tagged_one("expression").unwrap()
    }
    pub fn key(&self) -> KeyNode<'i> {
        self.pair.take_tagged_one("key").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KeyNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KEY)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("identifier") {
            return Ok(Self::Identifier(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::KEY, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::KEY
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::Identifier(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Identifier(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for AtomicNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::ATOMIC)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("group_expression") {
            return Ok(Self::GroupExpression(s));
        }
        if let Ok(s) = pair.take_tagged_one("tuple") {
            return Ok(Self::Tuple(s));
        }
        if let Ok(s) = pair.take_tagged_one("special") {
            return Ok(Self::Special(s));
        }
        if let Ok(s) = pair.take_tagged_one("function_call") {
            return Ok(Self::FunctionCall(s));
        }
        if let Ok(s) = pair.take_tagged_one("identifier") {
            return Ok(Self::Identifier(s));
        }
        if let Ok(s) = pair.take_tagged_one("number") {
            return Ok(Self::Number(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::ATOMIC, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::ATOMIC
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::GroupExpression(s) => s.get_str(),
            Self::Tuple(s) => s.get_str(),
            Self::Special(s) => s.get_str(),
            Self::FunctionCall(s) => s.get_str(),
            Self::Identifier(s) => s.get_str(),
            Self::Number(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::GroupExpression(s) => s.get_range(),
            Self::Tuple(s) => s.get_range(),
            Self::Special(s) => s.get_range(),
            Self::FunctionCall(s) => s.get_range(),
            Self::Identifier(s) => s.get_range(),
            Self::Number(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GroupExpressionNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::GROUP_EXPRESSION)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::GROUP_EXPRESSION
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> GroupExpressionNode<'i> {
    pub fn expression(&self) -> ExpressionNode<'i> {
        self.pair.take_tagged_one("expression").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for SpecialNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::SPECIAL)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("null") {
            return Ok(Self::Null(s));
        }
        if let Ok(s) = pair.take_tagged_one("true") {
            return Ok(Self::True(s));
        }
        if let Ok(s) = pair.take_tagged_one("false") {
            return Ok(Self::False(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::SPECIAL, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::SPECIAL
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::Null(s) => s.get_str(),
            Self::True(s) => s.get_str(),
            Self::False(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Null(s) => s.get_range(),
            Self::True(s) => s.get_range(),
            Self::False(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for NumberNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::NUMBER)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("dec") {
            return Ok(Self::Dec(s));
        }
        if let Ok(s) = pair.take_tagged_one("bin") {
            return Ok(Self::Bin(s));
        }
        if let Ok(s) = pair.take_tagged_one("hex") {
            return Ok(Self::Hex(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::NUMBER, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::NUMBER
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::Dec(s) => s.get_str(),
            Self::Bin(s) => s.get_str(),
            Self::Hex(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Dec(s) => s.get_range(),
            Self::Bin(s) => s.get_range(),
            Self::Hex(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for DecNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::DEC)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::DEC
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> DecNode<'i> {}
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
impl<'i> BinNode<'i> {}
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
impl<'i> OctNode<'i> {}
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
impl<'i> HexNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for UnitNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::UNIT)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::UNIT
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
impl<'i> YggdrasilNode<'i> for StringNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::STRING)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("double_quote") {
            return Ok(Self::DoubleQuote(s));
        }
        if let Ok(s) = pair.take_tagged_one("single_quote") {
            return Ok(Self::SingleQuote(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::STRING, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::STRING
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::DoubleQuote(s) => s.get_str(),
            Self::SingleQuote(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::DoubleQuote(s) => s.get_range(),
            Self::SingleQuote(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for NamepathFreeNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::NAMEPATH_FREE)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::NAMEPATH_FREE
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
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::NAMEPATH)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::NAMEPATH
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
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::IDENTIFIER)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::IDENTIFIER
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> IdentifierNode<'i> {}
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
impl<'i> KwExportNode<'i> {}
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
impl<'i> KwClassNode<'i> {}
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
impl<'i> KwTraitNode<'i> {}
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
impl<'i> KwToNode<'i> {}
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
impl<'i> KwByNode<'i> {}
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
impl<'i> KwForNode<'i> {}
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
impl<'i> KwInNode<'i> {}
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
impl<'i> KwIfNode<'i> {}
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
impl<'i> KwElseNode<'i> {}
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
impl<'i> KwEndNode<'i> {}
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
impl<'i> TemplateLNode<'i> {}
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
impl<'i> TemplateRNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for SpaceControlNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::SPACE_CONTROL)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("nothing") {
            return Ok(Self::Nothing(s));
        }
        if let Ok(s) = pair.take_tagged_one("break_0") {
            return Ok(Self::Break0(s));
        }
        if let Ok(s) = pair.take_tagged_one("break_1") {
            return Ok(Self::Break1(s));
        }
        if let Ok(s) = pair.take_tagged_one("delete_0") {
            return Ok(Self::Delete0(s));
        }
        if let Ok(s) = pair.take_tagged_one("delete_1") {
            return Ok(Self::Delete1(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::SPACE_CONTROL, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::SPACE_CONTROL
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::Nothing(s) => s.get_str(),
            Self::Break0(s) => s.get_str(),
            Self::Break1(s) => s.get_str(),
            Self::Delete0(s) => s.get_str(),
            Self::Delete1(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Nothing(s) => s.get_range(),
            Self::Break0(s) => s.get_range(),
            Self::Break1(s) => s.get_range(),
            Self::Delete0(s) => s.get_range(),
            Self::Delete1(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for DotNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::DOT)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::DOT
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> DotNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for CommaNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::COMMA)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::COMMA
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> CommaNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ColonNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::COLON)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::COLON
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ColonNode<'i> {}
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
impl<'i> TextSpaceNode<'i> {}
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
impl<'i> TextWordNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for WhiteSpaceNode<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::WHITE_SPACE)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::WHITE_SPACE
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> WhiteSpaceNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for Infix0Node<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::INFIX0)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::INFIX0
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> Infix0Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for Infix1Node<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::INFIX1)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::INFIX1
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> Infix1Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for Prefix0Node<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::PREFIX0)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::PREFIX0
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> Prefix0Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for Suffix0Node<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::SUFFIX0)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::SUFFIX0
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> Suffix0Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for Special0Node<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::SPECIAL0)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::SPECIAL0
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> Special0Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for Special1Node<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::SPECIAL1)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::SPECIAL1
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> Special1Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for Special2Node<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::SPECIAL2)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::SPECIAL2
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> Special2Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for String0Node<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::STRING0)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::STRING0
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> String0Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for String1Node<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::STRING1)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::STRING1
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> String1Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for SpaceControl0Node<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::SPACE_CONTROL0)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::SPACE_CONTROL0
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> SpaceControl0Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for SpaceControl1Node<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::SPACE_CONTROL1)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::SPACE_CONTROL1
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> SpaceControl1Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for SpaceControl2Node<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::SPACE_CONTROL2)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::SPACE_CONTROL2
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> SpaceControl2Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for SpaceControl3Node<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::SPACE_CONTROL3)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::SPACE_CONTROL3
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> SpaceControl3Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for SpaceControl4Node<'i> {
    type Rule = DejavuRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::SPACE_CONTROL4)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        DejavuRule::SPACE_CONTROL4
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> SpaceControl4Node<'i> {}
