use super::*;

pub(super) fn parse_cst(input: &str, rule: DejavuRule) -> OutputResult<DejavuRule> {
    state(input, |state| match rule {
        DejavuRule::Root => parse_root(state),
        DejavuRule::Element => parse_element(state),
        DejavuRule::TextMany => parse_text_many(state),
        DejavuRule::TextElement => parse_text_element(state),
        DejavuRule::TEMPLATE_E => parse_template_e(state),
        DejavuRule::TEXT_SPACE => parse_text_space(state),
        DejavuRule::TEXT_WORD => parse_text_word(state),
        DejavuRule::TEMPLATE_L => parse_template_l(state),
        DejavuRule::TEMPLATE_R => parse_template_r(state),
        DejavuRule::SpaceControl => parse_space_control(state),
        DejavuRule::KW_END => parse_kw_end(state),
        DejavuRule::TemplateExport => parse_template_export(state),
        DejavuRule::ExportItem => parse_export_item(state),
        DejavuRule::KW_EXPORT => parse_kw_export(state),
        DejavuRule::KW_CLASS => parse_kw_class(state),
        DejavuRule::KW_TRAIT => parse_kw_trait(state),
        DejavuRule::KW_TO => parse_kw_to(state),
        DejavuRule::KW_BY => parse_kw_by(state),
        DejavuRule::TemplateIf => parse_template_if(state),
        DejavuRule::IfBegin => parse_if_begin(state),
        DejavuRule::IfElse => parse_if_else(state),
        DejavuRule::IfElseIf => parse_if_else_if(state),
        DejavuRule::IfEnd => parse_if_end(state),
        DejavuRule::KW_IF => parse_kw_if(state),
        DejavuRule::KW_ELSE => parse_kw_else(state),
        DejavuRule::TemplateFor => parse_template_for(state),
        DejavuRule::ForBegin => parse_for_begin(state),
        DejavuRule::ForElse => parse_for_else(state),
        DejavuRule::ForEnd => parse_for_end(state),
        DejavuRule::KW_FOR => parse_kw_for(state),
        DejavuRule::KW_IN => parse_kw_in(state),
        DejavuRule::Pattern => parse_pattern(state),
        DejavuRule::BarePattern => parse_bare_pattern(state),
        DejavuRule::Expression => parse_expression(state),
        DejavuRule::ExpressionRest => parse_expression_rest(state),
        DejavuRule::Infix => parse_infix(state),
        DejavuRule::Term => parse_term(state),
        DejavuRule::Prefix => parse_prefix(state),
        DejavuRule::Suffix => parse_suffix(state),
        DejavuRule::Atomic => parse_atomic(state),
        DejavuRule::String => parse_string(state),
        DejavuRule::Number => parse_number(state),
        DejavuRule::Digits => parse_digits(state),
        DejavuRule::Unit => parse_unit(state),
        DejavuRule::BIN => parse_bin(state),
        DejavuRule::OCT => parse_oct(state),
        DejavuRule::HEX => parse_hex(state),
        DejavuRule::NamepathFree => parse_namepath_free(state),
        DejavuRule::Namepath => parse_namepath(state),
        DejavuRule::Identifier => parse_identifier(state),
        DejavuRule::Boolean => parse_boolean(state),
        DejavuRule::WhiteSpace => parse_white_space(state),
        DejavuRule::HiddenText => unreachable!(),
    })
}
#[inline]
fn parse_root(state: Input) -> Output {
    state.rule(DejavuRule::Root, |s| {
        s.repeat(0..4294967295, |s| s.sequence(|s| Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| parse_element(s).and_then(|s| s.tag_node("element")))))
    })
}
#[inline]
fn parse_element(state: Input) -> Output {
    state.rule(DejavuRule::Element, |s| {
        Err(s)
    })
}
#[inline]
fn parse_text_many(state: Input) -> Output {
    state.rule(DejavuRule::TextMany, |s| {
        s.repeat(1..4294967295, |s| parse_text_element(s).and_then(|s| s.tag_node("text_element")))
    })
}
#[inline]
fn parse_text_element(state: Input) -> Output {
    state.rule(DejavuRule::TextElement, |s| {
        Err(s)
    })
}
#[inline]
fn parse_template_e(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_E, |s| {
        s.match_string("<%!", false)
    })
}
#[inline]
fn parse_text_space(state: Input) -> Output {
    state.rule(DejavuRule::TEXT_SPACE, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(/\\p{White_Space}+/)").unwrap())
        })
    })
}
#[inline]
fn parse_text_word(state: Input) -> Output {
    state.rule(DejavuRule::TEXT_WORD, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(/[^<\\p{White_Space}]+/)").unwrap())
        })
    })
}
#[inline]
fn parse_template_l(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_L, |s| {
        s.sequence(|s| Ok(s).and_then(|s| builtin_text(s, "<%", false)).and_then(|s| s.optional(|s| parse_space_control(s).and_then(|s| s.tag_node("space_control")))))
    })
}
#[inline]
fn parse_template_r(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_R, |s| {
        s.sequence(|s| Ok(s).and_then(|s| s.optional(|s| parse_space_control(s).and_then(|s| s.tag_node("space_control")))).and_then(|s| builtin_text(s, "%>", false)))
    })
}
#[inline]
fn parse_space_control(state: Input) -> Output {
    state.rule(DejavuRule::SpaceControl, |s| {
        Err(s)
    })
}
#[inline]
fn parse_kw_end(state: Input) -> Output {
    state.rule(DejavuRule::KW_END, |s| {
        s.match_string("end", false)
    })
}
#[inline]
fn parse_template_export(state: Input) -> Output {
    state.rule(DejavuRule::TemplateExport, |s| {
        s.sequence(|s| Ok(s).and_then(|s| s.sequence(|s| Ok(s).and_then(|s| parse_template_l(s)).and_then(|s| builtin_ignore(s)))).and_then(|s| s.repeat(0..4294967295, |s| s.sequence(|s| Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| parse_export_item(s)))).and_then(|s| s.tag_node("exports"))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_template_r(s)))
    })
}
#[inline]
fn parse_export_item(state: Input) -> Output {
    state.rule(DejavuRule::ExportItem, |s| {
        s.sequence(|s| Ok(s).and_then(|s| s.sequence(|s| Ok(s).and_then(|s| parse_kw_export(s)).and_then(|s| builtin_ignore(s)))).and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("name"))).and_then(|s| builtin_ignore(s)).and_then(|s| s.optional(|s| s.sequence(|s| Ok(s).and_then(|s| s.sequence(|s| Ok(s).and_then(|s| parse_kw_to(s)).and_then(|s| builtin_ignore(s)))).and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("language")))))).and_then(|s| builtin_ignore(s)).and_then(|s| s.optional(|s| s.sequence(|s| Ok(s).and_then(|s| s.sequence(|s| Ok(s).and_then(|s| parse_kw_by(s)).and_then(|s| builtin_ignore(s)).and_then(|s| parse_kw_class(s)).and_then(|s| builtin_ignore(s)))).and_then(|s| parse_namepath_free(s).and_then(|s| s.tag_node("class")))))).and_then(|s| builtin_ignore(s)).and_then(|s| s.optional(|s| s.sequence(|s| Ok(s).and_then(|s| s.sequence(|s| Ok(s).and_then(|s| parse_kw_by(s)).and_then(|s| builtin_ignore(s)).and_then(|s| parse_kw_trait(s)).and_then(|s| builtin_ignore(s)))).and_then(|s| s.optional(|s| parse_namepath_free(s)).and_then(|s| s.tag_node("trait")))))))
    })
}
#[inline]
fn parse_kw_export(state: Input) -> Output {
    state.rule(DejavuRule::KW_EXPORT, |s| {
        s.match_string("export", false)
    })
}
#[inline]
fn parse_kw_class(state: Input) -> Output {
    state.rule(DejavuRule::KW_CLASS, |s| {
        s.match_string("class", false)
    })
}
#[inline]
fn parse_kw_trait(state: Input) -> Output {
    state.rule(DejavuRule::KW_TRAIT, |s| {
        s.match_string("trait", false)
    })
}
#[inline]
fn parse_kw_to(state: Input) -> Output {
    state.rule(DejavuRule::KW_TO, |s| {
        s.match_string("to", false)
    })
}
#[inline]
fn parse_kw_by(state: Input) -> Output {
    state.rule(DejavuRule::KW_BY, |s| {
        s.match_string("by", false)
    })
}
#[inline]
fn parse_template_if(state: Input) -> Output {
    state.rule(DejavuRule::TemplateIf, |s| {
        s.sequence(|s| Ok(s).and_then(|s| parse_if_begin(s).and_then(|s| s.tag_node("if_begin"))).and_then(|s| s.repeat(0..4294967295, |s| parse_if_else_if(s).and_then(|s| s.tag_node("if_else_if")))).and_then(|s| s.optional(|s| parse_if_else(s).and_then(|s| s.tag_node("if_else")))).and_then(|s| parse_if_end(s).and_then(|s| s.tag_node("if_end"))))
    })
}
#[inline]
fn parse_if_begin(state: Input) -> Output {
    state.rule(DejavuRule::IfBegin, |s| {
        s.sequence(|s| Ok(s).and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l"))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_kw_if(s)).and_then(|s| builtin_ignore(s)).and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression"))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r"))).and_then(|s| s.repeat(0..4294967295, |s| parse_element(s).and_then(|s| s.tag_node("element")))))
    })
}
#[inline]
fn parse_if_else(state: Input) -> Output {
    state.rule(DejavuRule::IfElse, |s| {
        s.sequence(|s| Ok(s).and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l"))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_kw_else(s)).and_then(|s| builtin_ignore(s)).and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r"))).and_then(|s| s.repeat(0..4294967295, |s| parse_element(s).and_then(|s| s.tag_node("element")))))
    })
}
#[inline]
fn parse_if_else_if(state: Input) -> Output {
    state.rule(DejavuRule::IfElseIf, |s| {
        s.sequence(|s| Ok(s).and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l"))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_kw_else(s)).and_then(|s| builtin_ignore(s)).and_then(|s| parse_kw_if(s)).and_then(|s| builtin_ignore(s)).and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression"))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r"))).and_then(|s| s.repeat(0..4294967295, |s| parse_element(s).and_then(|s| s.tag_node("element")))))
    })
}
#[inline]
fn parse_if_end(state: Input) -> Output {
    state.rule(DejavuRule::IfEnd, |s| {
        s.sequence(|s| Ok(s).and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l"))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_kw_end(s)).and_then(|s| builtin_ignore(s)).and_then(|s| s.optional(|s| parse_kw_if(s))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r"))))
    })
}
#[inline]
fn parse_kw_if(state: Input) -> Output {
    state.rule(DejavuRule::KW_IF, |s| {
        s.match_string("if", false)
    })
}
#[inline]
fn parse_kw_else(state: Input) -> Output {
    state.rule(DejavuRule::KW_ELSE, |s| {
        s.match_string("else", false)
    })
}
#[inline]
fn parse_template_for(state: Input) -> Output {
    state.rule(DejavuRule::TemplateFor, |s| {
        s.sequence(|s| Ok(s).and_then(|s| parse_for_begin(s).and_then(|s| s.tag_node("for_begin"))).and_then(|s| s.optional(|s| parse_for_else(s).and_then(|s| s.tag_node("for_else")))).and_then(|s| parse_for_end(s).and_then(|s| s.tag_node("for_end"))))
    })
}
#[inline]
fn parse_for_begin(state: Input) -> Output {
    state.rule(DejavuRule::ForBegin, |s| {
        s.sequence(|s| Ok(s).and_then(|s| s.sequence(|s| Ok(s).and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l"))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_kw_for(s)).and_then(|s| builtin_ignore(s)).and_then(|s| s.lookahead(false, |s| parse_kw_in(s).and_then(|s| s.tag_node("kw_in")))).and_then(|s| parse_pattern(s).and_then(|s| s.tag_node("pattern"))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_kw_in(s)).and_then(|s| builtin_ignore(s)))).and_then(|s| parse_expression(s).and_then(|s| s.tag_node("iterator"))).and_then(|s| builtin_ignore(s)).and_then(|s| s.optional(|s| s.sequence(|s| Ok(s).and_then(|s| s.sequence(|s| Ok(s).and_then(|s| parse_kw_if(s)).and_then(|s| builtin_ignore(s)))).and_then(|s| parse_expression(s).and_then(|s| s.tag_node("condition")))))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r"))).and_then(|s| s.repeat(0..4294967295, |s| parse_element(s).and_then(|s| s.tag_node("element")))))
    })
}
#[inline]
fn parse_for_else(state: Input) -> Output {
    state.rule(DejavuRule::ForElse, |s| {
        s.sequence(|s| Ok(s).and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l"))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_kw_else(s)).and_then(|s| builtin_ignore(s)).and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r"))).and_then(|s| s.repeat(0..4294967295, |s| parse_element(s).and_then(|s| s.tag_node("element")))))
    })
}
#[inline]
fn parse_for_end(state: Input) -> Output {
    state.rule(DejavuRule::ForEnd, |s| {
        s.sequence(|s| Ok(s).and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l"))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_kw_end(s)).and_then(|s| builtin_ignore(s)).and_then(|s| s.optional(|s| parse_kw_for(s))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r"))))
    })
}
#[inline]
fn parse_kw_for(state: Input) -> Output {
    state.rule(DejavuRule::KW_FOR, |s| {
        s.match_string("for", false)
    })
}
#[inline]
fn parse_kw_in(state: Input) -> Output {
    state.rule(DejavuRule::KW_IN, |s| {
        s.match_string("in", false)
    })
}
#[inline]
fn parse_pattern(state: Input) -> Output {
    state.rule(DejavuRule::Pattern, |s| {
        Err(s)
    })
}
#[inline]
fn parse_bare_pattern(state: Input) -> Output {
    state.rule(DejavuRule::BarePattern, |s| {
        s.sequence(|s| Ok(s).and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier"))).and_then(|s| builtin_ignore(s)).and_then(|s| s.repeat(0..4294967295, |s| s.sequence(|s| Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| s.sequence(|s| Ok(s).and_then(|s| builtin_text(s, ",", false)).and_then(|s| builtin_ignore(s)).and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))))))).and_then(|s| builtin_ignore(s)).and_then(|s| s.optional(|s| builtin_text(s, ",", false))))
    })
}
#[inline]
fn parse_expression(state: Input) -> Output {
    state.rule(DejavuRule::Expression, |s| {
        s.sequence(|s| Ok(s).and_then(|s| parse_term(s).and_then(|s| s.tag_node("term"))).and_then(|s| builtin_ignore(s)).and_then(|s| s.repeat(0..4294967295, |s| s.sequence(|s| Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| parse_expression_rest(s).and_then(|s| s.tag_node("expression_rest")))))))
    })
}
#[inline]
fn parse_expression_rest(state: Input) -> Output {
    state.rule(DejavuRule::ExpressionRest, |s| {
        s.sequence(|s| Ok(s).and_then(|s| parse_infix(s).and_then(|s| s.tag_node("infix"))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_term(s).and_then(|s| s.tag_node("term"))))
    })
}
#[inline]
fn parse_infix(state: Input) -> Output {
    state.rule(DejavuRule::Infix, |s| {
        Err(s)
    })
}
#[inline]
fn parse_term(state: Input) -> Output {
    state.rule(DejavuRule::Term, |s| {
        s.sequence(|s| Ok(s).and_then(|s| s.repeat(0..4294967295, |s| s.sequence(|s| Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| parse_prefix(s).and_then(|s| s.tag_node("prefix")))))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_atomic(s).and_then(|s| s.tag_node("atomic"))).and_then(|s| builtin_ignore(s)).and_then(|s| s.repeat(0..4294967295, |s| s.sequence(|s| Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| parse_suffix(s).and_then(|s| s.tag_node("suffix")))))))
    })
}
#[inline]
fn parse_prefix(state: Input) -> Output {
    state.rule(DejavuRule::Prefix, |s| {
        Err(s)
    })
}
#[inline]
fn parse_suffix(state: Input) -> Output {
    state.rule(DejavuRule::Suffix, |s| {
        Err(s)
    })
}
#[inline]
fn parse_atomic(state: Input) -> Output {
    state.rule(DejavuRule::Atomic, |s| {
        Err(s)
    })
}
#[inline]
fn parse_string(state: Input) -> Output {
    state.rule(DejavuRule::String, |s| {
        Err(s)
    })
}
#[inline]
fn parse_number(state: Input) -> Output {
    state.rule(DejavuRule::Number, |s| {
        Err(s)
    })
}
#[inline]
fn parse_digits(state: Input) -> Output {
    state.rule(DejavuRule::Digits, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(/(0|[1-9][0-9])(.[0-9]+)?/)").unwrap())
        })
    })
}
#[inline]
fn parse_unit(state: Input) -> Output {
    state.rule(DejavuRule::Unit, |s| {
        parse_identifier(s).and_then(|s| s.tag_node("identifier"))
    })
}
#[inline]
fn parse_bin(state: Input) -> Output {
    state.rule(DejavuRule::BIN, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(/[0-1]+/)").unwrap())
        })
    })
}
#[inline]
fn parse_oct(state: Input) -> Output {
    state.rule(DejavuRule::OCT, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(/[0-7]+/)").unwrap())
        })
    })
}
#[inline]
fn parse_hex(state: Input) -> Output {
    state.rule(DejavuRule::HEX, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(/[0-9a-fA-F]+/)").unwrap())
        })
    })
}
#[inline]
fn parse_namepath_free(state: Input) -> Output {
    state.rule(DejavuRule::NamepathFree, |s| {
        s.sequence(|s| Ok(s).and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier"))).and_then(|s| builtin_ignore(s)).and_then(|s| s.repeat(0..4294967295, |s| s.sequence(|s| Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| s.sequence(|s| Ok(s).and_then(|s| Err(s).or_else(|s| builtin_text(s, ".", false)).or_else(|s| builtin_text(s, "::", false))).and_then(|s| builtin_ignore(s)).and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))))))))
    })
}
#[inline]
fn parse_namepath(state: Input) -> Output {
    state.rule(DejavuRule::Namepath, |s| {
        s.sequence(|s| Ok(s).and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier"))).and_then(|s| builtin_ignore(s)).and_then(|s| s.repeat(0..4294967295, |s| s.sequence(|s| Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| s.sequence(|s| Ok(s).and_then(|s| builtin_text(s, "::", false)).and_then(|s| builtin_ignore(s)).and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))))))))
    })
}
#[inline]
fn parse_identifier(state: Input) -> Output {
    state.rule(DejavuRule::Identifier, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(/[_\\p{XID_start}]\\p{XID_continue}*/)").unwrap())
        })
    })
}
#[inline]
fn parse_boolean(state: Input) -> Output {
    state.rule(DejavuRule::Boolean, |s| {
        Err(s)
    })
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(DejavuRule::WhiteSpace, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(/\\p{White_Space}+/)").unwrap())
        })
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| {})

}

fn builtin_any(state: Input) -> Output {
    state.rule(DejavuRule::HiddenText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(DejavuRule::HiddenText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(DejavuRule::HiddenText, |s| s.match_regex(regex))
}