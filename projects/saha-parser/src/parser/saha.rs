// This file was generated by Peginator v0.3.0
// Hash of the grammar file: 7F15204979328ABCAB908FAC5932E7650CA8D2B8EC763982F4AB7CB553CBB298
// Any changes to it will be lost on regeneration

#[derive(Debug, Clone)]
pub struct SahaParser {
    pub statements: Vec<SahaStatements>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum SahaStatements {
    EndNode(EndNode),
    SlotExpressionNode(SlotExpressionNode),
}
#[derive(Debug, Clone)]
pub struct SlotExpressionNode {
    pub left: SlotL,
    pub value: IdentifierNode,
    pub right: SlotR,
}
#[derive(Debug, Clone)]
pub struct SlotL {
    pub space: Option<UnicodeSpace>,
    pub trim: Option<TrimMode>,
}
#[derive(Debug, Clone)]
pub struct SlotR {
    pub trim: Option<TrimMode>,
    pub space: Option<UnicodeSpace>,
}
#[derive(Debug, Clone)]
pub struct EndNode {
    pub left: SlotL,
    pub right: SlotR,
}
#[derive(Debug, Clone)]
pub struct EndWord;
pub type TrimMode = char;
#[derive(Debug, Clone)]
pub struct SpecialNode {
    pub string: String,
    pub position: std::ops::Range<usize>,
}
#[derive(Debug, Clone)]
pub struct IdentifierNode {
    pub string: String,
    pub position: std::ops::Range<usize>,
}
pub type XID_START = char;
pub type XID_CONTINUE = char;
#[derive(Debug, Clone)]
pub struct Whitespace;
pub type UnicodeSpace = String;
#[derive(Debug, Clone)]
pub struct Comment;
#[derive(Debug, Clone)]
pub struct CommentL {
    pub trim: TrimMode,
}
#[derive(Debug, Clone)]
pub struct CommentR {
    pub trim: TrimMode,
}
impl peginator_generated::PegParser for SahaParser {
    fn parse_advanced<T: peginator_generated::ParseTracer>(
        s: &str,
        settings: &peginator_generated::ParseSettings,
    ) -> Result<Self, peginator_generated::ParseError> {
        Ok(peginator_generated::parse_SahaParser(
            peginator_generated::ParseState::new(s, settings),
            T::new(),
            &mut Default::default(),
        )?
        .result)
    }
}
#[allow(non_snake_case, unused_variables, unused_imports, unused_mut, dead_code)]
mod peginator_generated {
    use super::*;
    use peginator::runtime::*;
    pub use peginator::runtime::{IndentedTracer, ParseError, ParseSettings, ParseState, ParseTracer, PegParser, PegPosition};
    #[derive(Default)]
    pub struct ParseCache<'a> {
        _please_dont_complain: std::marker::PhantomData<&'a ()>,
    }
    mod SahaParser_impl {
        use super::*;
        mod part_0 {
            use super::*;
            mod closure {
                use super::*;
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut iterations: usize = 0;
                let mut state = state;
                let mut statements: Vec<SahaStatements> = Vec::new();
                loop {
                    match parse_Whitespace(state.clone(), tracer, cache)
                        .and_then(|ParseOk { state, .. }| parse_SahaStatements(state, tracer, cache))
                        .map_inner(|result| vec![result])
                    {
                        Ok(ParseOk { result: __result, state: new_state, .. }) => {
                            statements.extend(__result);
                            state = new_state;
                        }
                        Err(err) => {
                            state = state.record_error(err);
                            break;
                        }
                    }
                    iterations += 1;
                }
                Ok(ParseOk { result: statements, state })
            }
            pub type Parsed = Vec<SahaStatements>;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { result: mut statements, state } = part_0::parse(state, tracer, cache)?;
            let ParseOk { state, .. } =
                parse_Whitespace(state, tracer, cache).and_then(|ParseOk { state, .. }| parse_end_of_input(state))?;
            Ok(ParseOk { result: statements, state })
        }
        pub type Parsed = Vec<SahaStatements>;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SahaParser> {
            let result = parse(state, tracer, cache)?.map(|r| super::SahaParser { statements: r });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_SahaParser<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SahaParser> {
        tracer.run_traced("SahaParser", state, |state, tracer| SahaParser_impl::rule_parser(state, tracer, cache))
    }
    mod SahaStatements_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            mut state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            ChoiceHelper::new(state)
                .choice(|state| {
                    parse_Whitespace(state, tracer, cache)
                        .and_then(|ParseOk { state, .. }| parse_SlotExpressionNode(state, tracer, cache))
                        .map_inner(Parsed__override::SlotExpressionNode)
                })
                .choice(|state| {
                    parse_Whitespace(state, tracer, cache)
                        .and_then(|ParseOk { state, .. }| parse_EndNode(state, tracer, cache))
                        .map_inner(Parsed__override::EndNode)
                })
                .end()
        }
        pub type Parsed = Parsed__override;
        use super::SahaStatements as Parsed__override;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SahaStatements> {
            let result = parse(state, tracer, cache)?;
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_SahaStatements<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SahaStatements> {
        tracer.run_traced("SahaStatements", state, |state, tracer| SahaStatements_impl::rule_parser(state, tracer, cache))
    }
    mod SlotExpressionNode_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { result: left, state } =
                parse_Whitespace(state, tracer, cache).and_then(|ParseOk { state, .. }| parse_SlotL(state, tracer, cache))?;
            let ParseOk { result: value, state } = parse_Whitespace(state, tracer, cache)
                .and_then(|ParseOk { state, .. }| parse_IdentifierNode(state, tracer, cache))?;
            let ParseOk { result: right, state } =
                parse_Whitespace(state, tracer, cache).and_then(|ParseOk { state, .. }| parse_SlotR(state, tracer, cache))?;
            Ok(ParseOk { result: Parsed { left, value, right }, state })
        }
        pub struct Parsed {
            pub left: SlotL,
            pub value: IdentifierNode,
            pub right: SlotR,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SlotExpressionNode> {
            let result = parse(state, tracer, cache)?.map(|r| super::SlotExpressionNode {
                left: r.left,
                value: r.value,
                right: r.right,
            });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_SlotExpressionNode<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SlotExpressionNode> {
        tracer
            .run_traced("SlotExpressionNode", state, |state, tracer| SlotExpressionNode_impl::rule_parser(state, tracer, cache))
    }
    mod SlotL_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { result: space, state } = parse_UnicodeSpace(state.clone(), tracer, cache)
                .map_inner(Some)
                .or_else(|err| Ok(ParseOk { result: Default::default(), state: state.record_error(err) }))?;
            let ParseOk { state, .. } = parse_string_literal(state, "{%").discard_result()?;
            let ParseOk { result: trim, state } = parse_TrimMode(state.clone(), tracer, cache)
                .map_inner(Some)
                .or_else(|err| Ok(ParseOk { result: Default::default(), state: state.record_error(err) }))?;
            Ok(ParseOk { result: Parsed { space, trim }, state })
        }
        pub struct Parsed {
            pub space: Option<UnicodeSpace>,
            pub trim: Option<TrimMode>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SlotL> {
            let result = parse(state, tracer, cache)?.map(|r| super::SlotL { space: r.space, trim: r.trim });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_SlotL<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SlotL> {
        tracer.run_traced("SlotL", state, |state, tracer| SlotL_impl::rule_parser(state, tracer, cache))
    }
    mod SlotR_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { result: trim, state } = parse_TrimMode(state.clone(), tracer, cache)
                .map_inner(Some)
                .or_else(|err| Ok(ParseOk { result: Default::default(), state: state.record_error(err) }))?;
            let ParseOk { state, .. } = parse_string_literal(state, "%}").discard_result()?;
            let ParseOk { result: space, state } = parse_UnicodeSpace(state.clone(), tracer, cache)
                .map_inner(Some)
                .or_else(|err| Ok(ParseOk { result: Default::default(), state: state.record_error(err) }))?;
            Ok(ParseOk { result: Parsed { trim, space }, state })
        }
        pub struct Parsed {
            pub trim: Option<TrimMode>,
            pub space: Option<UnicodeSpace>,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::SlotR> {
            let result = parse(state, tracer, cache)?.map(|r| super::SlotR { trim: r.trim, space: r.space });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_SlotR<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SlotR> {
        tracer.run_traced("SlotR", state, |state, tracer| SlotR_impl::rule_parser(state, tracer, cache))
    }
    mod EndNode_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { result: left, state } =
                parse_Whitespace(state, tracer, cache).and_then(|ParseOk { state, .. }| parse_SlotL(state, tracer, cache))?;
            let ParseOk { state, .. } = parse_Whitespace(state, tracer, cache)
                .and_then(|ParseOk { state, .. }| parse_EndWord(state, tracer, cache))
                .discard_result()?;
            let ParseOk { result: right, state } =
                parse_Whitespace(state, tracer, cache).and_then(|ParseOk { state, .. }| parse_SlotR(state, tracer, cache))?;
            Ok(ParseOk { result: Parsed { left, right }, state })
        }
        pub struct Parsed {
            pub left: SlotL,
            pub right: SlotR,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::EndNode> {
            let result = parse(state, tracer, cache)?.map(|r| super::EndNode { left: r.left, right: r.right });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_EndNode<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, EndNode> {
        tracer.run_traced("EndNode", state, |state, tracer| EndNode_impl::rule_parser(state, tracer, cache))
    }
    mod EndWord_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { state, .. } = parse_string_literal(state, "end").discard_result()?;
            let ParseOk { state, .. } = ChoiceHelper::new(state.clone())
                .choice(|state| parse_character_literal(state, '-').discard_result())
                .choice(|state| parse_character_literal(state, '_').discard_result())
                .end()
                .or_else(|err| Ok(ParseOk { result: (), state: state.record_error(err) }))?;
            let ParseOk { state, .. } = ChoiceHelper::new(state.clone())
                .choice(|state| parse_string_literal(state, "if").discard_result())
                .choice(|state| parse_string_literal(state, "for").discard_result())
                .end()
                .or_else(|err| Ok(ParseOk { result: (), state: state.record_error(err) }))?;
            Ok(ParseOk { result: (), state })
        }
        pub type Parsed = ();
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::EndWord> {
            let result = parse(state, tracer, cache)?.map(|r| super::EndWord {});
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_EndWord<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, EndWord> {
        tracer.run_traced("EndWord", state, |state, tracer| EndWord_impl::rule_parser(state, tracer, cache))
    }
    #[inline]
    pub(super) fn parse_TrimMode<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, TrimMode> {
        if let Ok(result) = parse_character_literal(state.clone(), '_') {
            return Ok(result);
        }
        if let Ok(result) = parse_character_literal(state.clone(), '-') {
            return Ok(result);
        }
        if let Ok(result) = parse_character_literal(state.clone(), '=') {
            return Ok(result);
        }
        Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "TrimMode" }))
    }
    mod SpecialNode_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            mut state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            ChoiceHelper::new(state)
                .choice(|state| {
                    parse_Whitespace(state, tracer, cache)
                        .and_then(|ParseOk { state, .. }| parse_string_literal(state, "null"))
                        .discard_result()
                })
                .choice(|state| {
                    parse_Whitespace(state, tracer, cache)
                        .and_then(|ParseOk { state, .. }| parse_string_literal(state, "default"))
                        .discard_result()
                })
                .choice(|state| {
                    parse_Whitespace(state, tracer, cache)
                        .and_then(|ParseOk { state, .. }| parse_string_literal(state, "true"))
                        .discard_result()
                })
                .choice(|state| {
                    parse_Whitespace(state, tracer, cache)
                        .and_then(|ParseOk { state, .. }| parse_string_literal(state, "false"))
                        .discard_result()
                })
                .end()
        }
        pub type Parsed = ();
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, SpecialNode> {
            let result = parse(state.clone(), tracer, cache)?.map_with_state(|_, new_state| {
                let string = state.slice_until(new_state).to_string();
                SpecialNode { string, position: state.range_until(new_state) }
            });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_SpecialNode<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, SpecialNode> {
        tracer.run_traced("SpecialNode", state, |state, tracer| SpecialNode_impl::rule_parser(state, tracer, cache))
    }
    impl PegPosition for SpecialNode {
        fn position(&self) -> &std::ops::Range<usize> {
            &self.position
        }
    }
    mod IdentifierNode_impl {
        use super::*;
        mod part_1 {
            use super::*;
            mod closure {
                use super::*;
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut iterations: usize = 0;
                let mut state = state;
                loop {
                    match parse_XID_CONTINUE(state.clone(), tracer, cache).discard_result() {
                        Ok(ParseOk { result: __result, state: new_state, .. }) => {
                            state = new_state;
                        }
                        Err(err) => {
                            state = state.record_error(err);
                            break;
                        }
                    }
                    iterations += 1;
                }
                Ok(ParseOk { result: (), state })
            }
            pub type Parsed = ();
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { state, .. } = ChoiceHelper::new(state)
                .choice(|state| parse_XID_START(state, tracer, cache).discard_result())
                .choice(|state| parse_character_literal(state, '_').discard_result())
                .end()?;
            let ParseOk { state, .. } = part_1::parse(state, tracer, cache)?;
            Ok(ParseOk { result: (), state })
        }
        pub type Parsed = ();
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, IdentifierNode> {
            let result = parse(state.clone(), tracer, cache)?.map_with_state(|_, new_state| {
                let string = state.slice_until(new_state).to_string();
                IdentifierNode { string, position: state.range_until(new_state) }
            });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_IdentifierNode<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, IdentifierNode> {
        tracer.run_traced("IdentifierNode", state, |state, tracer| IdentifierNode_impl::rule_parser(state, tracer, cache))
    }
    impl PegPosition for IdentifierNode {
        fn position(&self) -> &std::ops::Range<usize> {
            &self.position
        }
    }
    #[inline]
    pub(super) fn parse_XID_START<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, XID_START> {
        if let Some(c) = state.s().chars().next() {
            if !unicode_ident::is_xid_start(c) {
                return Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_START" }));
            }
        }
        else {
            return Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_START" }));
        }
        if let Ok(result) = parse_char(state.clone(), tracer, cache) {
            return Ok(result);
        }
        Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_START" }))
    }
    #[inline]
    pub(super) fn parse_XID_CONTINUE<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, XID_CONTINUE> {
        if let Some(c) = state.s().chars().next() {
            if !unicode_ident::is_xid_continue(c) {
                return Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_CONTINUE" }));
            }
        }
        else {
            return Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_CONTINUE" }));
        }
        if let Ok(result) = parse_char(state.clone(), tracer, cache) {
            return Ok(result);
        }
        Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_CONTINUE" }))
    }
    mod Whitespace_impl {
        use super::*;
        mod closure {
            use super::*;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut iterations: usize = 0;
            let mut state = state;
            loop {
                match ChoiceHelper::new(state.clone())
                    .choice(|state| parse_Comment(state, tracer, cache).discard_result())
                    .choice(|state| parse_UnicodeSpace(state, tracer, cache).discard_result())
                    .end()
                {
                    Ok(ParseOk { result: __result, state: new_state, .. }) => {
                        state = new_state;
                    }
                    Err(err) => {
                        state = state.record_error(err);
                        break;
                    }
                }
                iterations += 1;
            }
            Ok(ParseOk { result: (), state })
        }
        pub type Parsed = ();
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Whitespace> {
            let result = parse(state, tracer, cache)?.map(|r| super::Whitespace {});
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_Whitespace<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Whitespace> {
        tracer.run_traced("Whitespace", state, |state, tracer| Whitespace_impl::rule_parser(state, tracer, cache))
    }
    #[inline]
    pub(super) fn parse_UnicodeSpace<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, UnicodeSpace> {
        match crate::utils::parse_identifier(state.s()) {
            Ok((result, advance)) => Ok(ParseOk { result: result.into(), state: state.advance_safe(advance) }),
            Err(error_string) => Err(state.report_error(ParseErrorSpecifics::ExternRuleFailed {
                function_name: "crate::utils::parse_identifier",
                error_string,
            })),
        }
    }
    mod Comment_impl {
        use super::*;
        mod part_1 {
            use super::*;
            mod closure {
                use super::*;
                mod part_0 {
                    use super::*;
                    mod negative_lookahead {
                        use super::*;
                        #[inline(always)]
                        pub fn parse<'a>(
                            state: ParseState<'a>,
                            tracer: impl ParseTracer,
                            cache: &mut ParseCache<'a>,
                        ) -> ParseResult<'a, Parsed> {
                            parse_character_literal(state, '\n').discard_result()
                        }
                        pub type Parsed = ();
                    }
                    #[inline(always)]
                    pub fn parse<'a>(
                        state: ParseState<'a>,
                        tracer: impl ParseTracer,
                        cache: &mut ParseCache<'a>,
                    ) -> ParseResult<'a, Parsed> {
                        match negative_lookahead::parse(state.clone(), tracer, cache) {
                            Ok(_) => Err(state.report_error(ParseErrorSpecifics::NegativeLookaheadFailed)),
                            Err(_) => Ok(ParseOk { result: (), state }),
                        }
                    }
                    pub type Parsed = ();
                }
                #[inline(always)]
                pub fn parse<'a>(
                    state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    let ParseOk { state, .. } = part_0::parse(state, tracer, cache)?;
                    let ParseOk { state, .. } = parse_char(state, tracer, cache).discard_result()?;
                    Ok(ParseOk { result: (), state })
                }
                pub type Parsed = ();
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut iterations: usize = 0;
                let mut state = state;
                loop {
                    match closure::parse(state.clone(), tracer, cache) {
                        Ok(ParseOk { result: __result, state: new_state, .. }) => {
                            state = new_state;
                        }
                        Err(err) => {
                            state = state.record_error(err);
                            break;
                        }
                    }
                    iterations += 1;
                }
                Ok(ParseOk { result: (), state })
            }
            pub type Parsed = ();
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { state, .. } = parse_character_literal(state, '#').discard_result()?;
            let ParseOk { state, .. } = part_1::parse(state, tracer, cache)?;
            let ParseOk { state, .. } = parse_character_literal(state, '\n').discard_result()?;
            Ok(ParseOk { result: (), state })
        }
        pub type Parsed = ();
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::Comment> {
            let result = parse(state, tracer, cache)?.map(|r| super::Comment {});
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_Comment<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, Comment> {
        tracer.run_traced("Comment", state, |state, tracer| Comment_impl::rule_parser(state, tracer, cache))
    }
    mod CommentL_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { state, .. } = parse_string_literal(state, "{#").discard_result()?;
            let ParseOk { result: trim, state } = parse_TrimMode(state, tracer, cache)?;
            Ok(ParseOk { result: trim, state })
        }
        pub type Parsed = TrimMode;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::CommentL> {
            let result = parse(state, tracer, cache)?.map(|r| super::CommentL { trim: r });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_CommentL<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, CommentL> {
        tracer.run_traced("CommentL", state, |state, tracer| CommentL_impl::rule_parser(state, tracer, cache))
    }
    mod CommentR_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { result: trim, state } = parse_TrimMode(state, tracer, cache)?;
            let ParseOk { state, .. } = parse_string_literal(state, "#}").discard_result()?;
            Ok(ParseOk { result: trim, state })
        }
        pub type Parsed = TrimMode;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::CommentR> {
            let result = parse(state, tracer, cache)?.map(|r| super::CommentR { trim: r });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_CommentR<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, CommentR> {
        tracer.run_traced("CommentR", state, |state, tracer| CommentR_impl::rule_parser(state, tracer, cache))
    }
}
