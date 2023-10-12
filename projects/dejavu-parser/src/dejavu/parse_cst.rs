use super::*;

pub(super) fn parse_cst(input: &str, rule: DejavuRule) -> OutputResult<DejavuRule> {
    state(input, |state| match rule {
        DejavuRule::Root => parse_root(state),
        DejavuRule::Element => parse_element(state),
        DejavuRule::TemplateIf => parse_template_if(state),
        DejavuRule::IfBegin => parse_if_begin(state),
        DejavuRule::IfElse => parse_if_else(state),
        DejavuRule::IfElseIf => parse_if_else_if(state),
        DejavuRule::IfEnd => parse_if_end(state),
        DejavuRule::KW_END => parse_kw_end(state),
        DejavuRule::KW_IF => parse_kw_if(state),
        DejavuRule::TEMPLATE_L => parse_template_l(state),
        DejavuRule::TEMPLATE_R => parse_template_r(state),
        DejavuRule::TextElements => parse_text_elements(state),
        DejavuRule::TEMPLATE_E => parse_template_e(state),
        DejavuRule::TEXT_SPACE => parse_text_space(state),
        DejavuRule::TEXT_WORD => parse_text_word(state),
        DejavuRule::WhiteSpace => parse_white_space(state),
        DejavuRule::IgnoreText => unreachable!(),
        DejavuRule::IgnoreRegex => unreachable!(),
    })
}
#[inline]
fn parse_root(state: Input) -> Output {
    state.rule(DejavuRule::Root, |s| {
        s.repeat(0..4294967295, |s|parse_element(s).and_then(|s| s.tag_node("element")))
    })
}
#[inline]
fn parse_element(state: Input) -> Output {
    state.rule(DejavuRule::Element, |s| {
        Err(s).or_else(|s|parse_text_elements(s).and_then(|s| s.tag_node("text_elements"))).or_else(|s|parse_template_if(s).and_then(|s| s.tag_node("template_if")))
    })
}
#[inline]
fn parse_template_if(state: Input) -> Output {
    state.rule(DejavuRule::TemplateIf, |s| {
        s.sequence(|s|Ok(s).and_then(|s|parse_if_begin(s).and_then(|s| s.tag_node("if_begin"))).and_then(|s|s.repeat(0..4294967295, |s|parse_text_elements(s).and_then(|s| s.tag_node("text_elements")))).and_then(|s|parse_if_end(s).and_then(|s| s.tag_node("if_end"))))
    })
}
#[inline]
fn parse_if_begin(state: Input) -> Output {
    state.rule(DejavuRule::IfBegin, |s| {
        s.sequence(|s|Ok(s).and_then(|s|parse_template_l(s).and_then(|s| s.tag_node("template_l"))).and_then(|s|builtin_ignore(s)).and_then(|s|parse_kw_end(s).and_then(|s| s.tag_node("kw_end"))).and_then(|s|builtin_ignore(s)).and_then(|s|s.optional(|s|parse_kw_if(s).and_then(|s| s.tag_node("kw_if")))).and_then(|s|builtin_ignore(s)).and_then(|s|parse_template_r(s).and_then(|s| s.tag_node("template_r"))))
    })
}
#[inline]
fn parse_if_else(state: Input) -> Output {
    state.rule(DejavuRule::IfElse, |s| {
        s.sequence(|s|Ok(s).and_then(|s|parse_template_l(s).and_then(|s| s.tag_node("template_l"))).and_then(|s|builtin_ignore(s)).and_then(|s|parse_kw_end(s).and_then(|s| s.tag_node("kw_end"))).and_then(|s|builtin_ignore(s)).and_then(|s|s.optional(|s|parse_kw_if(s).and_then(|s| s.tag_node("kw_if")))).and_then(|s|builtin_ignore(s)).and_then(|s|parse_template_r(s).and_then(|s| s.tag_node("template_r"))))
    })
}
#[inline]
fn parse_if_else_if(state: Input) -> Output {
    state.rule(DejavuRule::IfElseIf, |s| {
        s.sequence(|s|Ok(s).and_then(|s|parse_template_l(s).and_then(|s| s.tag_node("template_l"))).and_then(|s|builtin_ignore(s)).and_then(|s|parse_kw_end(s).and_then(|s| s.tag_node("kw_end"))).and_then(|s|builtin_ignore(s)).and_then(|s|s.optional(|s|parse_kw_if(s).and_then(|s| s.tag_node("kw_if")))).and_then(|s|builtin_ignore(s)).and_then(|s|parse_template_r(s).and_then(|s| s.tag_node("template_r"))))
    })
}
#[inline]
fn parse_if_end(state: Input) -> Output {
    state.rule(DejavuRule::IfEnd, |s| {
        s.sequence(|s|Ok(s).and_then(|s|parse_template_l(s).and_then(|s| s.tag_node("template_l"))).and_then(|s|builtin_ignore(s)).and_then(|s|parse_kw_end(s).and_then(|s| s.tag_node("kw_end"))).and_then(|s|builtin_ignore(s)).and_then(|s|s.optional(|s|parse_kw_if(s).and_then(|s| s.tag_node("kw_if")))).and_then(|s|builtin_ignore(s)).and_then(|s|parse_template_r(s).and_then(|s| s.tag_node("template_r"))))
    })
}
#[inline]
fn parse_kw_end(state: Input) -> Output {
    state.rule(DejavuRule::KW_END, |s| {
        s.match_string("end", false)
    })
}
#[inline]
fn parse_kw_if(state: Input) -> Output {
    state.rule(DejavuRule::KW_IF, |s| {
        s.match_string("if", false)
    })
}
#[inline]
fn parse_template_l(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_L, |s| {
        s.match_string("<%", false)
    })
}
#[inline]
fn parse_template_r(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_R, |s| {
        s.match_string("%>", false)
    })
}
#[inline]
fn parse_text_elements(state: Input) -> Output {
    state.rule(DejavuRule::TextElements, |s| {
        Err(s).or_else(|s|parse_template_e(s).and_then(|s| s.tag_node("template_e"))).or_else(|s|parse_text_space(s).and_then(|s| s.tag_node("text_space"))).or_else(|s|parse_text_word(s).and_then(|s| s.tag_node("text_word")))
    })
}
#[inline]
fn parse_template_e(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_E, |s| {
        s.match_string("<<%", false)
    })
}
#[inline]
fn parse_text_space(state: Input) -> Output {
    state.rule(DejavuRule::TEXT_SPACE, |s| {
        s.match_regex({static REGEX:OnceLock<Regex>=OnceLock::new();REGEX.get_or_init(|| Regex::new("^(\\p{White_Space}+)").unwrap())})
    })
}
#[inline]
fn parse_text_word(state: Input) -> Output {
    state.rule(DejavuRule::TEXT_WORD, |s| {
        s.match_regex({static REGEX:OnceLock<Regex>=OnceLock::new();REGEX.get_or_init(|| Regex::new("^([^<\\p{White_Space}]+)").unwrap())})
    })
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(DejavuRule::WhiteSpace, |s| {
        s.match_regex({static REGEX:OnceLock<Regex>=OnceLock::new();REGEX.get_or_init(|| Regex::new("^(\\p{White_Space}+)").unwrap())})
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| {
        parse_white_space(s)
    })

}

fn builtin_any(state: Input) -> Output {
    state.rule(DejavuRule::IgnoreText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(DejavuRule::IgnoreText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(DejavuRule::IgnoreRegex, |s| s.match_regex(regex))
}