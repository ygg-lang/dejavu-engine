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
            element: pair.take_tagged_items::<ElementNode>(Cow::Borrowed("element"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl YggdrasilNode for ElementNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!()
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<TemplateIfNode>(Cow::Borrowed("template_if")) {
            return Ok(Self::TemplateIf(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TextElementsNode>(Cow::Borrowed("text_elements")) {
            return Ok(Self::TextElements(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::Element, _span))
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
            if_end: pair.take_tagged_one::<IfEndNode>(Cow::Borrowed("if_end"))?,
            text_elements: pair.take_tagged_items::<TextElementsNode>(Cow::Borrowed("text_elements"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
            kw_end: pair.take_tagged_one::<KwEndNode>(Cow::Borrowed("kw_end"))?,
            kw_if: pair.take_tagged_option::<KwIfNode>(Cow::Borrowed("kw_if")),
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
            kw_end: pair.take_tagged_one::<KwEndNode>(Cow::Borrowed("kw_end"))?,
            kw_if: pair.take_tagged_option::<KwIfNode>(Cow::Borrowed("kw_if")),
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
            kw_end: pair.take_tagged_one::<KwEndNode>(Cow::Borrowed("kw_end"))?,
            kw_if: pair.take_tagged_option::<KwIfNode>(Cow::Borrowed("kw_if")),
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
            kw_end: pair.take_tagged_one::<KwEndNode>(Cow::Borrowed("kw_end"))?,
            kw_if: pair.take_tagged_option::<KwIfNode>(Cow::Borrowed("kw_if")),
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
impl YggdrasilNode for TemplateLNode {
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
impl YggdrasilNode for TemplateRNode {
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
impl YggdrasilNode for TextElementsNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!()
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
        Err(YggdrasilError::invalid_node(DejavuRule::TextElements, _span))
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
impl YggdrasilNode for TextSpaceNode {
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
impl YggdrasilNode for TextWordNode {
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
