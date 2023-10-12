use super::*;

pub(super) fn parse_cst(input: &str, rule: DejavuRule) -> OutputResult<DejavuRule> {
    state(input, |state| match rule {
        DejavuRule::Root => parse_root(state),
        DejavuRule::Element => parse_element(state),
        DejavuRule::TEMPLATE_E => parse_template_e(state),
        DejavuRule::TEXT_SPACE => parse_text_space(state),
        DejavuRule::TEXT_WORD => parse_text_word(state),
        DejavuRule::ERROR => parse_error(state),
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
fn parse_error(state: Input) -> Output {
    state.rule(DejavuRule::ERROR, |s| {
        s.match_string("@@", false)
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| {
        parse_error(s)
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