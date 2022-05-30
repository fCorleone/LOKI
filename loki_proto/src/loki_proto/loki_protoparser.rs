// Generated from Loki_proto.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::loki_protolistener::*;
use super::loki_protovisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const BOOL:isize=1; 
		pub const BYTES:isize=2; 
		pub const DOUBLE:isize=3; 
		pub const ENUM:isize=4; 
		pub const FIXED32:isize=5; 
		pub const FIXED64:isize=6; 
		pub const FLOAT:isize=7; 
		pub const IMPORT:isize=8; 
		pub const INT32:isize=9; 
		pub const INT64:isize=10; 
		pub const MAP:isize=11; 
		pub const MESSAGE:isize=12; 
		pub const ONEOF:isize=13; 
		pub const OPTION:isize=14; 
		pub const PACKAGE:isize=15; 
		pub const PROTO3_DOUBLE:isize=16; 
		pub const PROTO3_SINGLE:isize=17; 
		pub const PUBLIC:isize=18; 
		pub const REPEATED:isize=19; 
		pub const RESERVED:isize=20; 
		pub const RETURNS:isize=21; 
		pub const RPC:isize=22; 
		pub const SERVICE:isize=23; 
		pub const SFIXED32:isize=24; 
		pub const SFIXED64:isize=25; 
		pub const SINT32:isize=26; 
		pub const SINT64:isize=27; 
		pub const STREAM:isize=28; 
		pub const STRING:isize=29; 
		pub const SYNTAX:isize=30; 
		pub const TO:isize=31; 
		pub const UINT32:isize=32; 
		pub const UINT64:isize=33; 
		pub const WEAK:isize=34; 
		pub const Ident:isize=35; 
		pub const IntLit:isize=36; 
		pub const FloatLit:isize=37; 
		pub const BoolLit:isize=38; 
		pub const StrLit:isize=39; 
		pub const Quote:isize=40; 
		pub const LPAREN:isize=41; 
		pub const RPAREN:isize=42; 
		pub const LBRACE:isize=43; 
		pub const RBRACE:isize=44; 
		pub const LBRACK:isize=45; 
		pub const RBRACK:isize=46; 
		pub const LCHEVR:isize=47; 
		pub const RCHEVR:isize=48; 
		pub const SEMI:isize=49; 
		pub const COMMA:isize=50; 
		pub const DOT:isize=51; 
		pub const MINUS:isize=52; 
		pub const PLUS:isize=53; 
		pub const ASSIGN:isize=54; 
		pub const WS:isize=55; 
		pub const COMMENT:isize=56; 
		pub const LINE_COMMENT:isize=57;
	pub const RULE_proto:usize = 0; 
	pub const RULE_syntax:usize = 1; 
	pub const RULE_importStatement:usize = 2; 
	pub const RULE_packageStatement:usize = 3; 
	pub const RULE_option:usize = 4; 
	pub const RULE_optionName:usize = 5; 
	pub const RULE_topLevelDef:usize = 6; 
	pub const RULE_message:usize = 7; 
	pub const RULE_messageBody:usize = 8; 
	pub const RULE_enumDefinition:usize = 9; 
	pub const RULE_enumBody:usize = 10; 
	pub const RULE_enumField:usize = 11; 
	pub const RULE_enumValueOption:usize = 12; 
	pub const RULE_service:usize = 13; 
	pub const RULE_rpc:usize = 14; 
	pub const RULE_reserved:usize = 15; 
	pub const RULE_ranges:usize = 16; 
	pub const RULE_range:usize = 17; 
	pub const RULE_fieldNames:usize = 18; 
	pub const RULE_eleType:usize = 19; 
	pub const RULE_fieldNumber:usize = 20; 
	pub const RULE_field:usize = 21; 
	pub const RULE_fieldOptions:usize = 22; 
	pub const RULE_fieldOption:usize = 23; 
	pub const RULE_oneof:usize = 24; 
	pub const RULE_oneofField:usize = 25; 
	pub const RULE_mapField:usize = 26; 
	pub const RULE_keyType:usize = 27; 
	pub const RULE_fullIdent:usize = 28; 
	pub const RULE_messageName:usize = 29; 
	pub const RULE_enumName:usize = 30; 
	pub const RULE_messageOrEnumName:usize = 31; 
	pub const RULE_fieldName:usize = 32; 
	pub const RULE_oneofName:usize = 33; 
	pub const RULE_mapName:usize = 34; 
	pub const RULE_serviceName:usize = 35; 
	pub const RULE_rpcName:usize = 36; 
	pub const RULE_messageType:usize = 37; 
	pub const RULE_messageOrEnumType:usize = 38; 
	pub const RULE_emptyStatement:usize = 39; 
	pub const RULE_constant:usize = 40;
	pub const ruleNames: [&'static str; 41] =  [
		"proto", "syntax", "importStatement", "packageStatement", "option", "optionName", 
		"topLevelDef", "message", "messageBody", "enumDefinition", "enumBody", 
		"enumField", "enumValueOption", "service", "rpc", "reserved", "ranges", 
		"range", "fieldNames", "eleType", "fieldNumber", "field", "fieldOptions", 
		"fieldOption", "oneof", "oneofField", "mapField", "keyType", "fullIdent", 
		"messageName", "enumName", "messageOrEnumName", "fieldName", "oneofName", 
		"mapName", "serviceName", "rpcName", "messageType", "messageOrEnumType", 
		"emptyStatement", "constant"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;55] = [
		None, Some("'bool'"), Some("'bytes'"), Some("'double'"), Some("'enum'"), 
		Some("'fixed32'"), Some("'fixed64'"), Some("'float'"), Some("'import'"), 
		Some("'int32'"), Some("'int64'"), Some("'map'"), Some("'message'"), Some("'oneof'"), 
		Some("'option'"), Some("'package'"), Some("'\"proto3\"'"), Some("''proto3''"), 
		Some("'public'"), Some("'repeated'"), Some("'reserved'"), Some("'returns'"), 
		Some("'rpc'"), Some("'service'"), Some("'sfixed32'"), Some("'sfixed64'"), 
		Some("'sint32'"), Some("'sint64'"), Some("'stream'"), Some("'string'"), 
		Some("'syntax'"), Some("'to'"), Some("'uint32'"), Some("'uint64'"), Some("'weak'"), 
		None, None, None, None, None, None, Some("'('"), Some("')'"), Some("'{'"), 
		Some("'}'"), Some("'['"), Some("']'"), Some("'<'"), Some("'>'"), Some("';'"), 
		Some("','"), Some("'.'"), Some("'-'"), Some("'+'"), Some("'='")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;58]  = [
		None, Some("BOOL"), Some("BYTES"), Some("DOUBLE"), Some("ENUM"), Some("FIXED32"), 
		Some("FIXED64"), Some("FLOAT"), Some("IMPORT"), Some("INT32"), Some("INT64"), 
		Some("MAP"), Some("MESSAGE"), Some("ONEOF"), Some("OPTION"), Some("PACKAGE"), 
		Some("PROTO3_DOUBLE"), Some("PROTO3_SINGLE"), Some("PUBLIC"), Some("REPEATED"), 
		Some("RESERVED"), Some("RETURNS"), Some("RPC"), Some("SERVICE"), Some("SFIXED32"), 
		Some("SFIXED64"), Some("SINT32"), Some("SINT64"), Some("STREAM"), Some("STRING"), 
		Some("SYNTAX"), Some("TO"), Some("UINT32"), Some("UINT64"), Some("WEAK"), 
		Some("Ident"), Some("IntLit"), Some("FloatLit"), Some("BoolLit"), Some("StrLit"), 
		Some("Quote"), Some("LPAREN"), Some("RPAREN"), Some("LBRACE"), Some("RBRACE"), 
		Some("LBRACK"), Some("RBRACK"), Some("LCHEVR"), Some("RCHEVR"), Some("SEMI"), 
		Some("COMMA"), Some("DOT"), Some("MINUS"), Some("PLUS"), Some("ASSIGN"), 
		Some("WS"), Some("COMMENT"), Some("LINE_COMMENT")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,Loki_protoParserExt, I, Loki_protoParserContextType , dyn Loki_protoListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type Loki_protoTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, Loki_protoParserContextType , dyn Loki_protoListener<'input> + 'a>;

/// Parser for Loki_proto grammar
pub struct Loki_protoParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","2");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				Loki_protoParserExt{
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> Loki_protoParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> Loki_protoParser<'input, I, DefaultErrorStrategy<'input,Loki_protoParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for Loki_protoParser
pub trait Loki_protoParserContext<'input>:
	for<'x> Listenable<dyn Loki_protoListener<'input> + 'x > + 
	for<'x> Visitable<dyn Loki_protoVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=Loki_protoParserContextType>
{}

impl<'input, 'x, T> VisitableDyn<T> for dyn Loki_protoParserContext<'input> + 'input
where
    T: Loki_protoVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn Loki_protoVisitor<'input> + 'x))
    }
}

impl<'input> Loki_protoParserContext<'input> for TerminalNode<'input,Loki_protoParserContextType> {}
impl<'input> Loki_protoParserContext<'input> for ErrorNode<'input,Loki_protoParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn Loki_protoParserContext<'input> + 'input{}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn Loki_protoListener<'input> + 'input{}

pub struct Loki_protoParserContextType;
antlr_rust::type_id!{Loki_protoParserContextType}

impl<'input> ParserNodeType<'input> for Loki_protoParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn Loki_protoParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct Loki_protoParserExt{
}

impl Loki_protoParserExt{
}


impl<'input> TokenAware<'input> for Loki_protoParserExt{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for Loki_protoParserExt{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for Loki_protoParserExt{
	fn get_grammar_file_name(&self) -> & str{ "Loki_proto.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- proto ----------------
pub type ProtoContextAll<'input> = ProtoContext<'input>;


pub type ProtoContext<'input> = BaseParserRuleContext<'input,ProtoContextExt<'input>>;

#[derive(Clone)]
pub struct ProtoContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for ProtoContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for ProtoContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_proto(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_proto(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for ProtoContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_proto(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProtoContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_proto }
	//fn type_rule_index() -> usize where Self: Sized { RULE_proto }
}
antlr_rust::type_id!{ProtoContextExt<'a>}

impl<'input> ProtoContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProtoContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProtoContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProtoContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<ProtoContextExt<'input>>{

fn syntax(&self) -> Option<Rc<SyntaxContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn importStatement_all(&self) ->  Vec<Rc<ImportStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn importStatement(&self, i: usize) -> Option<Rc<ImportStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn packageStatement_all(&self) ->  Vec<Rc<PackageStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn packageStatement(&self, i: usize) -> Option<Rc<PackageStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn option_all(&self) ->  Vec<Rc<OptionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn option(&self, i: usize) -> Option<Rc<OptionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn topLevelDef_all(&self) ->  Vec<Rc<TopLevelDefContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn topLevelDef(&self, i: usize) -> Option<Rc<TopLevelDefContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn emptyStatement_all(&self) ->  Vec<Rc<EmptyStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn emptyStatement(&self, i: usize) -> Option<Rc<EmptyStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProtoContextAttrs<'input> for ProtoContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn proto(&mut self,)
	-> Result<Rc<ProtoContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProtoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_proto);
        let mut _localctx: Rc<ProtoContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule syntax*/
			recog.base.set_state(82);
			recog.syntax()?;

			recog.base.set_state(90);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ENUM) | (1usize << IMPORT) | (1usize << MESSAGE) | (1usize << OPTION) | (1usize << PACKAGE) | (1usize << SERVICE) | (1usize << SEMI))) != 0 {
				{
				recog.base.set_state(88);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 IMPORT 
					=> {
						{
						/*InvokeRule importStatement*/
						recog.base.set_state(83);
						recog.importStatement()?;

						}
					}

				 PACKAGE 
					=> {
						{
						/*InvokeRule packageStatement*/
						recog.base.set_state(84);
						recog.packageStatement()?;

						}
					}

				 OPTION 
					=> {
						{
						/*InvokeRule option*/
						recog.base.set_state(85);
						recog.option()?;

						}
					}

				 ENUM | MESSAGE | SERVICE 
					=> {
						{
						/*InvokeRule topLevelDef*/
						recog.base.set_state(86);
						recog.topLevelDef()?;

						}
					}

				 SEMI 
					=> {
						{
						/*InvokeRule emptyStatement*/
						recog.base.set_state(87);
						recog.emptyStatement()?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(92);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(93);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- syntax ----------------
pub type SyntaxContextAll<'input> = SyntaxContext<'input>;


pub type SyntaxContext<'input> = BaseParserRuleContext<'input,SyntaxContextExt<'input>>;

#[derive(Clone)]
pub struct SyntaxContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for SyntaxContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for SyntaxContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_syntax(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_syntax(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for SyntaxContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_syntax(self);
	}
}

impl<'input> CustomRuleContext<'input> for SyntaxContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_syntax }
	//fn type_rule_index() -> usize where Self: Sized { RULE_syntax }
}
antlr_rust::type_id!{SyntaxContextExt<'a>}

impl<'input> SyntaxContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SyntaxContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SyntaxContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SyntaxContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<SyntaxContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SYNTAX
/// Returns `None` if there is no child corresponding to token SYNTAX
fn SYNTAX(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SYNTAX, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
/// Retrieves first TerminalNode corresponding to token PROTO3_DOUBLE
/// Returns `None` if there is no child corresponding to token PROTO3_DOUBLE
fn PROTO3_DOUBLE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(PROTO3_DOUBLE, 0)
}
/// Retrieves first TerminalNode corresponding to token PROTO3_SINGLE
/// Returns `None` if there is no child corresponding to token PROTO3_SINGLE
fn PROTO3_SINGLE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(PROTO3_SINGLE, 0)
}

}

impl<'input> SyntaxContextAttrs<'input> for SyntaxContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn syntax(&mut self,)
	-> Result<Rc<SyntaxContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SyntaxContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_syntax);
        let mut _localctx: Rc<SyntaxContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(95);
			recog.base.match_token(SYNTAX,&mut recog.err_handler)?;

			recog.base.set_state(96);
			recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

			recog.base.set_state(97);
			_la = recog.base.input.la(1);
			if  !(_la==PROTO3_DOUBLE || _la==PROTO3_SINGLE)  {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(98);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- importStatement ----------------
pub type ImportStatementContextAll<'input> = ImportStatementContext<'input>;


pub type ImportStatementContext<'input> = BaseParserRuleContext<'input,ImportStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ImportStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for ImportStatementContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for ImportStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_importStatement(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_importStatement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for ImportStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_importStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importStatement }
}
antlr_rust::type_id!{ImportStatementContextExt<'a>}

impl<'input> ImportStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportStatementContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<ImportStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IMPORT
/// Returns `None` if there is no child corresponding to token IMPORT
fn IMPORT(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(IMPORT, 0)
}
/// Retrieves first TerminalNode corresponding to token StrLit
/// Returns `None` if there is no child corresponding to token StrLit
fn StrLit(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(StrLit, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
/// Retrieves first TerminalNode corresponding to token WEAK
/// Returns `None` if there is no child corresponding to token WEAK
fn WEAK(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(WEAK, 0)
}
/// Retrieves first TerminalNode corresponding to token PUBLIC
/// Returns `None` if there is no child corresponding to token PUBLIC
fn PUBLIC(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(PUBLIC, 0)
}

}

impl<'input> ImportStatementContextAttrs<'input> for ImportStatementContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn importStatement(&mut self,)
	-> Result<Rc<ImportStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_importStatement);
        let mut _localctx: Rc<ImportStatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(100);
			recog.base.match_token(IMPORT,&mut recog.err_handler)?;

			recog.base.set_state(102);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==PUBLIC || _la==WEAK {
				{
				recog.base.set_state(101);
				_la = recog.base.input.la(1);
				if  !(_la==PUBLIC || _la==WEAK)  {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
			}

			recog.base.set_state(104);
			recog.base.match_token(StrLit,&mut recog.err_handler)?;

			recog.base.set_state(105);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- packageStatement ----------------
pub type PackageStatementContextAll<'input> = PackageStatementContext<'input>;


pub type PackageStatementContext<'input> = BaseParserRuleContext<'input,PackageStatementContextExt<'input>>;

#[derive(Clone)]
pub struct PackageStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for PackageStatementContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for PackageStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_packageStatement(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_packageStatement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for PackageStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_packageStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for PackageStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_packageStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_packageStatement }
}
antlr_rust::type_id!{PackageStatementContextExt<'a>}

impl<'input> PackageStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PackageStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PackageStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PackageStatementContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<PackageStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PACKAGE
/// Returns `None` if there is no child corresponding to token PACKAGE
fn PACKAGE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(PACKAGE, 0)
}
fn fullIdent(&self) -> Option<Rc<FullIdentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}

}

impl<'input> PackageStatementContextAttrs<'input> for PackageStatementContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn packageStatement(&mut self,)
	-> Result<Rc<PackageStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PackageStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_packageStatement);
        let mut _localctx: Rc<PackageStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(107);
			recog.base.match_token(PACKAGE,&mut recog.err_handler)?;

			/*InvokeRule fullIdent*/
			recog.base.set_state(108);
			recog.fullIdent()?;

			recog.base.set_state(109);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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


pub type OptionContext<'input> = BaseParserRuleContext<'input,OptionContextExt<'input>>;

#[derive(Clone)]
pub struct OptionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for OptionContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for OptionContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_option(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_option(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for OptionContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_option(self);
	}
}

impl<'input> CustomRuleContext<'input> for OptionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_option }
	//fn type_rule_index() -> usize where Self: Sized { RULE_option }
}
antlr_rust::type_id!{OptionContextExt<'a>}

impl<'input> OptionContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OptionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OptionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OptionContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<OptionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPTION
/// Returns `None` if there is no child corresponding to token OPTION
fn OPTION(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(OPTION, 0)
}
fn optionName(&self) -> Option<Rc<OptionNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
fn constant(&self) -> Option<Rc<ConstantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}

}

impl<'input> OptionContextAttrs<'input> for OptionContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn option(&mut self,)
	-> Result<Rc<OptionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OptionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_option);
        let mut _localctx: Rc<OptionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(111);
			recog.base.match_token(OPTION,&mut recog.err_handler)?;

			/*InvokeRule optionName*/
			recog.base.set_state(112);
			recog.optionName()?;

			recog.base.set_state(113);
			recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

			/*InvokeRule constant*/
			recog.base.set_state(114);
			recog.constant()?;

			recog.base.set_state(115);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- optionName ----------------
pub type OptionNameContextAll<'input> = OptionNameContext<'input>;


pub type OptionNameContext<'input> = BaseParserRuleContext<'input,OptionNameContextExt<'input>>;

#[derive(Clone)]
pub struct OptionNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for OptionNameContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for OptionNameContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_optionName(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_optionName(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for OptionNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_optionName(self);
	}
}

impl<'input> CustomRuleContext<'input> for OptionNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_optionName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_optionName }
}
antlr_rust::type_id!{OptionNameContextExt<'a>}

impl<'input> OptionNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OptionNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OptionNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OptionNameContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<OptionNameContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token Ident in current rule
fn Ident_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Ident, starting from 0.
/// Returns `None` if number of children corresponding to token Ident is less or equal than `i`.
fn Ident(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(Ident, i)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn fullIdent(&self) -> Option<Rc<FullIdentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}

}

impl<'input> OptionNameContextAttrs<'input> for OptionNameContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn optionName(&mut self,)
	-> Result<Rc<OptionNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OptionNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_optionName);
        let mut _localctx: Rc<OptionNameContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(122);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Ident 
				=> {
					{
					recog.base.set_state(117);
					recog.base.match_token(Ident,&mut recog.err_handler)?;

					}
				}

			 LPAREN 
				=> {
					{
					recog.base.set_state(118);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule fullIdent*/
					recog.base.set_state(119);
					recog.fullIdent()?;

					recog.base.set_state(120);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(128);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==DOT {
				{
				{
				recog.base.set_state(124);
				recog.base.match_token(DOT,&mut recog.err_handler)?;

				recog.base.set_state(125);
				recog.base.match_token(Ident,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(130);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- topLevelDef ----------------
pub type TopLevelDefContextAll<'input> = TopLevelDefContext<'input>;


pub type TopLevelDefContext<'input> = BaseParserRuleContext<'input,TopLevelDefContextExt<'input>>;

#[derive(Clone)]
pub struct TopLevelDefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for TopLevelDefContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for TopLevelDefContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_topLevelDef(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_topLevelDef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for TopLevelDefContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_topLevelDef(self);
	}
}

impl<'input> CustomRuleContext<'input> for TopLevelDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_topLevelDef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_topLevelDef }
}
antlr_rust::type_id!{TopLevelDefContextExt<'a>}

impl<'input> TopLevelDefContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TopLevelDefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TopLevelDefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TopLevelDefContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<TopLevelDefContextExt<'input>>{

fn message(&self) -> Option<Rc<MessageContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumDefinition(&self) -> Option<Rc<EnumDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn service(&self) -> Option<Rc<ServiceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TopLevelDefContextAttrs<'input> for TopLevelDefContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn topLevelDef(&mut self,)
	-> Result<Rc<TopLevelDefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TopLevelDefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_topLevelDef);
        let mut _localctx: Rc<TopLevelDefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(134);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 MESSAGE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule message*/
					recog.base.set_state(131);
					recog.message()?;

					}
				}

			 ENUM 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule enumDefinition*/
					recog.base.set_state(132);
					recog.enumDefinition()?;

					}
				}

			 SERVICE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule service*/
					recog.base.set_state(133);
					recog.service()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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


pub type MessageContext<'input> = BaseParserRuleContext<'input,MessageContextExt<'input>>;

#[derive(Clone)]
pub struct MessageContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for MessageContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for MessageContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_message(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_message(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for MessageContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_message(self);
	}
}

impl<'input> CustomRuleContext<'input> for MessageContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_message }
	//fn type_rule_index() -> usize where Self: Sized { RULE_message }
}
antlr_rust::type_id!{MessageContextExt<'a>}

impl<'input> MessageContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MessageContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MessageContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MessageContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<MessageContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MESSAGE
/// Returns `None` if there is no child corresponding to token MESSAGE
fn MESSAGE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(MESSAGE, 0)
}
fn messageName(&self) -> Option<Rc<MessageNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn messageBody(&self) -> Option<Rc<MessageBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MessageContextAttrs<'input> for MessageContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn message(&mut self,)
	-> Result<Rc<MessageContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MessageContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_message);
        let mut _localctx: Rc<MessageContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(136);
			recog.base.match_token(MESSAGE,&mut recog.err_handler)?;

			/*InvokeRule messageName*/
			recog.base.set_state(137);
			recog.messageName()?;

			/*InvokeRule messageBody*/
			recog.base.set_state(138);
			recog.messageBody()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- messageBody ----------------
pub type MessageBodyContextAll<'input> = MessageBodyContext<'input>;


pub type MessageBodyContext<'input> = BaseParserRuleContext<'input,MessageBodyContextExt<'input>>;

#[derive(Clone)]
pub struct MessageBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for MessageBodyContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for MessageBodyContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_messageBody(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_messageBody(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for MessageBodyContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_messageBody(self);
	}
}

impl<'input> CustomRuleContext<'input> for MessageBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_messageBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_messageBody }
}
antlr_rust::type_id!{MessageBodyContextExt<'a>}

impl<'input> MessageBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MessageBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MessageBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MessageBodyContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<MessageBodyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn field_all(&self) ->  Vec<Rc<FieldContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn field(&self, i: usize) -> Option<Rc<FieldContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn enumDefinition_all(&self) ->  Vec<Rc<EnumDefinitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumDefinition(&self, i: usize) -> Option<Rc<EnumDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn message_all(&self) ->  Vec<Rc<MessageContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn message(&self, i: usize) -> Option<Rc<MessageContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn option_all(&self) ->  Vec<Rc<OptionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn option(&self, i: usize) -> Option<Rc<OptionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn oneof_all(&self) ->  Vec<Rc<OneofContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oneof(&self, i: usize) -> Option<Rc<OneofContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn mapField_all(&self) ->  Vec<Rc<MapFieldContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn mapField(&self, i: usize) -> Option<Rc<MapFieldContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn reserved_all(&self) ->  Vec<Rc<ReservedContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn reserved(&self, i: usize) -> Option<Rc<ReservedContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn emptyStatement_all(&self) ->  Vec<Rc<EmptyStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn emptyStatement(&self, i: usize) -> Option<Rc<EmptyStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> MessageBodyContextAttrs<'input> for MessageBodyContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn messageBody(&mut self,)
	-> Result<Rc<MessageBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MessageBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_messageBody);
        let mut _localctx: Rc<MessageBodyContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(140);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(151);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOL) | (1usize << BYTES) | (1usize << DOUBLE) | (1usize << ENUM) | (1usize << FIXED32) | (1usize << FIXED64) | (1usize << FLOAT) | (1usize << INT32) | (1usize << INT64) | (1usize << MAP) | (1usize << MESSAGE) | (1usize << ONEOF) | (1usize << OPTION) | (1usize << REPEATED) | (1usize << RESERVED) | (1usize << SFIXED32) | (1usize << SFIXED64) | (1usize << SINT32) | (1usize << SINT64) | (1usize << STRING) | (1usize << UINT32) | (1usize << UINT64) | (1usize << Ident) | (1usize << SEMI) | (1usize << DOT))) != 0 {
				{
				recog.base.set_state(149);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 BOOL | BYTES | DOUBLE | FIXED32 | FIXED64 | FLOAT | INT32 | INT64 |
				 REPEATED | SFIXED32 | SFIXED64 | SINT32 | SINT64 | STRING | UINT32 |
				 UINT64 | Ident | DOT 
					=> {
						{
						/*InvokeRule field*/
						recog.base.set_state(141);
						recog.field()?;

						}
					}

				 ENUM 
					=> {
						{
						/*InvokeRule enumDefinition*/
						recog.base.set_state(142);
						recog.enumDefinition()?;

						}
					}

				 MESSAGE 
					=> {
						{
						/*InvokeRule message*/
						recog.base.set_state(143);
						recog.message()?;

						}
					}

				 OPTION 
					=> {
						{
						/*InvokeRule option*/
						recog.base.set_state(144);
						recog.option()?;

						}
					}

				 ONEOF 
					=> {
						{
						/*InvokeRule oneof*/
						recog.base.set_state(145);
						recog.oneof()?;

						}
					}

				 MAP 
					=> {
						{
						/*InvokeRule mapField*/
						recog.base.set_state(146);
						recog.mapField()?;

						}
					}

				 RESERVED 
					=> {
						{
						/*InvokeRule reserved*/
						recog.base.set_state(147);
						recog.reserved()?;

						}
					}

				 SEMI 
					=> {
						{
						/*InvokeRule emptyStatement*/
						recog.base.set_state(148);
						recog.emptyStatement()?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(153);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(154);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- enumDefinition ----------------
pub type EnumDefinitionContextAll<'input> = EnumDefinitionContext<'input>;


pub type EnumDefinitionContext<'input> = BaseParserRuleContext<'input,EnumDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct EnumDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for EnumDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for EnumDefinitionContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_enumDefinition(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_enumDefinition(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for EnumDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_enumDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumDefinition }
}
antlr_rust::type_id!{EnumDefinitionContextExt<'a>}

impl<'input> EnumDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumDefinitionContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<EnumDefinitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ENUM
/// Returns `None` if there is no child corresponding to token ENUM
fn ENUM(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(ENUM, 0)
}
fn enumName(&self) -> Option<Rc<EnumNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumBody(&self) -> Option<Rc<EnumBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EnumDefinitionContextAttrs<'input> for EnumDefinitionContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumDefinition(&mut self,)
	-> Result<Rc<EnumDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_enumDefinition);
        let mut _localctx: Rc<EnumDefinitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(156);
			recog.base.match_token(ENUM,&mut recog.err_handler)?;

			/*InvokeRule enumName*/
			recog.base.set_state(157);
			recog.enumName()?;

			/*InvokeRule enumBody*/
			recog.base.set_state(158);
			recog.enumBody()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- enumBody ----------------
pub type EnumBodyContextAll<'input> = EnumBodyContext<'input>;


pub type EnumBodyContext<'input> = BaseParserRuleContext<'input,EnumBodyContextExt<'input>>;

#[derive(Clone)]
pub struct EnumBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for EnumBodyContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for EnumBodyContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_enumBody(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_enumBody(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for EnumBodyContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_enumBody(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumBody }
}
antlr_rust::type_id!{EnumBodyContextExt<'a>}

impl<'input> EnumBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumBodyContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<EnumBodyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn option_all(&self) ->  Vec<Rc<OptionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn option(&self, i: usize) -> Option<Rc<OptionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn enumField_all(&self) ->  Vec<Rc<EnumFieldContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumField(&self, i: usize) -> Option<Rc<EnumFieldContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn emptyStatement_all(&self) ->  Vec<Rc<EmptyStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn emptyStatement(&self, i: usize) -> Option<Rc<EmptyStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> EnumBodyContextAttrs<'input> for EnumBodyContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumBody(&mut self,)
	-> Result<Rc<EnumBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_enumBody);
        let mut _localctx: Rc<EnumBodyContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(160);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(166);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << OPTION) | (1usize << Ident) | (1usize << SEMI))) != 0 {
				{
				recog.base.set_state(164);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 OPTION 
					=> {
						{
						/*InvokeRule option*/
						recog.base.set_state(161);
						recog.option()?;

						}
					}

				 Ident 
					=> {
						{
						/*InvokeRule enumField*/
						recog.base.set_state(162);
						recog.enumField()?;

						}
					}

				 SEMI 
					=> {
						{
						/*InvokeRule emptyStatement*/
						recog.base.set_state(163);
						recog.emptyStatement()?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(168);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(169);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- enumField ----------------
pub type EnumFieldContextAll<'input> = EnumFieldContext<'input>;


pub type EnumFieldContext<'input> = BaseParserRuleContext<'input,EnumFieldContextExt<'input>>;

#[derive(Clone)]
pub struct EnumFieldContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for EnumFieldContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for EnumFieldContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_enumField(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_enumField(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for EnumFieldContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_enumField(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumFieldContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumField }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumField }
}
antlr_rust::type_id!{EnumFieldContextExt<'a>}

impl<'input> EnumFieldContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumFieldContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumFieldContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumFieldContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<EnumFieldContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Ident
/// Returns `None` if there is no child corresponding to token Ident
fn Ident(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(Ident, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token IntLit
/// Returns `None` if there is no child corresponding to token IntLit
fn IntLit(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(IntLit, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACK
/// Returns `None` if there is no child corresponding to token LBRACK
fn LBRACK(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, 0)
}
fn enumValueOption_all(&self) ->  Vec<Rc<EnumValueOptionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumValueOption(&self, i: usize) -> Option<Rc<EnumValueOptionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RBRACK
/// Returns `None` if there is no child corresponding to token RBRACK
fn RBRACK(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> EnumFieldContextAttrs<'input> for EnumFieldContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumField(&mut self,)
	-> Result<Rc<EnumFieldContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumFieldContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_enumField);
        let mut _localctx: Rc<EnumFieldContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(171);
			recog.base.match_token(Ident,&mut recog.err_handler)?;

			recog.base.set_state(172);
			recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

			recog.base.set_state(174);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==MINUS {
				{
				recog.base.set_state(173);
				recog.base.match_token(MINUS,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(176);
			recog.base.match_token(IntLit,&mut recog.err_handler)?;

			recog.base.set_state(188);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LBRACK {
				{
				recog.base.set_state(177);
				recog.base.match_token(LBRACK,&mut recog.err_handler)?;

				/*InvokeRule enumValueOption*/
				recog.base.set_state(178);
				recog.enumValueOption()?;

				recog.base.set_state(183);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==COMMA {
					{
					{
					recog.base.set_state(179);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule enumValueOption*/
					recog.base.set_state(180);
					recog.enumValueOption()?;

					}
					}
					recog.base.set_state(185);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				recog.base.set_state(186);
				recog.base.match_token(RBRACK,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(190);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- enumValueOption ----------------
pub type EnumValueOptionContextAll<'input> = EnumValueOptionContext<'input>;


pub type EnumValueOptionContext<'input> = BaseParserRuleContext<'input,EnumValueOptionContextExt<'input>>;

#[derive(Clone)]
pub struct EnumValueOptionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for EnumValueOptionContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for EnumValueOptionContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_enumValueOption(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_enumValueOption(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for EnumValueOptionContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_enumValueOption(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumValueOptionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumValueOption }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumValueOption }
}
antlr_rust::type_id!{EnumValueOptionContextExt<'a>}

impl<'input> EnumValueOptionContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumValueOptionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumValueOptionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumValueOptionContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<EnumValueOptionContextExt<'input>>{

fn optionName(&self) -> Option<Rc<OptionNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
fn constant(&self) -> Option<Rc<ConstantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EnumValueOptionContextAttrs<'input> for EnumValueOptionContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumValueOption(&mut self,)
	-> Result<Rc<EnumValueOptionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumValueOptionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_enumValueOption);
        let mut _localctx: Rc<EnumValueOptionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule optionName*/
			recog.base.set_state(192);
			recog.optionName()?;

			recog.base.set_state(193);
			recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

			/*InvokeRule constant*/
			recog.base.set_state(194);
			recog.constant()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- service ----------------
pub type ServiceContextAll<'input> = ServiceContext<'input>;


pub type ServiceContext<'input> = BaseParserRuleContext<'input,ServiceContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for ServiceContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for ServiceContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_service(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_service(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for ServiceContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_service(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServiceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_service }
	//fn type_rule_index() -> usize where Self: Sized { RULE_service }
}
antlr_rust::type_id!{ServiceContextExt<'a>}

impl<'input> ServiceContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServiceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServiceContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServiceContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<ServiceContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SERVICE
/// Returns `None` if there is no child corresponding to token SERVICE
fn SERVICE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SERVICE, 0)
}
fn serviceName(&self) -> Option<Rc<ServiceNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn option_all(&self) ->  Vec<Rc<OptionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn option(&self, i: usize) -> Option<Rc<OptionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn rpc_all(&self) ->  Vec<Rc<RpcContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn rpc(&self, i: usize) -> Option<Rc<RpcContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn emptyStatement_all(&self) ->  Vec<Rc<EmptyStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn emptyStatement(&self, i: usize) -> Option<Rc<EmptyStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ServiceContextAttrs<'input> for ServiceContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn service(&mut self,)
	-> Result<Rc<ServiceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServiceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_service);
        let mut _localctx: Rc<ServiceContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(196);
			recog.base.match_token(SERVICE,&mut recog.err_handler)?;

			/*InvokeRule serviceName*/
			recog.base.set_state(197);
			recog.serviceName()?;

			recog.base.set_state(198);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(204);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << OPTION) | (1usize << RPC) | (1usize << SEMI))) != 0 {
				{
				recog.base.set_state(202);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 OPTION 
					=> {
						{
						/*InvokeRule option*/
						recog.base.set_state(199);
						recog.option()?;

						}
					}

				 RPC 
					=> {
						{
						/*InvokeRule rpc*/
						recog.base.set_state(200);
						recog.rpc()?;

						}
					}

				 SEMI 
					=> {
						{
						/*InvokeRule emptyStatement*/
						recog.base.set_state(201);
						recog.emptyStatement()?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(206);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(207);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- rpc ----------------
pub type RpcContextAll<'input> = RpcContext<'input>;


pub type RpcContext<'input> = BaseParserRuleContext<'input,RpcContextExt<'input>>;

#[derive(Clone)]
pub struct RpcContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for RpcContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for RpcContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_rpc(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_rpc(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for RpcContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_rpc(self);
	}
}

impl<'input> CustomRuleContext<'input> for RpcContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rpc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rpc }
}
antlr_rust::type_id!{RpcContextExt<'a>}

impl<'input> RpcContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RpcContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RpcContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RpcContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<RpcContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RPC
/// Returns `None` if there is no child corresponding to token RPC
fn RPC(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(RPC, 0)
}
fn rpcName(&self) -> Option<Rc<RpcNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token LPAREN in current rule
fn LPAREN_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LPAREN, starting from 0.
/// Returns `None` if number of children corresponding to token LPAREN is less or equal than `i`.
fn LPAREN(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, i)
}
fn messageType_all(&self) ->  Vec<Rc<MessageTypeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn messageType(&self, i: usize) -> Option<Rc<MessageTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token RPAREN in current rule
fn RPAREN_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RPAREN, starting from 0.
/// Returns `None` if number of children corresponding to token RPAREN is less or equal than `i`.
fn RPAREN(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, i)
}
/// Retrieves first TerminalNode corresponding to token RETURNS
/// Returns `None` if there is no child corresponding to token RETURNS
fn RETURNS(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(RETURNS, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token STREAM in current rule
fn STREAM_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token STREAM, starting from 0.
/// Returns `None` if number of children corresponding to token STREAM is less or equal than `i`.
fn STREAM(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(STREAM, i)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn option_all(&self) ->  Vec<Rc<OptionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn option(&self, i: usize) -> Option<Rc<OptionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn emptyStatement_all(&self) ->  Vec<Rc<EmptyStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn emptyStatement(&self, i: usize) -> Option<Rc<EmptyStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RpcContextAttrs<'input> for RpcContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn rpc(&mut self,)
	-> Result<Rc<RpcContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RpcContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_rpc);
        let mut _localctx: Rc<RpcContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(209);
			recog.base.match_token(RPC,&mut recog.err_handler)?;

			/*InvokeRule rpcName*/
			recog.base.set_state(210);
			recog.rpcName()?;

			recog.base.set_state(211);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(213);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==STREAM {
				{
				recog.base.set_state(212);
				recog.base.match_token(STREAM,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule messageType*/
			recog.base.set_state(215);
			recog.messageType()?;

			recog.base.set_state(216);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			recog.base.set_state(217);
			recog.base.match_token(RETURNS,&mut recog.err_handler)?;

			recog.base.set_state(218);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(220);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==STREAM {
				{
				recog.base.set_state(219);
				recog.base.match_token(STREAM,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule messageType*/
			recog.base.set_state(222);
			recog.messageType()?;

			recog.base.set_state(223);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			recog.base.set_state(234);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LBRACE 
				=> {
					{
					{
					recog.base.set_state(224);
					recog.base.match_token(LBRACE,&mut recog.err_handler)?;

					recog.base.set_state(229);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==OPTION || _la==SEMI {
						{
						recog.base.set_state(227);
						recog.err_handler.sync(&mut recog.base)?;
						match recog.base.input.la(1) {
						 OPTION 
							=> {
								{
								/*InvokeRule option*/
								recog.base.set_state(225);
								recog.option()?;

								}
							}

						 SEMI 
							=> {
								{
								/*InvokeRule emptyStatement*/
								recog.base.set_state(226);
								recog.emptyStatement()?;

								}
							}

							_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
						}
						}
						recog.base.set_state(231);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(232);
					recog.base.match_token(RBRACE,&mut recog.err_handler)?;

					}
					}
				}

			 SEMI 
				=> {
					{
					recog.base.set_state(233);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- reserved ----------------
pub type ReservedContextAll<'input> = ReservedContext<'input>;


pub type ReservedContext<'input> = BaseParserRuleContext<'input,ReservedContextExt<'input>>;

#[derive(Clone)]
pub struct ReservedContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for ReservedContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for ReservedContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_reserved(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_reserved(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for ReservedContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_reserved(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReservedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_reserved }
	//fn type_rule_index() -> usize where Self: Sized { RULE_reserved }
}
antlr_rust::type_id!{ReservedContextExt<'a>}

impl<'input> ReservedContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReservedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReservedContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReservedContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<ReservedContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RESERVED
/// Returns `None` if there is no child corresponding to token RESERVED
fn RESERVED(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(RESERVED, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
fn ranges(&self) -> Option<Rc<RangesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldNames(&self) -> Option<Rc<FieldNamesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReservedContextAttrs<'input> for ReservedContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn reserved(&mut self,)
	-> Result<Rc<ReservedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReservedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_reserved);
        let mut _localctx: Rc<ReservedContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(236);
			recog.base.match_token(RESERVED,&mut recog.err_handler)?;

			recog.base.set_state(239);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IntLit 
				=> {
					{
					/*InvokeRule ranges*/
					recog.base.set_state(237);
					recog.ranges()?;

					}
				}

			 StrLit 
				=> {
					{
					/*InvokeRule fieldNames*/
					recog.base.set_state(238);
					recog.fieldNames()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(241);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- ranges ----------------
pub type RangesContextAll<'input> = RangesContext<'input>;


pub type RangesContext<'input> = BaseParserRuleContext<'input,RangesContextExt<'input>>;

#[derive(Clone)]
pub struct RangesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for RangesContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for RangesContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ranges(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_ranges(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for RangesContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_ranges(self);
	}
}

impl<'input> CustomRuleContext<'input> for RangesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ranges }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ranges }
}
antlr_rust::type_id!{RangesContextExt<'a>}

impl<'input> RangesContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RangesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RangesContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RangesContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<RangesContextExt<'input>>{

fn range_all(&self) ->  Vec<Rc<RangeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn range(&self, i: usize) -> Option<Rc<RangeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> RangesContextAttrs<'input> for RangesContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ranges(&mut self,)
	-> Result<Rc<RangesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RangesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_ranges);
        let mut _localctx: Rc<RangesContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule range*/
			recog.base.set_state(243);
			recog.range()?;

			recog.base.set_state(248);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(244);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule range*/
				recog.base.set_state(245);
				recog.range()?;

				}
				}
				recog.base.set_state(250);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- range ----------------
pub type RangeContextAll<'input> = RangeContext<'input>;


pub type RangeContext<'input> = BaseParserRuleContext<'input,RangeContextExt<'input>>;

#[derive(Clone)]
pub struct RangeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for RangeContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for RangeContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_range(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_range(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for RangeContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_range(self);
	}
}

impl<'input> CustomRuleContext<'input> for RangeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_range }
	//fn type_rule_index() -> usize where Self: Sized { RULE_range }
}
antlr_rust::type_id!{RangeContextExt<'a>}

impl<'input> RangeContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RangeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RangeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RangeContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<RangeContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IntLit in current rule
fn IntLit_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IntLit, starting from 0.
/// Returns `None` if number of children corresponding to token IntLit is less or equal than `i`.
fn IntLit(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(IntLit, i)
}
/// Retrieves first TerminalNode corresponding to token TO
/// Returns `None` if there is no child corresponding to token TO
fn TO(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(TO, 0)
}

}

impl<'input> RangeContextAttrs<'input> for RangeContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn range(&mut self,)
	-> Result<Rc<RangeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RangeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_range);
        let mut _localctx: Rc<RangeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(255);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(22,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(251);
					recog.base.match_token(IntLit,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(252);
					recog.base.match_token(IntLit,&mut recog.err_handler)?;

					recog.base.set_state(253);
					recog.base.match_token(TO,&mut recog.err_handler)?;

					recog.base.set_state(254);
					recog.base.match_token(IntLit,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- fieldNames ----------------
pub type FieldNamesContextAll<'input> = FieldNamesContext<'input>;


pub type FieldNamesContext<'input> = BaseParserRuleContext<'input,FieldNamesContextExt<'input>>;

#[derive(Clone)]
pub struct FieldNamesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for FieldNamesContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for FieldNamesContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fieldNames(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_fieldNames(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for FieldNamesContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_fieldNames(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldNamesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldNames }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldNames }
}
antlr_rust::type_id!{FieldNamesContextExt<'a>}

impl<'input> FieldNamesContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldNamesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldNamesContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldNamesContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<FieldNamesContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token StrLit in current rule
fn StrLit_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token StrLit, starting from 0.
/// Returns `None` if number of children corresponding to token StrLit is less or equal than `i`.
fn StrLit(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(StrLit, i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> FieldNamesContextAttrs<'input> for FieldNamesContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldNames(&mut self,)
	-> Result<Rc<FieldNamesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldNamesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_fieldNames);
        let mut _localctx: Rc<FieldNamesContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(257);
			recog.base.match_token(StrLit,&mut recog.err_handler)?;

			recog.base.set_state(262);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(258);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				recog.base.set_state(259);
				recog.base.match_token(StrLit,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(264);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- eleType ----------------
pub type EleTypeContextAll<'input> = EleTypeContext<'input>;


pub type EleTypeContext<'input> = BaseParserRuleContext<'input,EleTypeContextExt<'input>>;

#[derive(Clone)]
pub struct EleTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for EleTypeContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for EleTypeContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_eleType(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_eleType(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for EleTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_eleType(self);
	}
}

impl<'input> CustomRuleContext<'input> for EleTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eleType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eleType }
}
antlr_rust::type_id!{EleTypeContextExt<'a>}

impl<'input> EleTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EleTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EleTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EleTypeContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<EleTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DOUBLE
/// Returns `None` if there is no child corresponding to token DOUBLE
fn DOUBLE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(DOUBLE, 0)
}
/// Retrieves first TerminalNode corresponding to token FLOAT
/// Returns `None` if there is no child corresponding to token FLOAT
fn FLOAT(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(FLOAT, 0)
}
/// Retrieves first TerminalNode corresponding to token INT32
/// Returns `None` if there is no child corresponding to token INT32
fn INT32(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(INT32, 0)
}
/// Retrieves first TerminalNode corresponding to token INT64
/// Returns `None` if there is no child corresponding to token INT64
fn INT64(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(INT64, 0)
}
/// Retrieves first TerminalNode corresponding to token UINT32
/// Returns `None` if there is no child corresponding to token UINT32
fn UINT32(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(UINT32, 0)
}
/// Retrieves first TerminalNode corresponding to token UINT64
/// Returns `None` if there is no child corresponding to token UINT64
fn UINT64(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(UINT64, 0)
}
/// Retrieves first TerminalNode corresponding to token SINT32
/// Returns `None` if there is no child corresponding to token SINT32
fn SINT32(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SINT32, 0)
}
/// Retrieves first TerminalNode corresponding to token SINT64
/// Returns `None` if there is no child corresponding to token SINT64
fn SINT64(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SINT64, 0)
}
/// Retrieves first TerminalNode corresponding to token FIXED32
/// Returns `None` if there is no child corresponding to token FIXED32
fn FIXED32(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(FIXED32, 0)
}
/// Retrieves first TerminalNode corresponding to token FIXED64
/// Returns `None` if there is no child corresponding to token FIXED64
fn FIXED64(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(FIXED64, 0)
}
/// Retrieves first TerminalNode corresponding to token SFIXED32
/// Returns `None` if there is no child corresponding to token SFIXED32
fn SFIXED32(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SFIXED32, 0)
}
/// Retrieves first TerminalNode corresponding to token SFIXED64
/// Returns `None` if there is no child corresponding to token SFIXED64
fn SFIXED64(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SFIXED64, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOL
/// Returns `None` if there is no child corresponding to token BOOL
fn BOOL(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(BOOL, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token BYTES
/// Returns `None` if there is no child corresponding to token BYTES
fn BYTES(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(BYTES, 0)
}
fn messageOrEnumType(&self) -> Option<Rc<MessageOrEnumTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EleTypeContextAttrs<'input> for EleTypeContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eleType(&mut self,)
	-> Result<Rc<EleTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EleTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_eleType);
        let mut _localctx: Rc<EleTypeContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(267);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BOOL | BYTES | DOUBLE | FIXED32 | FIXED64 | FLOAT | INT32 | INT64 | SFIXED32 |
			 SFIXED64 | SINT32 | SINT64 | STRING | UINT32 | UINT64 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(265);
					_la = recog.base.input.la(1);
					if  !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOL) | (1usize << BYTES) | (1usize << DOUBLE) | (1usize << FIXED32) | (1usize << FIXED64) | (1usize << FLOAT) | (1usize << INT32) | (1usize << INT64) | (1usize << SFIXED32) | (1usize << SFIXED64) | (1usize << SINT32) | (1usize << SINT64) | (1usize << STRING) | (1usize << UINT32) | (1usize << UINT64))) != 0))  {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}

			 Ident | DOT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule messageOrEnumType*/
					recog.base.set_state(266);
					recog.messageOrEnumType()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- fieldNumber ----------------
pub type FieldNumberContextAll<'input> = FieldNumberContext<'input>;


pub type FieldNumberContext<'input> = BaseParserRuleContext<'input,FieldNumberContextExt<'input>>;

#[derive(Clone)]
pub struct FieldNumberContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for FieldNumberContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for FieldNumberContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fieldNumber(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_fieldNumber(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for FieldNumberContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_fieldNumber(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldNumberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldNumber }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldNumber }
}
antlr_rust::type_id!{FieldNumberContextExt<'a>}

impl<'input> FieldNumberContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldNumberContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldNumberContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldNumberContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<FieldNumberContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IntLit
/// Returns `None` if there is no child corresponding to token IntLit
fn IntLit(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(IntLit, 0)
}

}

impl<'input> FieldNumberContextAttrs<'input> for FieldNumberContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldNumber(&mut self,)
	-> Result<Rc<FieldNumberContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldNumberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_fieldNumber);
        let mut _localctx: Rc<FieldNumberContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(269);
			recog.base.match_token(IntLit,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- field ----------------
pub type FieldContextAll<'input> = FieldContext<'input>;


pub type FieldContext<'input> = BaseParserRuleContext<'input,FieldContextExt<'input>>;

#[derive(Clone)]
pub struct FieldContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for FieldContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for FieldContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_field(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_field(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for FieldContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_field(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_field }
	//fn type_rule_index() -> usize where Self: Sized { RULE_field }
}
antlr_rust::type_id!{FieldContextExt<'a>}

impl<'input> FieldContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<FieldContextExt<'input>>{

fn eleType(&self) -> Option<Rc<EleTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldName(&self) -> Option<Rc<FieldNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
fn fieldNumber(&self) -> Option<Rc<FieldNumberContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
/// Retrieves first TerminalNode corresponding to token REPEATED
/// Returns `None` if there is no child corresponding to token REPEATED
fn REPEATED(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(REPEATED, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACK
/// Returns `None` if there is no child corresponding to token LBRACK
fn LBRACK(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, 0)
}
fn fieldOptions(&self) -> Option<Rc<FieldOptionsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RBRACK
/// Returns `None` if there is no child corresponding to token RBRACK
fn RBRACK(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, 0)
}

}

impl<'input> FieldContextAttrs<'input> for FieldContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn field(&mut self,)
	-> Result<Rc<FieldContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_field);
        let mut _localctx: Rc<FieldContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(272);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==REPEATED {
				{
				recog.base.set_state(271);
				recog.base.match_token(REPEATED,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule eleType*/
			recog.base.set_state(274);
			recog.eleType()?;

			/*InvokeRule fieldName*/
			recog.base.set_state(275);
			recog.fieldName()?;

			recog.base.set_state(276);
			recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

			/*InvokeRule fieldNumber*/
			recog.base.set_state(277);
			recog.fieldNumber()?;

			recog.base.set_state(282);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LBRACK {
				{
				recog.base.set_state(278);
				recog.base.match_token(LBRACK,&mut recog.err_handler)?;

				/*InvokeRule fieldOptions*/
				recog.base.set_state(279);
				recog.fieldOptions()?;

				recog.base.set_state(280);
				recog.base.match_token(RBRACK,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(284);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- fieldOptions ----------------
pub type FieldOptionsContextAll<'input> = FieldOptionsContext<'input>;


pub type FieldOptionsContext<'input> = BaseParserRuleContext<'input,FieldOptionsContextExt<'input>>;

#[derive(Clone)]
pub struct FieldOptionsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for FieldOptionsContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for FieldOptionsContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fieldOptions(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_fieldOptions(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for FieldOptionsContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_fieldOptions(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldOptionsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldOptions }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldOptions }
}
antlr_rust::type_id!{FieldOptionsContextExt<'a>}

impl<'input> FieldOptionsContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldOptionsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldOptionsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldOptionsContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<FieldOptionsContextExt<'input>>{

fn fieldOption_all(&self) ->  Vec<Rc<FieldOptionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fieldOption(&self, i: usize) -> Option<Rc<FieldOptionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> FieldOptionsContextAttrs<'input> for FieldOptionsContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldOptions(&mut self,)
	-> Result<Rc<FieldOptionsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldOptionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_fieldOptions);
        let mut _localctx: Rc<FieldOptionsContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fieldOption*/
			recog.base.set_state(286);
			recog.fieldOption()?;

			recog.base.set_state(291);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(287);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule fieldOption*/
				recog.base.set_state(288);
				recog.fieldOption()?;

				}
				}
				recog.base.set_state(293);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- fieldOption ----------------
pub type FieldOptionContextAll<'input> = FieldOptionContext<'input>;


pub type FieldOptionContext<'input> = BaseParserRuleContext<'input,FieldOptionContextExt<'input>>;

#[derive(Clone)]
pub struct FieldOptionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for FieldOptionContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for FieldOptionContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fieldOption(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_fieldOption(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for FieldOptionContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_fieldOption(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldOptionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldOption }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldOption }
}
antlr_rust::type_id!{FieldOptionContextExt<'a>}

impl<'input> FieldOptionContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldOptionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldOptionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldOptionContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<FieldOptionContextExt<'input>>{

fn optionName(&self) -> Option<Rc<OptionNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
fn constant(&self) -> Option<Rc<ConstantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FieldOptionContextAttrs<'input> for FieldOptionContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldOption(&mut self,)
	-> Result<Rc<FieldOptionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldOptionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_fieldOption);
        let mut _localctx: Rc<FieldOptionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule optionName*/
			recog.base.set_state(294);
			recog.optionName()?;

			recog.base.set_state(295);
			recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

			/*InvokeRule constant*/
			recog.base.set_state(296);
			recog.constant()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- oneof ----------------
pub type OneofContextAll<'input> = OneofContext<'input>;


pub type OneofContext<'input> = BaseParserRuleContext<'input,OneofContextExt<'input>>;

#[derive(Clone)]
pub struct OneofContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for OneofContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for OneofContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_oneof(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_oneof(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for OneofContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_oneof(self);
	}
}

impl<'input> CustomRuleContext<'input> for OneofContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oneof }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oneof }
}
antlr_rust::type_id!{OneofContextExt<'a>}

impl<'input> OneofContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OneofContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OneofContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OneofContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<OneofContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ONEOF
/// Returns `None` if there is no child corresponding to token ONEOF
fn ONEOF(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(ONEOF, 0)
}
fn oneofName(&self) -> Option<Rc<OneofNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn oneofField_all(&self) ->  Vec<Rc<OneofFieldContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn oneofField(&self, i: usize) -> Option<Rc<OneofFieldContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn emptyStatement_all(&self) ->  Vec<Rc<EmptyStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn emptyStatement(&self, i: usize) -> Option<Rc<EmptyStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OneofContextAttrs<'input> for OneofContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn oneof(&mut self,)
	-> Result<Rc<OneofContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OneofContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_oneof);
        let mut _localctx: Rc<OneofContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(298);
			recog.base.match_token(ONEOF,&mut recog.err_handler)?;

			/*InvokeRule oneofName*/
			recog.base.set_state(299);
			recog.oneofName()?;

			recog.base.set_state(300);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(305);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOL) | (1usize << BYTES) | (1usize << DOUBLE) | (1usize << FIXED32) | (1usize << FIXED64) | (1usize << FLOAT) | (1usize << INT32) | (1usize << INT64) | (1usize << SFIXED32) | (1usize << SFIXED64) | (1usize << SINT32) | (1usize << SINT64) | (1usize << STRING) | (1usize << UINT32) | (1usize << UINT64) | (1usize << Ident) | (1usize << SEMI) | (1usize << DOT))) != 0 {
				{
				recog.base.set_state(303);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 BOOL | BYTES | DOUBLE | FIXED32 | FIXED64 | FLOAT | INT32 | INT64 |
				 SFIXED32 | SFIXED64 | SINT32 | SINT64 | STRING | UINT32 | UINT64 | Ident |
				 DOT 
					=> {
						{
						/*InvokeRule oneofField*/
						recog.base.set_state(301);
						recog.oneofField()?;

						}
					}

				 SEMI 
					=> {
						{
						/*InvokeRule emptyStatement*/
						recog.base.set_state(302);
						recog.emptyStatement()?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(307);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(308);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- oneofField ----------------
pub type OneofFieldContextAll<'input> = OneofFieldContext<'input>;


pub type OneofFieldContext<'input> = BaseParserRuleContext<'input,OneofFieldContextExt<'input>>;

#[derive(Clone)]
pub struct OneofFieldContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for OneofFieldContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for OneofFieldContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_oneofField(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_oneofField(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for OneofFieldContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_oneofField(self);
	}
}

impl<'input> CustomRuleContext<'input> for OneofFieldContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oneofField }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oneofField }
}
antlr_rust::type_id!{OneofFieldContextExt<'a>}

impl<'input> OneofFieldContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OneofFieldContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OneofFieldContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OneofFieldContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<OneofFieldContextExt<'input>>{

fn eleType(&self) -> Option<Rc<EleTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldName(&self) -> Option<Rc<FieldNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
fn fieldNumber(&self) -> Option<Rc<FieldNumberContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACK
/// Returns `None` if there is no child corresponding to token LBRACK
fn LBRACK(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, 0)
}
fn fieldOptions(&self) -> Option<Rc<FieldOptionsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RBRACK
/// Returns `None` if there is no child corresponding to token RBRACK
fn RBRACK(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, 0)
}

}

impl<'input> OneofFieldContextAttrs<'input> for OneofFieldContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn oneofField(&mut self,)
	-> Result<Rc<OneofFieldContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OneofFieldContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_oneofField);
        let mut _localctx: Rc<OneofFieldContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule eleType*/
			recog.base.set_state(310);
			recog.eleType()?;

			/*InvokeRule fieldName*/
			recog.base.set_state(311);
			recog.fieldName()?;

			recog.base.set_state(312);
			recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

			/*InvokeRule fieldNumber*/
			recog.base.set_state(313);
			recog.fieldNumber()?;

			recog.base.set_state(318);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LBRACK {
				{
				recog.base.set_state(314);
				recog.base.match_token(LBRACK,&mut recog.err_handler)?;

				/*InvokeRule fieldOptions*/
				recog.base.set_state(315);
				recog.fieldOptions()?;

				recog.base.set_state(316);
				recog.base.match_token(RBRACK,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(320);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- mapField ----------------
pub type MapFieldContextAll<'input> = MapFieldContext<'input>;


pub type MapFieldContext<'input> = BaseParserRuleContext<'input,MapFieldContextExt<'input>>;

#[derive(Clone)]
pub struct MapFieldContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for MapFieldContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for MapFieldContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_mapField(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_mapField(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for MapFieldContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_mapField(self);
	}
}

impl<'input> CustomRuleContext<'input> for MapFieldContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mapField }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mapField }
}
antlr_rust::type_id!{MapFieldContextExt<'a>}

impl<'input> MapFieldContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MapFieldContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MapFieldContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MapFieldContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<MapFieldContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MAP
/// Returns `None` if there is no child corresponding to token MAP
fn MAP(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(MAP, 0)
}
/// Retrieves first TerminalNode corresponding to token LCHEVR
/// Returns `None` if there is no child corresponding to token LCHEVR
fn LCHEVR(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(LCHEVR, 0)
}
fn keyType(&self) -> Option<Rc<KeyTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}
fn eleType(&self) -> Option<Rc<EleTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RCHEVR
/// Returns `None` if there is no child corresponding to token RCHEVR
fn RCHEVR(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(RCHEVR, 0)
}
fn mapName(&self) -> Option<Rc<MapNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
fn fieldNumber(&self) -> Option<Rc<FieldNumberContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACK
/// Returns `None` if there is no child corresponding to token LBRACK
fn LBRACK(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, 0)
}
fn fieldOptions(&self) -> Option<Rc<FieldOptionsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RBRACK
/// Returns `None` if there is no child corresponding to token RBRACK
fn RBRACK(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, 0)
}

}

impl<'input> MapFieldContextAttrs<'input> for MapFieldContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mapField(&mut self,)
	-> Result<Rc<MapFieldContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MapFieldContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_mapField);
        let mut _localctx: Rc<MapFieldContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(322);
			recog.base.match_token(MAP,&mut recog.err_handler)?;

			recog.base.set_state(323);
			recog.base.match_token(LCHEVR,&mut recog.err_handler)?;

			/*InvokeRule keyType*/
			recog.base.set_state(324);
			recog.keyType()?;

			recog.base.set_state(325);
			recog.base.match_token(COMMA,&mut recog.err_handler)?;

			/*InvokeRule eleType*/
			recog.base.set_state(326);
			recog.eleType()?;

			recog.base.set_state(327);
			recog.base.match_token(RCHEVR,&mut recog.err_handler)?;

			/*InvokeRule mapName*/
			recog.base.set_state(328);
			recog.mapName()?;

			recog.base.set_state(329);
			recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

			/*InvokeRule fieldNumber*/
			recog.base.set_state(330);
			recog.fieldNumber()?;

			recog.base.set_state(335);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LBRACK {
				{
				recog.base.set_state(331);
				recog.base.match_token(LBRACK,&mut recog.err_handler)?;

				/*InvokeRule fieldOptions*/
				recog.base.set_state(332);
				recog.fieldOptions()?;

				recog.base.set_state(333);
				recog.base.match_token(RBRACK,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(337);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- keyType ----------------
pub type KeyTypeContextAll<'input> = KeyTypeContext<'input>;


pub type KeyTypeContext<'input> = BaseParserRuleContext<'input,KeyTypeContextExt<'input>>;

#[derive(Clone)]
pub struct KeyTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for KeyTypeContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for KeyTypeContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_keyType(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_keyType(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for KeyTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_keyType(self);
	}
}

impl<'input> CustomRuleContext<'input> for KeyTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_keyType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_keyType }
}
antlr_rust::type_id!{KeyTypeContextExt<'a>}

impl<'input> KeyTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<KeyTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,KeyTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait KeyTypeContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<KeyTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INT32
/// Returns `None` if there is no child corresponding to token INT32
fn INT32(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(INT32, 0)
}
/// Retrieves first TerminalNode corresponding to token INT64
/// Returns `None` if there is no child corresponding to token INT64
fn INT64(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(INT64, 0)
}
/// Retrieves first TerminalNode corresponding to token UINT32
/// Returns `None` if there is no child corresponding to token UINT32
fn UINT32(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(UINT32, 0)
}
/// Retrieves first TerminalNode corresponding to token UINT64
/// Returns `None` if there is no child corresponding to token UINT64
fn UINT64(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(UINT64, 0)
}
/// Retrieves first TerminalNode corresponding to token SINT32
/// Returns `None` if there is no child corresponding to token SINT32
fn SINT32(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SINT32, 0)
}
/// Retrieves first TerminalNode corresponding to token SINT64
/// Returns `None` if there is no child corresponding to token SINT64
fn SINT64(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SINT64, 0)
}
/// Retrieves first TerminalNode corresponding to token FIXED32
/// Returns `None` if there is no child corresponding to token FIXED32
fn FIXED32(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(FIXED32, 0)
}
/// Retrieves first TerminalNode corresponding to token FIXED64
/// Returns `None` if there is no child corresponding to token FIXED64
fn FIXED64(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(FIXED64, 0)
}
/// Retrieves first TerminalNode corresponding to token SFIXED32
/// Returns `None` if there is no child corresponding to token SFIXED32
fn SFIXED32(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SFIXED32, 0)
}
/// Retrieves first TerminalNode corresponding to token SFIXED64
/// Returns `None` if there is no child corresponding to token SFIXED64
fn SFIXED64(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SFIXED64, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOL
/// Returns `None` if there is no child corresponding to token BOOL
fn BOOL(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(BOOL, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> KeyTypeContextAttrs<'input> for KeyTypeContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn keyType(&mut self,)
	-> Result<Rc<KeyTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = KeyTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_keyType);
        let mut _localctx: Rc<KeyTypeContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(339);
			_la = recog.base.input.la(1);
			if  !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOL) | (1usize << FIXED32) | (1usize << FIXED64) | (1usize << INT32) | (1usize << INT64) | (1usize << SFIXED32) | (1usize << SFIXED64) | (1usize << SINT32) | (1usize << SINT64) | (1usize << STRING) | (1usize << UINT32) | (1usize << UINT64))) != 0))  {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- fullIdent ----------------
pub type FullIdentContextAll<'input> = FullIdentContext<'input>;


pub type FullIdentContext<'input> = BaseParserRuleContext<'input,FullIdentContextExt<'input>>;

#[derive(Clone)]
pub struct FullIdentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for FullIdentContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for FullIdentContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fullIdent(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_fullIdent(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for FullIdentContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_fullIdent(self);
	}
}

impl<'input> CustomRuleContext<'input> for FullIdentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fullIdent }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fullIdent }
}
antlr_rust::type_id!{FullIdentContextExt<'a>}

impl<'input> FullIdentContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FullIdentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FullIdentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FullIdentContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<FullIdentContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token Ident in current rule
fn Ident_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Ident, starting from 0.
/// Returns `None` if number of children corresponding to token Ident is less or equal than `i`.
fn Ident(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(Ident, i)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}

}

impl<'input> FullIdentContextAttrs<'input> for FullIdentContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fullIdent(&mut self,)
	-> Result<Rc<FullIdentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FullIdentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_fullIdent);
        let mut _localctx: Rc<FullIdentContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(341);
			recog.base.match_token(Ident,&mut recog.err_handler)?;

			recog.base.set_state(346);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==DOT {
				{
				{
				recog.base.set_state(342);
				recog.base.match_token(DOT,&mut recog.err_handler)?;

				recog.base.set_state(343);
				recog.base.match_token(Ident,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(348);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- messageName ----------------
pub type MessageNameContextAll<'input> = MessageNameContext<'input>;


pub type MessageNameContext<'input> = BaseParserRuleContext<'input,MessageNameContextExt<'input>>;

#[derive(Clone)]
pub struct MessageNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for MessageNameContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for MessageNameContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_messageName(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_messageName(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for MessageNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_messageName(self);
	}
}

impl<'input> CustomRuleContext<'input> for MessageNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_messageName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_messageName }
}
antlr_rust::type_id!{MessageNameContextExt<'a>}

impl<'input> MessageNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MessageNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MessageNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MessageNameContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<MessageNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Ident
/// Returns `None` if there is no child corresponding to token Ident
fn Ident(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(Ident, 0)
}

}

impl<'input> MessageNameContextAttrs<'input> for MessageNameContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn messageName(&mut self,)
	-> Result<Rc<MessageNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MessageNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_messageName);
        let mut _localctx: Rc<MessageNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(349);
			recog.base.match_token(Ident,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- enumName ----------------
pub type EnumNameContextAll<'input> = EnumNameContext<'input>;


pub type EnumNameContext<'input> = BaseParserRuleContext<'input,EnumNameContextExt<'input>>;

#[derive(Clone)]
pub struct EnumNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for EnumNameContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for EnumNameContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_enumName(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_enumName(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for EnumNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_enumName(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumName }
}
antlr_rust::type_id!{EnumNameContextExt<'a>}

impl<'input> EnumNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumNameContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<EnumNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Ident
/// Returns `None` if there is no child corresponding to token Ident
fn Ident(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(Ident, 0)
}

}

impl<'input> EnumNameContextAttrs<'input> for EnumNameContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumName(&mut self,)
	-> Result<Rc<EnumNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_enumName);
        let mut _localctx: Rc<EnumNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(351);
			recog.base.match_token(Ident,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- messageOrEnumName ----------------
pub type MessageOrEnumNameContextAll<'input> = MessageOrEnumNameContext<'input>;


pub type MessageOrEnumNameContext<'input> = BaseParserRuleContext<'input,MessageOrEnumNameContextExt<'input>>;

#[derive(Clone)]
pub struct MessageOrEnumNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for MessageOrEnumNameContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for MessageOrEnumNameContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_messageOrEnumName(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_messageOrEnumName(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for MessageOrEnumNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_messageOrEnumName(self);
	}
}

impl<'input> CustomRuleContext<'input> for MessageOrEnumNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_messageOrEnumName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_messageOrEnumName }
}
antlr_rust::type_id!{MessageOrEnumNameContextExt<'a>}

impl<'input> MessageOrEnumNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MessageOrEnumNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MessageOrEnumNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MessageOrEnumNameContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<MessageOrEnumNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Ident
/// Returns `None` if there is no child corresponding to token Ident
fn Ident(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(Ident, 0)
}

}

impl<'input> MessageOrEnumNameContextAttrs<'input> for MessageOrEnumNameContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn messageOrEnumName(&mut self,)
	-> Result<Rc<MessageOrEnumNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MessageOrEnumNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_messageOrEnumName);
        let mut _localctx: Rc<MessageOrEnumNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(353);
			recog.base.match_token(Ident,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- fieldName ----------------
pub type FieldNameContextAll<'input> = FieldNameContext<'input>;


pub type FieldNameContext<'input> = BaseParserRuleContext<'input,FieldNameContextExt<'input>>;

#[derive(Clone)]
pub struct FieldNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for FieldNameContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for FieldNameContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fieldName(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_fieldName(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for FieldNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_fieldName(self);
	}
}

impl<'input> CustomRuleContext<'input> for FieldNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldName }
}
antlr_rust::type_id!{FieldNameContextExt<'a>}

impl<'input> FieldNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldNameContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<FieldNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Ident
/// Returns `None` if there is no child corresponding to token Ident
fn Ident(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(Ident, 0)
}

}

impl<'input> FieldNameContextAttrs<'input> for FieldNameContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldName(&mut self,)
	-> Result<Rc<FieldNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_fieldName);
        let mut _localctx: Rc<FieldNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(355);
			recog.base.match_token(Ident,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- oneofName ----------------
pub type OneofNameContextAll<'input> = OneofNameContext<'input>;


pub type OneofNameContext<'input> = BaseParserRuleContext<'input,OneofNameContextExt<'input>>;

#[derive(Clone)]
pub struct OneofNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for OneofNameContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for OneofNameContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_oneofName(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_oneofName(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for OneofNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_oneofName(self);
	}
}

impl<'input> CustomRuleContext<'input> for OneofNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oneofName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oneofName }
}
antlr_rust::type_id!{OneofNameContextExt<'a>}

impl<'input> OneofNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OneofNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OneofNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OneofNameContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<OneofNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Ident
/// Returns `None` if there is no child corresponding to token Ident
fn Ident(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(Ident, 0)
}

}

impl<'input> OneofNameContextAttrs<'input> for OneofNameContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn oneofName(&mut self,)
	-> Result<Rc<OneofNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OneofNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_oneofName);
        let mut _localctx: Rc<OneofNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(357);
			recog.base.match_token(Ident,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- mapName ----------------
pub type MapNameContextAll<'input> = MapNameContext<'input>;


pub type MapNameContext<'input> = BaseParserRuleContext<'input,MapNameContextExt<'input>>;

#[derive(Clone)]
pub struct MapNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for MapNameContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for MapNameContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_mapName(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_mapName(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for MapNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_mapName(self);
	}
}

impl<'input> CustomRuleContext<'input> for MapNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mapName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mapName }
}
antlr_rust::type_id!{MapNameContextExt<'a>}

impl<'input> MapNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MapNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MapNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MapNameContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<MapNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Ident
/// Returns `None` if there is no child corresponding to token Ident
fn Ident(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(Ident, 0)
}

}

impl<'input> MapNameContextAttrs<'input> for MapNameContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mapName(&mut self,)
	-> Result<Rc<MapNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MapNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_mapName);
        let mut _localctx: Rc<MapNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(359);
			recog.base.match_token(Ident,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- serviceName ----------------
pub type ServiceNameContextAll<'input> = ServiceNameContext<'input>;


pub type ServiceNameContext<'input> = BaseParserRuleContext<'input,ServiceNameContextExt<'input>>;

#[derive(Clone)]
pub struct ServiceNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for ServiceNameContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for ServiceNameContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_serviceName(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_serviceName(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for ServiceNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_serviceName(self);
	}
}

impl<'input> CustomRuleContext<'input> for ServiceNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_serviceName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_serviceName }
}
antlr_rust::type_id!{ServiceNameContextExt<'a>}

impl<'input> ServiceNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ServiceNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ServiceNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ServiceNameContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<ServiceNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Ident
/// Returns `None` if there is no child corresponding to token Ident
fn Ident(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(Ident, 0)
}

}

impl<'input> ServiceNameContextAttrs<'input> for ServiceNameContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn serviceName(&mut self,)
	-> Result<Rc<ServiceNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ServiceNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_serviceName);
        let mut _localctx: Rc<ServiceNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(361);
			recog.base.match_token(Ident,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- rpcName ----------------
pub type RpcNameContextAll<'input> = RpcNameContext<'input>;


pub type RpcNameContext<'input> = BaseParserRuleContext<'input,RpcNameContextExt<'input>>;

#[derive(Clone)]
pub struct RpcNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for RpcNameContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for RpcNameContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_rpcName(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_rpcName(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for RpcNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_rpcName(self);
	}
}

impl<'input> CustomRuleContext<'input> for RpcNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rpcName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rpcName }
}
antlr_rust::type_id!{RpcNameContextExt<'a>}

impl<'input> RpcNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RpcNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RpcNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RpcNameContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<RpcNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Ident
/// Returns `None` if there is no child corresponding to token Ident
fn Ident(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(Ident, 0)
}

}

impl<'input> RpcNameContextAttrs<'input> for RpcNameContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn rpcName(&mut self,)
	-> Result<Rc<RpcNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RpcNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_rpcName);
        let mut _localctx: Rc<RpcNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(363);
			recog.base.match_token(Ident,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- messageType ----------------
pub type MessageTypeContextAll<'input> = MessageTypeContext<'input>;


pub type MessageTypeContext<'input> = BaseParserRuleContext<'input,MessageTypeContextExt<'input>>;

#[derive(Clone)]
pub struct MessageTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for MessageTypeContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for MessageTypeContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_messageType(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_messageType(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for MessageTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_messageType(self);
	}
}

impl<'input> CustomRuleContext<'input> for MessageTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_messageType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_messageType }
}
antlr_rust::type_id!{MessageTypeContextExt<'a>}

impl<'input> MessageTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MessageTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MessageTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MessageTypeContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<MessageTypeContextExt<'input>>{

fn messageName(&self) -> Option<Rc<MessageNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Ident in current rule
fn Ident_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Ident, starting from 0.
/// Returns `None` if number of children corresponding to token Ident is less or equal than `i`.
fn Ident(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(Ident, i)
}

}

impl<'input> MessageTypeContextAttrs<'input> for MessageTypeContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn messageType(&mut self,)
	-> Result<Rc<MessageTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MessageTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_messageType);
        let mut _localctx: Rc<MessageTypeContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(366);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==DOT {
				{
				recog.base.set_state(365);
				recog.base.match_token(DOT,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(372);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(34,&mut recog.base)?;
			while  _alt!=2 && _alt!=INVALID_ALT  {
				if _alt==1 {
					{
					{
					recog.base.set_state(368);
					recog.base.match_token(Ident,&mut recog.err_handler)?;

					recog.base.set_state(369);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(374);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(34,&mut recog.base)?;
			}
			/*InvokeRule messageName*/
			recog.base.set_state(375);
			recog.messageName()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- messageOrEnumType ----------------
pub type MessageOrEnumTypeContextAll<'input> = MessageOrEnumTypeContext<'input>;


pub type MessageOrEnumTypeContext<'input> = BaseParserRuleContext<'input,MessageOrEnumTypeContextExt<'input>>;

#[derive(Clone)]
pub struct MessageOrEnumTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for MessageOrEnumTypeContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for MessageOrEnumTypeContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_messageOrEnumType(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_messageOrEnumType(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for MessageOrEnumTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_messageOrEnumType(self);
	}
}

impl<'input> CustomRuleContext<'input> for MessageOrEnumTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_messageOrEnumType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_messageOrEnumType }
}
antlr_rust::type_id!{MessageOrEnumTypeContextExt<'a>}

impl<'input> MessageOrEnumTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MessageOrEnumTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MessageOrEnumTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MessageOrEnumTypeContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<MessageOrEnumTypeContextExt<'input>>{

fn messageOrEnumName(&self) -> Option<Rc<MessageOrEnumNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Ident in current rule
fn Ident_all(&self) -> Vec<Rc<TerminalNode<'input,Loki_protoParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Ident, starting from 0.
/// Returns `None` if number of children corresponding to token Ident is less or equal than `i`.
fn Ident(&self, i: usize) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(Ident, i)
}

}

impl<'input> MessageOrEnumTypeContextAttrs<'input> for MessageOrEnumTypeContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn messageOrEnumType(&mut self,)
	-> Result<Rc<MessageOrEnumTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MessageOrEnumTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_messageOrEnumType);
        let mut _localctx: Rc<MessageOrEnumTypeContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(378);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==DOT {
				{
				recog.base.set_state(377);
				recog.base.match_token(DOT,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(384);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(36,&mut recog.base)?;
			while  _alt!=2 && _alt!=INVALID_ALT  {
				if _alt==1 {
					{
					{
					recog.base.set_state(380);
					recog.base.match_token(Ident,&mut recog.err_handler)?;

					recog.base.set_state(381);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(386);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(36,&mut recog.base)?;
			}
			/*InvokeRule messageOrEnumName*/
			recog.base.set_state(387);
			recog.messageOrEnumName()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- emptyStatement ----------------
pub type EmptyStatementContextAll<'input> = EmptyStatementContext<'input>;


pub type EmptyStatementContext<'input> = BaseParserRuleContext<'input,EmptyStatementContextExt<'input>>;

#[derive(Clone)]
pub struct EmptyStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for EmptyStatementContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for EmptyStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_emptyStatement(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_emptyStatement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for EmptyStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_emptyStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for EmptyStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_emptyStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_emptyStatement }
}
antlr_rust::type_id!{EmptyStatementContextExt<'a>}

impl<'input> EmptyStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EmptyStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EmptyStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EmptyStatementContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<EmptyStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}

}

impl<'input> EmptyStatementContextAttrs<'input> for EmptyStatementContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn emptyStatement(&mut self,)
	-> Result<Rc<EmptyStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EmptyStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_emptyStatement);
        let mut _localctx: Rc<EmptyStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(389);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- constant ----------------
pub type ConstantContextAll<'input> = ConstantContext<'input>;


pub type ConstantContext<'input> = BaseParserRuleContext<'input,ConstantContextExt<'input>>;

#[derive(Clone)]
pub struct ConstantContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Loki_protoParserContext<'input> for ConstantContext<'input>{}

impl<'input,'a> Listenable<dyn Loki_protoListener<'input> + 'a> for ConstantContext<'input>{
	fn enter(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_constant(self);
	}
	fn exit(&self,listener: &mut (dyn Loki_protoListener<'input> + 'a)) {
		listener.exit_constant(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn Loki_protoVisitor<'input> + 'a> for ConstantContext<'input>{
	fn accept(&self,visitor: &mut (dyn Loki_protoVisitor<'input> + 'a)) {
		visitor.visit_constant(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Loki_protoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constant }
}
antlr_rust::type_id!{ConstantContextExt<'a>}

impl<'input> ConstantContextExt<'input>{
	fn new(parent: Option<Rc<dyn Loki_protoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConstantContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstantContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConstantContextAttrs<'input>: Loki_protoParserContext<'input> + BorrowMut<ConstantContextExt<'input>>{

fn fullIdent(&self) -> Option<Rc<FullIdentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IntLit
/// Returns `None` if there is no child corresponding to token IntLit
fn IntLit(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(IntLit, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}
/// Retrieves first TerminalNode corresponding to token PLUS
/// Returns `None` if there is no child corresponding to token PLUS
fn PLUS(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(PLUS, 0)
}
/// Retrieves first TerminalNode corresponding to token FloatLit
/// Returns `None` if there is no child corresponding to token FloatLit
fn FloatLit(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(FloatLit, 0)
}
/// Retrieves first TerminalNode corresponding to token StrLit
/// Returns `None` if there is no child corresponding to token StrLit
fn StrLit(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(StrLit, 0)
}
/// Retrieves first TerminalNode corresponding to token BoolLit
/// Returns `None` if there is no child corresponding to token BoolLit
fn BoolLit(&self) -> Option<Rc<TerminalNode<'input,Loki_protoParserContextType>>> where Self:Sized{
	self.get_token(BoolLit, 0)
}

}

impl<'input> ConstantContextAttrs<'input> for ConstantContext<'input>{}

impl<'input, I, H> Loki_protoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constant(&mut self,)
	-> Result<Rc<ConstantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_constant);
        let mut _localctx: Rc<ConstantContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(401);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(39,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fullIdent*/
					recog.base.set_state(391);
					recog.fullIdent()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(393);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==MINUS || _la==PLUS {
						{
						recog.base.set_state(392);
						_la = recog.base.input.la(1);
						if  !(_la==MINUS || _la==PLUS)  {
							recog.err_handler.recover_inline(&mut recog.base)?;

						}
						else {
							if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler);
						}
						}
					}

					recog.base.set_state(395);
					recog.base.match_token(IntLit,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(397);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==MINUS || _la==PLUS {
						{
						recog.base.set_state(396);
						_la = recog.base.input.la(1);
						if  !(_la==MINUS || _la==PLUS)  {
							recog.err_handler.recover_inline(&mut recog.base)?;

						}
						else {
							if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler);
						}
						}
					}

					recog.base.set_state(399);
					recog.base.match_token(FloatLit,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(400);
					_la = recog.base.input.la(1);
					if  !(_la==BoolLit || _la==StrLit)  {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
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
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x3b\u{196}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x07\
	\x02\x5b\x0a\x02\x0c\x02\x0e\x02\x5e\x0b\x02\x03\x02\x03\x02\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x04\x03\x04\x05\x04\x69\x0a\x04\x03\x04\
	\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x03\x05\x03\x06\x03\x06\x03\x06\
	\x03\x06\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x05\x07\
	\x7d\x0a\x07\x03\x07\x03\x07\x07\x07\u{81}\x0a\x07\x0c\x07\x0e\x07\u{84}\
	\x0b\x07\x03\x08\x03\x08\x03\x08\x05\x08\u{89}\x0a\x08\x03\x09\x03\x09\x03\
	\x09\x03\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0a\x03\x0a\x07\x0a\u{98}\x0a\x0a\x0c\x0a\x0e\x0a\u{9b}\x0b\x0a\x03\x0a\
	\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x07\x0c\u{a7}\x0a\x0c\x0c\x0c\x0e\x0c\u{aa}\x0b\x0c\x03\x0c\x03\x0c\x03\
	\x0d\x03\x0d\x03\x0d\x05\x0d\u{b1}\x0a\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\
	\x03\x0d\x07\x0d\u{b8}\x0a\x0d\x0c\x0d\x0e\x0d\u{bb}\x0b\x0d\x03\x0d\x03\
	\x0d\x05\x0d\u{bf}\x0a\x0d\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
	\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x07\x0f\u{cd}\x0a\x0f\x0c\
	\x0f\x0e\x0f\u{d0}\x0b\x0f\x03\x0f\x03\x0f\x03\x10\x03\x10\x03\x10\x03\x10\
	\x05\x10\u{d8}\x0a\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x05\x10\u{df}\
	\x0a\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x07\x10\u{e6}\x0a\x10\x0c\
	\x10\x0e\x10\u{e9}\x0b\x10\x03\x10\x03\x10\x05\x10\u{ed}\x0a\x10\x03\x11\
	\x03\x11\x03\x11\x05\x11\u{f2}\x0a\x11\x03\x11\x03\x11\x03\x12\x03\x12\x03\
	\x12\x07\x12\u{f9}\x0a\x12\x0c\x12\x0e\x12\u{fc}\x0b\x12\x03\x13\x03\x13\
	\x03\x13\x03\x13\x05\x13\u{102}\x0a\x13\x03\x14\x03\x14\x03\x14\x07\x14\
	\u{107}\x0a\x14\x0c\x14\x0e\x14\u{10a}\x0b\x14\x03\x15\x03\x15\x05\x15\u{10e}\
	\x0a\x15\x03\x16\x03\x16\x03\x17\x05\x17\u{113}\x0a\x17\x03\x17\x03\x17\
	\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x05\x17\u{11d}\x0a\x17\
	\x03\x17\x03\x17\x03\x18\x03\x18\x03\x18\x07\x18\u{124}\x0a\x18\x0c\x18\
	\x0e\x18\u{127}\x0b\x18\x03\x19\x03\x19\x03\x19\x03\x19\x03\x1a\x03\x1a\
	\x03\x1a\x03\x1a\x03\x1a\x07\x1a\u{132}\x0a\x1a\x0c\x1a\x0e\x1a\u{135}\x0b\
	\x1a\x03\x1a\x03\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\
	\x1b\x03\x1b\x05\x1b\u{141}\x0a\x1b\x03\x1b\x03\x1b\x03\x1c\x03\x1c\x03\
	\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\
	\x1c\x03\x1c\x05\x1c\u{152}\x0a\x1c\x03\x1c\x03\x1c\x03\x1d\x03\x1d\x03\
	\x1e\x03\x1e\x03\x1e\x07\x1e\u{15b}\x0a\x1e\x0c\x1e\x0e\x1e\u{15e}\x0b\x1e\
	\x03\x1f\x03\x1f\x03\x20\x03\x20\x03\x21\x03\x21\x03\x22\x03\x22\x03\x23\
	\x03\x23\x03\x24\x03\x24\x03\x25\x03\x25\x03\x26\x03\x26\x03\x27\x05\x27\
	\u{171}\x0a\x27\x03\x27\x03\x27\x07\x27\u{175}\x0a\x27\x0c\x27\x0e\x27\u{178}\
	\x0b\x27\x03\x27\x03\x27\x03\x28\x05\x28\u{17d}\x0a\x28\x03\x28\x03\x28\
	\x07\x28\u{181}\x0a\x28\x0c\x28\x0e\x28\u{184}\x0b\x28\x03\x28\x03\x28\x03\
	\x29\x03\x29\x03\x2a\x03\x2a\x05\x2a\u{18c}\x0a\x2a\x03\x2a\x03\x2a\x05\
	\x2a\u{190}\x0a\x2a\x03\x2a\x03\x2a\x05\x2a\u{194}\x0a\x2a\x03\x2a\x02\x02\
	\x2b\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\
	\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\
	\x48\x4a\x4c\x4e\x50\x52\x02\x08\x03\x02\x12\x13\x04\x02\x14\x14\x24\x24\
	\x08\x02\x03\x05\x07\x09\x0b\x0c\x1a\x1d\x1f\x1f\x22\x23\x08\x02\x03\x03\
	\x07\x08\x0b\x0c\x1a\x1d\x1f\x1f\x22\x23\x03\x02\x36\x37\x03\x02\x28\x29\
	\x02\u{1a2}\x02\x54\x03\x02\x02\x02\x04\x61\x03\x02\x02\x02\x06\x66\x03\
	\x02\x02\x02\x08\x6d\x03\x02\x02\x02\x0a\x71\x03\x02\x02\x02\x0c\x7c\x03\
	\x02\x02\x02\x0e\u{88}\x03\x02\x02\x02\x10\u{8a}\x03\x02\x02\x02\x12\u{8e}\
	\x03\x02\x02\x02\x14\u{9e}\x03\x02\x02\x02\x16\u{a2}\x03\x02\x02\x02\x18\
	\u{ad}\x03\x02\x02\x02\x1a\u{c2}\x03\x02\x02\x02\x1c\u{c6}\x03\x02\x02\x02\
	\x1e\u{d3}\x03\x02\x02\x02\x20\u{ee}\x03\x02\x02\x02\x22\u{f5}\x03\x02\x02\
	\x02\x24\u{101}\x03\x02\x02\x02\x26\u{103}\x03\x02\x02\x02\x28\u{10d}\x03\
	\x02\x02\x02\x2a\u{10f}\x03\x02\x02\x02\x2c\u{112}\x03\x02\x02\x02\x2e\u{120}\
	\x03\x02\x02\x02\x30\u{128}\x03\x02\x02\x02\x32\u{12c}\x03\x02\x02\x02\x34\
	\u{138}\x03\x02\x02\x02\x36\u{144}\x03\x02\x02\x02\x38\u{155}\x03\x02\x02\
	\x02\x3a\u{157}\x03\x02\x02\x02\x3c\u{15f}\x03\x02\x02\x02\x3e\u{161}\x03\
	\x02\x02\x02\x40\u{163}\x03\x02\x02\x02\x42\u{165}\x03\x02\x02\x02\x44\u{167}\
	\x03\x02\x02\x02\x46\u{169}\x03\x02\x02\x02\x48\u{16b}\x03\x02\x02\x02\x4a\
	\u{16d}\x03\x02\x02\x02\x4c\u{170}\x03\x02\x02\x02\x4e\u{17c}\x03\x02\x02\
	\x02\x50\u{187}\x03\x02\x02\x02\x52\u{193}\x03\x02\x02\x02\x54\x5c\x05\x04\
	\x03\x02\x55\x5b\x05\x06\x04\x02\x56\x5b\x05\x08\x05\x02\x57\x5b\x05\x0a\
	\x06\x02\x58\x5b\x05\x0e\x08\x02\x59\x5b\x05\x50\x29\x02\x5a\x55\x03\x02\
	\x02\x02\x5a\x56\x03\x02\x02\x02\x5a\x57\x03\x02\x02\x02\x5a\x58\x03\x02\
	\x02\x02\x5a\x59\x03\x02\x02\x02\x5b\x5e\x03\x02\x02\x02\x5c\x5a\x03\x02\
	\x02\x02\x5c\x5d\x03\x02\x02\x02\x5d\x5f\x03\x02\x02\x02\x5e\x5c\x03\x02\
	\x02\x02\x5f\x60\x07\x02\x02\x03\x60\x03\x03\x02\x02\x02\x61\x62\x07\x20\
	\x02\x02\x62\x63\x07\x38\x02\x02\x63\x64\x09\x02\x02\x02\x64\x65\x07\x33\
	\x02\x02\x65\x05\x03\x02\x02\x02\x66\x68\x07\x0a\x02\x02\x67\x69\x09\x03\
	\x02\x02\x68\x67\x03\x02\x02\x02\x68\x69\x03\x02\x02\x02\x69\x6a\x03\x02\
	\x02\x02\x6a\x6b\x07\x29\x02\x02\x6b\x6c\x07\x33\x02\x02\x6c\x07\x03\x02\
	\x02\x02\x6d\x6e\x07\x11\x02\x02\x6e\x6f\x05\x3a\x1e\x02\x6f\x70\x07\x33\
	\x02\x02\x70\x09\x03\x02\x02\x02\x71\x72\x07\x10\x02\x02\x72\x73\x05\x0c\
	\x07\x02\x73\x74\x07\x38\x02\x02\x74\x75\x05\x52\x2a\x02\x75\x76\x07\x33\
	\x02\x02\x76\x0b\x03\x02\x02\x02\x77\x7d\x07\x25\x02\x02\x78\x79\x07\x2b\
	\x02\x02\x79\x7a\x05\x3a\x1e\x02\x7a\x7b\x07\x2c\x02\x02\x7b\x7d\x03\x02\
	\x02\x02\x7c\x77\x03\x02\x02\x02\x7c\x78\x03\x02\x02\x02\x7d\u{82}\x03\x02\
	\x02\x02\x7e\x7f\x07\x35\x02\x02\x7f\u{81}\x07\x25\x02\x02\u{80}\x7e\x03\
	\x02\x02\x02\u{81}\u{84}\x03\x02\x02\x02\u{82}\u{80}\x03\x02\x02\x02\u{82}\
	\u{83}\x03\x02\x02\x02\u{83}\x0d\x03\x02\x02\x02\u{84}\u{82}\x03\x02\x02\
	\x02\u{85}\u{89}\x05\x10\x09\x02\u{86}\u{89}\x05\x14\x0b\x02\u{87}\u{89}\
	\x05\x1c\x0f\x02\u{88}\u{85}\x03\x02\x02\x02\u{88}\u{86}\x03\x02\x02\x02\
	\u{88}\u{87}\x03\x02\x02\x02\u{89}\x0f\x03\x02\x02\x02\u{8a}\u{8b}\x07\x0e\
	\x02\x02\u{8b}\u{8c}\x05\x3c\x1f\x02\u{8c}\u{8d}\x05\x12\x0a\x02\u{8d}\x11\
	\x03\x02\x02\x02\u{8e}\u{99}\x07\x2d\x02\x02\u{8f}\u{98}\x05\x2c\x17\x02\
	\u{90}\u{98}\x05\x14\x0b\x02\u{91}\u{98}\x05\x10\x09\x02\u{92}\u{98}\x05\
	\x0a\x06\x02\u{93}\u{98}\x05\x32\x1a\x02\u{94}\u{98}\x05\x36\x1c\x02\u{95}\
	\u{98}\x05\x20\x11\x02\u{96}\u{98}\x05\x50\x29\x02\u{97}\u{8f}\x03\x02\x02\
	\x02\u{97}\u{90}\x03\x02\x02\x02\u{97}\u{91}\x03\x02\x02\x02\u{97}\u{92}\
	\x03\x02\x02\x02\u{97}\u{93}\x03\x02\x02\x02\u{97}\u{94}\x03\x02\x02\x02\
	\u{97}\u{95}\x03\x02\x02\x02\u{97}\u{96}\x03\x02\x02\x02\u{98}\u{9b}\x03\
	\x02\x02\x02\u{99}\u{97}\x03\x02\x02\x02\u{99}\u{9a}\x03\x02\x02\x02\u{9a}\
	\u{9c}\x03\x02\x02\x02\u{9b}\u{99}\x03\x02\x02\x02\u{9c}\u{9d}\x07\x2e\x02\
	\x02\u{9d}\x13\x03\x02\x02\x02\u{9e}\u{9f}\x07\x06\x02\x02\u{9f}\u{a0}\x05\
	\x3e\x20\x02\u{a0}\u{a1}\x05\x16\x0c\x02\u{a1}\x15\x03\x02\x02\x02\u{a2}\
	\u{a8}\x07\x2d\x02\x02\u{a3}\u{a7}\x05\x0a\x06\x02\u{a4}\u{a7}\x05\x18\x0d\
	\x02\u{a5}\u{a7}\x05\x50\x29\x02\u{a6}\u{a3}\x03\x02\x02\x02\u{a6}\u{a4}\
	\x03\x02\x02\x02\u{a6}\u{a5}\x03\x02\x02\x02\u{a7}\u{aa}\x03\x02\x02\x02\
	\u{a8}\u{a6}\x03\x02\x02\x02\u{a8}\u{a9}\x03\x02\x02\x02\u{a9}\u{ab}\x03\
	\x02\x02\x02\u{aa}\u{a8}\x03\x02\x02\x02\u{ab}\u{ac}\x07\x2e\x02\x02\u{ac}\
	\x17\x03\x02\x02\x02\u{ad}\u{ae}\x07\x25\x02\x02\u{ae}\u{b0}\x07\x38\x02\
	\x02\u{af}\u{b1}\x07\x36\x02\x02\u{b0}\u{af}\x03\x02\x02\x02\u{b0}\u{b1}\
	\x03\x02\x02\x02\u{b1}\u{b2}\x03\x02\x02\x02\u{b2}\u{be}\x07\x26\x02\x02\
	\u{b3}\u{b4}\x07\x2f\x02\x02\u{b4}\u{b9}\x05\x1a\x0e\x02\u{b5}\u{b6}\x07\
	\x34\x02\x02\u{b6}\u{b8}\x05\x1a\x0e\x02\u{b7}\u{b5}\x03\x02\x02\x02\u{b8}\
	\u{bb}\x03\x02\x02\x02\u{b9}\u{b7}\x03\x02\x02\x02\u{b9}\u{ba}\x03\x02\x02\
	\x02\u{ba}\u{bc}\x03\x02\x02\x02\u{bb}\u{b9}\x03\x02\x02\x02\u{bc}\u{bd}\
	\x07\x30\x02\x02\u{bd}\u{bf}\x03\x02\x02\x02\u{be}\u{b3}\x03\x02\x02\x02\
	\u{be}\u{bf}\x03\x02\x02\x02\u{bf}\u{c0}\x03\x02\x02\x02\u{c0}\u{c1}\x07\
	\x33\x02\x02\u{c1}\x19\x03\x02\x02\x02\u{c2}\u{c3}\x05\x0c\x07\x02\u{c3}\
	\u{c4}\x07\x38\x02\x02\u{c4}\u{c5}\x05\x52\x2a\x02\u{c5}\x1b\x03\x02\x02\
	\x02\u{c6}\u{c7}\x07\x19\x02\x02\u{c7}\u{c8}\x05\x48\x25\x02\u{c8}\u{ce}\
	\x07\x2d\x02\x02\u{c9}\u{cd}\x05\x0a\x06\x02\u{ca}\u{cd}\x05\x1e\x10\x02\
	\u{cb}\u{cd}\x05\x50\x29\x02\u{cc}\u{c9}\x03\x02\x02\x02\u{cc}\u{ca}\x03\
	\x02\x02\x02\u{cc}\u{cb}\x03\x02\x02\x02\u{cd}\u{d0}\x03\x02\x02\x02\u{ce}\
	\u{cc}\x03\x02\x02\x02\u{ce}\u{cf}\x03\x02\x02\x02\u{cf}\u{d1}\x03\x02\x02\
	\x02\u{d0}\u{ce}\x03\x02\x02\x02\u{d1}\u{d2}\x07\x2e\x02\x02\u{d2}\x1d\x03\
	\x02\x02\x02\u{d3}\u{d4}\x07\x18\x02\x02\u{d4}\u{d5}\x05\x4a\x26\x02\u{d5}\
	\u{d7}\x07\x2b\x02\x02\u{d6}\u{d8}\x07\x1e\x02\x02\u{d7}\u{d6}\x03\x02\x02\
	\x02\u{d7}\u{d8}\x03\x02\x02\x02\u{d8}\u{d9}\x03\x02\x02\x02\u{d9}\u{da}\
	\x05\x4c\x27\x02\u{da}\u{db}\x07\x2c\x02\x02\u{db}\u{dc}\x07\x17\x02\x02\
	\u{dc}\u{de}\x07\x2b\x02\x02\u{dd}\u{df}\x07\x1e\x02\x02\u{de}\u{dd}\x03\
	\x02\x02\x02\u{de}\u{df}\x03\x02\x02\x02\u{df}\u{e0}\x03\x02\x02\x02\u{e0}\
	\u{e1}\x05\x4c\x27\x02\u{e1}\u{ec}\x07\x2c\x02\x02\u{e2}\u{e7}\x07\x2d\x02\
	\x02\u{e3}\u{e6}\x05\x0a\x06\x02\u{e4}\u{e6}\x05\x50\x29\x02\u{e5}\u{e3}\
	\x03\x02\x02\x02\u{e5}\u{e4}\x03\x02\x02\x02\u{e6}\u{e9}\x03\x02\x02\x02\
	\u{e7}\u{e5}\x03\x02\x02\x02\u{e7}\u{e8}\x03\x02\x02\x02\u{e8}\u{ea}\x03\
	\x02\x02\x02\u{e9}\u{e7}\x03\x02\x02\x02\u{ea}\u{ed}\x07\x2e\x02\x02\u{eb}\
	\u{ed}\x07\x33\x02\x02\u{ec}\u{e2}\x03\x02\x02\x02\u{ec}\u{eb}\x03\x02\x02\
	\x02\u{ed}\x1f\x03\x02\x02\x02\u{ee}\u{f1}\x07\x16\x02\x02\u{ef}\u{f2}\x05\
	\x22\x12\x02\u{f0}\u{f2}\x05\x26\x14\x02\u{f1}\u{ef}\x03\x02\x02\x02\u{f1}\
	\u{f0}\x03\x02\x02\x02\u{f2}\u{f3}\x03\x02\x02\x02\u{f3}\u{f4}\x07\x33\x02\
	\x02\u{f4}\x21\x03\x02\x02\x02\u{f5}\u{fa}\x05\x24\x13\x02\u{f6}\u{f7}\x07\
	\x34\x02\x02\u{f7}\u{f9}\x05\x24\x13\x02\u{f8}\u{f6}\x03\x02\x02\x02\u{f9}\
	\u{fc}\x03\x02\x02\x02\u{fa}\u{f8}\x03\x02\x02\x02\u{fa}\u{fb}\x03\x02\x02\
	\x02\u{fb}\x23\x03\x02\x02\x02\u{fc}\u{fa}\x03\x02\x02\x02\u{fd}\u{102}\
	\x07\x26\x02\x02\u{fe}\u{ff}\x07\x26\x02\x02\u{ff}\u{100}\x07\x21\x02\x02\
	\u{100}\u{102}\x07\x26\x02\x02\u{101}\u{fd}\x03\x02\x02\x02\u{101}\u{fe}\
	\x03\x02\x02\x02\u{102}\x25\x03\x02\x02\x02\u{103}\u{108}\x07\x29\x02\x02\
	\u{104}\u{105}\x07\x34\x02\x02\u{105}\u{107}\x07\x29\x02\x02\u{106}\u{104}\
	\x03\x02\x02\x02\u{107}\u{10a}\x03\x02\x02\x02\u{108}\u{106}\x03\x02\x02\
	\x02\u{108}\u{109}\x03\x02\x02\x02\u{109}\x27\x03\x02\x02\x02\u{10a}\u{108}\
	\x03\x02\x02\x02\u{10b}\u{10e}\x09\x04\x02\x02\u{10c}\u{10e}\x05\x4e\x28\
	\x02\u{10d}\u{10b}\x03\x02\x02\x02\u{10d}\u{10c}\x03\x02\x02\x02\u{10e}\
	\x29\x03\x02\x02\x02\u{10f}\u{110}\x07\x26\x02\x02\u{110}\x2b\x03\x02\x02\
	\x02\u{111}\u{113}\x07\x15\x02\x02\u{112}\u{111}\x03\x02\x02\x02\u{112}\
	\u{113}\x03\x02\x02\x02\u{113}\u{114}\x03\x02\x02\x02\u{114}\u{115}\x05\
	\x28\x15\x02\u{115}\u{116}\x05\x42\x22\x02\u{116}\u{117}\x07\x38\x02\x02\
	\u{117}\u{11c}\x05\x2a\x16\x02\u{118}\u{119}\x07\x2f\x02\x02\u{119}\u{11a}\
	\x05\x2e\x18\x02\u{11a}\u{11b}\x07\x30\x02\x02\u{11b}\u{11d}\x03\x02\x02\
	\x02\u{11c}\u{118}\x03\x02\x02\x02\u{11c}\u{11d}\x03\x02\x02\x02\u{11d}\
	\u{11e}\x03\x02\x02\x02\u{11e}\u{11f}\x07\x33\x02\x02\u{11f}\x2d\x03\x02\
	\x02\x02\u{120}\u{125}\x05\x30\x19\x02\u{121}\u{122}\x07\x34\x02\x02\u{122}\
	\u{124}\x05\x30\x19\x02\u{123}\u{121}\x03\x02\x02\x02\u{124}\u{127}\x03\
	\x02\x02\x02\u{125}\u{123}\x03\x02\x02\x02\u{125}\u{126}\x03\x02\x02\x02\
	\u{126}\x2f\x03\x02\x02\x02\u{127}\u{125}\x03\x02\x02\x02\u{128}\u{129}\
	\x05\x0c\x07\x02\u{129}\u{12a}\x07\x38\x02\x02\u{12a}\u{12b}\x05\x52\x2a\
	\x02\u{12b}\x31\x03\x02\x02\x02\u{12c}\u{12d}\x07\x0f\x02\x02\u{12d}\u{12e}\
	\x05\x44\x23\x02\u{12e}\u{133}\x07\x2d\x02\x02\u{12f}\u{132}\x05\x34\x1b\
	\x02\u{130}\u{132}\x05\x50\x29\x02\u{131}\u{12f}\x03\x02\x02\x02\u{131}\
	\u{130}\x03\x02\x02\x02\u{132}\u{135}\x03\x02\x02\x02\u{133}\u{131}\x03\
	\x02\x02\x02\u{133}\u{134}\x03\x02\x02\x02\u{134}\u{136}\x03\x02\x02\x02\
	\u{135}\u{133}\x03\x02\x02\x02\u{136}\u{137}\x07\x2e\x02\x02\u{137}\x33\
	\x03\x02\x02\x02\u{138}\u{139}\x05\x28\x15\x02\u{139}\u{13a}\x05\x42\x22\
	\x02\u{13a}\u{13b}\x07\x38\x02\x02\u{13b}\u{140}\x05\x2a\x16\x02\u{13c}\
	\u{13d}\x07\x2f\x02\x02\u{13d}\u{13e}\x05\x2e\x18\x02\u{13e}\u{13f}\x07\
	\x30\x02\x02\u{13f}\u{141}\x03\x02\x02\x02\u{140}\u{13c}\x03\x02\x02\x02\
	\u{140}\u{141}\x03\x02\x02\x02\u{141}\u{142}\x03\x02\x02\x02\u{142}\u{143}\
	\x07\x33\x02\x02\u{143}\x35\x03\x02\x02\x02\u{144}\u{145}\x07\x0d\x02\x02\
	\u{145}\u{146}\x07\x31\x02\x02\u{146}\u{147}\x05\x38\x1d\x02\u{147}\u{148}\
	\x07\x34\x02\x02\u{148}\u{149}\x05\x28\x15\x02\u{149}\u{14a}\x07\x32\x02\
	\x02\u{14a}\u{14b}\x05\x46\x24\x02\u{14b}\u{14c}\x07\x38\x02\x02\u{14c}\
	\u{151}\x05\x2a\x16\x02\u{14d}\u{14e}\x07\x2f\x02\x02\u{14e}\u{14f}\x05\
	\x2e\x18\x02\u{14f}\u{150}\x07\x30\x02\x02\u{150}\u{152}\x03\x02\x02\x02\
	\u{151}\u{14d}\x03\x02\x02\x02\u{151}\u{152}\x03\x02\x02\x02\u{152}\u{153}\
	\x03\x02\x02\x02\u{153}\u{154}\x07\x33\x02\x02\u{154}\x37\x03\x02\x02\x02\
	\u{155}\u{156}\x09\x05\x02\x02\u{156}\x39\x03\x02\x02\x02\u{157}\u{15c}\
	\x07\x25\x02\x02\u{158}\u{159}\x07\x35\x02\x02\u{159}\u{15b}\x07\x25\x02\
	\x02\u{15a}\u{158}\x03\x02\x02\x02\u{15b}\u{15e}\x03\x02\x02\x02\u{15c}\
	\u{15a}\x03\x02\x02\x02\u{15c}\u{15d}\x03\x02\x02\x02\u{15d}\x3b\x03\x02\
	\x02\x02\u{15e}\u{15c}\x03\x02\x02\x02\u{15f}\u{160}\x07\x25\x02\x02\u{160}\
	\x3d\x03\x02\x02\x02\u{161}\u{162}\x07\x25\x02\x02\u{162}\x3f\x03\x02\x02\
	\x02\u{163}\u{164}\x07\x25\x02\x02\u{164}\x41\x03\x02\x02\x02\u{165}\u{166}\
	\x07\x25\x02\x02\u{166}\x43\x03\x02\x02\x02\u{167}\u{168}\x07\x25\x02\x02\
	\u{168}\x45\x03\x02\x02\x02\u{169}\u{16a}\x07\x25\x02\x02\u{16a}\x47\x03\
	\x02\x02\x02\u{16b}\u{16c}\x07\x25\x02\x02\u{16c}\x49\x03\x02\x02\x02\u{16d}\
	\u{16e}\x07\x25\x02\x02\u{16e}\x4b\x03\x02\x02\x02\u{16f}\u{171}\x07\x35\
	\x02\x02\u{170}\u{16f}\x03\x02\x02\x02\u{170}\u{171}\x03\x02\x02\x02\u{171}\
	\u{176}\x03\x02\x02\x02\u{172}\u{173}\x07\x25\x02\x02\u{173}\u{175}\x07\
	\x35\x02\x02\u{174}\u{172}\x03\x02\x02\x02\u{175}\u{178}\x03\x02\x02\x02\
	\u{176}\u{174}\x03\x02\x02\x02\u{176}\u{177}\x03\x02\x02\x02\u{177}\u{179}\
	\x03\x02\x02\x02\u{178}\u{176}\x03\x02\x02\x02\u{179}\u{17a}\x05\x3c\x1f\
	\x02\u{17a}\x4d\x03\x02\x02\x02\u{17b}\u{17d}\x07\x35\x02\x02\u{17c}\u{17b}\
	\x03\x02\x02\x02\u{17c}\u{17d}\x03\x02\x02\x02\u{17d}\u{182}\x03\x02\x02\
	\x02\u{17e}\u{17f}\x07\x25\x02\x02\u{17f}\u{181}\x07\x35\x02\x02\u{180}\
	\u{17e}\x03\x02\x02\x02\u{181}\u{184}\x03\x02\x02\x02\u{182}\u{180}\x03\
	\x02\x02\x02\u{182}\u{183}\x03\x02\x02\x02\u{183}\u{185}\x03\x02\x02\x02\
	\u{184}\u{182}\x03\x02\x02\x02\u{185}\u{186}\x05\x40\x21\x02\u{186}\x4f\
	\x03\x02\x02\x02\u{187}\u{188}\x07\x33\x02\x02\u{188}\x51\x03\x02\x02\x02\
	\u{189}\u{194}\x05\x3a\x1e\x02\u{18a}\u{18c}\x09\x06\x02\x02\u{18b}\u{18a}\
	\x03\x02\x02\x02\u{18b}\u{18c}\x03\x02\x02\x02\u{18c}\u{18d}\x03\x02\x02\
	\x02\u{18d}\u{194}\x07\x26\x02\x02\u{18e}\u{190}\x09\x06\x02\x02\u{18f}\
	\u{18e}\x03\x02\x02\x02\u{18f}\u{190}\x03\x02\x02\x02\u{190}\u{191}\x03\
	\x02\x02\x02\u{191}\u{194}\x07\x27\x02\x02\u{192}\u{194}\x09\x07\x02\x02\
	\u{193}\u{189}\x03\x02\x02\x02\u{193}\u{18b}\x03\x02\x02\x02\u{193}\u{18f}\
	\x03\x02\x02\x02\u{193}\u{192}\x03\x02\x02\x02\u{194}\x53\x03\x02\x02\x02\
	\x2a\x5a\x5c\x68\x7c\u{82}\u{88}\u{97}\u{99}\u{a6}\u{a8}\u{b0}\u{b9}\u{be}\
	\u{cc}\u{ce}\u{d7}\u{de}\u{e5}\u{e7}\u{ec}\u{f1}\u{fa}\u{101}\u{108}\u{10d}\
	\u{112}\u{11c}\u{125}\u{131}\u{133}\u{140}\u{151}\u{15c}\u{170}\u{176}\u{17c}\
	\u{182}\u{18b}\u{18f}\u{193}";

