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
            _ => unimplemented!(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<TemplateExportNode>(Cow::Borrowed("template_export")) {
            return Ok(Self::TemplateExport(s));
        }
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
impl YggdrasilNode for TextElementsNode {
    type Rule = DejavuRule;

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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
            space_control: pair.take_tagged_one::<SpaceControlNode>(Cow::Borrowed("space_control"))?,
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
            space_control: pair.take_tagged_one::<SpaceControlNode>(Cow::Borrowed("space_control"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl YggdrasilNode for SpaceControlNode {
    type Rule = DejavuRule;

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
        Err(YggdrasilError::invalid_node(DejavuRule::SPACE_CONTROL, _span))
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
            export_item: pair.take_tagged_items::<ExportItemNode>(Cow::Borrowed("export_item"))?,
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            kw_by: pair.take_tagged_items::<KwByNode>(Cow::Borrowed("kw_by"))?,
            kw_class: pair.take_tagged_one::<KwClassNode>(Cow::Borrowed("kw_class"))?,
            kw_export: pair.take_tagged_one::<KwExportNode>(Cow::Borrowed("kw_export"))?,
            kw_to: pair.take_tagged_one::<KwToNode>(Cow::Borrowed("kw_to"))?,
            kw_trait: pair.take_tagged_one::<KwTraitNode>(Cow::Borrowed("kw_trait"))?,
            namepath_free: pair.take_tagged_option::<NamepathFreeNode>(Cow::Borrowed("namepath_free")),
            class: pair.take_tagged_one::<NamepathFreeNode>(Cow::Borrowed("class"))?,
            language: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("language"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
            if_else_if: pair.take_tagged_items::<IfElseIfNode>(Cow::Borrowed("if_else_if"))?,
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
            atomic: pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic"))?,
            kw_if: pair.take_tagged_one::<KwIfNode>(Cow::Borrowed("kw_if"))?,
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            text_elements: pair.take_tagged_items::<TextElementsNode>(Cow::Borrowed("text_elements"))?,
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
            kw_else: pair.take_tagged_one::<KwElseNode>(Cow::Borrowed("kw_else"))?,
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            text_elements: pair.take_tagged_items::<TextElementsNode>(Cow::Borrowed("text_elements"))?,
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
            kw_else: pair.take_tagged_one::<KwElseNode>(Cow::Borrowed("kw_else"))?,
            kw_if: pair.take_tagged_one::<KwIfNode>(Cow::Borrowed("kw_if"))?,
            template_l: pair.take_tagged_one::<TemplateLNode>(Cow::Borrowed("template_l"))?,
            template_r: pair.take_tagged_one::<TemplateRNode>(Cow::Borrowed("template_r"))?,
            text_elements: pair.take_tagged_items::<TextElementsNode>(Cow::Borrowed("text_elements"))?,
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
impl YggdrasilNode for KwIfNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl YggdrasilNode for AtomicNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!(),
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
impl YggdrasilNode for StringNode {
    type Rule = DejavuRule;

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
        Err(YggdrasilError::invalid_node(DejavuRule::String, _span))
    }
}
#[automatically_derived]
impl YggdrasilNode for NumberNode {
    type Rule = DejavuRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl YggdrasilNode for BooleanNode {
    type Rule = DejavuRule;

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
        Err(YggdrasilError::invalid_node(DejavuRule::Boolean, _span))
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
