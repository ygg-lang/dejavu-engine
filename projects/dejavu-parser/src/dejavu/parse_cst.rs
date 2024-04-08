use super::*;

pub(super) fn parse_cst(input: &str, rule: DejavuRule) -> OutputResult<DejavuRule> {
    state(input, |state| match rule {
        DejavuRule::ROOT => parse_root(state),
        DejavuRule::ELEMENT => parse_element(state),
        DejavuRule::TEXT_MANY => parse_text_many(state),
        DejavuRule::TEXT_ELEMENT => parse_text_element(state),
        DejavuRule::TEMPLATE_EXPORT => parse_template_export(state),
        DejavuRule::EXPORT_ITEM => parse_export_item(state),
        DejavuRule::IF_BLOCK => parse_if_block(state),
        DejavuRule::TEMPLATE_IF => parse_template_if(state),
        DejavuRule::TEMPLATE_ELSE_IF => parse_template_else_if(state),
        DejavuRule::TEMPLATE_ELSE => parse_template_else(state),
        DejavuRule::FOR_BLOCK => parse_for_block(state),
        DejavuRule::TEMPLATE_FOR => parse_template_for(state),
        DejavuRule::TEMPLATE_END => parse_template_end(state),
        DejavuRule::PATTERN => parse_pattern(state),
        DejavuRule::BARE_PATTERN => parse_bare_pattern(state),
        DejavuRule::EXPRESSION => parse_expression(state),
        DejavuRule::EXPRESSION_REST => parse_expression_rest(state),
        DejavuRule::INFIX => parse_infix(state),
        DejavuRule::TERM => parse_term(state),
        DejavuRule::PREFIX => parse_prefix(state),
        DejavuRule::SUFFIX => parse_suffix(state),
        DejavuRule::FUNCTION_CALL => parse_function_call(state),
        DejavuRule::DOT_CALL => parse_dot_call(state),
        DejavuRule::TUPLE => parse_tuple(state),
        DejavuRule::ARGUEMENT => parse_arguement(state),
        DejavuRule::KEY => parse_key(state),
        DejavuRule::ATOMIC => parse_atomic(state),
        DejavuRule::GROUP_EXPRESSION => parse_group_expression(state),
        DejavuRule::SPECIAL => parse_special(state),
        DejavuRule::NUMBER => parse_number(state),
        DejavuRule::DEC => parse_dec(state),
        DejavuRule::BIN => parse_bin(state),
        DejavuRule::OCT => parse_oct(state),
        DejavuRule::HEX => parse_hex(state),
        DejavuRule::UNIT => parse_unit(state),
        DejavuRule::STRING => parse_string(state),
        DejavuRule::NAMEPATH_FREE => parse_namepath_free(state),
        DejavuRule::NAMEPATH => parse_namepath(state),
        DejavuRule::IDENTIFIER => parse_identifier(state),
        DejavuRule::KW_EXPORT => parse_kw_export(state),
        DejavuRule::KW_CLASS => parse_kw_class(state),
        DejavuRule::KW_TRAIT => parse_kw_trait(state),
        DejavuRule::KW_TO => parse_kw_to(state),
        DejavuRule::KW_BY => parse_kw_by(state),
        DejavuRule::KW_FOR => parse_kw_for(state),
        DejavuRule::KW_IN => parse_kw_in(state),
        DejavuRule::KW_IF => parse_kw_if(state),
        DejavuRule::KW_ELSE => parse_kw_else(state),
        DejavuRule::KW_END => parse_kw_end(state),
        DejavuRule::TEMPLATE_L => parse_template_l(state),
        DejavuRule::TEMPLATE_R => parse_template_r(state),
        DejavuRule::SPACE_CONTROL => parse_space_control(state),
        DejavuRule::DOT => parse_dot(state),
        DejavuRule::COMMA => parse_comma(state),
        DejavuRule::COLON => parse_colon(state),
        DejavuRule::TEXT_SPACE => parse_text_space(state),
        DejavuRule::TEXT_WORD => parse_text_word(state),
        DejavuRule::WHITE_SPACE => parse_white_space(state),
        DejavuRule::INFIX0 => parse_infix_0(state),
        DejavuRule::INFIX1 => parse_infix_1(state),
        DejavuRule::PREFIX0 => parse_prefix_0(state),
        DejavuRule::SUFFIX0 => parse_suffix_0(state),
        DejavuRule::SPECIAL0 => parse_special_0(state),
        DejavuRule::SPECIAL1 => parse_special_1(state),
        DejavuRule::SPECIAL2 => parse_special_2(state),
        DejavuRule::STRING0 => parse_string_0(state),
        DejavuRule::STRING1 => parse_string_1(state),
        DejavuRule::SPACE_CONTROL0 => parse_space_control_0(state),
        DejavuRule::SPACE_CONTROL1 => parse_space_control_1(state),
        DejavuRule::SPACE_CONTROL2 => parse_space_control_2(state),
        DejavuRule::SPACE_CONTROL3 => parse_space_control_3(state),
        DejavuRule::SPACE_CONTROL4 => parse_space_control_4(state),
        DejavuRule::HiddenText => unreachable!(),
    })
}
#[inline]
fn parse_root(state: Input) -> Output {
    state.rule(DejavuRule::ROOT, |s| {
        s.repeat(0..4294967295, |s| {
            s.sequence(|s| {
                Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| parse_element(s).and_then(|s| s.tag_node("element")))
            })
        })
    })
}
#[inline]
fn parse_element(state: Input) -> Output {
    state.rule(DejavuRule::ELEMENT, |s| {
        Err(s)
            .or_else(|s| parse_text_many(s).and_then(|s| s.tag_node("text_many")))
            .or_else(|s| parse_template_export(s).and_then(|s| s.tag_node("template_export")))
            .or_else(|s| parse_if_block(s).and_then(|s| s.tag_node("if_block")))
            .or_else(|s| parse_for_block(s).and_then(|s| s.tag_node("for_block")))
    })
}
#[inline]
fn parse_text_many(state: Input) -> Output {
    state.rule(DejavuRule::TEXT_MANY, |s| {
        s.repeat(1..4294967295, |s| parse_text_element(s).and_then(|s| s.tag_node("text_element")))
    })
}
#[inline]
fn parse_text_element(state: Input) -> Output {
    state.rule(DejavuRule::TEXT_ELEMENT, |s| {
        Err(s)
            .or_else(|s| parse_text_space(s).and_then(|s| s.tag_node("text_space")))
            .or_else(|s| parse_text_word(s).and_then(|s| s.tag_node("text_word")))
    })
}
#[inline]
fn parse_template_export(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_EXPORT, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| s.sequence(|s| Ok(s).and_then(|s| parse_template_l(s)).and_then(|s| builtin_ignore(s))))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| parse_export_item(s)))
                    })
                    .and_then(|s| s.tag_node("exports"))
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_template_r(s))
        })
    })
}
#[inline]
fn parse_export_item(state: Input) -> Output {
    state.rule(DejavuRule::EXPORT_ITEM, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| s.sequence(|s| Ok(s).and_then(|s| parse_kw_export(s)).and_then(|s| builtin_ignore(s))))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("name")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| {
                                    s.sequence(|s| Ok(s).and_then(|s| parse_kw_to(s)).and_then(|s| builtin_ignore(s)))
                                })
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
                                            .and_then(|s| builtin_ignore(s))
                                    })
                                })
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
                                            .and_then(|s| builtin_ignore(s))
                                    })
                                })
                                .and_then(|s| s.optional(|s| parse_namepath_free(s)).and_then(|s| s.tag_node("trait")))
                        })
                    })
                })
        })
    })
}
#[inline]
fn parse_if_block(state: Input) -> Output {
    state.rule(DejavuRule::IF_BLOCK, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_template_if(s).and_then(|s| s.tag_node("template_if")))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| parse_template_else_if(s).and_then(|s| s.tag_node("template_else_if")))
                })
                .and_then(|s| s.optional(|s| parse_template_else(s).and_then(|s| s.tag_node("template_else"))))
                .and_then(|s| parse_template_end(s).and_then(|s| s.tag_node("template_end")))
        })
    })
}
#[inline]
fn parse_template_if(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_IF, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_kw_if(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r")))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_element(s).and_then(|s| s.tag_node("element"))))
        })
    })
}
#[inline]
fn parse_template_else_if(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_ELSE_IF, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_kw_else(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_kw_if(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r")))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_element(s).and_then(|s| s.tag_node("element"))))
        })
    })
}
#[inline]
fn parse_template_else(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_ELSE, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_kw_else(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r")))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_element(s).and_then(|s| s.tag_node("element"))))
        })
    })
}
#[inline]
fn parse_for_block(state: Input) -> Output {
    state.rule(DejavuRule::FOR_BLOCK, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_template_for(s).and_then(|s| s.tag_node("template_for")))
                .and_then(|s| s.optional(|s| parse_template_else(s).and_then(|s| s.tag_node("template_else"))))
                .and_then(|s| parse_template_end(s).and_then(|s| s.tag_node("template_end")))
        })
    })
}
#[inline]
fn parse_template_for(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_FOR, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l")))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_kw_for(s))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| s.lookahead(false, |s| parse_kw_in(s).and_then(|s| s.tag_node("kw_in"))))
                            .and_then(|s| parse_pattern(s).and_then(|s| s.tag_node("pattern")))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_kw_in(s))
                            .and_then(|s| builtin_ignore(s))
                    })
                })
                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("iterator")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| {
                                    s.sequence(|s| Ok(s).and_then(|s| parse_kw_if(s)).and_then(|s| builtin_ignore(s)))
                                })
                                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("condition")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r")))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_element(s).and_then(|s| s.tag_node("element"))))
        })
    })
}
#[inline]
fn parse_template_end(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_END, |s| {
        Err(s)
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l")))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| parse_kw_end(s))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| s.optional(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier"))))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r")))
                })
            })
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| parse_template_l(s).and_then(|s| s.tag_node("template_l")))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| s.optional(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier"))))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| parse_kw_end(s))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| parse_template_r(s).and_then(|s| s.tag_node("template_r")))
                })
            })
    })
}
#[inline]
fn parse_pattern(state: Input) -> Output {
    state.rule(DejavuRule::PATTERN, |s| {
        Err(s)
            .or_else(|s| parse_bare_pattern(s).and_then(|s| s.tag_node("bare_pattern")))
            .or_else(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
    })
}
#[inline]
fn parse_bare_pattern(state: Input) -> Output {
    state.rule(DejavuRule::BARE_PATTERN, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_pattern(s).and_then(|s| s.tag_node("pattern")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| parse_comma(s).and_then(|s| s.tag_node("comma")))
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| parse_pattern(s).and_then(|s| s.tag_node("pattern")))
                                })
                            })
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_comma(s).and_then(|s| s.tag_node("comma"))))
        })
    })
}
#[inline]
fn parse_expression(state: Input) -> Output {
    state.rule(DejavuRule::EXPRESSION, |s| {
        s.sequence(|s| {
            Ok(s).and_then(|s| parse_term(s).and_then(|s| s.tag_node("term"))).and_then(|s| builtin_ignore(s)).and_then(|s| {
                s.repeat(0..4294967295, |s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_expression_rest(s).and_then(|s| s.tag_node("expression_rest")))
                    })
                })
            })
        })
    })
}
#[inline]
fn parse_expression_rest(state: Input) -> Output {
    state.rule(DejavuRule::EXPRESSION_REST, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_infix(s).and_then(|s| s.tag_node("infix")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_term(s).and_then(|s| s.tag_node("term")))
        })
    })
}
#[inline]
fn parse_infix(state: Input) -> Output {
    state.rule(DejavuRule::INFIX, |s| {
        Err(s)
            .or_else(|s| parse_infix_0(s).and_then(|s| s.tag_node("infix_0")))
            .or_else(|s| parse_infix_1(s).and_then(|s| s.tag_node("infix_1")))
    })
}
#[inline]
fn parse_term(state: Input) -> Output {
    state.rule(DejavuRule::TERM, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_prefix(s).and_then(|s| s.tag_node("prefix")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_atomic(s).and_then(|s| s.tag_node("atomic")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_suffix(s).and_then(|s| s.tag_node("suffix")))
                        })
                    })
                })
        })
    })
}
#[inline]
fn parse_prefix(state: Input) -> Output {
    state.rule(DejavuRule::PREFIX, |s| Err(s).or_else(|s| parse_prefix_0(s).and_then(|s| s.tag_node("prefix_0"))))
}
#[inline]
fn parse_suffix(state: Input) -> Output {
    state.rule(DejavuRule::SUFFIX, |s| {
        Err(s)
            .or_else(|s| parse_suffix_0(s).and_then(|s| s.tag_node("suffix_0")))
            .or_else(|s| parse_dot_call(s).and_then(|s| s.tag_node("dot_call")))
    })
}
#[inline]
fn parse_function_call(state: Input) -> Output {
    state.rule(DejavuRule::FUNCTION_CALL, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_namepath(s).and_then(|s| s.tag_node("namepath")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_tuple(s).and_then(|s| s.tag_node("tuple"))))
        })
    })
}
#[inline]
fn parse_dot_call(state: Input) -> Output {
    state.rule(DejavuRule::DOT_CALL, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, ".", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_tuple(s).and_then(|s| s.tag_node("tuple"))))
        })
    })
}
#[inline]
fn parse_tuple(state: Input) -> Output {
    state.rule(DejavuRule::TUPLE, |s| {
        Err(s)
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "(", false))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| builtin_text(s, ")", false))
                })
            })
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "(", false))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| parse_arguement(s).and_then(|s| s.tag_node("arguement")))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| parse_comma(s).and_then(|s| s.tag_node("comma")))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| builtin_text(s, ")", false))
                })
            })
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "(", false))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| parse_arguement(s).and_then(|s| s.tag_node("arguement")))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| {
                            s.repeat(0..4294967295, |s| {
                                s.sequence(|s| {
                                    Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                        s.sequence(|s| {
                                            Ok(s)
                                                .and_then(|s| parse_comma(s).and_then(|s| s.tag_node("comma")))
                                                .and_then(|s| builtin_ignore(s))
                                                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                                        })
                                    })
                                })
                            })
                        })
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| s.optional(|s| parse_comma(s).and_then(|s| s.tag_node("comma"))))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| builtin_text(s, ")", false))
                })
            })
    })
}
#[inline]
fn parse_arguement(state: Input) -> Output {
    state.rule(DejavuRule::ARGUEMENT, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_key(s).and_then(|s| s.tag_node("key")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_colon(s).and_then(|s| s.tag_node("colon")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
        })
    })
}
#[inline]
fn parse_key(state: Input) -> Output {
    state.rule(DejavuRule::KEY, |s| Err(s).or_else(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier"))))
}
#[inline]
fn parse_atomic(state: Input) -> Output {
    state.rule(DejavuRule::ATOMIC, |s| {
        Err(s)
            .or_else(|s| parse_group_expression(s).and_then(|s| s.tag_node("group_expression")))
            .or_else(|s| parse_tuple(s).and_then(|s| s.tag_node("tuple")))
            .or_else(|s| parse_special(s).and_then(|s| s.tag_node("special")))
            .or_else(|s| parse_function_call(s).and_then(|s| s.tag_node("function_call")))
            .or_else(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
            .or_else(|s| parse_number(s).and_then(|s| s.tag_node("number")))
    })
}
#[inline]
fn parse_group_expression(state: Input) -> Output {
    state.rule(DejavuRule::GROUP_EXPRESSION, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "(", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, ")", false))
        })
    })
}
#[inline]
fn parse_special(state: Input) -> Output {
    state.rule(DejavuRule::SPECIAL, |s| {
        Err(s)
            .or_else(|s| parse_special_0(s).and_then(|s| s.tag_node("special_0")))
            .or_else(|s| parse_special_1(s).and_then(|s| s.tag_node("special_1")))
            .or_else(|s| parse_special_2(s).and_then(|s| s.tag_node("special_2")))
    })
}
#[inline]
fn parse_number(state: Input) -> Output {
    state.rule(DejavuRule::NUMBER, |s| {
        Err(s)
            .or_else(|s| parse_dec(s).and_then(|s| s.tag_node("dec")))
            .or_else(|s| parse_bin(s).and_then(|s| s.tag_node("bin")))
            .or_else(|s| parse_hex(s).and_then(|s| s.tag_node("hex")))
    })
}
#[inline]
fn parse_dec(state: Input) -> Output {
    state.rule(DejavuRule::DEC, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)((0|[1-9][0-9])(.[0-9]+)?)").unwrap())
        })
    })
}
#[inline]
fn parse_bin(state: Input) -> Output {
    state.rule(DejavuRule::BIN, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(0b[0-1]+)").unwrap())
        })
    })
}
#[inline]
fn parse_oct(state: Input) -> Output {
    state.rule(DejavuRule::OCT, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(0o[0-7]+)").unwrap())
        })
    })
}
#[inline]
fn parse_hex(state: Input) -> Output {
    state.rule(DejavuRule::HEX, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(0x[0-9a-fA-F]+)").unwrap())
        })
    })
}
#[inline]
fn parse_unit(state: Input) -> Output {
    state.rule(DejavuRule::UNIT, |s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
}
#[inline]
fn parse_string(state: Input) -> Output {
    state.rule(DejavuRule::STRING, |s| {
        Err(s)
            .or_else(|s| parse_string_0(s).and_then(|s| s.tag_node("string_0")))
            .or_else(|s| parse_string_1(s).and_then(|s| s.tag_node("string_1")))
    })
}
#[inline]
fn parse_namepath_free(state: Input) -> Output {
    state.rule(DejavuRule::NAMEPATH_FREE, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| {
                                            Err(s)
                                                .or_else(|s| builtin_text(s, ".", false))
                                                .or_else(|s| builtin_text(s, "::", false))
                                        })
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                                })
                            })
                        })
                    })
                })
        })
    })
}
#[inline]
fn parse_namepath(state: Input) -> Output {
    state.rule(DejavuRule::NAMEPATH, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
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
        })
    })
}
#[inline]
fn parse_identifier(state: Input) -> Output {
    state.rule(DejavuRule::IDENTIFIER, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([_\\p{XID_start}]\\p{XID_continue}*)").unwrap())
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
fn parse_kw_for(state: Input) -> Output {
    state.rule(DejavuRule::KW_FOR, |s| s.match_string("for", false))
}
#[inline]
fn parse_kw_in(state: Input) -> Output {
    state.rule(DejavuRule::KW_IN, |s| s.match_string("in", false))
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
fn parse_kw_end(state: Input) -> Output {
    state.rule(DejavuRule::KW_END, |s| s.match_string("end", false))
}
#[inline]
fn parse_template_l(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_L, |s| {
        s.sequence(|s| Ok(s).and_then(|s| builtin_text(s, "<%", false)).and_then(|s| s.optional(|s| parse_space_control(s))))
    })
}
#[inline]
fn parse_template_r(state: Input) -> Output {
    state.rule(DejavuRule::TEMPLATE_R, |s| {
        s.sequence(|s| Ok(s).and_then(|s| s.optional(|s| parse_space_control(s))).and_then(|s| builtin_text(s, "%>", false)))
    })
}
#[inline]
fn parse_space_control(state: Input) -> Output {
    state.rule(DejavuRule::SPACE_CONTROL, |s| {
        Err(s)
            .or_else(|s| parse_space_control_0(s).and_then(|s| s.tag_node("space_control_0")))
            .or_else(|s| parse_space_control_1(s).and_then(|s| s.tag_node("space_control_1")))
            .or_else(|s| parse_space_control_2(s).and_then(|s| s.tag_node("space_control_2")))
            .or_else(|s| parse_space_control_3(s).and_then(|s| s.tag_node("space_control_3")))
            .or_else(|s| parse_space_control_4(s).and_then(|s| s.tag_node("space_control_4")))
    })
}
#[inline]
fn parse_dot(state: Input) -> Output {
    state.rule(DejavuRule::DOT, |s| s.match_string(".", false))
}
#[inline]
fn parse_comma(state: Input) -> Output {
    state.rule(DejavuRule::COMMA, |s| s.match_string(",", false))
}
#[inline]
fn parse_colon(state: Input) -> Output {
    state.rule(DejavuRule::COLON, |s| s.match_string(":", false))
}
#[inline]
fn parse_text_space(state: Input) -> Output {
    state.rule(DejavuRule::TEXT_SPACE, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(\\p{White_Space}+)").unwrap())
        })
    })
}
#[inline]
fn parse_text_word(state: Input) -> Output {
    state.rule(DejavuRule::TEXT_WORD, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([^<\\p{White_Space}]+)").unwrap())
        })
    })
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(DejavuRule::WHITE_SPACE, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(\\p{White_Space}+)").unwrap())
        })
    })
}
#[inline]
fn parse_infix_0(state: Input) -> Output {
    state.rule(DejavuRule::INFIX0, |s| s.match_string("+", false))
}
#[inline]
fn parse_infix_1(state: Input) -> Output {
    state.rule(DejavuRule::INFIX1, |s| s.match_string("-", false))
}
#[inline]
fn parse_prefix_0(state: Input) -> Output {
    state.rule(DejavuRule::PREFIX0, |s| s.match_string("!", false))
}
#[inline]
fn parse_suffix_0(state: Input) -> Output {
    state.rule(DejavuRule::SUFFIX0, |s| s.match_string("?", false))
}
#[inline]
fn parse_special_0(state: Input) -> Output {
    state.rule(DejavuRule::SPECIAL0, |s| s.match_string("null", false))
}
#[inline]
fn parse_special_1(state: Input) -> Output {
    state.rule(DejavuRule::SPECIAL1, |s| s.match_string("true", false))
}
#[inline]
fn parse_special_2(state: Input) -> Output {
    state.rule(DejavuRule::SPECIAL2, |s| s.match_string("false", false))
}
#[inline]
fn parse_string_0(state: Input) -> Output {
    state.rule(DejavuRule::STRING0, |s| {
        s.sequence(|s| Ok(s).and_then(|s| builtin_text(s, "'", false)).and_then(|s| builtin_text(s, "'", false)))
    })
}
#[inline]
fn parse_string_1(state: Input) -> Output {
    state.rule(DejavuRule::STRING1, |s| {
        s.sequence(|s| Ok(s).and_then(|s| builtin_text(s, "\"", false)).and_then(|s| builtin_text(s, "\"", false)))
    })
}
#[inline]
fn parse_space_control_0(state: Input) -> Output {
    state.rule(DejavuRule::SPACE_CONTROL0, |s| s.match_string("=", false))
}
#[inline]
fn parse_space_control_1(state: Input) -> Output {
    state.rule(DejavuRule::SPACE_CONTROL1, |s| s.match_string("~", false))
}
#[inline]
fn parse_space_control_2(state: Input) -> Output {
    state.rule(DejavuRule::SPACE_CONTROL2, |s| s.match_string("-", false))
}
#[inline]
fn parse_space_control_3(state: Input) -> Output {
    state.rule(DejavuRule::SPACE_CONTROL3, |s| s.match_string("_", false))
}
#[inline]
fn parse_space_control_4(state: Input) -> Output {
    state.rule(DejavuRule::SPACE_CONTROL4, |s| s.match_string(".", false))
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| parse_white_space(s))
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
