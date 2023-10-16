use super::*;

pub(super) fn parse_cst(input: &str, rule: NexusRule) -> OutputResult<NexusRule> {
    state(input, |state| match rule {
        NexusRule::Root => parse_root(state),
        NexusRule::Element => parse_element(state),
        NexusRule::TextMany => parse_text_many(state),
        NexusRule::TextElements => parse_text_elements(state),
        NexusRule::TEMPLATE_E => parse_template_e(state),
        NexusRule::TEXT_SPACE => parse_text_space(state),
        NexusRule::TEXT_WORD => parse_text_word(state),
        NexusRule::TEMPLATE_L => parse_template_l(state),
        NexusRule::TEMPLATE_R => parse_template_r(state),
        NexusRule::SpaceControl => parse_space_control(state),
        NexusRule::KW_END => parse_kw_end(state),
        NexusRule::TemplateExport => parse_template_export(state),
        NexusRule::ExportItem => parse_export_item(state),
        NexusRule::KW_EXPORT => parse_kw_export(state),
        NexusRule::KW_CLASS => parse_kw_class(state),
        NexusRule::KW_TRAIT => parse_kw_trait(state),
        NexusRule::KW_TO => parse_kw_to(state),
        NexusRule::KW_BY => parse_kw_by(state),
        NexusRule::TemplateIf => parse_template_if(state),
        NexusRule::IfBegin => parse_if_begin(state),
        NexusRule::IfElse => parse_if_else(state),
        NexusRule::IfElseIf => parse_if_else_if(state),
        NexusRule::IfEnd => parse_if_end(state),
        NexusRule::KW_IF => parse_kw_if(state),
        NexusRule::KW_ELSE => parse_kw_else(state),
        NexusRule::Atomic => parse_atomic(state),
        NexusRule::String => parse_string(state),
        NexusRule::Number => parse_number(state),
        NexusRule::NamepathFree => parse_namepath_free(state),
        NexusRule::Namepath => parse_namepath(state),
        NexusRule::Identifier => parse_identifier(state),
        NexusRule::Boolean => parse_boolean(state),
        NexusRule::WhiteSpace => parse_white_space(state),
        NexusRule::IgnoreText => unreachable!(),
        NexusRule::IgnoreRegex => unreachable!(),
    })
}
#[inline]
fn parse_root(state: Input) -> Output {
    state.rule(NexusRule::Root, |s| s.repeat(0..4294967295, |s| parse_element(s).and_then(|s| s.tag_node("element"))))
}
#[inline]
fn parse_element(state: Input) -> Output {
    state.rule(NexusRule::Element, |s| {
        Err(s)
            .or_else(|s| parse_text_many(s).and_then(|s| s.tag_node("text_many")))
            .or_else(|s| parse_template_export(s).and_then(|s| s.tag_node("template_export")))
            .or_else(|s| parse_template_if(s).and_then(|s| s.tag_node("template_if")))
    })
}

#[inline]
fn parse_text_many(state: Input) -> Output {
    state.rule(NexusRule::TextMany, |s| {
        s.repeat(1..4294967295, |s| parse_text_elements(s).and_then(|s| s.tag_node("text_elements")))
    })
}
#[inline]
fn parse_text_elements(state: Input) -> Output {
    state.rule(NexusRule::TextElements, |s| {
        Err(s)
            .or_else(|s| parse_template_e(s).and_then(|s| s.tag_node("template_e")))
            .or_else(|s| parse_text_space(s).and_then(|s| s.tag_node("text_space")))
            .or_else(|s| parse_text_word(s).and_then(|s| s.tag_node("text_word")))
    })
}
#[inline]
fn parse_template_e(state: Input) -> Output {
    state.rule(NexusRule::TEMPLATE_E, |s| s.match_string("<%!", false))
}
#[inline]
fn parse_text_space(state: Input) -> Output {
    state.rule(NexusRule::TEXT_SPACE, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(\\p{White_Space}+)").unwrap())
        })
    })
}
#[inline]
fn parse_text_word(state: Input) -> Output {
    state.rule(NexusRule::TEXT_WORD, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^([^<\\p{White_Space}]+)").unwrap())
        })
    })
}
#[inline]
fn parse_template_l(state: Input) -> Output {
    state.rule(NexusRule::TEMPLATE_L, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "<%", false))
                .and_then(|s| s.optional(|s| parse_space_control(s).and_then(|s| s.tag_node("space_control"))))
        })
    })
}
#[inline]
fn parse_template_r(state: Input) -> Output {
    state.rule(NexusRule::TEMPLATE_R, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| s.optional(|s| parse_space_control(s).and_then(|s| s.tag_node("space_control"))))
                .and_then(|s| builtin_text(s, "%>", false))
        })
    })
}
#[inline]
fn parse_space_control(state: Input) -> Output {
    state.rule(NexusRule::SpaceControl, |s| {
        Err(s)
            .or_else(|s| builtin_text(s, "_", false).and_then(|s| s.tag_node("space_control_0")))
            .or_else(|s| builtin_text(s, "-", false).and_then(|s| s.tag_node("space_control_1")))
            .or_else(|s| builtin_text(s, "~", false).and_then(|s| s.tag_node("space_control_2")))
            .or_else(|s| builtin_text(s, "=", false).and_then(|s| s.tag_node("space_control_3")))
    })
}
#[inline]
fn parse_kw_end(state: Input) -> Output {
    state.rule(NexusRule::KW_END, |s| s.match_string("end", false))
}
#[inline]
fn parse_template_export(state: Input) -> Output {
    state.rule(NexusRule::TemplateExport, |s| {
        s.sequence(|s| {
            Ok(s).and_then(|s| parse_template_l(s)).and_then(|s| builtin_ignore(s)).and_then(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| s.repeat(0..4294967295, |s| parse_export_item(s)))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| parse_template_r(s))
                })
                .and_then(|s| s.tag_node("exports"))
            })
        })
    })
}
#[inline]
fn parse_export_item(state: Input) -> Output {
    state.rule(NexusRule::ExportItem, |s| {
        s.sequence(|s| {
            Ok(s).and_then(|s| parse_kw_export(s)).and_then(|s| builtin_ignore(s)).and_then(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| parse_identifier(s))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| {
                            s.optional(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| parse_kw_to(s))
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("language")))
                                })
                            })
                        })
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| {
                            s.optional(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| {
                                            s.sequence(|s| {
                                                Ok(s)
                                                    .and_then(|s| parse_kw_by(s))
                                                    .and_then(|s| builtin_ignore(s))
                                                    .and_then(|s| parse_kw_class(s))
                                            })
                                        })
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| parse_namepath_free(s).and_then(|s| s.tag_node("class")))
                                })
                            })
                        })
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| {
                            s.optional(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| {
                                            s.sequence(|s| {
                                                Ok(s)
                                                    .and_then(|s| parse_kw_by(s))
                                                    .and_then(|s| builtin_ignore(s))
                                                    .and_then(|s| parse_kw_trait(s))
                                            })
                                        })
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| s.optional(|s| parse_namepath_free(s)).and_then(|s| s.tag_node("trait")))
                                })
                            })
                        })
                })
                .and_then(|s| s.tag_node("name"))
            })
        })
    })
}
#[inline]
fn parse_kw_export(state: Input) -> Output {
    state.rule(NexusRule::KW_EXPORT, |s| s.match_string("export", false))
}
#[inline]
fn parse_kw_class(state: Input) -> Output {
    state.rule(NexusRule::KW_CLASS, |s| s.match_string("class", false))
}
#[inline]
fn parse_kw_trait(state: Input) -> Output {
    state.rule(NexusRule::KW_TRAIT, |s| s.match_string("trait", false))
}
#[inline]
fn parse_kw_to(state: Input) -> Output {
    state.rule(NexusRule::KW_TO, |s| s.match_string("to", false))
}
#[inline]
fn parse_kw_by(state: Input) -> Output {
    state.rule(NexusRule::KW_BY, |s| s.match_string("by", false))
}
#[inline]
fn parse_template_if(state: Input) -> Output {
    state.rule(NexusRule::TemplateIf, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_if_begin(s).and_then(|s| s.tag_node("if_begin")))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_if_else_if(s).and_then(|s| s.tag_node("if_else_if"))))
                .and_then(|s| s.optional(|s| parse_if_else(s).and_then(|s| s.tag_node("if_else"))))
                .and_then(|s| parse_if_end(s))
        })
    })
}
#[inline]
fn parse_if_begin(state: Input) -> Output {
    state.rule(NexusRule::IfBegin, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s).and_then(|s| parse_template_l(s)).and_then(|s| builtin_ignore(s)).and_then(|s| parse_kw_if(s))
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| parse_atomic(s))
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| parse_template_r(s))
                                })
                            })
                            .and_then(|s| s.repeat(0..4294967295, |s| parse_text_elements(s)).and_then(|s| s.tag_node("text")))
                    })
                    .and_then(|s| s.tag_node("condition"))
                })
        })
    })
}
#[inline]
fn parse_if_else(state: Input) -> Output {
    state.rule(NexusRule::IfElse, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| parse_template_l(s))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_kw_else(s))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_template_r(s))
                    })
                })
                .and_then(|s| s.repeat(0..4294967295, |s| parse_text_elements(s)).and_then(|s| s.tag_node("text")))
        })
    })
}
#[inline]
fn parse_if_else_if(state: Input) -> Output {
    state.rule(NexusRule::IfElseIf, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| parse_template_l(s))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_kw_else(s))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_kw_if(s))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_template_r(s))
                    })
                })
                .and_then(|s| s.repeat(0..4294967295, |s| parse_text_elements(s)).and_then(|s| s.tag_node("text")))
        })
    })
}
#[inline]
fn parse_if_end(state: Input) -> Output {
    state.rule(NexusRule::IfEnd, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_template_l(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_kw_end(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_kw_if(s)))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_template_r(s))
        })
    })
}
#[inline]
fn parse_kw_if(state: Input) -> Output {
    state.rule(NexusRule::KW_IF, |s| s.match_string("if", false))
}
#[inline]
fn parse_kw_else(state: Input) -> Output {
    state.rule(NexusRule::KW_ELSE, |s| s.match_string("else", false))
}
#[inline]
fn parse_atomic(state: Input) -> Output {
    state.rule(NexusRule::Atomic, |s| {
        Err(s)
            .or_else(|s| parse_boolean(s).and_then(|s| s.tag_node("boolean")))
            .or_else(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
            .or_else(|s| parse_number(s).and_then(|s| s.tag_node("number")))
    })
}
#[inline]
fn parse_string(state: Input) -> Output {
    state.rule(NexusRule::String, |s| {
        Err(s)
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "\"", false))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| builtin_text(s, "\"", false))
                })
                .and_then(|s| s.tag_node("string_0"))
            })
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "'", false))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| builtin_text(s, "'", false))
                })
                .and_then(|s| s.tag_node("string_1"))
            })
    })
}
#[inline]
fn parse_number(state: Input) -> Output {
    state.rule(NexusRule::Number, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(0|[1-9][0-9])").unwrap())
        })
    })
}
#[inline]
fn parse_namepath_free(state: Input) -> Output {
    state.rule(NexusRule::NamepathFree, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| {
                                    Err(s).or_else(|s| builtin_text(s, ".", false)).or_else(|s| builtin_text(s, "::", false))
                                })
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                        })
                    })
                })
        })
    })
}
#[inline]
fn parse_namepath(state: Input) -> Output {
    state.rule(NexusRule::Namepath, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_text(s, "::", false))
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                        })
                    })
                })
        })
    })
}
#[inline]
fn parse_identifier(state: Input) -> Output {
    state.rule(NexusRule::Identifier, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^([_\\p{XID_start}]\\p{XID_continue}*)").unwrap())
        })
    })
}
#[inline]
fn parse_boolean(state: Input) -> Output {
    state.rule(NexusRule::Boolean, |s| {
        Err(s)
            .or_else(|s| builtin_text(s, "true", false).and_then(|s| s.tag_node("boolean_0")))
            .or_else(|s| builtin_text(s, "false", false).and_then(|s| s.tag_node("boolean_1")))
    })
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(NexusRule::WhiteSpace, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(\\p{White_Space}+)").unwrap())
        })
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| parse_white_space(s))
}

fn builtin_any(state: Input) -> Output {
    state.rule(NexusRule::IgnoreText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(NexusRule::IgnoreText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(NexusRule::IgnoreRegex, |s| s.match_regex(regex))
}
