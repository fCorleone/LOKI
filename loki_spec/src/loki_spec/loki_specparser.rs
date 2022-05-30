// Generated from Loki_spec.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use super::loki_speclistener::*;
use super::loki_specvisitor::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{cast, cast_mut, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::*;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const T__0: isize = 1;
pub const T__1: isize = 2;
pub const T__2: isize = 3;
pub const T__3: isize = 4;
pub const T__4: isize = 5;
pub const T__5: isize = 6;
pub const T__6: isize = 7;
pub const T__7: isize = 8;
pub const T__8: isize = 9;
pub const T__9: isize = 10;
pub const T__10: isize = 11;
pub const T__11: isize = 12;
pub const T__12: isize = 13;
pub const T__13: isize = 14;
pub const T__14: isize = 15;
pub const T__15: isize = 16;
pub const T__16: isize = 17;
pub const T__17: isize = 18;
pub const T__18: isize = 19;
pub const T__19: isize = 20;
pub const T__20: isize = 21;
pub const T__21: isize = 22;
pub const T__22: isize = 23;
pub const T__23: isize = 24;
pub const T__24: isize = 25;
pub const T__25: isize = 26;
pub const T__26: isize = 27;
pub const T__27: isize = 28;
pub const T__28: isize = 29;
pub const T__29: isize = 30;
pub const T__30: isize = 31;
pub const T__31: isize = 32;
pub const T__32: isize = 33;
pub const T__33: isize = 34;
pub const SPECNAME: isize = 35;
pub const LOKI: isize = 36;
pub const Message: isize = 37;
pub const Attribute: isize = 38;
pub const Name: isize = 39;
pub const Type: isize = 40;
pub const Unlimited: isize = 41;
pub const ExpectedMsg: isize = 42;
pub const Lsign: isize = 43;
pub const Rsign: isize = 44;
pub const Euqal: isize = 45;
pub const LINE_COMMENT: isize = 46;
pub const COMMENT: isize = 47;
pub const WS: isize = 48;
pub const ID: isize = 49;
pub const INT: isize = 50;
pub const FLOAT: isize = 51;
pub const STRING: isize = 52;
pub const SHEBANG: isize = 53;
pub const RULE_document: usize = 0;
pub const RULE_loki_begin: usize = 1;
pub const RULE_loki_end: usize = 2;
pub const RULE_spec_content: usize = 3;
pub const RULE_message: usize = 4;
pub const RULE_msg_begin: usize = 5;
pub const RULE_msg_end: usize = 6;
pub const RULE_msg_content: usize = 7;
pub const RULE_attribute: usize = 8;
pub const RULE_attr_begin: usize = 9;
pub const RULE_attr_end: usize = 10;
pub const RULE_attr_type: usize = 11;
pub const RULE_ele_type: usize = 12;
pub const RULE_reff: usize = 13;
pub const RULE_size: usize = 14;
pub const RULE_option: usize = 15;
pub const RULE_key_type: usize = 16;
pub const RULE_value_type: usize = 17;
pub const RULE_mutator: usize = 18;
pub const RULE_mutation: usize = 19;
pub const RULE_param: usize = 20;
pub const RULE_algo: usize = 21;
pub const RULE_expectedMsg: usize = 22;
pub const RULE_exc_begin: usize = 23;
pub const RULE_exc_end: usize = 24;
pub const ruleNames: [&'static str; 25] = [
    "document",
    "loki_begin",
    "loki_end",
    "spec_content",
    "message",
    "msg_begin",
    "msg_end",
    "msg_content",
    "attribute",
    "attr_begin",
    "attr_end",
    "attr_type",
    "ele_type",
    "reff",
    "size",
    "option",
    "key_type",
    "value_type",
    "mutator",
    "mutation",
    "param",
    "algo",
    "expectedMsg",
    "exc_begin",
    "exc_end",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 46] = [
    None,
    Some("'/'"),
    Some("'Number'"),
    Some("'String'"),
    Some("'Bool'"),
    Some("'Byte'"),
    Some("'Timestamp'"),
    Some("'Hash'"),
    Some("'BigNumber'"),
    Some("'Array'"),
    Some("'Signature'"),
    Some("'Struct'"),
    Some("'Oneof'"),
    Some("'Map'"),
    Some("'ele_type'"),
    Some("'ref'"),
    Some("'size'"),
    Some("'option'"),
    Some("'key_type'"),
    Some("'value_type'"),
    Some("'mutator'"),
    Some("'random_Number'"),
    Some("'random_String'"),
    Some("'random_Bool'"),
    Some("'random_Byte'"),
    Some("'random_Timestamp'"),
    Some("'func_hash'"),
    Some("'random_BigNumber'"),
    Some("'Decreasing'"),
    Some("'Increasing'"),
    Some("'edge_value'"),
    Some("'param'"),
    Some("'algo'"),
    Some("'identify'"),
    Some("'excptedIdentify'"),
    Some("'specname'"),
    Some("'LOKI'"),
    Some("'Message'"),
    Some("'Attribute'"),
    Some("'name'"),
    Some("'type'"),
    Some("'unlimited'"),
    Some("'ExpectedMsg'"),
    Some("'<'"),
    Some("'>'"),
    Some("'='"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 54] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("SPECNAME"),
    Some("LOKI"),
    Some("Message"),
    Some("Attribute"),
    Some("Name"),
    Some("Type"),
    Some("Unlimited"),
    Some("ExpectedMsg"),
    Some("Lsign"),
    Some("Rsign"),
    Some("Euqal"),
    Some("LINE_COMMENT"),
    Some("COMMENT"),
    Some("WS"),
    Some("ID"),
    Some("INT"),
    Some("FLOAT"),
    Some("STRING"),
    Some("SHEBANG"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

type BaseParserType<'input, I> = BaseParser<
    'input,
    Loki_specParserExt,
    I,
    Loki_specParserContextType,
    dyn Loki_specListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type Loki_specTreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, Loki_specParserContextType, dyn Loki_specListener<'input> + 'a>;

/// Parser for Loki_spec grammar
pub struct Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str {
        _serializedATN
    }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "2");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                Loki_specParserExt {},
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> Loki_specParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> Loki_specParser<'input, I, DefaultErrorStrategy<'input, Loki_specParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for Loki_specParser
pub trait Loki_specParserContext<'input>:
    for<'x> Listenable<dyn Loki_specListener<'input> + 'x>
    + for<'x> Visitable<dyn Loki_specVisitor<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = Loki_specParserContextType>
{
}

impl<'input, 'x, T> VisitableDyn<T> for dyn Loki_specParserContext<'input> + 'input
where
    T: Loki_specVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn Loki_specVisitor<'input> + 'x))
    }
}

impl<'input> Loki_specParserContext<'input> for TerminalNode<'input, Loki_specParserContextType> {}
impl<'input> Loki_specParserContext<'input> for ErrorNode<'input, Loki_specParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn Loki_specParserContext<'input> + 'input {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn Loki_specListener<'input> + 'input {}

pub struct Loki_specParserContextType;
antlr_rust::type_id! {Loki_specParserContextType}

impl<'input> ParserNodeType<'input> for Loki_specParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn Loki_specParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct Loki_specParserExt {}

impl Loki_specParserExt {}

impl<'input> TokenAware<'input> for Loki_specParserExt {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for Loki_specParserExt
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for Loki_specParserExt
{
    fn get_grammar_file_name(&self) -> &str {
        "Loki_spec.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
}
//------------------- document ----------------
pub type DocumentContextAll<'input> = DocumentContext<'input>;

pub type DocumentContext<'input> = BaseParserRuleContext<'input, DocumentContextExt<'input>>;

#[derive(Clone)]
pub struct DocumentContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for DocumentContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for DocumentContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_document(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_document(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for DocumentContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_document(self);
    }
}

impl<'input> CustomRuleContext<'input> for DocumentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_document
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_document }
}
antlr_rust::type_id! {DocumentContextExt<'a>}

impl<'input> DocumentContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DocumentContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            DocumentContextExt { ph: PhantomData },
        ))
    }
}

pub trait DocumentContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<DocumentContextExt<'input>>
{
    fn loki_begin(&self) -> Option<Rc<Loki_beginContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn loki_end(&self) -> Option<Rc<Loki_endContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn spec_content_all(&self) -> Vec<Rc<Spec_contentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn spec_content(&self, i: usize) -> Option<Rc<Spec_contentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> DocumentContextAttrs<'input> for DocumentContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn document(&mut self) -> Result<Rc<DocumentContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = DocumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_document);
        let mut _localctx: Rc<DocumentContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule loki_begin*/
                recog.base.set_state(50);
                recog.loki_begin()?;

                recog.base.set_state(54);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(0, &mut recog.base)?;
                while _alt != 2 && _alt != INVALID_ALT {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule spec_content*/
                                recog.base.set_state(51);
                                recog.spec_content()?;
                            }
                        }
                    }
                    recog.base.set_state(56);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(0, &mut recog.base)?;
                }
                /*InvokeRule loki_end*/
                recog.base.set_state(57);
                recog.loki_end()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- loki_begin ----------------
pub type Loki_beginContextAll<'input> = Loki_beginContext<'input>;

pub type Loki_beginContext<'input> = BaseParserRuleContext<'input, Loki_beginContextExt<'input>>;

#[derive(Clone)]
pub struct Loki_beginContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for Loki_beginContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for Loki_beginContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_loki_begin(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_loki_begin(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for Loki_beginContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_loki_begin(self);
    }
}

impl<'input> CustomRuleContext<'input> for Loki_beginContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_loki_begin
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_loki_begin }
}
antlr_rust::type_id! {Loki_beginContextExt<'a>}

impl<'input> Loki_beginContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Loki_beginContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Loki_beginContextExt { ph: PhantomData },
        ))
    }
}

pub trait Loki_beginContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<Loki_beginContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Lsign
    /// Returns `None` if there is no child corresponding to token Lsign
    fn Lsign(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Lsign, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LOKI
    /// Returns `None` if there is no child corresponding to token LOKI
    fn LOKI(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LOKI, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SPECNAME
    /// Returns `None` if there is no child corresponding to token SPECNAME
    fn SPECNAME(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SPECNAME, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Euqal
    /// Returns `None` if there is no child corresponding to token Euqal
    fn Euqal(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Euqal, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Rsign
    /// Returns `None` if there is no child corresponding to token Rsign
    fn Rsign(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Rsign, 0)
    }
}

impl<'input> Loki_beginContextAttrs<'input> for Loki_beginContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn loki_begin(&mut self) -> Result<Rc<Loki_beginContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Loki_beginContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_loki_begin);
        let mut _localctx: Rc<Loki_beginContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(59);
                recog.base.match_token(Lsign, &mut recog.err_handler)?;

                recog.base.set_state(60);
                recog.base.match_token(LOKI, &mut recog.err_handler)?;

                recog.base.set_state(61);
                recog.base.match_token(SPECNAME, &mut recog.err_handler)?;

                recog.base.set_state(62);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                recog.base.set_state(63);
                recog.base.match_token(STRING, &mut recog.err_handler)?;

                recog.base.set_state(64);
                recog.base.match_token(Rsign, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- loki_end ----------------
pub type Loki_endContextAll<'input> = Loki_endContext<'input>;

pub type Loki_endContext<'input> = BaseParserRuleContext<'input, Loki_endContextExt<'input>>;

#[derive(Clone)]
pub struct Loki_endContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for Loki_endContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for Loki_endContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_loki_end(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_loki_end(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for Loki_endContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_loki_end(self);
    }
}

impl<'input> CustomRuleContext<'input> for Loki_endContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_loki_end
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_loki_end }
}
antlr_rust::type_id! {Loki_endContextExt<'a>}

impl<'input> Loki_endContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Loki_endContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Loki_endContextExt { ph: PhantomData },
        ))
    }
}

pub trait Loki_endContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<Loki_endContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Lsign
    /// Returns `None` if there is no child corresponding to token Lsign
    fn Lsign(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Lsign, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LOKI
    /// Returns `None` if there is no child corresponding to token LOKI
    fn LOKI(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LOKI, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Rsign
    /// Returns `None` if there is no child corresponding to token Rsign
    fn Rsign(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Rsign, 0)
    }
}

impl<'input> Loki_endContextAttrs<'input> for Loki_endContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn loki_end(&mut self) -> Result<Rc<Loki_endContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Loki_endContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_loki_end);
        let mut _localctx: Rc<Loki_endContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(66);
                recog.base.match_token(Lsign, &mut recog.err_handler)?;

                recog.base.set_state(67);
                recog.base.match_token(T__0, &mut recog.err_handler)?;

                recog.base.set_state(68);
                recog.base.match_token(LOKI, &mut recog.err_handler)?;

                recog.base.set_state(69);
                recog.base.match_token(Rsign, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- spec_content ----------------
pub type Spec_contentContextAll<'input> = Spec_contentContext<'input>;

pub type Spec_contentContext<'input> =
    BaseParserRuleContext<'input, Spec_contentContextExt<'input>>;

#[derive(Clone)]
pub struct Spec_contentContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for Spec_contentContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for Spec_contentContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_spec_content(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_spec_content(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for Spec_contentContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_spec_content(self);
    }
}

impl<'input> CustomRuleContext<'input> for Spec_contentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_spec_content
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_spec_content }
}
antlr_rust::type_id! {Spec_contentContextExt<'a>}

impl<'input> Spec_contentContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Spec_contentContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Spec_contentContextExt { ph: PhantomData },
        ))
    }
}

pub trait Spec_contentContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<Spec_contentContextExt<'input>>
{
    fn message_all(&self) -> Vec<Rc<MessageContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn message(&self, i: usize) -> Option<Rc<MessageContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Spec_contentContextAttrs<'input> for Spec_contentContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn spec_content(&mut self) -> Result<Rc<Spec_contentContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Spec_contentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 6, RULE_spec_content);
        let mut _localctx: Rc<Spec_contentContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(72);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = 1;
                loop {
                    match _alt {
                        x if x == 1 => {
                            {
                                /*InvokeRule message*/
                                recog.base.set_state(71);
                                recog.message()?;
                            }
                        }

                        _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                            &mut recog.base,
                        )))?,
                    }
                    recog.base.set_state(74);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(1, &mut recog.base)?;
                    if _alt == 2 || _alt == INVALID_ALT {
                        break;
                    }
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- message ----------------
pub type MessageContextAll<'input> = MessageContext<'input>;

pub type MessageContext<'input> = BaseParserRuleContext<'input, MessageContextExt<'input>>;

#[derive(Clone)]
pub struct MessageContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for MessageContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for MessageContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_message(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_message(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for MessageContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_message(self);
    }
}

impl<'input> CustomRuleContext<'input> for MessageContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_message
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_message }
}
antlr_rust::type_id! {MessageContextExt<'a>}

impl<'input> MessageContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<MessageContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            MessageContextExt { ph: PhantomData },
        ))
    }
}

pub trait MessageContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<MessageContextExt<'input>>
{
    fn msg_begin(&self) -> Option<Rc<Msg_beginContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn msg_end(&self) -> Option<Rc<Msg_endContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn msg_content_all(&self) -> Vec<Rc<Msg_contentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn msg_content(&self, i: usize) -> Option<Rc<Msg_contentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> MessageContextAttrs<'input> for MessageContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn message(&mut self) -> Result<Rc<MessageContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = MessageContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_message);
        let mut _localctx: Rc<MessageContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule msg_begin*/
                recog.base.set_state(76);
                recog.msg_begin()?;

                recog.base.set_state(80);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(2, &mut recog.base)?;
                while _alt != 2 && _alt != INVALID_ALT {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule msg_content*/
                                recog.base.set_state(77);
                                recog.msg_content()?;
                            }
                        }
                    }
                    recog.base.set_state(82);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(2, &mut recog.base)?;
                }
                /*InvokeRule msg_end*/
                recog.base.set_state(83);
                recog.msg_end()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- msg_begin ----------------
pub type Msg_beginContextAll<'input> = Msg_beginContext<'input>;

pub type Msg_beginContext<'input> = BaseParserRuleContext<'input, Msg_beginContextExt<'input>>;

#[derive(Clone)]
pub struct Msg_beginContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for Msg_beginContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for Msg_beginContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_msg_begin(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_msg_begin(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for Msg_beginContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_msg_begin(self);
    }
}

impl<'input> CustomRuleContext<'input> for Msg_beginContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_msg_begin
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_msg_begin }
}
antlr_rust::type_id! {Msg_beginContextExt<'a>}

impl<'input> Msg_beginContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Msg_beginContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Msg_beginContextExt { ph: PhantomData },
        ))
    }
}

pub trait Msg_beginContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<Msg_beginContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Lsign
    /// Returns `None` if there is no child corresponding to token Lsign
    fn Lsign(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Lsign, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Message
    /// Returns `None` if there is no child corresponding to token Message
    fn Message(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Message, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Name
    /// Returns `None` if there is no child corresponding to token Name
    fn Name(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Name, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Euqal
    /// Returns `None` if there is no child corresponding to token Euqal
    fn Euqal(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Euqal, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Rsign
    /// Returns `None` if there is no child corresponding to token Rsign
    fn Rsign(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Rsign, 0)
    }
}

impl<'input> Msg_beginContextAttrs<'input> for Msg_beginContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn msg_begin(&mut self) -> Result<Rc<Msg_beginContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Msg_beginContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_msg_begin);
        let mut _localctx: Rc<Msg_beginContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(85);
                recog.base.match_token(Lsign, &mut recog.err_handler)?;

                recog.base.set_state(86);
                recog.base.match_token(Message, &mut recog.err_handler)?;

                recog.base.set_state(87);
                recog.base.match_token(Name, &mut recog.err_handler)?;

                recog.base.set_state(88);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                recog.base.set_state(89);
                recog.base.match_token(STRING, &mut recog.err_handler)?;

                recog.base.set_state(90);
                recog.base.match_token(Rsign, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- msg_end ----------------
pub type Msg_endContextAll<'input> = Msg_endContext<'input>;

pub type Msg_endContext<'input> = BaseParserRuleContext<'input, Msg_endContextExt<'input>>;

#[derive(Clone)]
pub struct Msg_endContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for Msg_endContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for Msg_endContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_msg_end(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_msg_end(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for Msg_endContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_msg_end(self);
    }
}

impl<'input> CustomRuleContext<'input> for Msg_endContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_msg_end
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_msg_end }
}
antlr_rust::type_id! {Msg_endContextExt<'a>}

impl<'input> Msg_endContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Msg_endContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Msg_endContextExt { ph: PhantomData },
        ))
    }
}

pub trait Msg_endContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<Msg_endContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Lsign
    /// Returns `None` if there is no child corresponding to token Lsign
    fn Lsign(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Lsign, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Message
    /// Returns `None` if there is no child corresponding to token Message
    fn Message(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Message, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Rsign
    /// Returns `None` if there is no child corresponding to token Rsign
    fn Rsign(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Rsign, 0)
    }
}

impl<'input> Msg_endContextAttrs<'input> for Msg_endContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn msg_end(&mut self) -> Result<Rc<Msg_endContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Msg_endContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_msg_end);
        let mut _localctx: Rc<Msg_endContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(92);
                recog.base.match_token(Lsign, &mut recog.err_handler)?;

                recog.base.set_state(93);
                recog.base.match_token(T__0, &mut recog.err_handler)?;

                recog.base.set_state(94);
                recog.base.match_token(Message, &mut recog.err_handler)?;

                recog.base.set_state(95);
                recog.base.match_token(Rsign, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- msg_content ----------------
pub type Msg_contentContextAll<'input> = Msg_contentContext<'input>;

pub type Msg_contentContext<'input> = BaseParserRuleContext<'input, Msg_contentContextExt<'input>>;

#[derive(Clone)]
pub struct Msg_contentContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for Msg_contentContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for Msg_contentContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_msg_content(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_msg_content(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for Msg_contentContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_msg_content(self);
    }
}

impl<'input> CustomRuleContext<'input> for Msg_contentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_msg_content
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_msg_content }
}
antlr_rust::type_id! {Msg_contentContextExt<'a>}

impl<'input> Msg_contentContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Msg_contentContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Msg_contentContextExt { ph: PhantomData },
        ))
    }
}

pub trait Msg_contentContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<Msg_contentContextExt<'input>>
{
    fn attribute_all(&self) -> Vec<Rc<AttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn attribute(&self, i: usize) -> Option<Rc<AttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn expectedMsg_all(&self) -> Vec<Rc<ExpectedMsgContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expectedMsg(&self, i: usize) -> Option<Rc<ExpectedMsgContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Msg_contentContextAttrs<'input> for Msg_contentContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn msg_content(&mut self) -> Result<Rc<Msg_contentContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Msg_contentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 14, RULE_msg_content);
        let mut _localctx: Rc<Msg_contentContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(98);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = 1;
                loop {
                    match _alt {
                        x if x == 1 => {
                            {
                                /*InvokeRule attribute*/
                                recog.base.set_state(97);
                                recog.attribute()?;
                            }
                        }

                        _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                            &mut recog.base,
                        )))?,
                    }
                    recog.base.set_state(100);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(3, &mut recog.base)?;
                    if _alt == 2 || _alt == INVALID_ALT {
                        break;
                    }
                }
                recog.base.set_state(105);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(4, &mut recog.base)?;
                while _alt != 2 && _alt != INVALID_ALT {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule expectedMsg*/
                                recog.base.set_state(102);
                                recog.expectedMsg()?;
                            }
                        }
                    }
                    recog.base.set_state(107);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(4, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- attribute ----------------
pub type AttributeContextAll<'input> = AttributeContext<'input>;

pub type AttributeContext<'input> = BaseParserRuleContext<'input, AttributeContextExt<'input>>;

#[derive(Clone)]
pub struct AttributeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for AttributeContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for AttributeContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_attribute(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_attribute(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for AttributeContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_attribute(self);
    }
}

impl<'input> CustomRuleContext<'input> for AttributeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_attribute
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_attribute }
}
antlr_rust::type_id! {AttributeContextExt<'a>}

impl<'input> AttributeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AttributeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AttributeContextExt { ph: PhantomData },
        ))
    }
}

pub trait AttributeContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<AttributeContextExt<'input>>
{
    fn attr_begin(&self) -> Option<Rc<Attr_beginContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn attr_end(&self) -> Option<Rc<Attr_endContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> AttributeContextAttrs<'input> for AttributeContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn attribute(&mut self) -> Result<Rc<AttributeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AttributeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_attribute);
        let mut _localctx: Rc<AttributeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule attr_begin*/
                recog.base.set_state(108);
                recog.attr_begin()?;

                /*InvokeRule attr_end*/
                recog.base.set_state(109);
                recog.attr_end()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- attr_begin ----------------
pub type Attr_beginContextAll<'input> = Attr_beginContext<'input>;

pub type Attr_beginContext<'input> = BaseParserRuleContext<'input, Attr_beginContextExt<'input>>;

#[derive(Clone)]
pub struct Attr_beginContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for Attr_beginContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for Attr_beginContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_attr_begin(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_attr_begin(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for Attr_beginContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_attr_begin(self);
    }
}

impl<'input> CustomRuleContext<'input> for Attr_beginContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_attr_begin
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_attr_begin }
}
antlr_rust::type_id! {Attr_beginContextExt<'a>}

impl<'input> Attr_beginContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Attr_beginContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Attr_beginContextExt { ph: PhantomData },
        ))
    }
}

pub trait Attr_beginContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<Attr_beginContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Lsign
    /// Returns `None` if there is no child corresponding to token Lsign
    fn Lsign(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Lsign, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Attribute
    /// Returns `None` if there is no child corresponding to token Attribute
    fn Attribute(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Attribute, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Name
    /// Returns `None` if there is no child corresponding to token Name
    fn Name(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Name, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token Euqal in current rule
    fn Euqal_all(&self) -> Vec<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token Euqal, starting from 0.
    /// Returns `None` if number of children corresponding to token Euqal is less or equal than `i`.
    fn Euqal(&self, i: usize) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Euqal, i)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Type
    /// Returns `None` if there is no child corresponding to token Type
    fn Type(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Type, 0)
    }
    fn attr_type(&self) -> Option<Rc<Attr_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn ele_type_all(&self) -> Vec<Rc<Ele_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn ele_type(&self, i: usize) -> Option<Rc<Ele_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn reff_all(&self) -> Vec<Rc<ReffContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn reff(&self, i: usize) -> Option<Rc<ReffContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn size_all(&self) -> Vec<Rc<SizeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn size(&self, i: usize) -> Option<Rc<SizeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn option_all(&self) -> Vec<Rc<OptionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn option(&self, i: usize) -> Option<Rc<OptionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn key_type_all(&self) -> Vec<Rc<Key_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn key_type(&self, i: usize) -> Option<Rc<Key_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn value_type_all(&self) -> Vec<Rc<Value_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn value_type(&self, i: usize) -> Option<Rc<Value_typeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn mutator_all(&self) -> Vec<Rc<MutatorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn mutator(&self, i: usize) -> Option<Rc<MutatorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn param_all(&self) -> Vec<Rc<ParamContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn param(&self, i: usize) -> Option<Rc<ParamContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn algo_all(&self) -> Vec<Rc<AlgoContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn algo(&self, i: usize) -> Option<Rc<AlgoContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Attr_beginContextAttrs<'input> for Attr_beginContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn attr_begin(&mut self) -> Result<Rc<Attr_beginContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Attr_beginContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 18, RULE_attr_begin);
        let mut _localctx: Rc<Attr_beginContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(111);
                recog.base.match_token(Lsign, &mut recog.err_handler)?;

                recog.base.set_state(112);
                recog.base.match_token(Attribute, &mut recog.err_handler)?;

                recog.base.set_state(113);
                recog.base.match_token(Name, &mut recog.err_handler)?;

                recog.base.set_state(114);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                recog.base.set_state(115);
                recog.base.match_token(STRING, &mut recog.err_handler)?;

                recog.base.set_state(116);
                recog.base.match_token(Type, &mut recog.err_handler)?;

                recog.base.set_state(117);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                /*InvokeRule attr_type*/
                recog.base.set_state(118);
                recog.attr_type()?;

                recog.base.set_state(122);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__13 {
                    {
                        {
                            /*InvokeRule ele_type*/
                            recog.base.set_state(119);
                            recog.ele_type()?;
                        }
                    }
                    recog.base.set_state(124);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(128);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__14 {
                    {
                        {
                            /*InvokeRule reff*/
                            recog.base.set_state(125);
                            recog.reff()?;
                        }
                    }
                    recog.base.set_state(130);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(134);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__15 {
                    {
                        {
                            /*InvokeRule size*/
                            recog.base.set_state(131);
                            recog.size()?;
                        }
                    }
                    recog.base.set_state(136);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(140);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__16 {
                    {
                        {
                            /*InvokeRule option*/
                            recog.base.set_state(137);
                            recog.option()?;
                        }
                    }
                    recog.base.set_state(142);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(146);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__17 {
                    {
                        {
                            /*InvokeRule key_type*/
                            recog.base.set_state(143);
                            recog.key_type()?;
                        }
                    }
                    recog.base.set_state(148);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(152);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__18 {
                    {
                        {
                            /*InvokeRule value_type*/
                            recog.base.set_state(149);
                            recog.value_type()?;
                        }
                    }
                    recog.base.set_state(154);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(158);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__19 {
                    {
                        {
                            /*InvokeRule mutator*/
                            recog.base.set_state(155);
                            recog.mutator()?;
                        }
                    }
                    recog.base.set_state(160);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(164);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__30 {
                    {
                        {
                            /*InvokeRule param*/
                            recog.base.set_state(161);
                            recog.param()?;
                        }
                    }
                    recog.base.set_state(166);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(170);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__31 {
                    {
                        {
                            /*InvokeRule algo*/
                            recog.base.set_state(167);
                            recog.algo()?;
                        }
                    }
                    recog.base.set_state(172);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- attr_end ----------------
pub type Attr_endContextAll<'input> = Attr_endContext<'input>;

pub type Attr_endContext<'input> = BaseParserRuleContext<'input, Attr_endContextExt<'input>>;

#[derive(Clone)]
pub struct Attr_endContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for Attr_endContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for Attr_endContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_attr_end(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_attr_end(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for Attr_endContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_attr_end(self);
    }
}

impl<'input> CustomRuleContext<'input> for Attr_endContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_attr_end
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_attr_end }
}
antlr_rust::type_id! {Attr_endContextExt<'a>}

impl<'input> Attr_endContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Attr_endContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Attr_endContextExt { ph: PhantomData },
        ))
    }
}

pub trait Attr_endContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<Attr_endContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Rsign
    /// Returns `None` if there is no child corresponding to token Rsign
    fn Rsign(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Rsign, 0)
    }
}

impl<'input> Attr_endContextAttrs<'input> for Attr_endContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn attr_end(&mut self) -> Result<Rc<Attr_endContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Attr_endContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_attr_end);
        let mut _localctx: Rc<Attr_endContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(173);
                recog.base.match_token(T__0, &mut recog.err_handler)?;

                recog.base.set_state(174);
                recog.base.match_token(Rsign, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- attr_type ----------------
pub type Attr_typeContextAll<'input> = Attr_typeContext<'input>;

pub type Attr_typeContext<'input> = BaseParserRuleContext<'input, Attr_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Attr_typeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for Attr_typeContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for Attr_typeContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_attr_type(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_attr_type(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for Attr_typeContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_attr_type(self);
    }
}

impl<'input> CustomRuleContext<'input> for Attr_typeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_attr_type
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_attr_type }
}
antlr_rust::type_id! {Attr_typeContextExt<'a>}

impl<'input> Attr_typeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Attr_typeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Attr_typeContextExt { ph: PhantomData },
        ))
    }
}

pub trait Attr_typeContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<Attr_typeContextExt<'input>>
{
}

impl<'input> Attr_typeContextAttrs<'input> for Attr_typeContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn attr_type(&mut self) -> Result<Rc<Attr_typeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Attr_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_attr_type);
        let mut _localctx: Rc<Attr_typeContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(176);
                _la = recog.base.input.la(1);
                if !(((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << T__1)
                            | (1usize << T__2)
                            | (1usize << T__3)
                            | (1usize << T__4)
                            | (1usize << T__5)
                            | (1usize << T__6)
                            | (1usize << T__7)
                            | (1usize << T__8)
                            | (1usize << T__9)
                            | (1usize << T__10)
                            | (1usize << T__11)
                            | (1usize << T__12)))
                        != 0)
                {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- ele_type ----------------
pub type Ele_typeContextAll<'input> = Ele_typeContext<'input>;

pub type Ele_typeContext<'input> = BaseParserRuleContext<'input, Ele_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Ele_typeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for Ele_typeContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for Ele_typeContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_ele_type(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_ele_type(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for Ele_typeContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_ele_type(self);
    }
}

impl<'input> CustomRuleContext<'input> for Ele_typeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_ele_type
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_ele_type }
}
antlr_rust::type_id! {Ele_typeContextExt<'a>}

impl<'input> Ele_typeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Ele_typeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Ele_typeContextExt { ph: PhantomData },
        ))
    }
}

pub trait Ele_typeContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<Ele_typeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Euqal
    /// Returns `None` if there is no child corresponding to token Euqal
    fn Euqal(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Euqal, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
}

impl<'input> Ele_typeContextAttrs<'input> for Ele_typeContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn ele_type(&mut self) -> Result<Rc<Ele_typeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Ele_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_ele_type);
        let mut _localctx: Rc<Ele_typeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(178);
                recog.base.match_token(T__13, &mut recog.err_handler)?;

                recog.base.set_state(179);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                recog.base.set_state(180);
                recog.base.match_token(STRING, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- reff ----------------
pub type ReffContextAll<'input> = ReffContext<'input>;

pub type ReffContext<'input> = BaseParserRuleContext<'input, ReffContextExt<'input>>;

#[derive(Clone)]
pub struct ReffContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for ReffContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for ReffContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_reff(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_reff(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for ReffContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_reff(self);
    }
}

impl<'input> CustomRuleContext<'input> for ReffContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_reff
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_reff }
}
antlr_rust::type_id! {ReffContextExt<'a>}

impl<'input> ReffContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ReffContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ReffContextExt { ph: PhantomData },
        ))
    }
}

pub trait ReffContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<ReffContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Euqal
    /// Returns `None` if there is no child corresponding to token Euqal
    fn Euqal(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Euqal, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
}

impl<'input> ReffContextAttrs<'input> for ReffContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn reff(&mut self) -> Result<Rc<ReffContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ReffContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_reff);
        let mut _localctx: Rc<ReffContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(182);
                recog.base.match_token(T__14, &mut recog.err_handler)?;

                recog.base.set_state(183);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                recog.base.set_state(184);
                recog.base.match_token(STRING, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- size ----------------
pub type SizeContextAll<'input> = SizeContext<'input>;

pub type SizeContext<'input> = BaseParserRuleContext<'input, SizeContextExt<'input>>;

#[derive(Clone)]
pub struct SizeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for SizeContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for SizeContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_size(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_size(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for SizeContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_size(self);
    }
}

impl<'input> CustomRuleContext<'input> for SizeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_size
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_size }
}
antlr_rust::type_id! {SizeContextExt<'a>}

impl<'input> SizeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SizeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SizeContextExt { ph: PhantomData },
        ))
    }
}

pub trait SizeContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<SizeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Euqal
    /// Returns `None` if there is no child corresponding to token Euqal
    fn Euqal(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Euqal, 0)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INT, 0)
    }
}

impl<'input> SizeContextAttrs<'input> for SizeContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn size(&mut self) -> Result<Rc<SizeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = SizeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_size);
        let mut _localctx: Rc<SizeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(186);
                recog.base.match_token(T__15, &mut recog.err_handler)?;

                recog.base.set_state(187);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                recog.base.set_state(188);
                recog.base.match_token(INT, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- option ----------------
pub type OptionContextAll<'input> = OptionContext<'input>;

pub type OptionContext<'input> = BaseParserRuleContext<'input, OptionContextExt<'input>>;

#[derive(Clone)]
pub struct OptionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for OptionContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for OptionContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_option(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_option(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for OptionContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_option(self);
    }
}

impl<'input> CustomRuleContext<'input> for OptionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_option
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_option }
}
antlr_rust::type_id! {OptionContextExt<'a>}

impl<'input> OptionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OptionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OptionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OptionContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<OptionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Euqal
    /// Returns `None` if there is no child corresponding to token Euqal
    fn Euqal(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Euqal, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
}

impl<'input> OptionContextAttrs<'input> for OptionContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn option(&mut self) -> Result<Rc<OptionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OptionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_option);
        let mut _localctx: Rc<OptionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(190);
                recog.base.match_token(T__16, &mut recog.err_handler)?;

                recog.base.set_state(191);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                recog.base.set_state(192);
                recog.base.match_token(STRING, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- key_type ----------------
pub type Key_typeContextAll<'input> = Key_typeContext<'input>;

pub type Key_typeContext<'input> = BaseParserRuleContext<'input, Key_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Key_typeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for Key_typeContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for Key_typeContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_key_type(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_key_type(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for Key_typeContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_key_type(self);
    }
}

impl<'input> CustomRuleContext<'input> for Key_typeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_key_type
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_key_type }
}
antlr_rust::type_id! {Key_typeContextExt<'a>}

impl<'input> Key_typeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Key_typeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Key_typeContextExt { ph: PhantomData },
        ))
    }
}

pub trait Key_typeContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<Key_typeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Euqal
    /// Returns `None` if there is no child corresponding to token Euqal
    fn Euqal(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Euqal, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
}

impl<'input> Key_typeContextAttrs<'input> for Key_typeContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn key_type(&mut self) -> Result<Rc<Key_typeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Key_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_key_type);
        let mut _localctx: Rc<Key_typeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(194);
                recog.base.match_token(T__17, &mut recog.err_handler)?;

                recog.base.set_state(195);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                recog.base.set_state(196);
                recog.base.match_token(STRING, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- value_type ----------------
pub type Value_typeContextAll<'input> = Value_typeContext<'input>;

pub type Value_typeContext<'input> = BaseParserRuleContext<'input, Value_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Value_typeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for Value_typeContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for Value_typeContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_value_type(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_value_type(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for Value_typeContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_value_type(self);
    }
}

impl<'input> CustomRuleContext<'input> for Value_typeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_value_type
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_value_type }
}
antlr_rust::type_id! {Value_typeContextExt<'a>}

impl<'input> Value_typeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Value_typeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Value_typeContextExt { ph: PhantomData },
        ))
    }
}

pub trait Value_typeContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<Value_typeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Euqal
    /// Returns `None` if there is no child corresponding to token Euqal
    fn Euqal(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Euqal, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
}

impl<'input> Value_typeContextAttrs<'input> for Value_typeContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn value_type(&mut self) -> Result<Rc<Value_typeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Value_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 34, RULE_value_type);
        let mut _localctx: Rc<Value_typeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(198);
                recog.base.match_token(T__18, &mut recog.err_handler)?;

                recog.base.set_state(199);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                recog.base.set_state(200);
                recog.base.match_token(STRING, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- mutator ----------------
pub type MutatorContextAll<'input> = MutatorContext<'input>;

pub type MutatorContext<'input> = BaseParserRuleContext<'input, MutatorContextExt<'input>>;

#[derive(Clone)]
pub struct MutatorContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for MutatorContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for MutatorContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_mutator(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_mutator(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for MutatorContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_mutator(self);
    }
}

impl<'input> CustomRuleContext<'input> for MutatorContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_mutator
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_mutator }
}
antlr_rust::type_id! {MutatorContextExt<'a>}

impl<'input> MutatorContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<MutatorContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            MutatorContextExt { ph: PhantomData },
        ))
    }
}

pub trait MutatorContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<MutatorContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Euqal
    /// Returns `None` if there is no child corresponding to token Euqal
    fn Euqal(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Euqal, 0)
    }
    fn mutation(&self) -> Option<Rc<MutationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> MutatorContextAttrs<'input> for MutatorContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn mutator(&mut self) -> Result<Rc<MutatorContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = MutatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_mutator);
        let mut _localctx: Rc<MutatorContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(202);
                recog.base.match_token(T__19, &mut recog.err_handler)?;

                recog.base.set_state(203);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                /*InvokeRule mutation*/
                recog.base.set_state(204);
                recog.mutation()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- mutation ----------------
pub type MutationContextAll<'input> = MutationContext<'input>;

pub type MutationContext<'input> = BaseParserRuleContext<'input, MutationContextExt<'input>>;

#[derive(Clone)]
pub struct MutationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for MutationContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for MutationContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_mutation(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_mutation(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for MutationContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_mutation(self);
    }
}

impl<'input> CustomRuleContext<'input> for MutationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_mutation
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_mutation }
}
antlr_rust::type_id! {MutationContextExt<'a>}

impl<'input> MutationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<MutationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            MutationContextExt { ph: PhantomData },
        ))
    }
}

pub trait MutationContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<MutationContextExt<'input>>
{
}

impl<'input> MutationContextAttrs<'input> for MutationContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn mutation(&mut self) -> Result<Rc<MutationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = MutationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_mutation);
        let mut _localctx: Rc<MutationContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(206);
                _la = recog.base.input.la(1);
                if !(((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << T__20)
                            | (1usize << T__21)
                            | (1usize << T__22)
                            | (1usize << T__23)
                            | (1usize << T__24)
                            | (1usize << T__25)
                            | (1usize << T__26)
                            | (1usize << T__27)
                            | (1usize << T__28)
                            | (1usize << T__29)))
                        != 0)
                {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- param ----------------
pub type ParamContextAll<'input> = ParamContext<'input>;

pub type ParamContext<'input> = BaseParserRuleContext<'input, ParamContextExt<'input>>;

#[derive(Clone)]
pub struct ParamContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for ParamContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for ParamContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_param(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_param(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for ParamContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_param(self);
    }
}

impl<'input> CustomRuleContext<'input> for ParamContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_param
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_param }
}
antlr_rust::type_id! {ParamContextExt<'a>}

impl<'input> ParamContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ParamContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ParamContextExt { ph: PhantomData },
        ))
    }
}

pub trait ParamContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<ParamContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Euqal
    /// Returns `None` if there is no child corresponding to token Euqal
    fn Euqal(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Euqal, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
}

impl<'input> ParamContextAttrs<'input> for ParamContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn param(&mut self) -> Result<Rc<ParamContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ParamContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_param);
        let mut _localctx: Rc<ParamContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(208);
                recog.base.match_token(T__30, &mut recog.err_handler)?;

                recog.base.set_state(209);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                recog.base.set_state(210);
                recog.base.match_token(STRING, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- algo ----------------
pub type AlgoContextAll<'input> = AlgoContext<'input>;

pub type AlgoContext<'input> = BaseParserRuleContext<'input, AlgoContextExt<'input>>;

#[derive(Clone)]
pub struct AlgoContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for AlgoContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for AlgoContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_algo(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_algo(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for AlgoContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_algo(self);
    }
}

impl<'input> CustomRuleContext<'input> for AlgoContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_algo
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_algo }
}
antlr_rust::type_id! {AlgoContextExt<'a>}

impl<'input> AlgoContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AlgoContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AlgoContextExt { ph: PhantomData },
        ))
    }
}

pub trait AlgoContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<AlgoContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Euqal
    /// Returns `None` if there is no child corresponding to token Euqal
    fn Euqal(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Euqal, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
}

impl<'input> AlgoContextAttrs<'input> for AlgoContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn algo(&mut self) -> Result<Rc<AlgoContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AlgoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_algo);
        let mut _localctx: Rc<AlgoContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(212);
                recog.base.match_token(T__31, &mut recog.err_handler)?;

                recog.base.set_state(213);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                recog.base.set_state(214);
                recog.base.match_token(STRING, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- expectedMsg ----------------
pub type ExpectedMsgContextAll<'input> = ExpectedMsgContext<'input>;

pub type ExpectedMsgContext<'input> = BaseParserRuleContext<'input, ExpectedMsgContextExt<'input>>;

#[derive(Clone)]
pub struct ExpectedMsgContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for ExpectedMsgContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for ExpectedMsgContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expectedMsg(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_expectedMsg(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for ExpectedMsgContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_expectedMsg(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExpectedMsgContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expectedMsg
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expectedMsg }
}
antlr_rust::type_id! {ExpectedMsgContextExt<'a>}

impl<'input> ExpectedMsgContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ExpectedMsgContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ExpectedMsgContextExt { ph: PhantomData },
        ))
    }
}

pub trait ExpectedMsgContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<ExpectedMsgContextExt<'input>>
{
    fn exc_begin(&self) -> Option<Rc<Exc_beginContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn exc_end(&self) -> Option<Rc<Exc_endContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ExpectedMsgContextAttrs<'input> for ExpectedMsgContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn expectedMsg(&mut self) -> Result<Rc<ExpectedMsgContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ExpectedMsgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 44, RULE_expectedMsg);
        let mut _localctx: Rc<ExpectedMsgContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule exc_begin*/
                recog.base.set_state(216);
                recog.exc_begin()?;

                /*InvokeRule exc_end*/
                recog.base.set_state(217);
                recog.exc_end()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- exc_begin ----------------
pub type Exc_beginContextAll<'input> = Exc_beginContext<'input>;

pub type Exc_beginContext<'input> = BaseParserRuleContext<'input, Exc_beginContextExt<'input>>;

#[derive(Clone)]
pub struct Exc_beginContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for Exc_beginContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for Exc_beginContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_exc_begin(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_exc_begin(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for Exc_beginContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_exc_begin(self);
    }
}

impl<'input> CustomRuleContext<'input> for Exc_beginContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_exc_begin
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_exc_begin }
}
antlr_rust::type_id! {Exc_beginContextExt<'a>}

impl<'input> Exc_beginContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Exc_beginContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Exc_beginContextExt { ph: PhantomData },
        ))
    }
}

pub trait Exc_beginContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<Exc_beginContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Lsign
    /// Returns `None` if there is no child corresponding to token Lsign
    fn Lsign(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Lsign, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ExpectedMsg
    /// Returns `None` if there is no child corresponding to token ExpectedMsg
    fn ExpectedMsg(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ExpectedMsg, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Name
    /// Returns `None` if there is no child corresponding to token Name
    fn Name(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Name, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token Euqal in current rule
    fn Euqal_all(&self) -> Vec<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token Euqal, starting from 0.
    /// Returns `None` if number of children corresponding to token Euqal is less or equal than `i`.
    fn Euqal(&self, i: usize) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Euqal, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token STRING in current rule
    fn STRING_all(&self) -> Vec<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token STRING, starting from 0.
    /// Returns `None` if number of children corresponding to token STRING is less or equal than `i`.
    fn STRING(&self, i: usize) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, i)
    }
}

impl<'input> Exc_beginContextAttrs<'input> for Exc_beginContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn exc_begin(&mut self) -> Result<Rc<Exc_beginContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Exc_beginContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_exc_begin);
        let mut _localctx: Rc<Exc_beginContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(219);
                recog.base.match_token(Lsign, &mut recog.err_handler)?;

                recog.base.set_state(220);
                recog
                    .base
                    .match_token(ExpectedMsg, &mut recog.err_handler)?;

                recog.base.set_state(221);
                recog.base.match_token(Name, &mut recog.err_handler)?;

                recog.base.set_state(222);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                recog.base.set_state(223);
                recog.base.match_token(STRING, &mut recog.err_handler)?;

                recog.base.set_state(224);
                recog.base.match_token(T__32, &mut recog.err_handler)?;

                recog.base.set_state(225);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                recog.base.set_state(226);
                recog.base.match_token(STRING, &mut recog.err_handler)?;

                recog.base.set_state(227);
                recog.base.match_token(T__33, &mut recog.err_handler)?;

                recog.base.set_state(228);
                recog.base.match_token(Euqal, &mut recog.err_handler)?;

                recog.base.set_state(229);
                recog.base.match_token(STRING, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- exc_end ----------------
pub type Exc_endContextAll<'input> = Exc_endContext<'input>;

pub type Exc_endContext<'input> = BaseParserRuleContext<'input, Exc_endContextExt<'input>>;

#[derive(Clone)]
pub struct Exc_endContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Loki_specParserContext<'input> for Exc_endContext<'input> {}

impl<'input, 'a> Listenable<dyn Loki_specListener<'input> + 'a> for Exc_endContext<'input> {
    fn enter(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_exc_end(self);
    }
    fn exit(&self, listener: &mut (dyn Loki_specListener<'input> + 'a)) {
        listener.exit_exc_end(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Loki_specVisitor<'input> + 'a> for Exc_endContext<'input> {
    fn accept(&self, visitor: &mut (dyn Loki_specVisitor<'input> + 'a)) {
        visitor.visit_exc_end(self);
    }
}

impl<'input> CustomRuleContext<'input> for Exc_endContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Loki_specParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_exc_end
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_exc_end }
}
antlr_rust::type_id! {Exc_endContextExt<'a>}

impl<'input> Exc_endContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Loki_specParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Exc_endContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Exc_endContextExt { ph: PhantomData },
        ))
    }
}

pub trait Exc_endContextAttrs<'input>:
    Loki_specParserContext<'input> + BorrowMut<Exc_endContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Rsign
    /// Returns `None` if there is no child corresponding to token Rsign
    fn Rsign(&self) -> Option<Rc<TerminalNode<'input, Loki_specParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Rsign, 0)
    }
}

impl<'input> Exc_endContextAttrs<'input> for Exc_endContext<'input> {}

impl<'input, I, H> Loki_specParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn exc_end(&mut self) -> Result<Rc<Exc_endContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Exc_endContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_exc_end);
        let mut _localctx: Rc<Exc_endContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(231);
                recog.base.match_token(T__0, &mut recog.err_handler)?;

                recog.base.set_state(232);
                recog.base.match_token(Rsign, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x37\u{ed}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\x0e\
	\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\x13\
	\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\x17\
	\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x03\x02\x03\x02\x07\x02\
	\x37\x0a\x02\x0c\x02\x0e\x02\x3a\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x04\x03\x04\x03\x04\x03\x04\
	\x03\x04\x03\x05\x06\x05\x4b\x0a\x05\x0d\x05\x0e\x05\x4c\x03\x06\x03\x06\
	\x07\x06\x51\x0a\x06\x0c\x06\x0e\x06\x54\x0b\x06\x03\x06\x03\x06\x03\x07\
	\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x08\x03\x08\x03\x08\
	\x03\x08\x03\x08\x03\x09\x06\x09\x65\x0a\x09\x0d\x09\x0e\x09\x66\x03\x09\
	\x07\x09\x6a\x0a\x09\x0c\x09\x0e\x09\x6d\x0b\x09\x03\x0a\x03\x0a\x03\x0a\
	\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
	\x07\x0b\x7b\x0a\x0b\x0c\x0b\x0e\x0b\x7e\x0b\x0b\x03\x0b\x07\x0b\u{81}\x0a\
	\x0b\x0c\x0b\x0e\x0b\u{84}\x0b\x0b\x03\x0b\x07\x0b\u{87}\x0a\x0b\x0c\x0b\
	\x0e\x0b\u{8a}\x0b\x0b\x03\x0b\x07\x0b\u{8d}\x0a\x0b\x0c\x0b\x0e\x0b\u{90}\
	\x0b\x0b\x03\x0b\x07\x0b\u{93}\x0a\x0b\x0c\x0b\x0e\x0b\u{96}\x0b\x0b\x03\
	\x0b\x07\x0b\u{99}\x0a\x0b\x0c\x0b\x0e\x0b\u{9c}\x0b\x0b\x03\x0b\x07\x0b\
	\u{9f}\x0a\x0b\x0c\x0b\x0e\x0b\u{a2}\x0b\x0b\x03\x0b\x07\x0b\u{a5}\x0a\x0b\
	\x0c\x0b\x0e\x0b\u{a8}\x0b\x0b\x03\x0b\x07\x0b\u{ab}\x0a\x0b\x0c\x0b\x0e\
	\x0b\u{ae}\x0b\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x10\x03\x10\x03\x10\
	\x03\x10\x03\x11\x03\x11\x03\x11\x03\x11\x03\x12\x03\x12\x03\x12\x03\x12\
	\x03\x13\x03\x13\x03\x13\x03\x13\x03\x14\x03\x14\x03\x14\x03\x14\x03\x15\
	\x03\x15\x03\x16\x03\x16\x03\x16\x03\x16\x03\x17\x03\x17\x03\x17\x03\x17\
	\x03\x18\x03\x18\x03\x18\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\
	\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\x1a\
	\x03\x1a\x02\x02\x1b\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\
	\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x02\x04\x03\x02\x04\x0f\
	\x03\x02\x17\x20\x02\u{e1}\x02\x34\x03\x02\x02\x02\x04\x3d\x03\x02\x02\x02\
	\x06\x44\x03\x02\x02\x02\x08\x4a\x03\x02\x02\x02\x0a\x4e\x03\x02\x02\x02\
	\x0c\x57\x03\x02\x02\x02\x0e\x5e\x03\x02\x02\x02\x10\x64\x03\x02\x02\x02\
	\x12\x6e\x03\x02\x02\x02\x14\x71\x03\x02\x02\x02\x16\u{af}\x03\x02\x02\x02\
	\x18\u{b2}\x03\x02\x02\x02\x1a\u{b4}\x03\x02\x02\x02\x1c\u{b8}\x03\x02\x02\
	\x02\x1e\u{bc}\x03\x02\x02\x02\x20\u{c0}\x03\x02\x02\x02\x22\u{c4}\x03\x02\
	\x02\x02\x24\u{c8}\x03\x02\x02\x02\x26\u{cc}\x03\x02\x02\x02\x28\u{d0}\x03\
	\x02\x02\x02\x2a\u{d2}\x03\x02\x02\x02\x2c\u{d6}\x03\x02\x02\x02\x2e\u{da}\
	\x03\x02\x02\x02\x30\u{dd}\x03\x02\x02\x02\x32\u{e9}\x03\x02\x02\x02\x34\
	\x38\x05\x04\x03\x02\x35\x37\x05\x08\x05\x02\x36\x35\x03\x02\x02\x02\x37\
	\x3a\x03\x02\x02\x02\x38\x36\x03\x02\x02\x02\x38\x39\x03\x02\x02\x02\x39\
	\x3b\x03\x02\x02\x02\x3a\x38\x03\x02\x02\x02\x3b\x3c\x05\x06\x04\x02\x3c\
	\x03\x03\x02\x02\x02\x3d\x3e\x07\x2d\x02\x02\x3e\x3f\x07\x26\x02\x02\x3f\
	\x40\x07\x25\x02\x02\x40\x41\x07\x2f\x02\x02\x41\x42\x07\x36\x02\x02\x42\
	\x43\x07\x2e\x02\x02\x43\x05\x03\x02\x02\x02\x44\x45\x07\x2d\x02\x02\x45\
	\x46\x07\x03\x02\x02\x46\x47\x07\x26\x02\x02\x47\x48\x07\x2e\x02\x02\x48\
	\x07\x03\x02\x02\x02\x49\x4b\x05\x0a\x06\x02\x4a\x49\x03\x02\x02\x02\x4b\
	\x4c\x03\x02\x02\x02\x4c\x4a\x03\x02\x02\x02\x4c\x4d\x03\x02\x02\x02\x4d\
	\x09\x03\x02\x02\x02\x4e\x52\x05\x0c\x07\x02\x4f\x51\x05\x10\x09\x02\x50\
	\x4f\x03\x02\x02\x02\x51\x54\x03\x02\x02\x02\x52\x50\x03\x02\x02\x02\x52\
	\x53\x03\x02\x02\x02\x53\x55\x03\x02\x02\x02\x54\x52\x03\x02\x02\x02\x55\
	\x56\x05\x0e\x08\x02\x56\x0b\x03\x02\x02\x02\x57\x58\x07\x2d\x02\x02\x58\
	\x59\x07\x27\x02\x02\x59\x5a\x07\x29\x02\x02\x5a\x5b\x07\x2f\x02\x02\x5b\
	\x5c\x07\x36\x02\x02\x5c\x5d\x07\x2e\x02\x02\x5d\x0d\x03\x02\x02\x02\x5e\
	\x5f\x07\x2d\x02\x02\x5f\x60\x07\x03\x02\x02\x60\x61\x07\x27\x02\x02\x61\
	\x62\x07\x2e\x02\x02\x62\x0f\x03\x02\x02\x02\x63\x65\x05\x12\x0a\x02\x64\
	\x63\x03\x02\x02\x02\x65\x66\x03\x02\x02\x02\x66\x64\x03\x02\x02\x02\x66\
	\x67\x03\x02\x02\x02\x67\x6b\x03\x02\x02\x02\x68\x6a\x05\x2e\x18\x02\x69\
	\x68\x03\x02\x02\x02\x6a\x6d\x03\x02\x02\x02\x6b\x69\x03\x02\x02\x02\x6b\
	\x6c\x03\x02\x02\x02\x6c\x11\x03\x02\x02\x02\x6d\x6b\x03\x02\x02\x02\x6e\
	\x6f\x05\x14\x0b\x02\x6f\x70\x05\x16\x0c\x02\x70\x13\x03\x02\x02\x02\x71\
	\x72\x07\x2d\x02\x02\x72\x73\x07\x28\x02\x02\x73\x74\x07\x29\x02\x02\x74\
	\x75\x07\x2f\x02\x02\x75\x76\x07\x36\x02\x02\x76\x77\x07\x2a\x02\x02\x77\
	\x78\x07\x2f\x02\x02\x78\x7c\x05\x18\x0d\x02\x79\x7b\x05\x1a\x0e\x02\x7a\
	\x79\x03\x02\x02\x02\x7b\x7e\x03\x02\x02\x02\x7c\x7a\x03\x02\x02\x02\x7c\
	\x7d\x03\x02\x02\x02\x7d\u{82}\x03\x02\x02\x02\x7e\x7c\x03\x02\x02\x02\x7f\
	\u{81}\x05\x1c\x0f\x02\u{80}\x7f\x03\x02\x02\x02\u{81}\u{84}\x03\x02\x02\
	\x02\u{82}\u{80}\x03\x02\x02\x02\u{82}\u{83}\x03\x02\x02\x02\u{83}\u{88}\
	\x03\x02\x02\x02\u{84}\u{82}\x03\x02\x02\x02\u{85}\u{87}\x05\x1e\x10\x02\
	\u{86}\u{85}\x03\x02\x02\x02\u{87}\u{8a}\x03\x02\x02\x02\u{88}\u{86}\x03\
	\x02\x02\x02\u{88}\u{89}\x03\x02\x02\x02\u{89}\u{8e}\x03\x02\x02\x02\u{8a}\
	\u{88}\x03\x02\x02\x02\u{8b}\u{8d}\x05\x20\x11\x02\u{8c}\u{8b}\x03\x02\x02\
	\x02\u{8d}\u{90}\x03\x02\x02\x02\u{8e}\u{8c}\x03\x02\x02\x02\u{8e}\u{8f}\
	\x03\x02\x02\x02\u{8f}\u{94}\x03\x02\x02\x02\u{90}\u{8e}\x03\x02\x02\x02\
	\u{91}\u{93}\x05\x22\x12\x02\u{92}\u{91}\x03\x02\x02\x02\u{93}\u{96}\x03\
	\x02\x02\x02\u{94}\u{92}\x03\x02\x02\x02\u{94}\u{95}\x03\x02\x02\x02\u{95}\
	\u{9a}\x03\x02\x02\x02\u{96}\u{94}\x03\x02\x02\x02\u{97}\u{99}\x05\x24\x13\
	\x02\u{98}\u{97}\x03\x02\x02\x02\u{99}\u{9c}\x03\x02\x02\x02\u{9a}\u{98}\
	\x03\x02\x02\x02\u{9a}\u{9b}\x03\x02\x02\x02\u{9b}\u{a0}\x03\x02\x02\x02\
	\u{9c}\u{9a}\x03\x02\x02\x02\u{9d}\u{9f}\x05\x26\x14\x02\u{9e}\u{9d}\x03\
	\x02\x02\x02\u{9f}\u{a2}\x03\x02\x02\x02\u{a0}\u{9e}\x03\x02\x02\x02\u{a0}\
	\u{a1}\x03\x02\x02\x02\u{a1}\u{a6}\x03\x02\x02\x02\u{a2}\u{a0}\x03\x02\x02\
	\x02\u{a3}\u{a5}\x05\x2a\x16\x02\u{a4}\u{a3}\x03\x02\x02\x02\u{a5}\u{a8}\
	\x03\x02\x02\x02\u{a6}\u{a4}\x03\x02\x02\x02\u{a6}\u{a7}\x03\x02\x02\x02\
	\u{a7}\u{ac}\x03\x02\x02\x02\u{a8}\u{a6}\x03\x02\x02\x02\u{a9}\u{ab}\x05\
	\x2c\x17\x02\u{aa}\u{a9}\x03\x02\x02\x02\u{ab}\u{ae}\x03\x02\x02\x02\u{ac}\
	\u{aa}\x03\x02\x02\x02\u{ac}\u{ad}\x03\x02\x02\x02\u{ad}\x15\x03\x02\x02\
	\x02\u{ae}\u{ac}\x03\x02\x02\x02\u{af}\u{b0}\x07\x03\x02\x02\u{b0}\u{b1}\
	\x07\x2e\x02\x02\u{b1}\x17\x03\x02\x02\x02\u{b2}\u{b3}\x09\x02\x02\x02\u{b3}\
	\x19\x03\x02\x02\x02\u{b4}\u{b5}\x07\x10\x02\x02\u{b5}\u{b6}\x07\x2f\x02\
	\x02\u{b6}\u{b7}\x07\x36\x02\x02\u{b7}\x1b\x03\x02\x02\x02\u{b8}\u{b9}\x07\
	\x11\x02\x02\u{b9}\u{ba}\x07\x2f\x02\x02\u{ba}\u{bb}\x07\x36\x02\x02\u{bb}\
	\x1d\x03\x02\x02\x02\u{bc}\u{bd}\x07\x12\x02\x02\u{bd}\u{be}\x07\x2f\x02\
	\x02\u{be}\u{bf}\x07\x34\x02\x02\u{bf}\x1f\x03\x02\x02\x02\u{c0}\u{c1}\x07\
	\x13\x02\x02\u{c1}\u{c2}\x07\x2f\x02\x02\u{c2}\u{c3}\x07\x36\x02\x02\u{c3}\
	\x21\x03\x02\x02\x02\u{c4}\u{c5}\x07\x14\x02\x02\u{c5}\u{c6}\x07\x2f\x02\
	\x02\u{c6}\u{c7}\x07\x36\x02\x02\u{c7}\x23\x03\x02\x02\x02\u{c8}\u{c9}\x07\
	\x15\x02\x02\u{c9}\u{ca}\x07\x2f\x02\x02\u{ca}\u{cb}\x07\x36\x02\x02\u{cb}\
	\x25\x03\x02\x02\x02\u{cc}\u{cd}\x07\x16\x02\x02\u{cd}\u{ce}\x07\x2f\x02\
	\x02\u{ce}\u{cf}\x05\x28\x15\x02\u{cf}\x27\x03\x02\x02\x02\u{d0}\u{d1}\x09\
	\x03\x02\x02\u{d1}\x29\x03\x02\x02\x02\u{d2}\u{d3}\x07\x21\x02\x02\u{d3}\
	\u{d4}\x07\x2f\x02\x02\u{d4}\u{d5}\x07\x36\x02\x02\u{d5}\x2b\x03\x02\x02\
	\x02\u{d6}\u{d7}\x07\x22\x02\x02\u{d7}\u{d8}\x07\x2f\x02\x02\u{d8}\u{d9}\
	\x07\x36\x02\x02\u{d9}\x2d\x03\x02\x02\x02\u{da}\u{db}\x05\x30\x19\x02\u{db}\
	\u{dc}\x05\x32\x1a\x02\u{dc}\x2f\x03\x02\x02\x02\u{dd}\u{de}\x07\x2d\x02\
	\x02\u{de}\u{df}\x07\x2c\x02\x02\u{df}\u{e0}\x07\x29\x02\x02\u{e0}\u{e1}\
	\x07\x2f\x02\x02\u{e1}\u{e2}\x07\x36\x02\x02\u{e2}\u{e3}\x07\x23\x02\x02\
	\u{e3}\u{e4}\x07\x2f\x02\x02\u{e4}\u{e5}\x07\x36\x02\x02\u{e5}\u{e6}\x07\
	\x24\x02\x02\u{e6}\u{e7}\x07\x2f\x02\x02\u{e7}\u{e8}\x07\x36\x02\x02\u{e8}\
	\x31\x03\x02\x02\x02\u{e9}\u{ea}\x07\x03\x02\x02\u{ea}\u{eb}\x07\x2e\x02\
	\x02\u{eb}\x33\x03\x02\x02\x02\x10\x38\x4c\x52\x66\x6b\x7c\u{82}\u{88}\u{8e}\
	\u{94}\u{9a}\u{a0}\u{a6}\u{ac}";
