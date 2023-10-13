use super::*;

pub(super) fn parse_cst(input: &str, rule: DejavuRule) -> OutputResult<DejavuRule> {
    state(input, |state| match rule {
        DejavuRule::Root => parse_root(state),
        DejavuRule::Element => parse_element(state),
        DejavuRule::TextElements => parse_text_elements(state),
        DejavuRule::TEMPLATE_E => parse_template_e(state),
        DejavuRule::TEXT_SPACE => parse_text_space(state),
        DejavuRule::TEXT_WORD => parse_text_word(state),
        DejavuRule::TEMPLATE_L => parse_template_l(state),
        DejavuRule::TEMPLATE_R => parse_template_r(state),
        DejavuRule::SPACE_CONTROL => parse_space_control(state),
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
        DejavuRule::Atomic => parse_atomic(state),
        DejavuRule::String => parse_string(state),
        DejavuRule::Number => parse_number(state),
        DejavuRule::NamepathFree => parse_namepath_free(state),
        DejavuRule::Namepath => parse_namepath(state),
        DejavuRule::Identifier => parse_identifier(state),
        DejavuRule::Boolean => parse_boolean(state),
        DejavuRule::WhiteSpace => parse_white_space(state),
        DejavuRule::IgnoreText => unreachable!(),
        DejavuRule::IgnoreRegex => unreachable!(),
    })
}
#[inline]
fn parse_root(state: Input) -> Output {
    state.rule(DejavuRule::Root, |s| s.repeat(0..4294967295, |s| parse_element(s).and_then(|s| s.tag_node("element"))))
}
#[inline]
fn parse_element(state: Input) -> Output {
    state.rule(DejavuRule::Element, |s| {
        Err(s)
            .or_else(|s| parse_text_elements(s).and_then(|s| s.tag_node("text_elements")))
            .or_else(|s| parse_template_export(s).and_then(|s| s.tag_node("template_export")))
            .or_else(|s| parse_template_if(s).and_then(|s| s.tag_node("template_if")))
    })
}
#[inline]
fn parse_text_elements(state: Input) -> Output {
    state.rule(DejavuRule::TextElements, |s| {
        Err(s)
            .or_else(|s| parse_template_e(s).and_then(|s| s.tag_node("template_e")))
            .or_else(|s| parse_text_space(s).and_then(|s| s.tag_node("text_space")))
            .or_else(|s| parse_text_word(s).and_then(|s| s.tag_node("text_word")))
    })
}
#[inline]
fn parse_template_e(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_E, |s| s.match_string("<<%", false))
}
#[inline]
fn parse_text_space(state: Input) -> Output {
    state.rule(DejavuRule::TEXT_SPACE, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(\\p{White_Space}+)").unwrap())
        })
    })
}
#[inline]
fn parse_text_word(state: Input) -> Output {
    state.rule(DejavuRule::TEXT_WORD, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^([^<\\p{White_Space}]+)").unwrap())
        })
    })
}
#[inline]
fn parse_template_l(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_L, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "<%", false))
                .and_then(|s| parse_space_control(s).and_then(|s| s.tag_node("space_control")))
        })
    })
}
#[inline]
fn parse_template_r(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_R, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_space_control(s).and_then(|s| s.tag_node("space_control")))
                .and_then(|s| builtin_text(s, "%>", false))
        })
    })
}
#[inline]
fn parse_space_control(state: Input) -> Output {
    state.rule(DejavuRule::SPACE_CONTROL, |s| {
        Err(s)
            .or_else(|s| builtin_text(s, "_", false).and_then(|s| s.tag_node("space_control_0")))
            .or_else(|s| builtin_text(s, "-", false).and_then(|s| s.tag_node("space_control_1")))
            .or_else(|s| builtin_text(s, "~", false).and_then(|s| s.tag_node("space_control_2")))
            .or_else(|s| builtin_text(s, "=", false).and_then(|s| s.tag_node("space_control_3")))
    })
}

#[inline]
fn parse_kw_end(state: Input) -> Output {
    state.rule(DejavuRule::KW_END, |s| s.match_string("end", false))
}

#[inline]
fn parse_template_export(state: Input) -> Output {
    state.rule(DejavuRule::TemplateExport, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| {
                                s.repeat(0..4294967295, |s| parse_export_item(s).and_then(|s| s.tag_node("export_item")))
                            })
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r")))
                    })
                    .and_then(|s| s.tag_node("exports"))
                })
        })
    })
}

#[inline]
fn parse_export_item(state: Input) -> Output {
    state.rule(DejavuRule::ExportItem, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_kw_export(s).and_then(|s| s.tag_node("kw_export")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| {
                                s.optional(|s| {
                                    s.sequence(|s| {
                                        Ok(s)
                                            .and_then(|s| parse_kw_to(s).and_then(|s| s.tag_node("kw_to")))
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
                                                        .and_then(|s| parse_kw_by(s).and_then(|s| s.tag_node("kw_by")))
                                                        .and_then(|s| builtin_ignore(s))
                                                        .and_then(|s| parse_kw_class(s).and_then(|s| s.tag_node("kw_class")))
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
                                                        .and_then(|s| parse_kw_by(s).and_then(|s| s.tag_node("kw_by")))
                                                        .and_then(|s| builtin_ignore(s))
                                                        .and_then(|s| parse_kw_trait(s).and_then(|s| s.tag_node("kw_trait")))
                                                })
                                            })
                                            .and_then(|s| builtin_ignore(s))
                                            .and_then(|s| {
                                                s.optional(|s| parse_namepath_free(s).and_then(|s| s.tag_node("namepath_free")))
                                                    .and_then(|s| s.tag_node("trait"))
                                            })
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
    state.rule(DejavuRule::KW_EXPORT, |s| s.match_string("export", false))
}

#[inline]
fn parse_kw_class(state: Input) -> Output {
    state.rule(DejavuRule::KW_CLASS, |s| s.match_string("class", false))
}

#[inline]
fn parse_kw_trait(state: Input) -> Output {
    state.rule(DejavuRule::KW_TRAIT, |s| s.match_string("trait", false))
}

#[inline]
fn parse_kw_to(state: Input) -> Output {
    state.rule(DejavuRule::KW_TO, |s| s.match_string("to", false))
}

#[inline]
fn parse_kw_by(state: Input) -> Output {
    state.rule(DejavuRule::KW_BY, |s| s.match_string("by", false))
}
#[inline]
fn parse_template_if(state: Input) -> Output {
    state.rule(DejavuRule::TemplateIf, |s| {
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
    state.rule(DejavuRule::IfBegin, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l")))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_kw_if(s).and_then(|s| s.tag_node("kw_if")))
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| parse_atomic(s).and_then(|s| s.tag_node("atomic")))
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r")))
                                })
                            })
                            .and_then(|s| {
                                s.repeat(0..4294967295, |s| parse_text_elements(s).and_then(|s| s.tag_node("text_elements")))
                                    .and_then(|s| s.tag_node("text"))
                            })
                    })
                    .and_then(|s| s.tag_node("condition"))
                })
        })
    })
}
#[inline]
fn parse_if_else(state: Input) -> Output {
    state.rule(DejavuRule::IfElse, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l")))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_kw_else(s).and_then(|s| s.tag_node("kw_else")))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r")))
                    })
                })
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| parse_text_elements(s).and_then(|s| s.tag_node("text_elements")))
                        .and_then(|s| s.tag_node("text"))
                })
        })
    })
}
#[inline]
fn parse_if_else_if(state: Input) -> Output {
    state.rule(DejavuRule::IfElseIf, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l")))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_kw_else(s).and_then(|s| s.tag_node("kw_else")))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_kw_if(s).and_then(|s| s.tag_node("kw_if")))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r")))
                    })
                })
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| parse_text_elements(s).and_then(|s| s.tag_node("text_elements")))
                        .and_then(|s| s.tag_node("text"))
                })
        })
    })
}
#[inline]
fn parse_if_end(state: Input) -> Output {
    state.rule(DejavuRule::IfEnd, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_kw_end(s).and_then(|s| s.tag_node("kw_end")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_kw_if(s).and_then(|s| s.tag_node("kw_if"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r")))
        })
    })
}
#[inline]
fn parse_kw_if(state: Input) -> Output {
    state.rule(DejavuRule::KW_IF, |s| s.match_string("if", false))
}
#[inline]
fn parse_kw_else(state: Input) -> Output {
    state.rule(DejavuRule::KW_ELSE, |s| s.match_string("else", false))
}
#[inline]
fn parse_atomic(state: Input) -> Output {
    state.rule(DejavuRule::Atomic, |s| {
        Err(s)
            .or_else(|s| parse_boolean(s).and_then(|s| s.tag_node("boolean")))
            .or_else(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
            .or_else(|s| parse_number(s).and_then(|s| s.tag_node("number")))
    })
}

#[inline]
fn parse_string(state: Input) -> Output {
    state.rule(DejavuRule::String, |s| {
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
    state.rule(DejavuRule::Number, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(0|[1-9][0-9])").unwrap())
        })
    })
}

#[inline]
fn parse_namepath_free(state: Input) -> Output {
    state.rule(DejavuRule::NamepathFree, |s| {
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
    state.rule(DejavuRule::Namepath, |s| {
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
    state.rule(DejavuRule::Identifier, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^([_\\p{XID_start}]\\p{XID_continue}*)").unwrap())
        })
    })
}
#[inline]
fn parse_boolean(state: Input) -> Output {
    state.rule(DejavuRule::Boolean, |s| {
        Err(s)
            .or_else(|s| builtin_text(s, "true", false).and_then(|s| s.tag_node("boolean_0")))
            .or_else(|s| builtin_text(s, "false", false).and_then(|s| s.tag_node("boolean_1")))
    })
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(DejavuRule::WhiteSpace, |s| {
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
    state.rule(DejavuRule::IgnoreText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(DejavuRule::IgnoreText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(DejavuRule::IgnoreRegex, |s| s.match_regex(regex))
}
