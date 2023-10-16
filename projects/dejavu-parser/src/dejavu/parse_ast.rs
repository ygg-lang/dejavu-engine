use super::*;
#[automatically_derived]
impl YggdrasilNode for RootNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            element: pair.take_tagged_items::<ElementNode>(Cow::Borrowed("element"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for RootNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::Root)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ElementNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<TemplateExportNode>(Cow::Borrowed("template_export")) {
            return Ok(Self::TemplateExport(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TemplateForNode>(Cow::Borrowed("template_for")) {
            return Ok(Self::TemplateFor(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TemplateIfNode>(Cow::Borrowed("template_if")) {
            return Ok(Self::TemplateIf(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TextManyNode>(Cow::Borrowed("text_many")) {
            return Ok(Self::TextMany(s));
        }
        Err(YggdrasilError::invalid_node(NexusRule::Element, _span))
    }
}
#[automatically_derived]
impl FromStr for ElementNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::Element)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextManyNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            text_element: pair.take_tagged_items::<TextElementNode>(Cow::Borrowed("text_element"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TextManyNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::TextMany)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextElementNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<TemplateENode>(Cow::Borrowed("template_e")) {
            return Ok(Self::TemplateE(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TextSpaceNode>(Cow::Borrowed("text_space")) {
            return Ok(Self::TextSpace(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TextWordNode>(Cow::Borrowed("text_word")) {
            return Ok(Self::TextWord(s));
        }
        Err(YggdrasilError::invalid_node(NexusRule::TextElement, _span))
    }
}
#[automatically_derived]
impl FromStr for TextElementNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::TextElement)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TemplateENode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for TemplateENode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::TEMPLATE_E)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextSpaceNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for TextSpaceNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::TEXT_SPACE)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextWordNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for TextWordNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::TEXT_WORD)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TemplateLNode {
    type Rule = NexusRule;

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
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::TEMPLATE_L)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TemplateRNode {
    type Rule = NexusRule;

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
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::TEMPLATE_R)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for SpaceControlNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("space_control_0") {
            return Ok(Self::SpaceControl0);
        }
        if let Some(_) = pair.find_first_tag("space_control_1") {
            return Ok(Self::SpaceControl1);
        }
        if let Some(_) = pair.find_first_tag("space_control_2") {
            return Ok(Self::SpaceControl2);
        }
        if let Some(_) = pair.find_first_tag("space_control_3") {
            return Ok(Self::SpaceControl3);
        }
        if let Some(_) = pair.find_first_tag("space_control_4") {
            return Ok(Self::SpaceControl4);
        }
        Err(YggdrasilError::invalid_node(NexusRule::SpaceControl, _span))
    }
}
#[automatically_derived]
impl FromStr for SpaceControlNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::SpaceControl)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwEndNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for KwEndNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::KW_END)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TemplateExportNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for TemplateExportNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::TemplateExport)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ExportItemNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            class: pair.take_tagged_one::<NamepathFreeNode>(Cow::Borrowed("class"))?,
            language: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("language"))?,
            r#trait: pair.take_tagged_option::<NamepathFreeNode>(Cow::Borrowed("trait")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ExportItemNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::ExportItem)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwExportNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for KwExportNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::KW_EXPORT)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwClassNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for KwClassNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::KW_CLASS)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwTraitNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for KwTraitNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::KW_TRAIT)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwToNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for KwToNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::KW_TO)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwByNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for KwByNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::KW_BY)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TemplateIfNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            if_begin: pair.take_tagged_one::<IfBeginNode>(Cow::Borrowed("if_begin"))?,
            if_else: pair.take_tagged_option::<IfElseNode>(Cow::Borrowed("if_else")),
            if_else_if: pair.take_tagged_items::<IfElseIfNode>(Cow::Borrowed("if_else_if"))?,
            if_end: pair.take_tagged_one::<IfEndNode>(Cow::Borrowed("if_end"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TemplateIfNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::TemplateIf)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IfBeginNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            element: pair.take_tagged_items::<ElementNode>(Cow::Borrowed("element"))?,
            expression: pair.take_tagged_one::<ExpressionNode>(Cow::Borrowed("expression"))?,
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for IfBeginNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::IfBegin)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IfElseNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            element: pair.take_tagged_items::<ElementNode>(Cow::Borrowed("element"))?,
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for IfElseNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::IfElse)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IfElseIfNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            element: pair.take_tagged_items::<ElementNode>(Cow::Borrowed("element"))?,
            expression: pair.take_tagged_one::<ExpressionNode>(Cow::Borrowed("expression"))?,
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for IfElseIfNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::IfElseIf)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IfEndNode {
    type Rule = NexusRule;

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
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::IfEnd)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwIfNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for KwIfNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::KW_IF)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwElseNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for KwElseNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::KW_ELSE)?)
    }
}

#[automatically_derived]
impl YggdrasilNode for TemplateForNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            for_else: pair.take_tagged_option::<ForElseNode>(Cow::Borrowed("for_else")),
            for_end: pair.take_tagged_one::<ForEndNode>(Cow::Borrowed("for_end"))?,
            if_begin: pair.take_tagged_one::<IfBeginNode>(Cow::Borrowed("if_begin"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}

#[automatically_derived]
impl FromStr for TemplateForNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::TemplateFor)?)
    }
}

#[automatically_derived]
impl YggdrasilNode for ForElseNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            element: pair.take_tagged_items::<ElementNode>(Cow::Borrowed("element"))?,
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}

#[automatically_derived]
impl FromStr for ForElseNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::ForElse)?)
    }
}

#[automatically_derived]
impl YggdrasilNode for ForEndNode {
    type Rule = NexusRule;

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
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::ForEnd)?)
    }
}

#[automatically_derived]
impl YggdrasilNode for KwForNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}

#[automatically_derived]
impl FromStr for KwForNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::KW_FOR)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ExpressionNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression_rest: pair.take_tagged_items::<ExpressionRestNode>(Cow::Borrowed("expression_rest"))?,
            term: pair.take_tagged_one::<TermNode>(Cow::Borrowed("term"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ExpressionNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::Expression)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ExpressionRestNode {
    type Rule = NexusRule;

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
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::ExpressionRest)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for InfixNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("infix_0") {
            return Ok(Self::Infix0);
        }
        if let Some(_) = pair.find_first_tag("infix_1") {
            return Ok(Self::Infix1);
        }
        Err(YggdrasilError::invalid_node(NexusRule::Infix, _span))
    }
}
#[automatically_derived]
impl FromStr for InfixNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::Infix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TermNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            atomic: pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic"))?,
            prefix: pair.take_tagged_items::<PrefixNode>(Cow::Borrowed("prefix"))?,
            suffix: pair.take_tagged_items::<SuffixNode>(Cow::Borrowed("suffix"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TermNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::Term)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for PrefixNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("prefix_0") {
            return Ok(Self::Prefix0);
        }
        Err(YggdrasilError::invalid_node(NexusRule::Prefix, _span))
    }
}
#[automatically_derived]
impl FromStr for PrefixNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::Prefix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for SuffixNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("suffix_0") {
            return Ok(Self::Suffix0);
        }
        if let Ok(s) = pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("suffix_1")) {
            return Ok(Self::Suffix1(s));
        }
        Err(YggdrasilError::invalid_node(NexusRule::Suffix, _span))
    }
}
#[automatically_derived]
impl FromStr for SuffixNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::Suffix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for AtomicNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("atomic_0") {
            return Ok(Self::Atomic0);
        }
        if let Ok(s) = pair.take_tagged_one::<BooleanNode>(Cow::Borrowed("boolean")) {
            return Ok(Self::Boolean(s));
        }
        if let Ok(s) = pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier")) {
            return Ok(Self::Identifier(s));
        }
        if let Ok(s) = pair.take_tagged_one::<NumberNode>(Cow::Borrowed("number")) {
            return Ok(Self::Number(s));
        }
        Err(YggdrasilError::invalid_node(NexusRule::Atomic, _span))
    }
}
#[automatically_derived]
impl FromStr for AtomicNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::Atomic)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for StringNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("string_0") {
            return Ok(Self::String0);
        }
        if let Some(_) = pair.find_first_tag("string_1") {
            return Ok(Self::String1);
        }
        Err(YggdrasilError::invalid_node(NexusRule::String, _span))
    }
}
#[automatically_derived]
impl FromStr for StringNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::String)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for NumberNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for NumberNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::Number)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for NamepathFreeNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for NamepathFreeNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::NamepathFree)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for NamepathNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for NamepathNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::Namepath)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IdentifierNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for IdentifierNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::Identifier)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for BooleanNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("boolean_0") {
            return Ok(Self::Boolean0);
        }
        if let Some(_) = pair.find_first_tag("boolean_1") {
            return Ok(Self::Boolean1);
        }
        Err(YggdrasilError::invalid_node(NexusRule::Boolean, _span))
    }
}
#[automatically_derived]
impl FromStr for BooleanNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::Boolean)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for WhiteSpaceNode {
    type Rule = NexusRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for WhiteSpaceNode {
    type Err = YggdrasilError<NexusRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<NexusRule>> {
        Self::from_cst(NexusParser::parse_cst(input, NexusRule::WhiteSpace)?)
    }
}
