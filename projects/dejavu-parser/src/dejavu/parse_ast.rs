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
        if let Ok(s) = pair.take_tagged_one::<TemplateENode>(Cow::Borrowed("template_e")) {
            return Ok(Self::TemplateE(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TextSpaceNode>(Cow::Borrowed("text_space")) {
            return Ok(Self::TextSpace(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TextWordNode>(Cow::Borrowed("text_word")) {
            return Ok(Self::TextWord(s));
        }
        Err(YggdrasilError::invalid_node(DejavuRule::Element, _span))
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
impl YggdrasilNode for ErrorNode {
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
