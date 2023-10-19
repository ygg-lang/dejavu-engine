use super::*;
#[automatically_derived]
impl YggdrasilNode for RootNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            element: pair.take_tagged_items::<ElementNode>(Cow::Borrowed("element")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for RootNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Root)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ElementNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Export(s) => s.get_range(),
            Self::For(s) => s.get_range(),
            Self::If(s) => s.get_range(),
            Self::Text(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<TemplateExportNode>(Cow::Borrowed("export")) {
            return Ok(Self::Export(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TemplateForNode>(Cow::Borrowed("for")) {
            return Ok(Self::For(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TemplateIfNode>(Cow::Borrowed("if")) {
            return Ok(Self::If(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TextManyNode>(Cow::Borrowed("text")) {
            return Ok(Self::Text(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::Element, _span))
    }
}
#[automatically_derived]
impl FromStr for ElementNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Element)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextManyNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            text_element: pair.take_tagged_items::<TextElementNode>(Cow::Borrowed("text_element")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TextManyNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TextMany)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextElementNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Escape(s) => s.get_range(),
            Self::TextSpace(s) => s.get_range(),
            Self::TextWord(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<TemplateENode>(Cow::Borrowed("escape")) {
            return Ok(Self::Escape(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TextSpaceNode>(Cow::Borrowed("text_space")) {
            return Ok(Self::TextSpace(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TextWordNode>(Cow::Borrowed("text_word")) {
            return Ok(Self::TextWord(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::TextElement, _span))
    }
}
#[automatically_derived]
impl FromStr for TextElementNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TextElement)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TemplateENode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TemplateENode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEMPLATE_E)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextSpaceNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            text: pair.get_string(),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TextSpaceNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEXT_SPACE)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextWordNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            text: pair.get_string(),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TextWordNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEXT_WORD)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TemplateLNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            space_control: pair.take_tagged_option::<SpaceControlNode>(Cow::Borrowed("space_control")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TemplateLNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEMPLATE_L)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TemplateRNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            space_control: pair.take_tagged_option::<SpaceControlNode>(Cow::Borrowed("space_control")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TemplateRNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TEMPLATE_R)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for SpaceControlNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::A => None,
            Self::B => None,
            Self::C => None,
            Self::D => None,
            Self::E => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("a") {
            return Ok(Self::A)
        }
        if let Some(_) = pair.find_first_tag("b") {
            return Ok(Self::B)
        }
        if let Some(_) = pair.find_first_tag("c") {
            return Ok(Self::C)
        }
        if let Some(_) = pair.find_first_tag("d") {
            return Ok(Self::D)
        }
        if let Some(_) = pair.find_first_tag("e") {
            return Ok(Self::E)
        }
        Err(YggdrasilError::invalid_node(DejavuRule::SpaceControl, _span))
    }
}
#[automatically_derived]
impl FromStr for SpaceControlNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::SpaceControl)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwEndNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwEndNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_END)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TemplateExportNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TemplateExportNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TemplateExport)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ExportItemNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            class: pair.take_tagged_option::<NamepathFreeNode>(Cow::Borrowed("class")),
            language: pair.take_tagged_option::<IdentifierNode>(Cow::Borrowed("language")),
            name: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("name"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ExportItemNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::ExportItem)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwExportNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwExportNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_EXPORT)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwClassNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwClassNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_CLASS)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwTraitNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwTraitNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_TRAIT)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwToNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwToNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_TO)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwByNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwByNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_BY)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TemplateIfNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            if_begin: pair.take_tagged_one::<IfBeginNode>(Cow::Borrowed("if_begin"))?,
            if_else: pair.take_tagged_option::<IfElseNode>(Cow::Borrowed("if_else")),
            if_else_if: pair.take_tagged_items::<IfElseIfNode>(Cow::Borrowed("if_else_if")).collect::<Result<Vec<_>, _>>()?,
            if_end: pair.take_tagged_one::<IfEndNode>(Cow::Borrowed("if_end"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TemplateIfNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TemplateIf)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IfBeginNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            element: pair.take_tagged_items::<ElementNode>(Cow::Borrowed("element")).collect::<Result<Vec<_>, _>>()?,
            expression: pair.take_tagged_one::<ExpressionNode>(Cow::Borrowed("expression"))?,
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for IfBeginNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::IfBegin)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IfElseNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            element: pair.take_tagged_items::<ElementNode>(Cow::Borrowed("element")).collect::<Result<Vec<_>, _>>()?,
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for IfElseNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::IfElse)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IfElseIfNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            element: pair.take_tagged_items::<ElementNode>(Cow::Borrowed("element")).collect::<Result<Vec<_>, _>>()?,
            expression: pair.take_tagged_one::<ExpressionNode>(Cow::Borrowed("expression"))?,
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for IfElseIfNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::IfElseIf)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IfEndNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for IfEndNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::IfEnd)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwIfNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwIfNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_IF)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwElseNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwElseNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_ELSE)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TemplateForNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            for_begin: pair.take_tagged_one::<ForBeginNode>(Cow::Borrowed("for_begin"))?,
            for_else: pair.take_tagged_option::<ForElseNode>(Cow::Borrowed("for_else")),
            for_end: pair.take_tagged_one::<ForEndNode>(Cow::Borrowed("for_end"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TemplateForNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::TemplateFor)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ForBeginNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            element: pair.take_tagged_items::<ElementNode>(Cow::Borrowed("element")).collect::<Result<Vec<_>, _>>()?,
            expression: pair.take_tagged_one::<ExpressionNode>(Cow::Borrowed("expression"))?,
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ForBeginNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::ForBegin)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ForElseNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            element: pair.take_tagged_items::<ElementNode>(Cow::Borrowed("element")).collect::<Result<Vec<_>, _>>()?,
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ForElseNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::ForElse)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ForEndNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ForEndNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::ForEnd)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwForNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwForNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::KW_FOR)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ExpressionNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression_rest: pair.take_tagged_items::<ExpressionRestNode>(Cow::Borrowed("expression_rest")).collect::<Result<Vec<_>, _>>()?,
            term: pair.take_tagged_one::<TermNode>(Cow::Borrowed("term"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ExpressionNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Expression)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ExpressionRestNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            infix: pair.take_tagged_one::<InfixNode>(Cow::Borrowed("infix"))?,
            term: pair.take_tagged_one::<TermNode>(Cow::Borrowed("term"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ExpressionRestNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::ExpressionRest)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for InfixNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Add => None,
            Self::Mul => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("add") {
            return Ok(Self::Add)
        }
        if let Some(_) = pair.find_first_tag("mul") {
            return Ok(Self::Mul)
        }
        Err(YggdrasilError::invalid_node(DejavuRule::Infix, _span))
    }
}
#[automatically_derived]
impl FromStr for InfixNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Infix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TermNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            atomic: pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic"))?,
            prefix: pair.take_tagged_items::<PrefixNode>(Cow::Borrowed("prefix")).collect::<Result<Vec<_>, _>>()?,
            suffix: pair.take_tagged_items::<SuffixNode>(Cow::Borrowed("suffix")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TermNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Term)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for PrefixNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Not => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("not") {
            return Ok(Self::Not)
        }
        Err(YggdrasilError::invalid_node(DejavuRule::Prefix, _span))
    }
}
#[automatically_derived]
impl FromStr for PrefixNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Prefix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for SuffixNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Null => None,
            Self::Suffix1(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("null") {
            return Ok(Self::Null)
        }
        if let Ok(s) = pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("suffix_1")) {
            return Ok(Self::Suffix1(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::Suffix, _span))
    }
}
#[automatically_derived]
impl FromStr for SuffixNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Suffix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for AtomicNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Boolean(s) => s.get_range(),
            Self::Identifier(s) => s.get_range(),
            Self::Number(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<BooleanNode>(Cow::Borrowed("boolean")) {
            return Ok(Self::Boolean(s));
        }
        if let Ok(s) = pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier")) {
            return Ok(Self::Identifier(s));
        }
        if let Ok(s) = pair.take_tagged_one::<NumberNode>(Cow::Borrowed("number")) {
            return Ok(Self::Number(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::Atomic, _span))
    }
}
#[automatically_derived]
impl FromStr for AtomicNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Atomic)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for StringNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::DoubleQuote => None,
            Self::SingleQuote => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("double_quote") {
            return Ok(Self::DoubleQuote)
        }
        if let Some(_) = pair.find_first_tag("single_quote") {
            return Ok(Self::SingleQuote)
        }
        Err(YggdrasilError::invalid_node(DejavuRule::String, _span))
    }
}
#[automatically_derived]
impl FromStr for StringNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::String)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for NumberNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Dec(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<DigitsNode>(Cow::Borrowed("dec")) {
            return Ok(Self::Dec(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::Number, _span))
    }
}
#[automatically_derived]
impl FromStr for NumberNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Number)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for DigitsNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            text: pair.get_string(),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for DigitsNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Digits)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for UnitNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for UnitNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Unit)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for BinNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for BinNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::BIN)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for OctNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for OctNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::OCT)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for HexNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for HexNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::HEX)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for NamepathFreeNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for NamepathFreeNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::NamepathFree)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for NamepathNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for NamepathNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Namepath)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IdentifierNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for IdentifierNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Identifier)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for BooleanNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::False => None,
            Self::True => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("false") {
            return Ok(Self::False)
        }
        if let Some(_) = pair.find_first_tag("true") {
            return Ok(Self::True)
        }
        Err(YggdrasilError::invalid_node(DejavuRule::Boolean, _span))
    }
}
#[automatically_derived]
impl FromStr for BooleanNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::Boolean)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for WhiteSpaceNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for WhiteSpaceNode {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<DejavuRule>> {
        Self::from_cst(DejavuParser::parse_cst(input, DejavuRule::WhiteSpace)?)
    }
}
