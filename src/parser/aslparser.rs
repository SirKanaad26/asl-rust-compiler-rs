// Generated from grammar/asl.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
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
use super::asllistener::*;
use super::aslvisitor::*;

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

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const T__7:isize=8; 
		pub const T__8:isize=9; 
		pub const T__9:isize=10; 
		pub const T__10:isize=11; 
		pub const T__11:isize=12; 
		pub const T__12:isize=13; 
		pub const T__13:isize=14; 
		pub const T__14:isize=15; 
		pub const T__15:isize=16; 
		pub const T__16:isize=17; 
		pub const T__17:isize=18; 
		pub const T__18:isize=19; 
		pub const T__19:isize=20; 
		pub const T__20:isize=21; 
		pub const T__21:isize=22; 
		pub const T__22:isize=23; 
		pub const T__23:isize=24; 
		pub const T__24:isize=25; 
		pub const T__25:isize=26; 
		pub const T__26:isize=27; 
		pub const T__27:isize=28; 
		pub const T__28:isize=29; 
		pub const T__29:isize=30; 
		pub const T__30:isize=31; 
		pub const T__31:isize=32; 
		pub const T__32:isize=33; 
		pub const T__33:isize=34; 
		pub const T__34:isize=35; 
		pub const T__35:isize=36; 
		pub const T__36:isize=37; 
		pub const T__37:isize=38; 
		pub const T__38:isize=39; 
		pub const T__39:isize=40; 
		pub const T__40:isize=41; 
		pub const T__41:isize=42; 
		pub const T__42:isize=43; 
		pub const T__43:isize=44; 
		pub const T__44:isize=45; 
		pub const T__45:isize=46; 
		pub const T__46:isize=47; 
		pub const T__47:isize=48; 
		pub const T__48:isize=49; 
		pub const T__49:isize=50; 
		pub const T__50:isize=51; 
		pub const T__51:isize=52; 
		pub const T__52:isize=53; 
		pub const T__53:isize=54; 
		pub const T__54:isize=55; 
		pub const T__55:isize=56; 
		pub const T__56:isize=57; 
		pub const T__57:isize=58; 
		pub const T__58:isize=59; 
		pub const T__59:isize=60; 
		pub const T__60:isize=61; 
		pub const T__61:isize=62; 
		pub const T__62:isize=63; 
		pub const T__63:isize=64; 
		pub const T__64:isize=65; 
		pub const T__65:isize=66; 
		pub const T__66:isize=67; 
		pub const T__67:isize=68; 
		pub const T__68:isize=69; 
		pub const T__69:isize=70; 
		pub const T__70:isize=71; 
		pub const T__71:isize=72; 
		pub const T__72:isize=73; 
		pub const T__73:isize=74; 
		pub const T__74:isize=75; 
		pub const T__75:isize=76; 
		pub const T__76:isize=77; 
		pub const T__77:isize=78; 
		pub const T__78:isize=79; 
		pub const T__79:isize=80; 
		pub const T__80:isize=81; 
		pub const T__81:isize=82; 
		pub const T__82:isize=83; 
		pub const T__83:isize=84; 
		pub const T__84:isize=85; 
		pub const T__85:isize=86; 
		pub const T__86:isize=87; 
		pub const T__87:isize=88; 
		pub const T__88:isize=89; 
		pub const T__89:isize=90; 
		pub const SEMICOLON:isize=91; 
		pub const IDENTIFIER:isize=92; 
		pub const NAT_LIT:isize=93; 
		pub const HEX_LIT:isize=94; 
		pub const BIN_LIT:isize=95; 
		pub const MASK_LIT:isize=96; 
		pub const REAL_LIT:isize=97; 
		pub const STRING_LIT:isize=98; 
		pub const SEE_TOK:isize=99; 
		pub const COMMENT:isize=100; 
		pub const LINE_COMMENT:isize=101; 
		pub const ENDLINE:isize=102; 
		pub const WS:isize=103; 
		pub const INDENT:isize=104; 
		pub const DEDENT:isize=105;
	pub const RULE_registers:usize = 0; 
	pub const RULE_registerDefinition:usize = 1; 
	pub const RULE_register:usize = 2; 
	pub const RULE_arrayRegister:usize = 3; 
	pub const RULE_registerField:usize = 4; 
	pub const RULE_registerFieldCommaList:usize = 5; 
	pub const RULE_instructions:usize = 6; 
	pub const RULE_instruction:usize = 7; 
	pub const RULE_encoding:usize = 8; 
	pub const RULE_instructionField:usize = 9; 
	pub const RULE_instrUnpredictableUnless:usize = 10; 
	pub const RULE_definitions:usize = 11; 
	pub const RULE_definition:usize = 12; 
	pub const RULE_setterArg:usize = 13; 
	pub const RULE_setterArgCommaList:usize = 14; 
	pub const RULE_returnType:usize = 15; 
	pub const RULE_typeSpec:usize = 16; 
	pub const RULE_ixType:usize = 17; 
	pub const RULE_regField:usize = 18; 
	pub const RULE_indentedBlock:usize = 19; 
	pub const RULE_blockOrEmbed0:usize = 20; 
	pub const RULE_blockOrEmbed1:usize = 21; 
	pub const RULE_stmt:usize = 22; 
	pub const RULE_inlineStmt:usize = 23; 
	pub const RULE_stmtElsIf:usize = 24; 
	pub const RULE_catchAlt:usize = 25; 
	pub const RULE_caseAlt:usize = 26; 
	pub const RULE_casePattern:usize = 27; 
	pub const RULE_lValExpr:usize = 28; 
	pub const RULE_expr:usize = 29; 
	pub const RULE_sliceExpr:usize = 30; 
	pub const RULE_slice:usize = 31; 
	pub const RULE_exprElsIf:usize = 32; 
	pub const RULE_setElement:usize = 33; 
	pub const RULE_set:usize = 34; 
	pub const RULE_sliceCommaList0:usize = 35; 
	pub const RULE_sliceCommaList1:usize = 36; 
	pub const RULE_exprCommaList1:usize = 37; 
	pub const RULE_exprCommaList0:usize = 38; 
	pub const RULE_symDecl:usize = 39; 
	pub const RULE_identifierCommaList0:usize = 40; 
	pub const RULE_identifierCommaList1:usize = 41; 
	pub const RULE_symDeclCommaList:usize = 42; 
	pub const RULE_qualId:usize = 43; 
	pub const RULE_idWithDots:usize = 44; 
	pub const RULE_id:usize = 45;
	pub const ruleNames: [&'static str; 46] =  [
		"registers", "registerDefinition", "register", "arrayRegister", "registerField", 
		"registerFieldCommaList", "instructions", "instruction", "encoding", "instructionField", 
		"instrUnpredictableUnless", "definitions", "definition", "setterArg", 
		"setterArgCommaList", "returnType", "typeSpec", "ixType", "regField", 
		"indentedBlock", "blockOrEmbed0", "blockOrEmbed1", "stmt", "inlineStmt", 
		"stmtElsIf", "catchAlt", "caseAlt", "casePattern", "lValExpr", "expr", 
		"sliceExpr", "slice", "exprElsIf", "setElement", "set", "sliceCommaList0", 
		"sliceCommaList1", "exprCommaList1", "exprCommaList0", "symDecl", "identifierCommaList0", 
		"identifierCommaList1", "symDeclCommaList", "qualId", "idWithDots", "id"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;91] = [
		None, Some("'__register'"), Some("'{'"), Some("'}'"), Some("'array'"), 
		Some("'['"), Some("'..'"), Some("']'"), Some("'of'"), Some("':'"), Some("','"), 
		Some("'__instruction'"), Some("'__postdecode'"), Some("'__execute'"), 
		Some("'__conditional'"), Some("'__encoding'"), Some("'__instruction_set'"), 
		Some("'A64'"), Some("'A32'"), Some("'T32'"), Some("'T16'"), Some("'__opcode'"), 
		Some("'__guard'"), Some("'__decode'"), Some("'__field'"), Some("'+:'"), 
		Some("'__unpredictable_unless'"), Some("'=='"), Some("'__builtin'"), Some("'type'"), 
		Some("'='"), Some("'is'"), Some("'('"), Some("')'"), Some("'enumeration'"), 
		Some("'constant'"), Some("'&'"), Some("'typeof'"), Some("'register'"), 
		Some("';'"), Some("'if'"), Some("'then'"), Some("'else'"), Some("'case'"), 
		Some("'for'"), Some("'to'"), Some("'downto'"), Some("'while'"), Some("'do'"), 
		Some("'try'"), Some("'catch'"), Some("'return'"), Some("'assert'"), Some("'UNPREDICTABLE'"), 
		Some("'IMPLEMENTATION_DEFINED'"), Some("'repeat'"), Some("'until'"), Some("'throw'"), 
		Some("'UNDEFINED'"), Some("'elsif'"), Some("'when'"), Some("'otherwise'"), 
		Some("'&&'"), Some("'-'"), Some("'.'"), Some("'<'"), Some("'>'"), Some("'!'"), 
		Some("'NOT'"), Some("'UNKNOWN'"), Some("'IN'"), Some("'^'"), Some("'*'"), 
		Some("'/'"), Some("'+'"), Some("'>>'"), Some("'<<'"), Some("'QUOT'"), 
		Some("'REM'"), Some("'DIV'"), Some("'MOD'"), Some("'OR'"), Some("'EOR'"), 
		Some("'AND'"), Some("'++'"), Some("'!='"), Some("'>='"), Some("'<='"), 
		Some("'||'"), Some("'AArch32'"), Some("'AArch64'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;106]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, Some("SEMICOLON"), Some("IDENTIFIER"), 
		Some("NAT_LIT"), Some("HEX_LIT"), Some("BIN_LIT"), Some("MASK_LIT"), Some("REAL_LIT"), 
		Some("STRING_LIT"), Some("SEE_TOK"), Some("COMMENT"), Some("LINE_COMMENT"), 
		Some("ENDLINE"), Some("WS"), Some("INDENT"), Some("DEDENT")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,aslParserExt<'input>, I, aslParserContextType , dyn aslListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type aslTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, aslParserContextType , dyn aslListener<'input> + 'a>;

/// Parser for asl grammar
pub struct aslParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				aslParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> aslParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> aslParser<'input, I, DefaultErrorStrategy<'input,aslParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for aslParser
pub trait aslParserContext<'input>:
	for<'x> Listenable<dyn aslListener<'input> + 'x > + 
	for<'x> Visitable<dyn aslVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=aslParserContextType>
{}

antlr_rust::coerce_from!{ 'input : aslParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn aslParserContext<'input> + 'input
where
    T: aslVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn aslVisitor<'input> + 'x))
    }
}

impl<'input> aslParserContext<'input> for TerminalNode<'input,aslParserContextType> {}
impl<'input> aslParserContext<'input> for ErrorNode<'input,aslParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn aslParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn aslListener<'input> + 'input }

pub struct aslParserContextType;
antlr_rust::tid!{aslParserContextType}

impl<'input> ParserNodeType<'input> for aslParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn aslParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct aslParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> aslParserExt<'input>{
}
antlr_rust::tid! { aslParserExt<'a> }

impl<'input> TokenAware<'input> for aslParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for aslParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for aslParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "asl.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn aslParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					28 => aslParser::<'input,I,_>::lValExpr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					29 => aslParser::<'input,I,_>::expr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					30 => aslParser::<'input,I,_>::sliceExpr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> aslParser<'input, I, DefaultErrorStrategy<'input,aslParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn lValExpr_sempred(_localctx: Option<&LValExprContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 10)
				}
				1=>{
					recog.precpred(None, 9)
				}
				2=>{
					recog.precpred(None, 8)
				}
				3=>{
					recog.precpred(None, 5)
				}
				4=>{
					recog.precpred(None, 4)
				}
			_ => true
		}
	}
	fn expr_sempred(_localctx: Option<&ExprContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				5=>{
					recog.precpred(None, 7)
				}
				6=>{
					recog.precpred(None, 6)
				}
				7=>{
					recog.precpred(None, 5)
				}
				8=>{
					recog.precpred(None, 4)
				}
				9=>{
					recog.precpred(None, 3)
				}
				10=>{
					recog.precpred(None, 2)
				}
				11=>{
					recog.precpred(None, 17)
				}
				12=>{
					recog.precpred(None, 13)
				}
				13=>{
					recog.precpred(None, 12)
				}
				14=>{
					recog.precpred(None, 11)
				}
				15=>{
					recog.precpred(None, 10)
				}
				16=>{
					recog.precpred(None, 9)
				}
				17=>{
					recog.precpred(None, 8)
				}
			_ => true
		}
	}
	fn sliceExpr_sempred(_localctx: Option<&SliceExprContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				18=>{
					recog.precpred(None, 5)
				}
				19=>{
					recog.precpred(None, 4)
				}
				20=>{
					recog.precpred(None, 3)
				}
				21=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
}
//------------------- registers ----------------
pub type RegistersContextAll<'input> = RegistersContext<'input>;


pub type RegistersContext<'input> = BaseParserRuleContext<'input,RegistersContextExt<'input>>;

#[derive(Clone)]
pub struct RegistersContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for RegistersContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for RegistersContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_registers(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_registers(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for RegistersContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_registers(self);
	}
}

impl<'input> CustomRuleContext<'input> for RegistersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_registers }
	//fn type_rule_index() -> usize where Self: Sized { RULE_registers }
}
antlr_rust::tid!{RegistersContextExt<'a>}

impl<'input> RegistersContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RegistersContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RegistersContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RegistersContextAttrs<'input>: aslParserContext<'input> + BorrowMut<RegistersContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn registerDefinition_all(&self) ->  Vec<Rc<RegisterDefinitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn registerDefinition(&self, i: usize) -> Option<Rc<RegisterDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RegistersContextAttrs<'input> for RegistersContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn registers(&mut self,)
	-> Result<Rc<RegistersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RegistersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_registers);
        let mut _localctx: Rc<RegistersContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(95);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__0 || _la==T__3 {
				{
				{
				/*InvokeRule registerDefinition*/
				recog.base.set_state(92);
				recog.registerDefinition()?;

				}
				}
				recog.base.set_state(97);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(98);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- registerDefinition ----------------
#[derive(Debug)]
pub enum RegisterDefinitionContextAll<'input>{
	RegDefArrayContext(RegDefArrayContext<'input>),
	RegDefBasicContext(RegDefBasicContext<'input>),
Error(RegisterDefinitionContext<'input>)
}
antlr_rust::tid!{RegisterDefinitionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for RegisterDefinitionContextAll<'input>{}

impl<'input> aslParserContext<'input> for RegisterDefinitionContextAll<'input>{}

impl<'input> Deref for RegisterDefinitionContextAll<'input>{
	type Target = dyn RegisterDefinitionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use RegisterDefinitionContextAll::*;
		match self{
			RegDefArrayContext(inner) => inner,
			RegDefBasicContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for RegisterDefinitionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for RegisterDefinitionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type RegisterDefinitionContext<'input> = BaseParserRuleContext<'input,RegisterDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct RegisterDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for RegisterDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for RegisterDefinitionContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for RegisterDefinitionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for RegisterDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_registerDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_registerDefinition }
}
antlr_rust::tid!{RegisterDefinitionContextExt<'a>}

impl<'input> RegisterDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RegisterDefinitionContextAll<'input>> {
		Rc::new(
		RegisterDefinitionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RegisterDefinitionContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait RegisterDefinitionContextAttrs<'input>: aslParserContext<'input> + BorrowMut<RegisterDefinitionContextExt<'input>>{


}

impl<'input> RegisterDefinitionContextAttrs<'input> for RegisterDefinitionContext<'input>{}

pub type RegDefArrayContext<'input> = BaseParserRuleContext<'input,RegDefArrayContextExt<'input>>;

pub trait RegDefArrayContextAttrs<'input>: aslParserContext<'input>{
	fn arrayRegister(&self) -> Option<Rc<ArrayRegisterContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> RegDefArrayContextAttrs<'input> for RegDefArrayContext<'input>{}

pub struct RegDefArrayContextExt<'input>{
	base:RegisterDefinitionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RegDefArrayContextExt<'a>}

impl<'input> aslParserContext<'input> for RegDefArrayContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for RegDefArrayContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_RegDefArray(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_RegDefArray(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for RegDefArrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_RegDefArray(self);
	}
}

impl<'input> CustomRuleContext<'input> for RegDefArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_registerDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_registerDefinition }
}

impl<'input> Borrow<RegisterDefinitionContextExt<'input>> for RegDefArrayContext<'input>{
	fn borrow(&self) -> &RegisterDefinitionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<RegisterDefinitionContextExt<'input>> for RegDefArrayContext<'input>{
	fn borrow_mut(&mut self) -> &mut RegisterDefinitionContextExt<'input> { &mut self.base }
}

impl<'input> RegisterDefinitionContextAttrs<'input> for RegDefArrayContext<'input> {}

impl<'input> RegDefArrayContextExt<'input>{
	fn new(ctx: &dyn RegisterDefinitionContextAttrs<'input>) -> Rc<RegisterDefinitionContextAll<'input>>  {
		Rc::new(
			RegisterDefinitionContextAll::RegDefArrayContext(
				BaseParserRuleContext::copy_from(ctx,RegDefArrayContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RegDefBasicContext<'input> = BaseParserRuleContext<'input,RegDefBasicContextExt<'input>>;

pub trait RegDefBasicContextAttrs<'input>: aslParserContext<'input>{
	fn register(&self) -> Option<Rc<RegisterContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> RegDefBasicContextAttrs<'input> for RegDefBasicContext<'input>{}

pub struct RegDefBasicContextExt<'input>{
	base:RegisterDefinitionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RegDefBasicContextExt<'a>}

impl<'input> aslParserContext<'input> for RegDefBasicContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for RegDefBasicContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_RegDefBasic(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_RegDefBasic(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for RegDefBasicContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_RegDefBasic(self);
	}
}

impl<'input> CustomRuleContext<'input> for RegDefBasicContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_registerDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_registerDefinition }
}

impl<'input> Borrow<RegisterDefinitionContextExt<'input>> for RegDefBasicContext<'input>{
	fn borrow(&self) -> &RegisterDefinitionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<RegisterDefinitionContextExt<'input>> for RegDefBasicContext<'input>{
	fn borrow_mut(&mut self) -> &mut RegisterDefinitionContextExt<'input> { &mut self.base }
}

impl<'input> RegisterDefinitionContextAttrs<'input> for RegDefBasicContext<'input> {}

impl<'input> RegDefBasicContextExt<'input>{
	fn new(ctx: &dyn RegisterDefinitionContextAttrs<'input>) -> Rc<RegisterDefinitionContextAll<'input>>  {
		Rc::new(
			RegisterDefinitionContextAll::RegDefBasicContext(
				BaseParserRuleContext::copy_from(ctx,RegDefBasicContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn registerDefinition(&mut self,)
	-> Result<Rc<RegisterDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RegisterDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_registerDefinition);
        let mut _localctx: Rc<RegisterDefinitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(102);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__3 
				=> {
					let tmp = RegDefArrayContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule arrayRegister*/
					recog.base.set_state(100);
					recog.arrayRegister()?;

					}
				}

			 T__0 
				=> {
					let tmp = RegDefBasicContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule register*/
					recog.base.set_state(101);
					recog.register()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
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
//------------------- register ----------------
pub type RegisterContextAll<'input> = RegisterContext<'input>;


pub type RegisterContext<'input> = BaseParserRuleContext<'input,RegisterContextExt<'input>>;

#[derive(Clone)]
pub struct RegisterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for RegisterContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for RegisterContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_register(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_register(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for RegisterContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_register(self);
	}
}

impl<'input> CustomRuleContext<'input> for RegisterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_register }
	//fn type_rule_index() -> usize where Self: Sized { RULE_register }
}
antlr_rust::tid!{RegisterContextExt<'a>}

impl<'input> RegisterContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RegisterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RegisterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RegisterContextAttrs<'input>: aslParserContext<'input> + BorrowMut<RegisterContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NAT_LIT
/// Returns `None` if there is no child corresponding to token NAT_LIT
fn NAT_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(NAT_LIT, 0)
}
fn registerFieldCommaList(&self) -> Option<Rc<RegisterFieldCommaListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> RegisterContextAttrs<'input> for RegisterContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn register(&mut self,)
	-> Result<Rc<RegisterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RegisterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_register);
        let mut _localctx: Rc<RegisterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(104);
			recog.base.match_token(T__0,&mut recog.err_handler)?;

			recog.base.set_state(105);
			recog.base.match_token(NAT_LIT,&mut recog.err_handler)?;

			recog.base.set_state(106);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule registerFieldCommaList*/
			recog.base.set_state(107);
			recog.registerFieldCommaList()?;

			recog.base.set_state(108);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			/*InvokeRule id*/
			recog.base.set_state(109);
			recog.id()?;

			recog.base.set_state(110);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- arrayRegister ----------------
pub type ArrayRegisterContextAll<'input> = ArrayRegisterContext<'input>;


pub type ArrayRegisterContext<'input> = BaseParserRuleContext<'input,ArrayRegisterContextExt<'input>>;

#[derive(Clone)]
pub struct ArrayRegisterContextExt<'input>{
	pub lo: Option<TokenType<'input>>,
	pub hi: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for ArrayRegisterContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ArrayRegisterContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_arrayRegister(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_arrayRegister(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ArrayRegisterContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_arrayRegister(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrayRegisterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arrayRegister }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arrayRegister }
}
antlr_rust::tid!{ArrayRegisterContextExt<'a>}

impl<'input> ArrayRegisterContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArrayRegisterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArrayRegisterContextExt{
				lo: None, hi: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ArrayRegisterContextAttrs<'input>: aslParserContext<'input> + BorrowMut<ArrayRegisterContextExt<'input>>{

fn register(&self) -> Option<Rc<RegisterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token NAT_LIT in current rule
fn NAT_LIT_all(&self) -> Vec<Rc<TerminalNode<'input,aslParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NAT_LIT, starting from 0.
/// Returns `None` if number of children corresponding to token NAT_LIT is less or equal than `i`.
fn NAT_LIT(&self, i: usize) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(NAT_LIT, i)
}

}

impl<'input> ArrayRegisterContextAttrs<'input> for ArrayRegisterContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn arrayRegister(&mut self,)
	-> Result<Rc<ArrayRegisterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArrayRegisterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_arrayRegister);
        let mut _localctx: Rc<ArrayRegisterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(112);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			recog.base.set_state(113);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

			recog.base.set_state(114);
			let tmp = recog.base.match_token(NAT_LIT,&mut recog.err_handler)?;
			 cast_mut::<_,ArrayRegisterContext >(&mut _localctx).lo = Some(tmp.clone());
			  

			recog.base.set_state(115);
			recog.base.match_token(T__5,&mut recog.err_handler)?;

			recog.base.set_state(116);
			let tmp = recog.base.match_token(NAT_LIT,&mut recog.err_handler)?;
			 cast_mut::<_,ArrayRegisterContext >(&mut _localctx).hi = Some(tmp.clone());
			  

			recog.base.set_state(117);
			recog.base.match_token(T__6,&mut recog.err_handler)?;

			recog.base.set_state(118);
			recog.base.match_token(T__7,&mut recog.err_handler)?;

			/*InvokeRule register*/
			recog.base.set_state(119);
			recog.register()?;

			}
			Ok(())
		})();
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
//------------------- registerField ----------------
pub type RegisterFieldContextAll<'input> = RegisterFieldContext<'input>;


pub type RegisterFieldContext<'input> = BaseParserRuleContext<'input,RegisterFieldContextExt<'input>>;

#[derive(Clone)]
pub struct RegisterFieldContextExt<'input>{
	pub hi: Option<TokenType<'input>>,
	pub lo: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for RegisterFieldContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for RegisterFieldContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_registerField(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_registerField(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for RegisterFieldContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_registerField(self);
	}
}

impl<'input> CustomRuleContext<'input> for RegisterFieldContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_registerField }
	//fn type_rule_index() -> usize where Self: Sized { RULE_registerField }
}
antlr_rust::tid!{RegisterFieldContextExt<'a>}

impl<'input> RegisterFieldContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RegisterFieldContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RegisterFieldContextExt{
				hi: None, lo: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait RegisterFieldContextAttrs<'input>: aslParserContext<'input> + BorrowMut<RegisterFieldContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token NAT_LIT in current rule
fn NAT_LIT_all(&self) -> Vec<Rc<TerminalNode<'input,aslParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NAT_LIT, starting from 0.
/// Returns `None` if number of children corresponding to token NAT_LIT is less or equal than `i`.
fn NAT_LIT(&self, i: usize) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(NAT_LIT, i)
}
fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RegisterFieldContextAttrs<'input> for RegisterFieldContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn registerField(&mut self,)
	-> Result<Rc<RegisterFieldContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RegisterFieldContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_registerField);
        let mut _localctx: Rc<RegisterFieldContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(121);
			let tmp = recog.base.match_token(NAT_LIT,&mut recog.err_handler)?;
			 cast_mut::<_,RegisterFieldContext >(&mut _localctx).hi = Some(tmp.clone());
			  

			recog.base.set_state(122);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			recog.base.set_state(123);
			let tmp = recog.base.match_token(NAT_LIT,&mut recog.err_handler)?;
			 cast_mut::<_,RegisterFieldContext >(&mut _localctx).lo = Some(tmp.clone());
			  

			recog.base.set_state(125);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__33 || _la==T__37 || _la==T__67 || _la==IDENTIFIER {
				{
				/*InvokeRule id*/
				recog.base.set_state(124);
				recog.id()?;

				}
			}

			}
			Ok(())
		})();
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
//------------------- registerFieldCommaList ----------------
pub type RegisterFieldCommaListContextAll<'input> = RegisterFieldCommaListContext<'input>;


pub type RegisterFieldCommaListContext<'input> = BaseParserRuleContext<'input,RegisterFieldCommaListContextExt<'input>>;

#[derive(Clone)]
pub struct RegisterFieldCommaListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for RegisterFieldCommaListContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for RegisterFieldCommaListContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_registerFieldCommaList(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_registerFieldCommaList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for RegisterFieldCommaListContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_registerFieldCommaList(self);
	}
}

impl<'input> CustomRuleContext<'input> for RegisterFieldCommaListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_registerFieldCommaList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_registerFieldCommaList }
}
antlr_rust::tid!{RegisterFieldCommaListContextExt<'a>}

impl<'input> RegisterFieldCommaListContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RegisterFieldCommaListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RegisterFieldCommaListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RegisterFieldCommaListContextAttrs<'input>: aslParserContext<'input> + BorrowMut<RegisterFieldCommaListContextExt<'input>>{

fn registerField_all(&self) ->  Vec<Rc<RegisterFieldContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn registerField(&self, i: usize) -> Option<Rc<RegisterFieldContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RegisterFieldCommaListContextAttrs<'input> for RegisterFieldCommaListContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn registerFieldCommaList(&mut self,)
	-> Result<Rc<RegisterFieldCommaListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RegisterFieldCommaListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_registerFieldCommaList);
        let mut _localctx: Rc<RegisterFieldCommaListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(135);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==NAT_LIT {
				{
				/*InvokeRule registerField*/
				recog.base.set_state(127);
				recog.registerField()?;

				recog.base.set_state(132);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__9 {
					{
					{
					recog.base.set_state(128);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule registerField*/
					recog.base.set_state(129);
					recog.registerField()?;

					}
					}
					recog.base.set_state(134);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			}
			Ok(())
		})();
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
//------------------- instructions ----------------
pub type InstructionsContextAll<'input> = InstructionsContext<'input>;


pub type InstructionsContext<'input> = BaseParserRuleContext<'input,InstructionsContextExt<'input>>;

#[derive(Clone)]
pub struct InstructionsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for InstructionsContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for InstructionsContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_instructions(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_instructions(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for InstructionsContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_instructions(self);
	}
}

impl<'input> CustomRuleContext<'input> for InstructionsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_instructions }
	//fn type_rule_index() -> usize where Self: Sized { RULE_instructions }
}
antlr_rust::tid!{InstructionsContextExt<'a>}

impl<'input> InstructionsContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InstructionsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InstructionsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InstructionsContextAttrs<'input>: aslParserContext<'input> + BorrowMut<InstructionsContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn instruction_all(&self) ->  Vec<Rc<InstructionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn instruction(&self, i: usize) -> Option<Rc<InstructionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> InstructionsContextAttrs<'input> for InstructionsContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn instructions(&mut self,)
	-> Result<Rc<InstructionsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InstructionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_instructions);
        let mut _localctx: Rc<InstructionsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(140);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__10 {
				{
				{
				/*InvokeRule instruction*/
				recog.base.set_state(137);
				recog.instruction()?;

				}
				}
				recog.base.set_state(142);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(143);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- instruction ----------------
pub type InstructionContextAll<'input> = InstructionContext<'input>;


pub type InstructionContext<'input> = BaseParserRuleContext<'input,InstructionContextExt<'input>>;

#[derive(Clone)]
pub struct InstructionContextExt<'input>{
	pub postDecodeBlock: Option<Rc<IndentedBlockContextAll<'input>>>,
	pub conditional: Option<TokenType<'input>>,
	pub executeBlock: Option<Rc<IndentedBlockContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for InstructionContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for InstructionContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_instruction(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_instruction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for InstructionContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_instruction(self);
	}
}

impl<'input> CustomRuleContext<'input> for InstructionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_instruction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_instruction }
}
antlr_rust::tid!{InstructionContextExt<'a>}

impl<'input> InstructionContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InstructionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InstructionContextExt{
				conditional: None, 
				postDecodeBlock: None, executeBlock: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait InstructionContextAttrs<'input>: aslParserContext<'input> + BorrowMut<InstructionContextExt<'input>>{

fn idWithDots(&self) -> Option<Rc<IdWithDotsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token INDENT
/// Returns `None` if there is no child corresponding to token INDENT
fn INDENT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(INDENT, 0)
}
/// Retrieves first TerminalNode corresponding to token DEDENT
/// Returns `None` if there is no child corresponding to token DEDENT
fn DEDENT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(DEDENT, 0)
}
fn encoding_all(&self) ->  Vec<Rc<EncodingContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn encoding(&self, i: usize) -> Option<Rc<EncodingContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn indentedBlock_all(&self) ->  Vec<Rc<IndentedBlockContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn indentedBlock(&self, i: usize) -> Option<Rc<IndentedBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> InstructionContextAttrs<'input> for InstructionContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn instruction(&mut self,)
	-> Result<Rc<InstructionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InstructionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_instruction);
        let mut _localctx: Rc<InstructionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(145);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			/*InvokeRule idWithDots*/
			recog.base.set_state(146);
			recog.idWithDots()?;

			recog.base.set_state(147);
			recog.base.match_token(INDENT,&mut recog.err_handler)?;

			recog.base.set_state(149); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule encoding*/
				recog.base.set_state(148);
				recog.encoding()?;

				}
				}
				recog.base.set_state(151); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==T__14) {break}
			}
			recog.base.set_state(157);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__11 {
				{
				recog.base.set_state(153);
				recog.base.match_token(T__11,&mut recog.err_handler)?;

				recog.base.set_state(155);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if ((((_la - 4)) & !0x3f) == 0 && ((1usize << (_la - 4)) & ((1usize << (T__3 - 4)) | (1usize << (T__4 - 4)) | (1usize << (T__31 - 4)) | (1usize << (T__33 - 4)) | (1usize << (T__34 - 4)))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (T__36 - 37)) | (1usize << (T__37 - 37)) | (1usize << (T__39 - 37)) | (1usize << (T__42 - 37)) | (1usize << (T__43 - 37)) | (1usize << (T__46 - 37)) | (1usize << (T__48 - 37)) | (1usize << (T__50 - 37)) | (1usize << (T__51 - 37)) | (1usize << (T__52 - 37)) | (1usize << (T__53 - 37)) | (1usize << (T__54 - 37)) | (1usize << (T__56 - 37)) | (1usize << (T__57 - 37)) | (1usize << (T__62 - 37)) | (1usize << (T__64 - 37)) | (1usize << (T__67 - 37)))) != 0) || ((((_la - 89)) & !0x3f) == 0 && ((1usize << (_la - 89)) & ((1usize << (T__88 - 89)) | (1usize << (T__89 - 89)) | (1usize << (IDENTIFIER - 89)) | (1usize << (SEE_TOK - 89)))) != 0) {
					{
					/*InvokeRule indentedBlock*/
					recog.base.set_state(154);
					let tmp = recog.indentedBlock()?;
					 cast_mut::<_,InstructionContext >(&mut _localctx).postDecodeBlock = Some(tmp.clone());
					  

					}
				}

				}
			}

			recog.base.set_state(159);
			recog.base.match_token(T__12,&mut recog.err_handler)?;

			recog.base.set_state(161);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__13 {
				{
				recog.base.set_state(160);
				let tmp = recog.base.match_token(T__13,&mut recog.err_handler)?;
				 cast_mut::<_,InstructionContext >(&mut _localctx).conditional = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(164);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 4)) & !0x3f) == 0 && ((1usize << (_la - 4)) & ((1usize << (T__3 - 4)) | (1usize << (T__4 - 4)) | (1usize << (T__31 - 4)) | (1usize << (T__33 - 4)) | (1usize << (T__34 - 4)))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (T__36 - 37)) | (1usize << (T__37 - 37)) | (1usize << (T__39 - 37)) | (1usize << (T__42 - 37)) | (1usize << (T__43 - 37)) | (1usize << (T__46 - 37)) | (1usize << (T__48 - 37)) | (1usize << (T__50 - 37)) | (1usize << (T__51 - 37)) | (1usize << (T__52 - 37)) | (1usize << (T__53 - 37)) | (1usize << (T__54 - 37)) | (1usize << (T__56 - 37)) | (1usize << (T__57 - 37)) | (1usize << (T__62 - 37)) | (1usize << (T__64 - 37)) | (1usize << (T__67 - 37)))) != 0) || ((((_la - 89)) & !0x3f) == 0 && ((1usize << (_la - 89)) & ((1usize << (T__88 - 89)) | (1usize << (T__89 - 89)) | (1usize << (IDENTIFIER - 89)) | (1usize << (SEE_TOK - 89)))) != 0) {
				{
				/*InvokeRule indentedBlock*/
				recog.base.set_state(163);
				let tmp = recog.indentedBlock()?;
				 cast_mut::<_,InstructionContext >(&mut _localctx).executeBlock = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(166);
			recog.base.match_token(DEDENT,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- encoding ----------------
pub type EncodingContextAll<'input> = EncodingContext<'input>;


pub type EncodingContext<'input> = BaseParserRuleContext<'input,EncodingContextExt<'input>>;

#[derive(Clone)]
pub struct EncodingContextExt<'input>{
	pub instructionSet: Option<TokenType<'input>>,
	pub opcode: Option<TokenType<'input>>,
	pub decode: Option<Rc<IndentedBlockContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for EncodingContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for EncodingContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_encoding(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_encoding(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for EncodingContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_encoding(self);
	}
}

impl<'input> CustomRuleContext<'input> for EncodingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_encoding }
	//fn type_rule_index() -> usize where Self: Sized { RULE_encoding }
}
antlr_rust::tid!{EncodingContextExt<'a>}

impl<'input> EncodingContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EncodingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EncodingContextExt{
				instructionSet: None, opcode: None, 
				decode: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait EncodingContextAttrs<'input>: aslParserContext<'input> + BorrowMut<EncodingContextExt<'input>>{

fn idWithDots(&self) -> Option<Rc<IdWithDotsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token INDENT
/// Returns `None` if there is no child corresponding to token INDENT
fn INDENT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(INDENT, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DEDENT
/// Returns `None` if there is no child corresponding to token DEDENT
fn DEDENT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(DEDENT, 0)
}
/// Retrieves first TerminalNode corresponding to token MASK_LIT
/// Returns `None` if there is no child corresponding to token MASK_LIT
fn MASK_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(MASK_LIT, 0)
}
/// Retrieves first TerminalNode corresponding to token BIN_LIT
/// Returns `None` if there is no child corresponding to token BIN_LIT
fn BIN_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(BIN_LIT, 0)
}
fn instructionField_all(&self) ->  Vec<Rc<InstructionFieldContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn instructionField(&self, i: usize) -> Option<Rc<InstructionFieldContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn instrUnpredictableUnless_all(&self) ->  Vec<Rc<InstrUnpredictableUnlessContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn instrUnpredictableUnless(&self, i: usize) -> Option<Rc<InstrUnpredictableUnlessContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn indentedBlock(&self) -> Option<Rc<IndentedBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EncodingContextAttrs<'input> for EncodingContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn encoding(&mut self,)
	-> Result<Rc<EncodingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EncodingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_encoding);
        let mut _localctx: Rc<EncodingContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(168);
			recog.base.match_token(T__14,&mut recog.err_handler)?;

			/*InvokeRule idWithDots*/
			recog.base.set_state(169);
			recog.idWithDots()?;

			recog.base.set_state(170);
			recog.base.match_token(INDENT,&mut recog.err_handler)?;

			recog.base.set_state(171);
			recog.base.match_token(T__15,&mut recog.err_handler)?;

			recog.base.set_state(172);
			 cast_mut::<_,EncodingContext >(&mut _localctx).instructionSet = recog.base.input.lt(1).cloned();
			 
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__16) | (1usize << T__17) | (1usize << T__18) | (1usize << T__19))) != 0)) } {
				let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
				 cast_mut::<_,EncodingContext >(&mut _localctx).instructionSet = Some(tmp.clone());
				  

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(176);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__23 {
				{
				{
				/*InvokeRule instructionField*/
				recog.base.set_state(173);
				recog.instructionField()?;

				}
				}
				recog.base.set_state(178);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(179);
			recog.base.match_token(T__20,&mut recog.err_handler)?;

			recog.base.set_state(180);
			 cast_mut::<_,EncodingContext >(&mut _localctx).opcode = recog.base.input.lt(1).cloned();
			 
			_la = recog.base.input.la(1);
			if { !(_la==BIN_LIT || _la==MASK_LIT) } {
				let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
				 cast_mut::<_,EncodingContext >(&mut _localctx).opcode = Some(tmp.clone());
				  

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(181);
			recog.base.match_token(T__21,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(182);
			recog.expr_rec(0)?;

			recog.base.set_state(186);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				/*InvokeRule instrUnpredictableUnless*/
				recog.base.set_state(183);
				recog.instrUnpredictableUnless()?;

				}
				}
				recog.base.set_state(188);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(189);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			recog.base.set_state(191);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 4)) & !0x3f) == 0 && ((1usize << (_la - 4)) & ((1usize << (T__3 - 4)) | (1usize << (T__4 - 4)) | (1usize << (T__31 - 4)) | (1usize << (T__33 - 4)) | (1usize << (T__34 - 4)))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (T__36 - 37)) | (1usize << (T__37 - 37)) | (1usize << (T__39 - 37)) | (1usize << (T__42 - 37)) | (1usize << (T__43 - 37)) | (1usize << (T__46 - 37)) | (1usize << (T__48 - 37)) | (1usize << (T__50 - 37)) | (1usize << (T__51 - 37)) | (1usize << (T__52 - 37)) | (1usize << (T__53 - 37)) | (1usize << (T__54 - 37)) | (1usize << (T__56 - 37)) | (1usize << (T__57 - 37)) | (1usize << (T__62 - 37)) | (1usize << (T__64 - 37)) | (1usize << (T__67 - 37)))) != 0) || ((((_la - 89)) & !0x3f) == 0 && ((1usize << (_la - 89)) & ((1usize << (T__88 - 89)) | (1usize << (T__89 - 89)) | (1usize << (IDENTIFIER - 89)) | (1usize << (SEE_TOK - 89)))) != 0) {
				{
				/*InvokeRule indentedBlock*/
				recog.base.set_state(190);
				let tmp = recog.indentedBlock()?;
				 cast_mut::<_,EncodingContext >(&mut _localctx).decode = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(193);
			recog.base.match_token(DEDENT,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- instructionField ----------------
pub type InstructionFieldContextAll<'input> = InstructionFieldContext<'input>;


pub type InstructionFieldContext<'input> = BaseParserRuleContext<'input,InstructionFieldContextExt<'input>>;

#[derive(Clone)]
pub struct InstructionFieldContextExt<'input>{
	pub begin: Option<TokenType<'input>>,
	pub len: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for InstructionFieldContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for InstructionFieldContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_instructionField(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_instructionField(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for InstructionFieldContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_instructionField(self);
	}
}

impl<'input> CustomRuleContext<'input> for InstructionFieldContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_instructionField }
	//fn type_rule_index() -> usize where Self: Sized { RULE_instructionField }
}
antlr_rust::tid!{InstructionFieldContextExt<'a>}

impl<'input> InstructionFieldContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InstructionFieldContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InstructionFieldContextExt{
				begin: None, len: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait InstructionFieldContextAttrs<'input>: aslParserContext<'input> + BorrowMut<InstructionFieldContextExt<'input>>{

fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token NAT_LIT in current rule
fn NAT_LIT_all(&self) -> Vec<Rc<TerminalNode<'input,aslParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NAT_LIT, starting from 0.
/// Returns `None` if number of children corresponding to token NAT_LIT is less or equal than `i`.
fn NAT_LIT(&self, i: usize) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(NAT_LIT, i)
}

}

impl<'input> InstructionFieldContextAttrs<'input> for InstructionFieldContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn instructionField(&mut self,)
	-> Result<Rc<InstructionFieldContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InstructionFieldContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_instructionField);
        let mut _localctx: Rc<InstructionFieldContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(195);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			/*InvokeRule id*/
			recog.base.set_state(196);
			recog.id()?;

			recog.base.set_state(197);
			let tmp = recog.base.match_token(NAT_LIT,&mut recog.err_handler)?;
			 cast_mut::<_,InstructionFieldContext >(&mut _localctx).begin = Some(tmp.clone());
			  

			recog.base.set_state(198);
			recog.base.match_token(T__24,&mut recog.err_handler)?;

			recog.base.set_state(199);
			let tmp = recog.base.match_token(NAT_LIT,&mut recog.err_handler)?;
			 cast_mut::<_,InstructionFieldContext >(&mut _localctx).len = Some(tmp.clone());
			  

			}
			Ok(())
		})();
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
//------------------- instrUnpredictableUnless ----------------
pub type InstrUnpredictableUnlessContextAll<'input> = InstrUnpredictableUnlessContext<'input>;


pub type InstrUnpredictableUnlessContext<'input> = BaseParserRuleContext<'input,InstrUnpredictableUnlessContextExt<'input>>;

#[derive(Clone)]
pub struct InstrUnpredictableUnlessContextExt<'input>{
	pub idx: Option<TokenType<'input>>,
	pub bin: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for InstrUnpredictableUnlessContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for InstrUnpredictableUnlessContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_instrUnpredictableUnless(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_instrUnpredictableUnless(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for InstrUnpredictableUnlessContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_instrUnpredictableUnless(self);
	}
}

impl<'input> CustomRuleContext<'input> for InstrUnpredictableUnlessContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_instrUnpredictableUnless }
	//fn type_rule_index() -> usize where Self: Sized { RULE_instrUnpredictableUnless }
}
antlr_rust::tid!{InstrUnpredictableUnlessContextExt<'a>}

impl<'input> InstrUnpredictableUnlessContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InstrUnpredictableUnlessContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InstrUnpredictableUnlessContextExt{
				idx: None, bin: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait InstrUnpredictableUnlessContextAttrs<'input>: aslParserContext<'input> + BorrowMut<InstrUnpredictableUnlessContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NAT_LIT
/// Returns `None` if there is no child corresponding to token NAT_LIT
fn NAT_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(NAT_LIT, 0)
}
/// Retrieves first TerminalNode corresponding to token BIN_LIT
/// Returns `None` if there is no child corresponding to token BIN_LIT
fn BIN_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(BIN_LIT, 0)
}

}

impl<'input> InstrUnpredictableUnlessContextAttrs<'input> for InstrUnpredictableUnlessContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn instrUnpredictableUnless(&mut self,)
	-> Result<Rc<InstrUnpredictableUnlessContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InstrUnpredictableUnlessContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_instrUnpredictableUnless);
        let mut _localctx: Rc<InstrUnpredictableUnlessContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(201);
			recog.base.match_token(T__25,&mut recog.err_handler)?;

			recog.base.set_state(202);
			let tmp = recog.base.match_token(NAT_LIT,&mut recog.err_handler)?;
			 cast_mut::<_,InstrUnpredictableUnlessContext >(&mut _localctx).idx = Some(tmp.clone());
			  

			recog.base.set_state(203);
			recog.base.match_token(T__26,&mut recog.err_handler)?;

			recog.base.set_state(204);
			let tmp = recog.base.match_token(BIN_LIT,&mut recog.err_handler)?;
			 cast_mut::<_,InstrUnpredictableUnlessContext >(&mut _localctx).bin = Some(tmp.clone());
			  

			}
			Ok(())
		})();
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
//------------------- definitions ----------------
pub type DefinitionsContextAll<'input> = DefinitionsContext<'input>;


pub type DefinitionsContext<'input> = BaseParserRuleContext<'input,DefinitionsContextExt<'input>>;

#[derive(Clone)]
pub struct DefinitionsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for DefinitionsContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for DefinitionsContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_definitions(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_definitions(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for DefinitionsContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_definitions(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefinitionsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_definitions }
	//fn type_rule_index() -> usize where Self: Sized { RULE_definitions }
}
antlr_rust::tid!{DefinitionsContextExt<'a>}

impl<'input> DefinitionsContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DefinitionsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DefinitionsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DefinitionsContextAttrs<'input>: aslParserContext<'input> + BorrowMut<DefinitionsContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn definition_all(&self) ->  Vec<Rc<DefinitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn definition(&self, i: usize) -> Option<Rc<DefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DefinitionsContextAttrs<'input> for DefinitionsContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn definitions(&mut self,)
	-> Result<Rc<DefinitionsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DefinitionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_definitions);
        let mut _localctx: Rc<DefinitionsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(209);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__3) | (1usize << T__27) | (1usize << T__28))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (T__31 - 32)) | (1usize << (T__33 - 32)) | (1usize << (T__34 - 32)) | (1usize << (T__36 - 32)) | (1usize << (T__37 - 32)))) != 0) || ((((_la - 68)) & !0x3f) == 0 && ((1usize << (_la - 68)) & ((1usize << (T__67 - 68)) | (1usize << (T__88 - 68)) | (1usize << (T__89 - 68)) | (1usize << (IDENTIFIER - 68)))) != 0) {
				{
				{
				/*InvokeRule definition*/
				recog.base.set_state(206);
				recog.definition()?;

				}
				}
				recog.base.set_state(211);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(212);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- definition ----------------
#[derive(Debug)]
pub enum DefinitionContextAll<'input>{
	DefTypeEnumContext(DefTypeEnumContext<'input>),
	DefSetterContext(DefSetterContext<'input>),
	DefTypeAbstractContext(DefTypeAbstractContext<'input>),
	DefConstantContext(DefConstantContext<'input>),
	DefArrayContext(DefArrayContext<'input>),
	DefTypeAliasContext(DefTypeAliasContext<'input>),
	DefTypeBuiltinContext(DefTypeBuiltinContext<'input>),
	DefTypeStructContext(DefTypeStructContext<'input>),
	DefVariableContext(DefVariableContext<'input>),
	DefGetterContext(DefGetterContext<'input>),
	DefCallableContext(DefCallableContext<'input>),
Error(DefinitionContext<'input>)
}
antlr_rust::tid!{DefinitionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for DefinitionContextAll<'input>{}

impl<'input> aslParserContext<'input> for DefinitionContextAll<'input>{}

impl<'input> Deref for DefinitionContextAll<'input>{
	type Target = dyn DefinitionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use DefinitionContextAll::*;
		match self{
			DefTypeEnumContext(inner) => inner,
			DefSetterContext(inner) => inner,
			DefTypeAbstractContext(inner) => inner,
			DefConstantContext(inner) => inner,
			DefArrayContext(inner) => inner,
			DefTypeAliasContext(inner) => inner,
			DefTypeBuiltinContext(inner) => inner,
			DefTypeStructContext(inner) => inner,
			DefVariableContext(inner) => inner,
			DefGetterContext(inner) => inner,
			DefCallableContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for DefinitionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for DefinitionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type DefinitionContext<'input> = BaseParserRuleContext<'input,DefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct DefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for DefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for DefinitionContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for DefinitionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for DefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_definition }
}
antlr_rust::tid!{DefinitionContextExt<'a>}

impl<'input> DefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DefinitionContextAll<'input>> {
		Rc::new(
		DefinitionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DefinitionContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait DefinitionContextAttrs<'input>: aslParserContext<'input> + BorrowMut<DefinitionContextExt<'input>>{


}

impl<'input> DefinitionContextAttrs<'input> for DefinitionContext<'input>{}

pub type DefTypeEnumContext<'input> = BaseParserRuleContext<'input,DefTypeEnumContextExt<'input>>;

pub trait DefTypeEnumContextAttrs<'input>: aslParserContext<'input>{
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn identifierCommaList0(&self) -> Option<Rc<IdentifierCommaList0ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> DefTypeEnumContextAttrs<'input> for DefTypeEnumContext<'input>{}

pub struct DefTypeEnumContextExt<'input>{
	base:DefinitionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DefTypeEnumContextExt<'a>}

impl<'input> aslParserContext<'input> for DefTypeEnumContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for DefTypeEnumContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_DefTypeEnum(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_DefTypeEnum(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for DefTypeEnumContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_DefTypeEnum(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefTypeEnumContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_definition }
}

impl<'input> Borrow<DefinitionContextExt<'input>> for DefTypeEnumContext<'input>{
	fn borrow(&self) -> &DefinitionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DefinitionContextExt<'input>> for DefTypeEnumContext<'input>{
	fn borrow_mut(&mut self) -> &mut DefinitionContextExt<'input> { &mut self.base }
}

impl<'input> DefinitionContextAttrs<'input> for DefTypeEnumContext<'input> {}

impl<'input> DefTypeEnumContextExt<'input>{
	fn new(ctx: &dyn DefinitionContextAttrs<'input>) -> Rc<DefinitionContextAll<'input>>  {
		Rc::new(
			DefinitionContextAll::DefTypeEnumContext(
				BaseParserRuleContext::copy_from(ctx,DefTypeEnumContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DefSetterContext<'input> = BaseParserRuleContext<'input,DefSetterContextExt<'input>>;

pub trait DefSetterContextAttrs<'input>: aslParserContext<'input>{
	fn qualId(&self) -> Option<Rc<QualIdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn setterArgCommaList(&self) -> Option<Rc<SetterArgCommaListContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn symDecl(&self) -> Option<Rc<SymDeclContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn indentedBlock(&self) -> Option<Rc<IndentedBlockContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> DefSetterContextAttrs<'input> for DefSetterContext<'input>{}

pub struct DefSetterContextExt<'input>{
	base:DefinitionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DefSetterContextExt<'a>}

impl<'input> aslParserContext<'input> for DefSetterContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for DefSetterContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_DefSetter(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_DefSetter(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for DefSetterContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_DefSetter(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefSetterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_definition }
}

impl<'input> Borrow<DefinitionContextExt<'input>> for DefSetterContext<'input>{
	fn borrow(&self) -> &DefinitionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DefinitionContextExt<'input>> for DefSetterContext<'input>{
	fn borrow_mut(&mut self) -> &mut DefinitionContextExt<'input> { &mut self.base }
}

impl<'input> DefinitionContextAttrs<'input> for DefSetterContext<'input> {}

impl<'input> DefSetterContextExt<'input>{
	fn new(ctx: &dyn DefinitionContextAttrs<'input>) -> Rc<DefinitionContextAll<'input>>  {
		Rc::new(
			DefinitionContextAll::DefSetterContext(
				BaseParserRuleContext::copy_from(ctx,DefSetterContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DefTypeAbstractContext<'input> = BaseParserRuleContext<'input,DefTypeAbstractContextExt<'input>>;

pub trait DefTypeAbstractContextAttrs<'input>: aslParserContext<'input>{
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> DefTypeAbstractContextAttrs<'input> for DefTypeAbstractContext<'input>{}

pub struct DefTypeAbstractContextExt<'input>{
	base:DefinitionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DefTypeAbstractContextExt<'a>}

impl<'input> aslParserContext<'input> for DefTypeAbstractContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for DefTypeAbstractContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_DefTypeAbstract(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_DefTypeAbstract(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for DefTypeAbstractContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_DefTypeAbstract(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefTypeAbstractContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_definition }
}

impl<'input> Borrow<DefinitionContextExt<'input>> for DefTypeAbstractContext<'input>{
	fn borrow(&self) -> &DefinitionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DefinitionContextExt<'input>> for DefTypeAbstractContext<'input>{
	fn borrow_mut(&mut self) -> &mut DefinitionContextExt<'input> { &mut self.base }
}

impl<'input> DefinitionContextAttrs<'input> for DefTypeAbstractContext<'input> {}

impl<'input> DefTypeAbstractContextExt<'input>{
	fn new(ctx: &dyn DefinitionContextAttrs<'input>) -> Rc<DefinitionContextAll<'input>>  {
		Rc::new(
			DefinitionContextAll::DefTypeAbstractContext(
				BaseParserRuleContext::copy_from(ctx,DefTypeAbstractContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DefConstantContext<'input> = BaseParserRuleContext<'input,DefConstantContextExt<'input>>;

pub trait DefConstantContextAttrs<'input>: aslParserContext<'input>{
	fn typeSpec(&self) -> Option<Rc<TypeSpecContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> DefConstantContextAttrs<'input> for DefConstantContext<'input>{}

pub struct DefConstantContextExt<'input>{
	base:DefinitionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DefConstantContextExt<'a>}

impl<'input> aslParserContext<'input> for DefConstantContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for DefConstantContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_DefConstant(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_DefConstant(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for DefConstantContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_DefConstant(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefConstantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_definition }
}

impl<'input> Borrow<DefinitionContextExt<'input>> for DefConstantContext<'input>{
	fn borrow(&self) -> &DefinitionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DefinitionContextExt<'input>> for DefConstantContext<'input>{
	fn borrow_mut(&mut self) -> &mut DefinitionContextExt<'input> { &mut self.base }
}

impl<'input> DefinitionContextAttrs<'input> for DefConstantContext<'input> {}

impl<'input> DefConstantContextExt<'input>{
	fn new(ctx: &dyn DefinitionContextAttrs<'input>) -> Rc<DefinitionContextAll<'input>>  {
		Rc::new(
			DefinitionContextAll::DefConstantContext(
				BaseParserRuleContext::copy_from(ctx,DefConstantContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DefArrayContext<'input> = BaseParserRuleContext<'input,DefArrayContextExt<'input>>;

pub trait DefArrayContextAttrs<'input>: aslParserContext<'input>{
	fn typeSpec(&self) -> Option<Rc<TypeSpecContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn ixType(&self) -> Option<Rc<IxTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> DefArrayContextAttrs<'input> for DefArrayContext<'input>{}

pub struct DefArrayContextExt<'input>{
	base:DefinitionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DefArrayContextExt<'a>}

impl<'input> aslParserContext<'input> for DefArrayContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for DefArrayContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_DefArray(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_DefArray(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for DefArrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_DefArray(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_definition }
}

impl<'input> Borrow<DefinitionContextExt<'input>> for DefArrayContext<'input>{
	fn borrow(&self) -> &DefinitionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DefinitionContextExt<'input>> for DefArrayContext<'input>{
	fn borrow_mut(&mut self) -> &mut DefinitionContextExt<'input> { &mut self.base }
}

impl<'input> DefinitionContextAttrs<'input> for DefArrayContext<'input> {}

impl<'input> DefArrayContextExt<'input>{
	fn new(ctx: &dyn DefinitionContextAttrs<'input>) -> Rc<DefinitionContextAll<'input>>  {
		Rc::new(
			DefinitionContextAll::DefArrayContext(
				BaseParserRuleContext::copy_from(ctx,DefArrayContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DefTypeAliasContext<'input> = BaseParserRuleContext<'input,DefTypeAliasContextExt<'input>>;

pub trait DefTypeAliasContextAttrs<'input>: aslParserContext<'input>{
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn typeSpec(&self) -> Option<Rc<TypeSpecContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> DefTypeAliasContextAttrs<'input> for DefTypeAliasContext<'input>{}

pub struct DefTypeAliasContextExt<'input>{
	base:DefinitionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DefTypeAliasContextExt<'a>}

impl<'input> aslParserContext<'input> for DefTypeAliasContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for DefTypeAliasContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_DefTypeAlias(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_DefTypeAlias(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for DefTypeAliasContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_DefTypeAlias(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefTypeAliasContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_definition }
}

impl<'input> Borrow<DefinitionContextExt<'input>> for DefTypeAliasContext<'input>{
	fn borrow(&self) -> &DefinitionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DefinitionContextExt<'input>> for DefTypeAliasContext<'input>{
	fn borrow_mut(&mut self) -> &mut DefinitionContextExt<'input> { &mut self.base }
}

impl<'input> DefinitionContextAttrs<'input> for DefTypeAliasContext<'input> {}

impl<'input> DefTypeAliasContextExt<'input>{
	fn new(ctx: &dyn DefinitionContextAttrs<'input>) -> Rc<DefinitionContextAll<'input>>  {
		Rc::new(
			DefinitionContextAll::DefTypeAliasContext(
				BaseParserRuleContext::copy_from(ctx,DefTypeAliasContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DefTypeBuiltinContext<'input> = BaseParserRuleContext<'input,DefTypeBuiltinContextExt<'input>>;

pub trait DefTypeBuiltinContextAttrs<'input>: aslParserContext<'input>{
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> DefTypeBuiltinContextAttrs<'input> for DefTypeBuiltinContext<'input>{}

pub struct DefTypeBuiltinContextExt<'input>{
	base:DefinitionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DefTypeBuiltinContextExt<'a>}

impl<'input> aslParserContext<'input> for DefTypeBuiltinContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for DefTypeBuiltinContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_DefTypeBuiltin(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_DefTypeBuiltin(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for DefTypeBuiltinContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_DefTypeBuiltin(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefTypeBuiltinContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_definition }
}

impl<'input> Borrow<DefinitionContextExt<'input>> for DefTypeBuiltinContext<'input>{
	fn borrow(&self) -> &DefinitionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DefinitionContextExt<'input>> for DefTypeBuiltinContext<'input>{
	fn borrow_mut(&mut self) -> &mut DefinitionContextExt<'input> { &mut self.base }
}

impl<'input> DefinitionContextAttrs<'input> for DefTypeBuiltinContext<'input> {}

impl<'input> DefTypeBuiltinContextExt<'input>{
	fn new(ctx: &dyn DefinitionContextAttrs<'input>) -> Rc<DefinitionContextAll<'input>>  {
		Rc::new(
			DefinitionContextAll::DefTypeBuiltinContext(
				BaseParserRuleContext::copy_from(ctx,DefTypeBuiltinContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DefTypeStructContext<'input> = BaseParserRuleContext<'input,DefTypeStructContextExt<'input>>;

pub trait DefTypeStructContextAttrs<'input>: aslParserContext<'input>{
	fn qualId(&self) -> Option<Rc<QualIdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn symDeclCommaList(&self) -> Option<Rc<SymDeclCommaListContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> DefTypeStructContextAttrs<'input> for DefTypeStructContext<'input>{}

pub struct DefTypeStructContextExt<'input>{
	base:DefinitionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DefTypeStructContextExt<'a>}

impl<'input> aslParserContext<'input> for DefTypeStructContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for DefTypeStructContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_DefTypeStruct(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_DefTypeStruct(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for DefTypeStructContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_DefTypeStruct(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefTypeStructContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_definition }
}

impl<'input> Borrow<DefinitionContextExt<'input>> for DefTypeStructContext<'input>{
	fn borrow(&self) -> &DefinitionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DefinitionContextExt<'input>> for DefTypeStructContext<'input>{
	fn borrow_mut(&mut self) -> &mut DefinitionContextExt<'input> { &mut self.base }
}

impl<'input> DefinitionContextAttrs<'input> for DefTypeStructContext<'input> {}

impl<'input> DefTypeStructContextExt<'input>{
	fn new(ctx: &dyn DefinitionContextAttrs<'input>) -> Rc<DefinitionContextAll<'input>>  {
		Rc::new(
			DefinitionContextAll::DefTypeStructContext(
				BaseParserRuleContext::copy_from(ctx,DefTypeStructContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DefVariableContext<'input> = BaseParserRuleContext<'input,DefVariableContextExt<'input>>;

pub trait DefVariableContextAttrs<'input>: aslParserContext<'input>{
	fn typeSpec(&self) -> Option<Rc<TypeSpecContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn qualId(&self) -> Option<Rc<QualIdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> DefVariableContextAttrs<'input> for DefVariableContext<'input>{}

pub struct DefVariableContextExt<'input>{
	base:DefinitionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DefVariableContextExt<'a>}

impl<'input> aslParserContext<'input> for DefVariableContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for DefVariableContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_DefVariable(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_DefVariable(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for DefVariableContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_DefVariable(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefVariableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_definition }
}

impl<'input> Borrow<DefinitionContextExt<'input>> for DefVariableContext<'input>{
	fn borrow(&self) -> &DefinitionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DefinitionContextExt<'input>> for DefVariableContext<'input>{
	fn borrow_mut(&mut self) -> &mut DefinitionContextExt<'input> { &mut self.base }
}

impl<'input> DefinitionContextAttrs<'input> for DefVariableContext<'input> {}

impl<'input> DefVariableContextExt<'input>{
	fn new(ctx: &dyn DefinitionContextAttrs<'input>) -> Rc<DefinitionContextAll<'input>>  {
		Rc::new(
			DefinitionContextAll::DefVariableContext(
				BaseParserRuleContext::copy_from(ctx,DefVariableContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DefGetterContext<'input> = BaseParserRuleContext<'input,DefGetterContextExt<'input>>;

pub trait DefGetterContextAttrs<'input>: aslParserContext<'input>{
	fn returnType(&self) -> Option<Rc<ReturnTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn qualId(&self) -> Option<Rc<QualIdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn indentedBlock(&self) -> Option<Rc<IndentedBlockContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn symDeclCommaList(&self) -> Option<Rc<SymDeclCommaListContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> DefGetterContextAttrs<'input> for DefGetterContext<'input>{}

pub struct DefGetterContextExt<'input>{
	base:DefinitionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DefGetterContextExt<'a>}

impl<'input> aslParserContext<'input> for DefGetterContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for DefGetterContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_DefGetter(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_DefGetter(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for DefGetterContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_DefGetter(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefGetterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_definition }
}

impl<'input> Borrow<DefinitionContextExt<'input>> for DefGetterContext<'input>{
	fn borrow(&self) -> &DefinitionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DefinitionContextExt<'input>> for DefGetterContext<'input>{
	fn borrow_mut(&mut self) -> &mut DefinitionContextExt<'input> { &mut self.base }
}

impl<'input> DefinitionContextAttrs<'input> for DefGetterContext<'input> {}

impl<'input> DefGetterContextExt<'input>{
	fn new(ctx: &dyn DefinitionContextAttrs<'input>) -> Rc<DefinitionContextAll<'input>>  {
		Rc::new(
			DefinitionContextAll::DefGetterContext(
				BaseParserRuleContext::copy_from(ctx,DefGetterContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DefCallableContext<'input> = BaseParserRuleContext<'input,DefCallableContextExt<'input>>;

pub trait DefCallableContextAttrs<'input>: aslParserContext<'input>{
	fn qualId(&self) -> Option<Rc<QualIdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn symDeclCommaList(&self) -> Option<Rc<SymDeclCommaListContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn indentedBlock(&self) -> Option<Rc<IndentedBlockContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
	fn returnType(&self) -> Option<Rc<ReturnTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> DefCallableContextAttrs<'input> for DefCallableContext<'input>{}

pub struct DefCallableContextExt<'input>{
	base:DefinitionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DefCallableContextExt<'a>}

impl<'input> aslParserContext<'input> for DefCallableContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for DefCallableContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_DefCallable(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_DefCallable(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for DefCallableContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_DefCallable(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefCallableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_definition }
}

impl<'input> Borrow<DefinitionContextExt<'input>> for DefCallableContext<'input>{
	fn borrow(&self) -> &DefinitionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DefinitionContextExt<'input>> for DefCallableContext<'input>{
	fn borrow_mut(&mut self) -> &mut DefinitionContextExt<'input> { &mut self.base }
}

impl<'input> DefinitionContextAttrs<'input> for DefCallableContext<'input> {}

impl<'input> DefCallableContextExt<'input>{
	fn new(ctx: &dyn DefinitionContextAttrs<'input>) -> Rc<DefinitionContextAll<'input>>  {
		Rc::new(
			DefinitionContextAll::DefCallableContext(
				BaseParserRuleContext::copy_from(ctx,DefCallableContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn definition(&mut self,)
	-> Result<Rc<DefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_definition);
        let mut _localctx: Rc<DefinitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(303);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(20,&mut recog.base)? {
				1 =>{
					let tmp = DefTypeBuiltinContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(214);
					recog.base.match_token(T__27,&mut recog.err_handler)?;

					recog.base.set_state(215);
					recog.base.match_token(T__28,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(216);
					recog.id()?;

					recog.base.set_state(217);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					let tmp = DefTypeAbstractContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(219);
					recog.base.match_token(T__28,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(220);
					recog.id()?;

					recog.base.set_state(221);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					let tmp = DefTypeAliasContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(223);
					recog.base.match_token(T__28,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(224);
					recog.id()?;

					recog.base.set_state(225);
					recog.base.match_token(T__29,&mut recog.err_handler)?;

					/*InvokeRule typeSpec*/
					recog.base.set_state(226);
					recog.typeSpec()?;

					recog.base.set_state(227);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					let tmp = DefTypeStructContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(229);
					recog.base.match_token(T__28,&mut recog.err_handler)?;

					/*InvokeRule qualId*/
					recog.base.set_state(230);
					recog.qualId()?;

					recog.base.set_state(231);
					recog.base.match_token(T__30,&mut recog.err_handler)?;

					recog.base.set_state(232);
					recog.base.match_token(T__31,&mut recog.err_handler)?;

					/*InvokeRule symDeclCommaList*/
					recog.base.set_state(233);
					recog.symDeclCommaList()?;

					recog.base.set_state(234);
					recog.base.match_token(T__32,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					let tmp = DefTypeEnumContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(236);
					recog.base.match_token(T__33,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(237);
					recog.id()?;

					recog.base.set_state(238);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					/*InvokeRule identifierCommaList0*/
					recog.base.set_state(239);
					recog.identifierCommaList0()?;

					recog.base.set_state(240);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					recog.base.set_state(241);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					let tmp = DefVariableContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6);
					_localctx = tmp;
					{
					/*InvokeRule typeSpec*/
					recog.base.set_state(243);
					recog.typeSpec()?;

					/*InvokeRule qualId*/
					recog.base.set_state(244);
					recog.qualId()?;

					recog.base.set_state(245);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					let tmp = DefConstantContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7);
					_localctx = tmp;
					{
					recog.base.set_state(247);
					recog.base.match_token(T__34,&mut recog.err_handler)?;

					/*InvokeRule typeSpec*/
					recog.base.set_state(248);
					recog.typeSpec()?;

					/*InvokeRule id*/
					recog.base.set_state(249);
					recog.id()?;

					recog.base.set_state(250);
					recog.base.match_token(T__29,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(251);
					recog.expr_rec(0)?;

					recog.base.set_state(252);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					let tmp = DefArrayContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8);
					_localctx = tmp;
					{
					recog.base.set_state(254);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					/*InvokeRule typeSpec*/
					recog.base.set_state(255);
					recog.typeSpec()?;

					/*InvokeRule id*/
					recog.base.set_state(256);
					recog.id()?;

					recog.base.set_state(257);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					/*InvokeRule ixType*/
					recog.base.set_state(258);
					recog.ixType()?;

					recog.base.set_state(259);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					recog.base.set_state(260);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				9 =>{
					let tmp = DefCallableContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 9);
					_localctx = tmp;
					{
					recog.base.set_state(263);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(15,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule returnType*/
							recog.base.set_state(262);
							recog.returnType()?;

							}
						}

						_ => {}
					}
					/*InvokeRule qualId*/
					recog.base.set_state(265);
					recog.qualId()?;

					recog.base.set_state(266);
					recog.base.match_token(T__31,&mut recog.err_handler)?;

					/*InvokeRule symDeclCommaList*/
					recog.base.set_state(267);
					recog.symDeclCommaList()?;

					recog.base.set_state(268);
					recog.base.match_token(T__32,&mut recog.err_handler)?;

					recog.base.set_state(271);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__3 | T__4 | T__31 | T__33 | T__34 | T__36 | T__37 | T__39 | T__42 |
					 T__43 | T__46 | T__48 | T__50 | T__51 | T__52 | T__53 | T__54 | T__56 |
					 T__57 | T__62 | T__64 | T__67 | T__88 | T__89 | IDENTIFIER | SEE_TOK 
						=> {
							{
							/*InvokeRule indentedBlock*/
							recog.base.set_state(269);
							recog.indentedBlock()?;

							}
						}

					 SEMICOLON 
						=> {
							{
							recog.base.set_state(270);
							recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}
			,
				10 =>{
					let tmp = DefGetterContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 10);
					_localctx = tmp;
					{
					/*InvokeRule returnType*/
					recog.base.set_state(273);
					recog.returnType()?;

					/*InvokeRule qualId*/
					recog.base.set_state(274);
					recog.qualId()?;

					/*InvokeRule indentedBlock*/
					recog.base.set_state(275);
					recog.indentedBlock()?;

					}
				}
			,
				11 =>{
					let tmp = DefGetterContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 11);
					_localctx = tmp;
					{
					/*InvokeRule returnType*/
					recog.base.set_state(277);
					recog.returnType()?;

					/*InvokeRule qualId*/
					recog.base.set_state(278);
					recog.qualId()?;

					recog.base.set_state(279);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					/*InvokeRule symDeclCommaList*/
					recog.base.set_state(280);
					recog.symDeclCommaList()?;

					recog.base.set_state(281);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					recog.base.set_state(284);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__3 | T__4 | T__31 | T__33 | T__34 | T__36 | T__37 | T__39 | T__42 |
					 T__43 | T__46 | T__48 | T__50 | T__51 | T__52 | T__53 | T__54 | T__56 |
					 T__57 | T__62 | T__64 | T__67 | T__88 | T__89 | IDENTIFIER | SEE_TOK 
						=> {
							{
							/*InvokeRule indentedBlock*/
							recog.base.set_state(282);
							recog.indentedBlock()?;

							}
						}

					 SEMICOLON 
						=> {
							{
							recog.base.set_state(283);
							recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}
			,
				12 =>{
					let tmp = DefSetterContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 12);
					_localctx = tmp;
					{
					/*InvokeRule qualId*/
					recog.base.set_state(286);
					recog.qualId()?;

					recog.base.set_state(287);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					/*InvokeRule setterArgCommaList*/
					recog.base.set_state(288);
					recog.setterArgCommaList()?;

					recog.base.set_state(289);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					recog.base.set_state(290);
					recog.base.match_token(T__29,&mut recog.err_handler)?;

					/*InvokeRule symDecl*/
					recog.base.set_state(291);
					recog.symDecl()?;

					recog.base.set_state(294);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__3 | T__4 | T__31 | T__33 | T__34 | T__36 | T__37 | T__39 | T__42 |
					 T__43 | T__46 | T__48 | T__50 | T__51 | T__52 | T__53 | T__54 | T__56 |
					 T__57 | T__62 | T__64 | T__67 | T__88 | T__89 | IDENTIFIER | SEE_TOK 
						=> {
							{
							/*InvokeRule indentedBlock*/
							recog.base.set_state(292);
							recog.indentedBlock()?;

							}
						}

					 SEMICOLON 
						=> {
							{
							recog.base.set_state(293);
							recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}
			,
				13 =>{
					let tmp = DefSetterContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 13);
					_localctx = tmp;
					{
					/*InvokeRule qualId*/
					recog.base.set_state(296);
					recog.qualId()?;

					recog.base.set_state(297);
					recog.base.match_token(T__29,&mut recog.err_handler)?;

					/*InvokeRule symDecl*/
					recog.base.set_state(298);
					recog.symDecl()?;

					recog.base.set_state(301);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__3 | T__4 | T__31 | T__33 | T__34 | T__36 | T__37 | T__39 | T__42 |
					 T__43 | T__46 | T__48 | T__50 | T__51 | T__52 | T__53 | T__54 | T__56 |
					 T__57 | T__62 | T__64 | T__67 | T__88 | T__89 | IDENTIFIER | SEE_TOK 
						=> {
							{
							/*InvokeRule indentedBlock*/
							recog.base.set_state(299);
							recog.indentedBlock()?;

							}
						}

					 SEMICOLON 
						=> {
							{
							recog.base.set_state(300);
							recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
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
//------------------- setterArg ----------------
#[derive(Debug)]
pub enum SetterArgContextAll<'input>{
	SetterRefArgContext(SetterRefArgContext<'input>),
	SetterValArgContext(SetterValArgContext<'input>),
Error(SetterArgContext<'input>)
}
antlr_rust::tid!{SetterArgContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for SetterArgContextAll<'input>{}

impl<'input> aslParserContext<'input> for SetterArgContextAll<'input>{}

impl<'input> Deref for SetterArgContextAll<'input>{
	type Target = dyn SetterArgContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use SetterArgContextAll::*;
		match self{
			SetterRefArgContext(inner) => inner,
			SetterValArgContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SetterArgContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SetterArgContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type SetterArgContext<'input> = BaseParserRuleContext<'input,SetterArgContextExt<'input>>;

#[derive(Clone)]
pub struct SetterArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for SetterArgContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SetterArgContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SetterArgContext<'input>{
}

impl<'input> CustomRuleContext<'input> for SetterArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_setterArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_setterArg }
}
antlr_rust::tid!{SetterArgContextExt<'a>}

impl<'input> SetterArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SetterArgContextAll<'input>> {
		Rc::new(
		SetterArgContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SetterArgContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait SetterArgContextAttrs<'input>: aslParserContext<'input> + BorrowMut<SetterArgContextExt<'input>>{


}

impl<'input> SetterArgContextAttrs<'input> for SetterArgContext<'input>{}

pub type SetterRefArgContext<'input> = BaseParserRuleContext<'input,SetterRefArgContextExt<'input>>;

pub trait SetterRefArgContextAttrs<'input>: aslParserContext<'input>{
	fn typeSpec(&self) -> Option<Rc<TypeSpecContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SetterRefArgContextAttrs<'input> for SetterRefArgContext<'input>{}

pub struct SetterRefArgContextExt<'input>{
	base:SetterArgContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SetterRefArgContextExt<'a>}

impl<'input> aslParserContext<'input> for SetterRefArgContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SetterRefArgContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SetterRefArg(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SetterRefArg(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SetterRefArgContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SetterRefArg(self);
	}
}

impl<'input> CustomRuleContext<'input> for SetterRefArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_setterArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_setterArg }
}

impl<'input> Borrow<SetterArgContextExt<'input>> for SetterRefArgContext<'input>{
	fn borrow(&self) -> &SetterArgContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SetterArgContextExt<'input>> for SetterRefArgContext<'input>{
	fn borrow_mut(&mut self) -> &mut SetterArgContextExt<'input> { &mut self.base }
}

impl<'input> SetterArgContextAttrs<'input> for SetterRefArgContext<'input> {}

impl<'input> SetterRefArgContextExt<'input>{
	fn new(ctx: &dyn SetterArgContextAttrs<'input>) -> Rc<SetterArgContextAll<'input>>  {
		Rc::new(
			SetterArgContextAll::SetterRefArgContext(
				BaseParserRuleContext::copy_from(ctx,SetterRefArgContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SetterValArgContext<'input> = BaseParserRuleContext<'input,SetterValArgContextExt<'input>>;

pub trait SetterValArgContextAttrs<'input>: aslParserContext<'input>{
	fn typeSpec(&self) -> Option<Rc<TypeSpecContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SetterValArgContextAttrs<'input> for SetterValArgContext<'input>{}

pub struct SetterValArgContextExt<'input>{
	base:SetterArgContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SetterValArgContextExt<'a>}

impl<'input> aslParserContext<'input> for SetterValArgContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SetterValArgContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SetterValArg(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SetterValArg(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SetterValArgContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SetterValArg(self);
	}
}

impl<'input> CustomRuleContext<'input> for SetterValArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_setterArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_setterArg }
}

impl<'input> Borrow<SetterArgContextExt<'input>> for SetterValArgContext<'input>{
	fn borrow(&self) -> &SetterArgContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SetterArgContextExt<'input>> for SetterValArgContext<'input>{
	fn borrow_mut(&mut self) -> &mut SetterArgContextExt<'input> { &mut self.base }
}

impl<'input> SetterArgContextAttrs<'input> for SetterValArgContext<'input> {}

impl<'input> SetterValArgContextExt<'input>{
	fn new(ctx: &dyn SetterArgContextAttrs<'input>) -> Rc<SetterArgContextAll<'input>>  {
		Rc::new(
			SetterArgContextAll::SetterValArgContext(
				BaseParserRuleContext::copy_from(ctx,SetterValArgContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn setterArg(&mut self,)
	-> Result<Rc<SetterArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SetterArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_setterArg);
        let mut _localctx: Rc<SetterArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(312);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(21,&mut recog.base)? {
				1 =>{
					let tmp = SetterRefArgContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule typeSpec*/
					recog.base.set_state(305);
					recog.typeSpec()?;

					recog.base.set_state(306);
					recog.base.match_token(T__35,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(307);
					recog.id()?;

					}
				}
			,
				2 =>{
					let tmp = SetterValArgContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule typeSpec*/
					recog.base.set_state(309);
					recog.typeSpec()?;

					/*InvokeRule id*/
					recog.base.set_state(310);
					recog.id()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
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
//------------------- setterArgCommaList ----------------
pub type SetterArgCommaListContextAll<'input> = SetterArgCommaListContext<'input>;


pub type SetterArgCommaListContext<'input> = BaseParserRuleContext<'input,SetterArgCommaListContextExt<'input>>;

#[derive(Clone)]
pub struct SetterArgCommaListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for SetterArgCommaListContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SetterArgCommaListContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_setterArgCommaList(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_setterArgCommaList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SetterArgCommaListContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_setterArgCommaList(self);
	}
}

impl<'input> CustomRuleContext<'input> for SetterArgCommaListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_setterArgCommaList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_setterArgCommaList }
}
antlr_rust::tid!{SetterArgCommaListContextExt<'a>}

impl<'input> SetterArgCommaListContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SetterArgCommaListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SetterArgCommaListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SetterArgCommaListContextAttrs<'input>: aslParserContext<'input> + BorrowMut<SetterArgCommaListContextExt<'input>>{

fn setterArg_all(&self) ->  Vec<Rc<SetterArgContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn setterArg(&self, i: usize) -> Option<Rc<SetterArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SetterArgCommaListContextAttrs<'input> for SetterArgCommaListContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn setterArgCommaList(&mut self,)
	-> Result<Rc<SetterArgCommaListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SetterArgCommaListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_setterArgCommaList);
        let mut _localctx: Rc<SetterArgCommaListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(322);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__3 || ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (T__33 - 34)) | (1usize << (T__36 - 34)) | (1usize << (T__37 - 34)))) != 0) || ((((_la - 68)) & !0x3f) == 0 && ((1usize << (_la - 68)) & ((1usize << (T__67 - 68)) | (1usize << (T__88 - 68)) | (1usize << (T__89 - 68)) | (1usize << (IDENTIFIER - 68)))) != 0) {
				{
				/*InvokeRule setterArg*/
				recog.base.set_state(314);
				recog.setterArg()?;

				recog.base.set_state(319);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__9 {
					{
					{
					recog.base.set_state(315);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule setterArg*/
					recog.base.set_state(316);
					recog.setterArg()?;

					}
					}
					recog.base.set_state(321);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			}
			Ok(())
		})();
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
//------------------- returnType ----------------
pub type ReturnTypeContextAll<'input> = ReturnTypeContext<'input>;


pub type ReturnTypeContext<'input> = BaseParserRuleContext<'input,ReturnTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ReturnTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for ReturnTypeContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ReturnTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_returnType(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_returnType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ReturnTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_returnType(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReturnTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_returnType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_returnType }
}
antlr_rust::tid!{ReturnTypeContextExt<'a>}

impl<'input> ReturnTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReturnTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReturnTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReturnTypeContextAttrs<'input>: aslParserContext<'input> + BorrowMut<ReturnTypeContextExt<'input>>{

fn typeSpec_all(&self) ->  Vec<Rc<TypeSpecContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeSpec(&self, i: usize) -> Option<Rc<TypeSpecContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ReturnTypeContextAttrs<'input> for ReturnTypeContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn returnType(&mut self,)
	-> Result<Rc<ReturnTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReturnTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_returnType);
        let mut _localctx: Rc<ReturnTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(336);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__3 | T__33 | T__36 | T__37 | T__67 | T__88 | T__89 | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule typeSpec*/
					recog.base.set_state(324);
					recog.typeSpec()?;

					}
				}

			 T__31 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(325);
					recog.base.match_token(T__31,&mut recog.err_handler)?;

					/*InvokeRule typeSpec*/
					recog.base.set_state(326);
					recog.typeSpec()?;

					recog.base.set_state(331);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__9 {
						{
						{
						recog.base.set_state(327);
						recog.base.match_token(T__9,&mut recog.err_handler)?;

						/*InvokeRule typeSpec*/
						recog.base.set_state(328);
						recog.typeSpec()?;

						}
						}
						recog.base.set_state(333);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(334);
					recog.base.match_token(T__32,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
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
//------------------- typeSpec ----------------
#[derive(Debug)]
pub enum TypeSpecContextAll<'input>{
	TypeArrayContext(TypeArrayContext<'input>),
	TypeIndexedContext(TypeIndexedContext<'input>),
	TypeOfContext(TypeOfContext<'input>),
	TypeRefContext(TypeRefContext<'input>),
	TypeRegisterContext(TypeRegisterContext<'input>),
Error(TypeSpecContext<'input>)
}
antlr_rust::tid!{TypeSpecContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for TypeSpecContextAll<'input>{}

impl<'input> aslParserContext<'input> for TypeSpecContextAll<'input>{}

impl<'input> Deref for TypeSpecContextAll<'input>{
	type Target = dyn TypeSpecContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use TypeSpecContextAll::*;
		match self{
			TypeArrayContext(inner) => inner,
			TypeIndexedContext(inner) => inner,
			TypeOfContext(inner) => inner,
			TypeRefContext(inner) => inner,
			TypeRegisterContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for TypeSpecContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for TypeSpecContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type TypeSpecContext<'input> = BaseParserRuleContext<'input,TypeSpecContextExt<'input>>;

#[derive(Clone)]
pub struct TypeSpecContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for TypeSpecContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for TypeSpecContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for TypeSpecContext<'input>{
}

impl<'input> CustomRuleContext<'input> for TypeSpecContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeSpec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeSpec }
}
antlr_rust::tid!{TypeSpecContextExt<'a>}

impl<'input> TypeSpecContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeSpecContextAll<'input>> {
		Rc::new(
		TypeSpecContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeSpecContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait TypeSpecContextAttrs<'input>: aslParserContext<'input> + BorrowMut<TypeSpecContextExt<'input>>{


}

impl<'input> TypeSpecContextAttrs<'input> for TypeSpecContext<'input>{}

pub type TypeArrayContext<'input> = BaseParserRuleContext<'input,TypeArrayContextExt<'input>>;

pub trait TypeArrayContextAttrs<'input>: aslParserContext<'input>{
	fn ixType(&self) -> Option<Rc<IxTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn typeSpec(&self) -> Option<Rc<TypeSpecContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TypeArrayContextAttrs<'input> for TypeArrayContext<'input>{}

pub struct TypeArrayContextExt<'input>{
	base:TypeSpecContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{TypeArrayContextExt<'a>}

impl<'input> aslParserContext<'input> for TypeArrayContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for TypeArrayContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_TypeArray(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_TypeArray(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for TypeArrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_TypeArray(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeSpec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeSpec }
}

impl<'input> Borrow<TypeSpecContextExt<'input>> for TypeArrayContext<'input>{
	fn borrow(&self) -> &TypeSpecContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TypeSpecContextExt<'input>> for TypeArrayContext<'input>{
	fn borrow_mut(&mut self) -> &mut TypeSpecContextExt<'input> { &mut self.base }
}

impl<'input> TypeSpecContextAttrs<'input> for TypeArrayContext<'input> {}

impl<'input> TypeArrayContextExt<'input>{
	fn new(ctx: &dyn TypeSpecContextAttrs<'input>) -> Rc<TypeSpecContextAll<'input>>  {
		Rc::new(
			TypeSpecContextAll::TypeArrayContext(
				BaseParserRuleContext::copy_from(ctx,TypeArrayContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TypeIndexedContext<'input> = BaseParserRuleContext<'input,TypeIndexedContextExt<'input>>;

pub trait TypeIndexedContextAttrs<'input>: aslParserContext<'input>{
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TypeIndexedContextAttrs<'input> for TypeIndexedContext<'input>{}

pub struct TypeIndexedContextExt<'input>{
	base:TypeSpecContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{TypeIndexedContextExt<'a>}

impl<'input> aslParserContext<'input> for TypeIndexedContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for TypeIndexedContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_TypeIndexed(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_TypeIndexed(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for TypeIndexedContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_TypeIndexed(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeIndexedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeSpec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeSpec }
}

impl<'input> Borrow<TypeSpecContextExt<'input>> for TypeIndexedContext<'input>{
	fn borrow(&self) -> &TypeSpecContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TypeSpecContextExt<'input>> for TypeIndexedContext<'input>{
	fn borrow_mut(&mut self) -> &mut TypeSpecContextExt<'input> { &mut self.base }
}

impl<'input> TypeSpecContextAttrs<'input> for TypeIndexedContext<'input> {}

impl<'input> TypeIndexedContextExt<'input>{
	fn new(ctx: &dyn TypeSpecContextAttrs<'input>) -> Rc<TypeSpecContextAll<'input>>  {
		Rc::new(
			TypeSpecContextAll::TypeIndexedContext(
				BaseParserRuleContext::copy_from(ctx,TypeIndexedContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TypeOfContext<'input> = BaseParserRuleContext<'input,TypeOfContextExt<'input>>;

pub trait TypeOfContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TypeOfContextAttrs<'input> for TypeOfContext<'input>{}

pub struct TypeOfContextExt<'input>{
	base:TypeSpecContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{TypeOfContextExt<'a>}

impl<'input> aslParserContext<'input> for TypeOfContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for TypeOfContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_TypeOf(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_TypeOf(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for TypeOfContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_TypeOf(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeOfContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeSpec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeSpec }
}

impl<'input> Borrow<TypeSpecContextExt<'input>> for TypeOfContext<'input>{
	fn borrow(&self) -> &TypeSpecContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TypeSpecContextExt<'input>> for TypeOfContext<'input>{
	fn borrow_mut(&mut self) -> &mut TypeSpecContextExt<'input> { &mut self.base }
}

impl<'input> TypeSpecContextAttrs<'input> for TypeOfContext<'input> {}

impl<'input> TypeOfContextExt<'input>{
	fn new(ctx: &dyn TypeSpecContextAttrs<'input>) -> Rc<TypeSpecContextAll<'input>>  {
		Rc::new(
			TypeSpecContextAll::TypeOfContext(
				BaseParserRuleContext::copy_from(ctx,TypeOfContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TypeRefContext<'input> = BaseParserRuleContext<'input,TypeRefContextExt<'input>>;

pub trait TypeRefContextAttrs<'input>: aslParserContext<'input>{
	fn qualId(&self) -> Option<Rc<QualIdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TypeRefContextAttrs<'input> for TypeRefContext<'input>{}

pub struct TypeRefContextExt<'input>{
	base:TypeSpecContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{TypeRefContextExt<'a>}

impl<'input> aslParserContext<'input> for TypeRefContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for TypeRefContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_TypeRef(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_TypeRef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for TypeRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_TypeRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeSpec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeSpec }
}

impl<'input> Borrow<TypeSpecContextExt<'input>> for TypeRefContext<'input>{
	fn borrow(&self) -> &TypeSpecContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TypeSpecContextExt<'input>> for TypeRefContext<'input>{
	fn borrow_mut(&mut self) -> &mut TypeSpecContextExt<'input> { &mut self.base }
}

impl<'input> TypeSpecContextAttrs<'input> for TypeRefContext<'input> {}

impl<'input> TypeRefContextExt<'input>{
	fn new(ctx: &dyn TypeSpecContextAttrs<'input>) -> Rc<TypeSpecContextAll<'input>>  {
		Rc::new(
			TypeSpecContextAll::TypeRefContext(
				BaseParserRuleContext::copy_from(ctx,TypeRefContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TypeRegisterContext<'input> = BaseParserRuleContext<'input,TypeRegisterContextExt<'input>>;

pub trait TypeRegisterContextAttrs<'input>: aslParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NAT_LIT
	/// Returns `None` if there is no child corresponding to token NAT_LIT
	fn NAT_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(NAT_LIT, 0)
	}
	fn regField_all(&self) ->  Vec<Rc<RegFieldContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn regField(&self, i: usize) -> Option<Rc<RegFieldContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> TypeRegisterContextAttrs<'input> for TypeRegisterContext<'input>{}

pub struct TypeRegisterContextExt<'input>{
	base:TypeSpecContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{TypeRegisterContextExt<'a>}

impl<'input> aslParserContext<'input> for TypeRegisterContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for TypeRegisterContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_TypeRegister(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_TypeRegister(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for TypeRegisterContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_TypeRegister(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeRegisterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeSpec }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeSpec }
}

impl<'input> Borrow<TypeSpecContextExt<'input>> for TypeRegisterContext<'input>{
	fn borrow(&self) -> &TypeSpecContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TypeSpecContextExt<'input>> for TypeRegisterContext<'input>{
	fn borrow_mut(&mut self) -> &mut TypeSpecContextExt<'input> { &mut self.base }
}

impl<'input> TypeSpecContextAttrs<'input> for TypeRegisterContext<'input> {}

impl<'input> TypeRegisterContextExt<'input>{
	fn new(ctx: &dyn TypeSpecContextAttrs<'input>) -> Rc<TypeSpecContextAll<'input>>  {
		Rc::new(
			TypeSpecContextAll::TypeRegisterContext(
				BaseParserRuleContext::copy_from(ctx,TypeRegisterContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeSpec(&mut self,)
	-> Result<Rc<TypeSpecContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeSpecContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_typeSpec);
        let mut _localctx: Rc<TypeSpecContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(369);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(27,&mut recog.base)? {
				1 =>{
					let tmp = TypeRefContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule qualId*/
					recog.base.set_state(338);
					recog.qualId()?;

					}
				}
			,
				2 =>{
					let tmp = TypeIndexedContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule id*/
					recog.base.set_state(339);
					recog.id()?;

					recog.base.set_state(340);
					recog.base.match_token(T__31,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(341);
					recog.expr_rec(0)?;

					recog.base.set_state(342);
					recog.base.match_token(T__32,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					let tmp = TypeOfContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(344);
					recog.base.match_token(T__36,&mut recog.err_handler)?;

					recog.base.set_state(345);
					recog.base.match_token(T__31,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(346);
					recog.expr_rec(0)?;

					recog.base.set_state(347);
					recog.base.match_token(T__32,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					let tmp = TypeRegisterContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(349);
					recog.base.match_token(T__37,&mut recog.err_handler)?;

					recog.base.set_state(350);
					recog.base.match_token(NAT_LIT,&mut recog.err_handler)?;

					recog.base.set_state(351);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					/*InvokeRule regField*/
					recog.base.set_state(352);
					recog.regField()?;

					recog.base.set_state(357);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__9 {
						{
						{
						recog.base.set_state(353);
						recog.base.match_token(T__9,&mut recog.err_handler)?;

						/*InvokeRule regField*/
						recog.base.set_state(354);
						recog.regField()?;

						}
						}
						recog.base.set_state(359);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(360);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					let tmp = TypeArrayContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(362);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					recog.base.set_state(363);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					/*InvokeRule ixType*/
					recog.base.set_state(364);
					recog.ixType()?;

					recog.base.set_state(365);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					recog.base.set_state(366);
					recog.base.match_token(T__7,&mut recog.err_handler)?;

					/*InvokeRule typeSpec*/
					recog.base.set_state(367);
					recog.typeSpec()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
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
//------------------- ixType ----------------
#[derive(Debug)]
pub enum IxTypeContextAll<'input>{
	IxTypeRefContext(IxTypeRefContext<'input>),
	IxTypeRangeContext(IxTypeRangeContext<'input>),
Error(IxTypeContext<'input>)
}
antlr_rust::tid!{IxTypeContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for IxTypeContextAll<'input>{}

impl<'input> aslParserContext<'input> for IxTypeContextAll<'input>{}

impl<'input> Deref for IxTypeContextAll<'input>{
	type Target = dyn IxTypeContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use IxTypeContextAll::*;
		match self{
			IxTypeRefContext(inner) => inner,
			IxTypeRangeContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for IxTypeContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for IxTypeContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type IxTypeContext<'input> = BaseParserRuleContext<'input,IxTypeContextExt<'input>>;

#[derive(Clone)]
pub struct IxTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for IxTypeContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for IxTypeContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for IxTypeContext<'input>{
}

impl<'input> CustomRuleContext<'input> for IxTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ixType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ixType }
}
antlr_rust::tid!{IxTypeContextExt<'a>}

impl<'input> IxTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IxTypeContextAll<'input>> {
		Rc::new(
		IxTypeContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IxTypeContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait IxTypeContextAttrs<'input>: aslParserContext<'input> + BorrowMut<IxTypeContextExt<'input>>{


}

impl<'input> IxTypeContextAttrs<'input> for IxTypeContext<'input>{}

pub type IxTypeRefContext<'input> = BaseParserRuleContext<'input,IxTypeRefContextExt<'input>>;

pub trait IxTypeRefContextAttrs<'input>: aslParserContext<'input>{
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> IxTypeRefContextAttrs<'input> for IxTypeRefContext<'input>{}

pub struct IxTypeRefContextExt<'input>{
	base:IxTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IxTypeRefContextExt<'a>}

impl<'input> aslParserContext<'input> for IxTypeRefContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for IxTypeRefContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_IxTypeRef(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_IxTypeRef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for IxTypeRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_IxTypeRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for IxTypeRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ixType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ixType }
}

impl<'input> Borrow<IxTypeContextExt<'input>> for IxTypeRefContext<'input>{
	fn borrow(&self) -> &IxTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<IxTypeContextExt<'input>> for IxTypeRefContext<'input>{
	fn borrow_mut(&mut self) -> &mut IxTypeContextExt<'input> { &mut self.base }
}

impl<'input> IxTypeContextAttrs<'input> for IxTypeRefContext<'input> {}

impl<'input> IxTypeRefContextExt<'input>{
	fn new(ctx: &dyn IxTypeContextAttrs<'input>) -> Rc<IxTypeContextAll<'input>>  {
		Rc::new(
			IxTypeContextAll::IxTypeRefContext(
				BaseParserRuleContext::copy_from(ctx,IxTypeRefContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IxTypeRangeContext<'input> = BaseParserRuleContext<'input,IxTypeRangeContextExt<'input>>;

pub trait IxTypeRangeContextAttrs<'input>: aslParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> IxTypeRangeContextAttrs<'input> for IxTypeRangeContext<'input>{}

pub struct IxTypeRangeContextExt<'input>{
	base:IxTypeContextExt<'input>,
	pub begin: Option<Rc<ExprContextAll<'input>>>,
	pub end: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IxTypeRangeContextExt<'a>}

impl<'input> aslParserContext<'input> for IxTypeRangeContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for IxTypeRangeContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_IxTypeRange(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_IxTypeRange(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for IxTypeRangeContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_IxTypeRange(self);
	}
}

impl<'input> CustomRuleContext<'input> for IxTypeRangeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ixType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ixType }
}

impl<'input> Borrow<IxTypeContextExt<'input>> for IxTypeRangeContext<'input>{
	fn borrow(&self) -> &IxTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<IxTypeContextExt<'input>> for IxTypeRangeContext<'input>{
	fn borrow_mut(&mut self) -> &mut IxTypeContextExt<'input> { &mut self.base }
}

impl<'input> IxTypeContextAttrs<'input> for IxTypeRangeContext<'input> {}

impl<'input> IxTypeRangeContextExt<'input>{
	fn new(ctx: &dyn IxTypeContextAttrs<'input>) -> Rc<IxTypeContextAll<'input>>  {
		Rc::new(
			IxTypeContextAll::IxTypeRangeContext(
				BaseParserRuleContext::copy_from(ctx,IxTypeRangeContextExt{
        			begin:None, end:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ixType(&mut self,)
	-> Result<Rc<IxTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IxTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_ixType);
        let mut _localctx: Rc<IxTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(376);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(28,&mut recog.base)? {
				1 =>{
					let tmp = IxTypeRefContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule id*/
					recog.base.set_state(371);
					recog.id()?;

					}
				}
			,
				2 =>{
					let tmp = IxTypeRangeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule expr*/
					recog.base.set_state(372);
					let tmp = recog.expr_rec(0)?;
					if let IxTypeContextAll::IxTypeRangeContext(ctx) = cast_mut::<_,IxTypeContextAll >(&mut _localctx){
					ctx.begin = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(373);
					recog.base.match_token(T__5,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(374);
					let tmp = recog.expr_rec(0)?;
					if let IxTypeContextAll::IxTypeRangeContext(ctx) = cast_mut::<_,IxTypeContextAll >(&mut _localctx){
					ctx.end = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}

				_ => {}
			}
			Ok(())
		})();
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
//------------------- regField ----------------
pub type RegFieldContextAll<'input> = RegFieldContext<'input>;


pub type RegFieldContext<'input> = BaseParserRuleContext<'input,RegFieldContextExt<'input>>;

#[derive(Clone)]
pub struct RegFieldContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for RegFieldContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for RegFieldContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_regField(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_regField(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for RegFieldContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_regField(self);
	}
}

impl<'input> CustomRuleContext<'input> for RegFieldContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_regField }
	//fn type_rule_index() -> usize where Self: Sized { RULE_regField }
}
antlr_rust::tid!{RegFieldContextExt<'a>}

impl<'input> RegFieldContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RegFieldContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RegFieldContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RegFieldContextAttrs<'input>: aslParserContext<'input> + BorrowMut<RegFieldContextExt<'input>>{

fn slice_all(&self) ->  Vec<Rc<SliceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn slice(&self, i: usize) -> Option<Rc<SliceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RegFieldContextAttrs<'input> for RegFieldContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn regField(&mut self,)
	-> Result<Rc<RegFieldContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RegFieldContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_regField);
        let mut _localctx: Rc<RegFieldContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule slice*/
			recog.base.set_state(378);
			recog.slice()?;

			recog.base.set_state(383);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__9 {
				{
				{
				recog.base.set_state(379);
				recog.base.match_token(T__9,&mut recog.err_handler)?;

				/*InvokeRule slice*/
				recog.base.set_state(380);
				recog.slice()?;

				}
				}
				recog.base.set_state(385);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule id*/
			recog.base.set_state(386);
			recog.id()?;

			}
			Ok(())
		})();
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
//------------------- indentedBlock ----------------
pub type IndentedBlockContextAll<'input> = IndentedBlockContext<'input>;


pub type IndentedBlockContext<'input> = BaseParserRuleContext<'input,IndentedBlockContextExt<'input>>;

#[derive(Clone)]
pub struct IndentedBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for IndentedBlockContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for IndentedBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_indentedBlock(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_indentedBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for IndentedBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_indentedBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for IndentedBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_indentedBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_indentedBlock }
}
antlr_rust::tid!{IndentedBlockContextExt<'a>}

impl<'input> IndentedBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IndentedBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IndentedBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IndentedBlockContextAttrs<'input>: aslParserContext<'input> + BorrowMut<IndentedBlockContextExt<'input>>{

fn stmt_all(&self) ->  Vec<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stmt(&self, i: usize) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> IndentedBlockContextAttrs<'input> for IndentedBlockContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn indentedBlock(&mut self,)
	-> Result<Rc<IndentedBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IndentedBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_indentedBlock);
        let mut _localctx: Rc<IndentedBlockContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(389); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					/*InvokeRule stmt*/
					recog.base.set_state(388);
					recog.stmt()?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(391); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(30,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			}
			Ok(())
		})();
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
//------------------- blockOrEmbed0 ----------------
pub type BlockOrEmbed0ContextAll<'input> = BlockOrEmbed0Context<'input>;


pub type BlockOrEmbed0Context<'input> = BaseParserRuleContext<'input,BlockOrEmbed0ContextExt<'input>>;

#[derive(Clone)]
pub struct BlockOrEmbed0ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for BlockOrEmbed0Context<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for BlockOrEmbed0Context<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_blockOrEmbed0(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_blockOrEmbed0(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for BlockOrEmbed0Context<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_blockOrEmbed0(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlockOrEmbed0ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_blockOrEmbed0 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_blockOrEmbed0 }
}
antlr_rust::tid!{BlockOrEmbed0ContextExt<'a>}

impl<'input> BlockOrEmbed0ContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlockOrEmbed0ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockOrEmbed0ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlockOrEmbed0ContextAttrs<'input>: aslParserContext<'input> + BorrowMut<BlockOrEmbed0ContextExt<'input>>{

fn blockOrEmbed1(&self) -> Option<Rc<BlockOrEmbed1ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BlockOrEmbed0ContextAttrs<'input> for BlockOrEmbed0Context<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn blockOrEmbed0(&mut self,)
	-> Result<Rc<BlockOrEmbed0ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlockOrEmbed0ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_blockOrEmbed0);
        let mut _localctx: Rc<BlockOrEmbed0ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(394);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(31,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule blockOrEmbed1*/
					recog.base.set_state(393);
					recog.blockOrEmbed1()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
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
//------------------- blockOrEmbed1 ----------------
#[derive(Debug)]
pub enum BlockOrEmbed1ContextAll<'input>{
	BlockIndentContext(BlockIndentContext<'input>),
	BlockInlineContext(BlockInlineContext<'input>),
Error(BlockOrEmbed1Context<'input>)
}
antlr_rust::tid!{BlockOrEmbed1ContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for BlockOrEmbed1ContextAll<'input>{}

impl<'input> aslParserContext<'input> for BlockOrEmbed1ContextAll<'input>{}

impl<'input> Deref for BlockOrEmbed1ContextAll<'input>{
	type Target = dyn BlockOrEmbed1ContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use BlockOrEmbed1ContextAll::*;
		match self{
			BlockIndentContext(inner) => inner,
			BlockInlineContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for BlockOrEmbed1ContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for BlockOrEmbed1ContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type BlockOrEmbed1Context<'input> = BaseParserRuleContext<'input,BlockOrEmbed1ContextExt<'input>>;

#[derive(Clone)]
pub struct BlockOrEmbed1ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for BlockOrEmbed1Context<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for BlockOrEmbed1Context<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for BlockOrEmbed1Context<'input>{
}

impl<'input> CustomRuleContext<'input> for BlockOrEmbed1ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_blockOrEmbed1 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_blockOrEmbed1 }
}
antlr_rust::tid!{BlockOrEmbed1ContextExt<'a>}

impl<'input> BlockOrEmbed1ContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlockOrEmbed1ContextAll<'input>> {
		Rc::new(
		BlockOrEmbed1ContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockOrEmbed1ContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait BlockOrEmbed1ContextAttrs<'input>: aslParserContext<'input> + BorrowMut<BlockOrEmbed1ContextExt<'input>>{


}

impl<'input> BlockOrEmbed1ContextAttrs<'input> for BlockOrEmbed1Context<'input>{}

pub type BlockIndentContext<'input> = BaseParserRuleContext<'input,BlockIndentContextExt<'input>>;

pub trait BlockIndentContextAttrs<'input>: aslParserContext<'input>{
	fn indentedBlock(&self) -> Option<Rc<IndentedBlockContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> BlockIndentContextAttrs<'input> for BlockIndentContext<'input>{}

pub struct BlockIndentContextExt<'input>{
	base:BlockOrEmbed1ContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{BlockIndentContextExt<'a>}

impl<'input> aslParserContext<'input> for BlockIndentContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for BlockIndentContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_BlockIndent(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_BlockIndent(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for BlockIndentContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_BlockIndent(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlockIndentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_blockOrEmbed1 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_blockOrEmbed1 }
}

impl<'input> Borrow<BlockOrEmbed1ContextExt<'input>> for BlockIndentContext<'input>{
	fn borrow(&self) -> &BlockOrEmbed1ContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BlockOrEmbed1ContextExt<'input>> for BlockIndentContext<'input>{
	fn borrow_mut(&mut self) -> &mut BlockOrEmbed1ContextExt<'input> { &mut self.base }
}

impl<'input> BlockOrEmbed1ContextAttrs<'input> for BlockIndentContext<'input> {}

impl<'input> BlockIndentContextExt<'input>{
	fn new(ctx: &dyn BlockOrEmbed1ContextAttrs<'input>) -> Rc<BlockOrEmbed1ContextAll<'input>>  {
		Rc::new(
			BlockOrEmbed1ContextAll::BlockIndentContext(
				BaseParserRuleContext::copy_from(ctx,BlockIndentContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type BlockInlineContext<'input> = BaseParserRuleContext<'input,BlockInlineContextExt<'input>>;

pub trait BlockInlineContextAttrs<'input>: aslParserContext<'input>{
	fn stmt(&self) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn inlineStmt_all(&self) ->  Vec<Rc<InlineStmtContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn inlineStmt(&self, i: usize) -> Option<Rc<InlineStmtContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> BlockInlineContextAttrs<'input> for BlockInlineContext<'input>{}

pub struct BlockInlineContextExt<'input>{
	base:BlockOrEmbed1ContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{BlockInlineContextExt<'a>}

impl<'input> aslParserContext<'input> for BlockInlineContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for BlockInlineContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_BlockInline(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_BlockInline(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for BlockInlineContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_BlockInline(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlockInlineContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_blockOrEmbed1 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_blockOrEmbed1 }
}

impl<'input> Borrow<BlockOrEmbed1ContextExt<'input>> for BlockInlineContext<'input>{
	fn borrow(&self) -> &BlockOrEmbed1ContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BlockOrEmbed1ContextExt<'input>> for BlockInlineContext<'input>{
	fn borrow_mut(&mut self) -> &mut BlockOrEmbed1ContextExt<'input> { &mut self.base }
}

impl<'input> BlockOrEmbed1ContextAttrs<'input> for BlockInlineContext<'input> {}

impl<'input> BlockInlineContextExt<'input>{
	fn new(ctx: &dyn BlockOrEmbed1ContextAttrs<'input>) -> Rc<BlockOrEmbed1ContextAll<'input>>  {
		Rc::new(
			BlockOrEmbed1ContextAll::BlockInlineContext(
				BaseParserRuleContext::copy_from(ctx,BlockInlineContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn blockOrEmbed1(&mut self,)
	-> Result<Rc<BlockOrEmbed1ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlockOrEmbed1ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_blockOrEmbed1);
        let mut _localctx: Rc<BlockOrEmbed1ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(406);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(33,&mut recog.base)? {
				1 =>{
					let tmp = BlockInlineContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(401);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(32,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule inlineStmt*/
							recog.base.set_state(396);
							recog.inlineStmt()?;

							recog.base.set_state(397);
							recog.base.match_token(T__38,&mut recog.err_handler)?;

							}
							} 
						}
						recog.base.set_state(403);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(32,&mut recog.base)?;
					}
					/*InvokeRule stmt*/
					recog.base.set_state(404);
					recog.stmt()?;

					}
				}
			,
				2 =>{
					let tmp = BlockIndentContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule indentedBlock*/
					recog.base.set_state(405);
					recog.indentedBlock()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
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
//------------------- stmt ----------------
#[derive(Debug)]
pub enum StmtContextAll<'input>{
	StmtsInlineContext(StmtsInlineContext<'input>),
	StmtWhileContext(StmtWhileContext<'input>),
	StmtForContext(StmtForContext<'input>),
	StmtIfContext(StmtIfContext<'input>),
	StmtTryContext(StmtTryContext<'input>),
	StmtCaseContext(StmtCaseContext<'input>),
Error(StmtContext<'input>)
}
antlr_rust::tid!{StmtContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for StmtContextAll<'input>{}

impl<'input> aslParserContext<'input> for StmtContextAll<'input>{}

impl<'input> Deref for StmtContextAll<'input>{
	type Target = dyn StmtContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use StmtContextAll::*;
		match self{
			StmtsInlineContext(inner) => inner,
			StmtWhileContext(inner) => inner,
			StmtForContext(inner) => inner,
			StmtIfContext(inner) => inner,
			StmtTryContext(inner) => inner,
			StmtCaseContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type StmtContext<'input> = BaseParserRuleContext<'input,StmtContextExt<'input>>;

#[derive(Clone)]
pub struct StmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for StmtContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtContext<'input>{
}

impl<'input> CustomRuleContext<'input> for StmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}
antlr_rust::tid!{StmtContextExt<'a>}

impl<'input> StmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StmtContextAll<'input>> {
		Rc::new(
		StmtContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StmtContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait StmtContextAttrs<'input>: aslParserContext<'input> + BorrowMut<StmtContextExt<'input>>{


}

impl<'input> StmtContextAttrs<'input> for StmtContext<'input>{}

pub type StmtsInlineContext<'input> = BaseParserRuleContext<'input,StmtsInlineContextExt<'input>>;

pub trait StmtsInlineContextAttrs<'input>: aslParserContext<'input>{
	fn inlineStmt(&self) -> Option<Rc<InlineStmtContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> StmtsInlineContextAttrs<'input> for StmtsInlineContext<'input>{}

pub struct StmtsInlineContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtsInlineContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtsInlineContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtsInlineContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtsInline(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtsInline(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtsInlineContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtsInline(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtsInlineContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for StmtsInlineContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for StmtsInlineContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for StmtsInlineContext<'input> {}

impl<'input> StmtsInlineContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::StmtsInlineContext(
				BaseParserRuleContext::copy_from(ctx,StmtsInlineContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtWhileContext<'input> = BaseParserRuleContext<'input,StmtWhileContextExt<'input>>;

pub trait StmtWhileContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn indentedBlock(&self) -> Option<Rc<IndentedBlockContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StmtWhileContextAttrs<'input> for StmtWhileContext<'input>{}

pub struct StmtWhileContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtWhileContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtWhileContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtWhileContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtWhile(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtWhile(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtWhileContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtWhile(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtWhileContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for StmtWhileContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for StmtWhileContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for StmtWhileContext<'input> {}

impl<'input> StmtWhileContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::StmtWhileContext(
				BaseParserRuleContext::copy_from(ctx,StmtWhileContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtForContext<'input> = BaseParserRuleContext<'input,StmtForContextExt<'input>>;

pub trait StmtForContextAttrs<'input>: aslParserContext<'input>{
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn indentedBlock(&self) -> Option<Rc<IndentedBlockContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> StmtForContextAttrs<'input> for StmtForContext<'input>{}

pub struct StmtForContextExt<'input>{
	base:StmtContextExt<'input>,
	pub begin: Option<Rc<ExprContextAll<'input>>>,
	pub direction: Option<TokenType<'input>>,
	pub end: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtForContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtForContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtForContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtFor(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtFor(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtForContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtFor(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtForContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for StmtForContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for StmtForContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for StmtForContext<'input> {}

impl<'input> StmtForContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::StmtForContext(
				BaseParserRuleContext::copy_from(ctx,StmtForContextExt{
					direction:None, 
        			begin:None, end:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtIfContext<'input> = BaseParserRuleContext<'input,StmtIfContextExt<'input>>;

pub trait StmtIfContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn blockOrEmbed1_all(&self) ->  Vec<Rc<BlockOrEmbed1ContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn blockOrEmbed1(&self, i: usize) -> Option<Rc<BlockOrEmbed1ContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn stmtElsIf_all(&self) ->  Vec<Rc<StmtElsIfContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn stmtElsIf(&self, i: usize) -> Option<Rc<StmtElsIfContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> StmtIfContextAttrs<'input> for StmtIfContext<'input>{}

pub struct StmtIfContextExt<'input>{
	base:StmtContextExt<'input>,
	pub test: Option<Rc<ExprContextAll<'input>>>,
	pub thenExpr: Option<Rc<BlockOrEmbed1ContextAll<'input>>>,
	pub elseExpr: Option<Rc<BlockOrEmbed1ContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtIfContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtIfContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtIfContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtIf(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtIf(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtIfContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtIf(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtIfContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for StmtIfContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for StmtIfContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for StmtIfContext<'input> {}

impl<'input> StmtIfContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::StmtIfContext(
				BaseParserRuleContext::copy_from(ctx,StmtIfContextExt{
        			test:None, thenExpr:None, elseExpr:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtTryContext<'input> = BaseParserRuleContext<'input,StmtTryContextExt<'input>>;

pub trait StmtTryContextAttrs<'input>: aslParserContext<'input>{
	fn indentedBlock(&self) -> Option<Rc<IndentedBlockContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token INDENT
	/// Returns `None` if there is no child corresponding to token INDENT
	fn INDENT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(INDENT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token DEDENT
	/// Returns `None` if there is no child corresponding to token DEDENT
	fn DEDENT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(DEDENT, 0)
	}
	fn catchAlt_all(&self) ->  Vec<Rc<CatchAltContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn catchAlt(&self, i: usize) -> Option<Rc<CatchAltContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> StmtTryContextAttrs<'input> for StmtTryContext<'input>{}

pub struct StmtTryContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtTryContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtTryContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtTryContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtTry(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtTry(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtTryContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtTry(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtTryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for StmtTryContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for StmtTryContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for StmtTryContext<'input> {}

impl<'input> StmtTryContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::StmtTryContext(
				BaseParserRuleContext::copy_from(ctx,StmtTryContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtCaseContext<'input> = BaseParserRuleContext<'input,StmtCaseContextExt<'input>>;

pub trait StmtCaseContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn caseAlt_all(&self) ->  Vec<Rc<CaseAltContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn caseAlt(&self, i: usize) -> Option<Rc<CaseAltContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> StmtCaseContextAttrs<'input> for StmtCaseContext<'input>{}

pub struct StmtCaseContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtCaseContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtCaseContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtCaseContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtCase(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtCase(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtCaseContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtCase(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtCaseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for StmtCaseContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for StmtCaseContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for StmtCaseContext<'input> {}

impl<'input> StmtCaseContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::StmtCaseContext(
				BaseParserRuleContext::copy_from(ctx,StmtCaseContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stmt(&mut self,)
	-> Result<Rc<StmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_stmt);
        let mut _localctx: Rc<StmtContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(458);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__3 | T__4 | T__31 | T__33 | T__34 | T__36 | T__37 | T__50 | T__51 |
			 T__52 | T__53 | T__54 | T__56 | T__57 | T__62 | T__64 | T__67 | T__88 |
			 T__89 | IDENTIFIER | SEE_TOK 
				=> {
					let tmp = StmtsInlineContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule inlineStmt*/
					recog.base.set_state(408);
					recog.inlineStmt()?;

					recog.base.set_state(409);
					_la = recog.base.input.la(1);
					if { !(_la==T__38 || _la==SEMICOLON) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}

			 T__39 
				=> {
					let tmp = StmtIfContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(411);
					recog.base.match_token(T__39,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(412);
					let tmp = recog.expr_rec(0)?;
					if let StmtContextAll::StmtIfContext(ctx) = cast_mut::<_,StmtContextAll >(&mut _localctx){
					ctx.test = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(413);
					recog.base.match_token(T__40,&mut recog.err_handler)?;

					/*InvokeRule blockOrEmbed1*/
					recog.base.set_state(414);
					let tmp = recog.blockOrEmbed1()?;
					if let StmtContextAll::StmtIfContext(ctx) = cast_mut::<_,StmtContextAll >(&mut _localctx){
					ctx.thenExpr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(418);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(34,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule stmtElsIf*/
							recog.base.set_state(415);
							recog.stmtElsIf()?;

							}
							} 
						}
						recog.base.set_state(420);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(34,&mut recog.base)?;
					}
					recog.base.set_state(423);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(35,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(421);
							recog.base.match_token(T__41,&mut recog.err_handler)?;

							/*InvokeRule blockOrEmbed1*/
							recog.base.set_state(422);
							let tmp = recog.blockOrEmbed1()?;
							if let StmtContextAll::StmtIfContext(ctx) = cast_mut::<_,StmtContextAll >(&mut _localctx){
							ctx.elseExpr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}

						_ => {}
					}
					}
				}

			 T__42 
				=> {
					let tmp = StmtCaseContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(425);
					recog.base.match_token(T__42,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(426);
					recog.expr_rec(0)?;

					recog.base.set_state(427);
					recog.base.match_token(T__7,&mut recog.err_handler)?;

					recog.base.set_state(429); 
					recog.err_handler.sync(&mut recog.base)?;
					_alt = 1;
					loop {
						match _alt {
						    x if x == 1=>
							{
							{
							/*InvokeRule caseAlt*/
							recog.base.set_state(428);
							recog.caseAlt()?;

							}
							}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
						}
						recog.base.set_state(431); 
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(36,&mut recog.base)?;
						if _alt==2 || _alt==INVALID_ALT { break }
					}
					}
				}

			 T__43 
				=> {
					let tmp = StmtForContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(433);
					recog.base.match_token(T__43,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(434);
					recog.id()?;

					recog.base.set_state(435);
					recog.base.match_token(T__29,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(436);
					let tmp = recog.expr_rec(0)?;
					if let StmtContextAll::StmtForContext(ctx) = cast_mut::<_,StmtContextAll >(&mut _localctx){
					ctx.begin = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(437);
					if let StmtContextAll::StmtForContext(ctx) = cast_mut::<_,StmtContextAll >(&mut _localctx){
					ctx.direction = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
					_la = recog.base.input.la(1);
					if { !(_la==T__44 || _la==T__45) } {
						let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
						if let StmtContextAll::StmtForContext(ctx) = cast_mut::<_,StmtContextAll >(&mut _localctx){
						ctx.direction = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule expr*/
					recog.base.set_state(438);
					let tmp = recog.expr_rec(0)?;
					if let StmtContextAll::StmtForContext(ctx) = cast_mut::<_,StmtContextAll >(&mut _localctx){
					ctx.end = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					/*InvokeRule indentedBlock*/
					recog.base.set_state(439);
					recog.indentedBlock()?;

					}
				}

			 T__46 
				=> {
					let tmp = StmtWhileContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(441);
					recog.base.match_token(T__46,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(442);
					recog.expr_rec(0)?;

					recog.base.set_state(443);
					recog.base.match_token(T__47,&mut recog.err_handler)?;

					/*InvokeRule indentedBlock*/
					recog.base.set_state(444);
					recog.indentedBlock()?;

					}
				}

			 T__48 
				=> {
					let tmp = StmtTryContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6);
					_localctx = tmp;
					{
					recog.base.set_state(446);
					recog.base.match_token(T__48,&mut recog.err_handler)?;

					/*InvokeRule indentedBlock*/
					recog.base.set_state(447);
					recog.indentedBlock()?;

					recog.base.set_state(448);
					recog.base.match_token(T__49,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(449);
					recog.id()?;

					recog.base.set_state(450);
					recog.base.match_token(INDENT,&mut recog.err_handler)?;

					recog.base.set_state(452); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						/*InvokeRule catchAlt*/
						recog.base.set_state(451);
						recog.catchAlt()?;

						}
						}
						recog.base.set_state(454); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==T__59 || _la==T__60) {break}
					}
					recog.base.set_state(456);
					recog.base.match_token(DEDENT,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
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
//------------------- inlineStmt ----------------
#[derive(Debug)]
pub enum InlineStmtContextAll<'input>{
	StmtReturnContext(StmtReturnContext<'input>),
	StmtUnpredictableContext(StmtUnpredictableContext<'input>),
	StmtSeeContext(StmtSeeContext<'input>),
	StmtDefEnumContext(StmtDefEnumContext<'input>),
	StmtImpDefContext(StmtImpDefContext<'input>),
	StmtAssignContext(StmtAssignContext<'input>),
	StmtVarDeclInitContext(StmtVarDeclInitContext<'input>),
	StmtCallContext(StmtCallContext<'input>),
	StmtAssertContext(StmtAssertContext<'input>),
	StmtRepeatContext(StmtRepeatContext<'input>),
	StmtUndefinedContext(StmtUndefinedContext<'input>),
	StmtThrowContext(StmtThrowContext<'input>),
	StmtVarsDeclContext(StmtVarsDeclContext<'input>),
	StmtConstDeclContext(StmtConstDeclContext<'input>),
Error(InlineStmtContext<'input>)
}
antlr_rust::tid!{InlineStmtContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for InlineStmtContextAll<'input>{}

impl<'input> aslParserContext<'input> for InlineStmtContextAll<'input>{}

impl<'input> Deref for InlineStmtContextAll<'input>{
	type Target = dyn InlineStmtContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use InlineStmtContextAll::*;
		match self{
			StmtReturnContext(inner) => inner,
			StmtUnpredictableContext(inner) => inner,
			StmtSeeContext(inner) => inner,
			StmtDefEnumContext(inner) => inner,
			StmtImpDefContext(inner) => inner,
			StmtAssignContext(inner) => inner,
			StmtVarDeclInitContext(inner) => inner,
			StmtCallContext(inner) => inner,
			StmtAssertContext(inner) => inner,
			StmtRepeatContext(inner) => inner,
			StmtUndefinedContext(inner) => inner,
			StmtThrowContext(inner) => inner,
			StmtVarsDeclContext(inner) => inner,
			StmtConstDeclContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for InlineStmtContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for InlineStmtContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type InlineStmtContext<'input> = BaseParserRuleContext<'input,InlineStmtContextExt<'input>>;

#[derive(Clone)]
pub struct InlineStmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for InlineStmtContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for InlineStmtContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for InlineStmtContext<'input>{
}

impl<'input> CustomRuleContext<'input> for InlineStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineStmt }
}
antlr_rust::tid!{InlineStmtContextExt<'a>}

impl<'input> InlineStmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InlineStmtContextAll<'input>> {
		Rc::new(
		InlineStmtContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InlineStmtContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait InlineStmtContextAttrs<'input>: aslParserContext<'input> + BorrowMut<InlineStmtContextExt<'input>>{


}

impl<'input> InlineStmtContextAttrs<'input> for InlineStmtContext<'input>{}

pub type StmtReturnContext<'input> = BaseParserRuleContext<'input,StmtReturnContextExt<'input>>;

pub trait StmtReturnContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StmtReturnContextAttrs<'input> for StmtReturnContext<'input>{}

pub struct StmtReturnContextExt<'input>{
	base:InlineStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtReturnContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtReturnContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtReturnContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtReturn(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtReturn(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtReturnContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtReturn(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtReturnContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineStmt }
}

impl<'input> Borrow<InlineStmtContextExt<'input>> for StmtReturnContext<'input>{
	fn borrow(&self) -> &InlineStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineStmtContextExt<'input>> for StmtReturnContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineStmtContextExt<'input> { &mut self.base }
}

impl<'input> InlineStmtContextAttrs<'input> for StmtReturnContext<'input> {}

impl<'input> StmtReturnContextExt<'input>{
	fn new(ctx: &dyn InlineStmtContextAttrs<'input>) -> Rc<InlineStmtContextAll<'input>>  {
		Rc::new(
			InlineStmtContextAll::StmtReturnContext(
				BaseParserRuleContext::copy_from(ctx,StmtReturnContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtUnpredictableContext<'input> = BaseParserRuleContext<'input,StmtUnpredictableContextExt<'input>>;

pub trait StmtUnpredictableContextAttrs<'input>: aslParserContext<'input>{
}

impl<'input> StmtUnpredictableContextAttrs<'input> for StmtUnpredictableContext<'input>{}

pub struct StmtUnpredictableContextExt<'input>{
	base:InlineStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtUnpredictableContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtUnpredictableContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtUnpredictableContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtUnpredictable(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtUnpredictable(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtUnpredictableContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtUnpredictable(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtUnpredictableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineStmt }
}

impl<'input> Borrow<InlineStmtContextExt<'input>> for StmtUnpredictableContext<'input>{
	fn borrow(&self) -> &InlineStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineStmtContextExt<'input>> for StmtUnpredictableContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineStmtContextExt<'input> { &mut self.base }
}

impl<'input> InlineStmtContextAttrs<'input> for StmtUnpredictableContext<'input> {}

impl<'input> StmtUnpredictableContextExt<'input>{
	fn new(ctx: &dyn InlineStmtContextAttrs<'input>) -> Rc<InlineStmtContextAll<'input>>  {
		Rc::new(
			InlineStmtContextAll::StmtUnpredictableContext(
				BaseParserRuleContext::copy_from(ctx,StmtUnpredictableContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtSeeContext<'input> = BaseParserRuleContext<'input,StmtSeeContextExt<'input>>;

pub trait StmtSeeContextAttrs<'input>: aslParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token SEE_TOK
	/// Returns `None` if there is no child corresponding to token SEE_TOK
	fn SEE_TOK(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(SEE_TOK, 0)
	}
}

impl<'input> StmtSeeContextAttrs<'input> for StmtSeeContext<'input>{}

pub struct StmtSeeContextExt<'input>{
	base:InlineStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtSeeContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtSeeContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtSeeContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtSee(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtSee(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtSeeContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtSee(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtSeeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineStmt }
}

impl<'input> Borrow<InlineStmtContextExt<'input>> for StmtSeeContext<'input>{
	fn borrow(&self) -> &InlineStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineStmtContextExt<'input>> for StmtSeeContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineStmtContextExt<'input> { &mut self.base }
}

impl<'input> InlineStmtContextAttrs<'input> for StmtSeeContext<'input> {}

impl<'input> StmtSeeContextExt<'input>{
	fn new(ctx: &dyn InlineStmtContextAttrs<'input>) -> Rc<InlineStmtContextAll<'input>>  {
		Rc::new(
			InlineStmtContextAll::StmtSeeContext(
				BaseParserRuleContext::copy_from(ctx,StmtSeeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtDefEnumContext<'input> = BaseParserRuleContext<'input,StmtDefEnumContextExt<'input>>;

pub trait StmtDefEnumContextAttrs<'input>: aslParserContext<'input>{
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn identifierCommaList0(&self) -> Option<Rc<IdentifierCommaList0ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StmtDefEnumContextAttrs<'input> for StmtDefEnumContext<'input>{}

pub struct StmtDefEnumContextExt<'input>{
	base:InlineStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtDefEnumContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtDefEnumContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtDefEnumContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtDefEnum(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtDefEnum(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtDefEnumContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtDefEnum(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtDefEnumContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineStmt }
}

impl<'input> Borrow<InlineStmtContextExt<'input>> for StmtDefEnumContext<'input>{
	fn borrow(&self) -> &InlineStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineStmtContextExt<'input>> for StmtDefEnumContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineStmtContextExt<'input> { &mut self.base }
}

impl<'input> InlineStmtContextAttrs<'input> for StmtDefEnumContext<'input> {}

impl<'input> StmtDefEnumContextExt<'input>{
	fn new(ctx: &dyn InlineStmtContextAttrs<'input>) -> Rc<InlineStmtContextAll<'input>>  {
		Rc::new(
			InlineStmtContextAll::StmtDefEnumContext(
				BaseParserRuleContext::copy_from(ctx,StmtDefEnumContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtImpDefContext<'input> = BaseParserRuleContext<'input,StmtImpDefContextExt<'input>>;

pub trait StmtImpDefContextAttrs<'input>: aslParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STRING_LIT
	/// Returns `None` if there is no child corresponding to token STRING_LIT
	fn STRING_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(STRING_LIT, 0)
	}
}

impl<'input> StmtImpDefContextAttrs<'input> for StmtImpDefContext<'input>{}

pub struct StmtImpDefContextExt<'input>{
	base:InlineStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtImpDefContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtImpDefContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtImpDefContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtImpDef(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtImpDef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtImpDefContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtImpDef(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtImpDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineStmt }
}

impl<'input> Borrow<InlineStmtContextExt<'input>> for StmtImpDefContext<'input>{
	fn borrow(&self) -> &InlineStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineStmtContextExt<'input>> for StmtImpDefContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineStmtContextExt<'input> { &mut self.base }
}

impl<'input> InlineStmtContextAttrs<'input> for StmtImpDefContext<'input> {}

impl<'input> StmtImpDefContextExt<'input>{
	fn new(ctx: &dyn InlineStmtContextAttrs<'input>) -> Rc<InlineStmtContextAll<'input>>  {
		Rc::new(
			InlineStmtContextAll::StmtImpDefContext(
				BaseParserRuleContext::copy_from(ctx,StmtImpDefContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtAssignContext<'input> = BaseParserRuleContext<'input,StmtAssignContextExt<'input>>;

pub trait StmtAssignContextAttrs<'input>: aslParserContext<'input>{
	fn lValExpr(&self) -> Option<Rc<LValExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StmtAssignContextAttrs<'input> for StmtAssignContext<'input>{}

pub struct StmtAssignContextExt<'input>{
	base:InlineStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtAssignContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtAssignContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtAssignContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtAssign(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtAssign(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtAssignContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtAssign(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtAssignContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineStmt }
}

impl<'input> Borrow<InlineStmtContextExt<'input>> for StmtAssignContext<'input>{
	fn borrow(&self) -> &InlineStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineStmtContextExt<'input>> for StmtAssignContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineStmtContextExt<'input> { &mut self.base }
}

impl<'input> InlineStmtContextAttrs<'input> for StmtAssignContext<'input> {}

impl<'input> StmtAssignContextExt<'input>{
	fn new(ctx: &dyn InlineStmtContextAttrs<'input>) -> Rc<InlineStmtContextAll<'input>>  {
		Rc::new(
			InlineStmtContextAll::StmtAssignContext(
				BaseParserRuleContext::copy_from(ctx,StmtAssignContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtVarDeclInitContext<'input> = BaseParserRuleContext<'input,StmtVarDeclInitContextExt<'input>>;

pub trait StmtVarDeclInitContextAttrs<'input>: aslParserContext<'input>{
	fn symDecl(&self) -> Option<Rc<SymDeclContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StmtVarDeclInitContextAttrs<'input> for StmtVarDeclInitContext<'input>{}

pub struct StmtVarDeclInitContextExt<'input>{
	base:InlineStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtVarDeclInitContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtVarDeclInitContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtVarDeclInitContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtVarDeclInit(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtVarDeclInit(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtVarDeclInitContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtVarDeclInit(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtVarDeclInitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineStmt }
}

impl<'input> Borrow<InlineStmtContextExt<'input>> for StmtVarDeclInitContext<'input>{
	fn borrow(&self) -> &InlineStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineStmtContextExt<'input>> for StmtVarDeclInitContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineStmtContextExt<'input> { &mut self.base }
}

impl<'input> InlineStmtContextAttrs<'input> for StmtVarDeclInitContext<'input> {}

impl<'input> StmtVarDeclInitContextExt<'input>{
	fn new(ctx: &dyn InlineStmtContextAttrs<'input>) -> Rc<InlineStmtContextAll<'input>>  {
		Rc::new(
			InlineStmtContextAll::StmtVarDeclInitContext(
				BaseParserRuleContext::copy_from(ctx,StmtVarDeclInitContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtCallContext<'input> = BaseParserRuleContext<'input,StmtCallContextExt<'input>>;

pub trait StmtCallContextAttrs<'input>: aslParserContext<'input>{
	fn qualId(&self) -> Option<Rc<QualIdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn exprCommaList0(&self) -> Option<Rc<ExprCommaList0ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StmtCallContextAttrs<'input> for StmtCallContext<'input>{}

pub struct StmtCallContextExt<'input>{
	base:InlineStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtCallContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtCallContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtCallContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtCall(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtCall(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtCallContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineStmt }
}

impl<'input> Borrow<InlineStmtContextExt<'input>> for StmtCallContext<'input>{
	fn borrow(&self) -> &InlineStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineStmtContextExt<'input>> for StmtCallContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineStmtContextExt<'input> { &mut self.base }
}

impl<'input> InlineStmtContextAttrs<'input> for StmtCallContext<'input> {}

impl<'input> StmtCallContextExt<'input>{
	fn new(ctx: &dyn InlineStmtContextAttrs<'input>) -> Rc<InlineStmtContextAll<'input>>  {
		Rc::new(
			InlineStmtContextAll::StmtCallContext(
				BaseParserRuleContext::copy_from(ctx,StmtCallContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtAssertContext<'input> = BaseParserRuleContext<'input,StmtAssertContextExt<'input>>;

pub trait StmtAssertContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StmtAssertContextAttrs<'input> for StmtAssertContext<'input>{}

pub struct StmtAssertContextExt<'input>{
	base:InlineStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtAssertContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtAssertContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtAssertContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtAssert(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtAssert(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtAssertContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtAssert(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtAssertContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineStmt }
}

impl<'input> Borrow<InlineStmtContextExt<'input>> for StmtAssertContext<'input>{
	fn borrow(&self) -> &InlineStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineStmtContextExt<'input>> for StmtAssertContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineStmtContextExt<'input> { &mut self.base }
}

impl<'input> InlineStmtContextAttrs<'input> for StmtAssertContext<'input> {}

impl<'input> StmtAssertContextExt<'input>{
	fn new(ctx: &dyn InlineStmtContextAttrs<'input>) -> Rc<InlineStmtContextAll<'input>>  {
		Rc::new(
			InlineStmtContextAll::StmtAssertContext(
				BaseParserRuleContext::copy_from(ctx,StmtAssertContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtRepeatContext<'input> = BaseParserRuleContext<'input,StmtRepeatContextExt<'input>>;

pub trait StmtRepeatContextAttrs<'input>: aslParserContext<'input>{
	fn indentedBlock(&self) -> Option<Rc<IndentedBlockContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StmtRepeatContextAttrs<'input> for StmtRepeatContext<'input>{}

pub struct StmtRepeatContextExt<'input>{
	base:InlineStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtRepeatContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtRepeatContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtRepeatContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtRepeat(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtRepeat(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtRepeatContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtRepeat(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtRepeatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineStmt }
}

impl<'input> Borrow<InlineStmtContextExt<'input>> for StmtRepeatContext<'input>{
	fn borrow(&self) -> &InlineStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineStmtContextExt<'input>> for StmtRepeatContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineStmtContextExt<'input> { &mut self.base }
}

impl<'input> InlineStmtContextAttrs<'input> for StmtRepeatContext<'input> {}

impl<'input> StmtRepeatContextExt<'input>{
	fn new(ctx: &dyn InlineStmtContextAttrs<'input>) -> Rc<InlineStmtContextAll<'input>>  {
		Rc::new(
			InlineStmtContextAll::StmtRepeatContext(
				BaseParserRuleContext::copy_from(ctx,StmtRepeatContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtUndefinedContext<'input> = BaseParserRuleContext<'input,StmtUndefinedContextExt<'input>>;

pub trait StmtUndefinedContextAttrs<'input>: aslParserContext<'input>{
}

impl<'input> StmtUndefinedContextAttrs<'input> for StmtUndefinedContext<'input>{}

pub struct StmtUndefinedContextExt<'input>{
	base:InlineStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtUndefinedContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtUndefinedContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtUndefinedContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtUndefined(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtUndefined(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtUndefinedContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtUndefined(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtUndefinedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineStmt }
}

impl<'input> Borrow<InlineStmtContextExt<'input>> for StmtUndefinedContext<'input>{
	fn borrow(&self) -> &InlineStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineStmtContextExt<'input>> for StmtUndefinedContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineStmtContextExt<'input> { &mut self.base }
}

impl<'input> InlineStmtContextAttrs<'input> for StmtUndefinedContext<'input> {}

impl<'input> StmtUndefinedContextExt<'input>{
	fn new(ctx: &dyn InlineStmtContextAttrs<'input>) -> Rc<InlineStmtContextAll<'input>>  {
		Rc::new(
			InlineStmtContextAll::StmtUndefinedContext(
				BaseParserRuleContext::copy_from(ctx,StmtUndefinedContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtThrowContext<'input> = BaseParserRuleContext<'input,StmtThrowContextExt<'input>>;

pub trait StmtThrowContextAttrs<'input>: aslParserContext<'input>{
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StmtThrowContextAttrs<'input> for StmtThrowContext<'input>{}

pub struct StmtThrowContextExt<'input>{
	base:InlineStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtThrowContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtThrowContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtThrowContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtThrow(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtThrow(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtThrowContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtThrow(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtThrowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineStmt }
}

impl<'input> Borrow<InlineStmtContextExt<'input>> for StmtThrowContext<'input>{
	fn borrow(&self) -> &InlineStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineStmtContextExt<'input>> for StmtThrowContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineStmtContextExt<'input> { &mut self.base }
}

impl<'input> InlineStmtContextAttrs<'input> for StmtThrowContext<'input> {}

impl<'input> StmtThrowContextExt<'input>{
	fn new(ctx: &dyn InlineStmtContextAttrs<'input>) -> Rc<InlineStmtContextAll<'input>>  {
		Rc::new(
			InlineStmtContextAll::StmtThrowContext(
				BaseParserRuleContext::copy_from(ctx,StmtThrowContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtVarsDeclContext<'input> = BaseParserRuleContext<'input,StmtVarsDeclContextExt<'input>>;

pub trait StmtVarsDeclContextAttrs<'input>: aslParserContext<'input>{
	fn typeSpec(&self) -> Option<Rc<TypeSpecContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn identifierCommaList1(&self) -> Option<Rc<IdentifierCommaList1ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StmtVarsDeclContextAttrs<'input> for StmtVarsDeclContext<'input>{}

pub struct StmtVarsDeclContextExt<'input>{
	base:InlineStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtVarsDeclContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtVarsDeclContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtVarsDeclContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtVarsDecl(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtVarsDecl(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtVarsDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtVarsDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtVarsDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineStmt }
}

impl<'input> Borrow<InlineStmtContextExt<'input>> for StmtVarsDeclContext<'input>{
	fn borrow(&self) -> &InlineStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineStmtContextExt<'input>> for StmtVarsDeclContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineStmtContextExt<'input> { &mut self.base }
}

impl<'input> InlineStmtContextAttrs<'input> for StmtVarsDeclContext<'input> {}

impl<'input> StmtVarsDeclContextExt<'input>{
	fn new(ctx: &dyn InlineStmtContextAttrs<'input>) -> Rc<InlineStmtContextAll<'input>>  {
		Rc::new(
			InlineStmtContextAll::StmtVarsDeclContext(
				BaseParserRuleContext::copy_from(ctx,StmtVarsDeclContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StmtConstDeclContext<'input> = BaseParserRuleContext<'input,StmtConstDeclContextExt<'input>>;

pub trait StmtConstDeclContextAttrs<'input>: aslParserContext<'input>{
	fn symDecl(&self) -> Option<Rc<SymDeclContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StmtConstDeclContextAttrs<'input> for StmtConstDeclContext<'input>{}

pub struct StmtConstDeclContextExt<'input>{
	base:InlineStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StmtConstDeclContextExt<'a>}

impl<'input> aslParserContext<'input> for StmtConstDeclContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtConstDeclContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StmtConstDecl(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_StmtConstDecl(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtConstDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_StmtConstDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtConstDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineStmt }
}

impl<'input> Borrow<InlineStmtContextExt<'input>> for StmtConstDeclContext<'input>{
	fn borrow(&self) -> &InlineStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<InlineStmtContextExt<'input>> for StmtConstDeclContext<'input>{
	fn borrow_mut(&mut self) -> &mut InlineStmtContextExt<'input> { &mut self.base }
}

impl<'input> InlineStmtContextAttrs<'input> for StmtConstDeclContext<'input> {}

impl<'input> StmtConstDeclContextExt<'input>{
	fn new(ctx: &dyn InlineStmtContextAttrs<'input>) -> Rc<InlineStmtContextAll<'input>>  {
		Rc::new(
			InlineStmtContextAll::StmtConstDeclContext(
				BaseParserRuleContext::copy_from(ctx,StmtConstDeclContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inlineStmt(&mut self,)
	-> Result<Rc<InlineStmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InlineStmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_inlineStmt);
        let mut _localctx: Rc<InlineStmtContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(505);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(40,&mut recog.base)? {
				1 =>{
					let tmp = StmtVarsDeclContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule typeSpec*/
					recog.base.set_state(460);
					recog.typeSpec()?;

					/*InvokeRule identifierCommaList1*/
					recog.base.set_state(461);
					recog.identifierCommaList1()?;

					}
				}
			,
				2 =>{
					let tmp = StmtVarDeclInitContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule symDecl*/
					recog.base.set_state(463);
					recog.symDecl()?;

					recog.base.set_state(464);
					recog.base.match_token(T__29,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(465);
					recog.expr_rec(0)?;

					}
				}
			,
				3 =>{
					let tmp = StmtConstDeclContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(467);
					recog.base.match_token(T__34,&mut recog.err_handler)?;

					/*InvokeRule symDecl*/
					recog.base.set_state(468);
					recog.symDecl()?;

					recog.base.set_state(469);
					recog.base.match_token(T__29,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(470);
					recog.expr_rec(0)?;

					}
				}
			,
				4 =>{
					let tmp = StmtAssignContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					/*InvokeRule lValExpr*/
					recog.base.set_state(472);
					recog.lValExpr_rec(0)?;

					recog.base.set_state(473);
					recog.base.match_token(T__29,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(474);
					recog.expr_rec(0)?;

					}
				}
			,
				5 =>{
					let tmp = StmtCallContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					/*InvokeRule qualId*/
					recog.base.set_state(476);
					recog.qualId()?;

					recog.base.set_state(477);
					recog.base.match_token(T__31,&mut recog.err_handler)?;

					/*InvokeRule exprCommaList0*/
					recog.base.set_state(478);
					recog.exprCommaList0()?;

					recog.base.set_state(479);
					recog.base.match_token(T__32,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					let tmp = StmtReturnContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6);
					_localctx = tmp;
					{
					recog.base.set_state(481);
					recog.base.match_token(T__50,&mut recog.err_handler)?;

					recog.base.set_state(483);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__3 || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (T__31 - 32)) | (1usize << (T__33 - 32)) | (1usize << (T__36 - 32)) | (1usize << (T__37 - 32)) | (1usize << (T__39 - 32)) | (1usize << (T__62 - 32)))) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (T__66 - 67)) | (1usize << (T__67 - 67)) | (1usize << (T__88 - 67)) | (1usize << (T__89 - 67)) | (1usize << (IDENTIFIER - 67)) | (1usize << (NAT_LIT - 67)) | (1usize << (HEX_LIT - 67)) | (1usize << (BIN_LIT - 67)) | (1usize << (MASK_LIT - 67)) | (1usize << (REAL_LIT - 67)) | (1usize << (STRING_LIT - 67)))) != 0) {
						{
						/*InvokeRule expr*/
						recog.base.set_state(482);
						recog.expr_rec(0)?;

						}
					}

					}
				}
			,
				7 =>{
					let tmp = StmtAssertContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7);
					_localctx = tmp;
					{
					recog.base.set_state(485);
					recog.base.match_token(T__51,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(486);
					recog.expr_rec(0)?;

					}
				}
			,
				8 =>{
					let tmp = StmtUnpredictableContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8);
					_localctx = tmp;
					{
					recog.base.set_state(487);
					recog.base.match_token(T__52,&mut recog.err_handler)?;

					}
				}
			,
				9 =>{
					let tmp = StmtImpDefContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 9);
					_localctx = tmp;
					{
					recog.base.set_state(488);
					recog.base.match_token(T__53,&mut recog.err_handler)?;

					recog.base.set_state(489);
					recog.base.match_token(STRING_LIT,&mut recog.err_handler)?;

					}
				}
			,
				10 =>{
					let tmp = StmtRepeatContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 10);
					_localctx = tmp;
					{
					recog.base.set_state(490);
					recog.base.match_token(T__54,&mut recog.err_handler)?;

					/*InvokeRule indentedBlock*/
					recog.base.set_state(491);
					recog.indentedBlock()?;

					recog.base.set_state(492);
					recog.base.match_token(T__55,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(493);
					recog.expr_rec(0)?;

					}
				}
			,
				11 =>{
					let tmp = StmtThrowContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 11);
					_localctx = tmp;
					{
					recog.base.set_state(495);
					recog.base.match_token(T__56,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(496);
					recog.id()?;

					}
				}
			,
				12 =>{
					let tmp = StmtUndefinedContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 12);
					_localctx = tmp;
					{
					recog.base.set_state(497);
					recog.base.match_token(T__57,&mut recog.err_handler)?;

					}
				}
			,
				13 =>{
					let tmp = StmtSeeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 13);
					_localctx = tmp;
					{
					recog.base.set_state(498);
					recog.base.match_token(SEE_TOK,&mut recog.err_handler)?;

					}
				}
			,
				14 =>{
					let tmp = StmtDefEnumContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 14);
					_localctx = tmp;
					{
					recog.base.set_state(499);
					recog.base.match_token(T__33,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(500);
					recog.id()?;

					recog.base.set_state(501);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					/*InvokeRule identifierCommaList0*/
					recog.base.set_state(502);
					recog.identifierCommaList0()?;

					recog.base.set_state(503);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
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
//------------------- stmtElsIf ----------------
pub type StmtElsIfContextAll<'input> = StmtElsIfContext<'input>;


pub type StmtElsIfContext<'input> = BaseParserRuleContext<'input,StmtElsIfContextExt<'input>>;

#[derive(Clone)]
pub struct StmtElsIfContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for StmtElsIfContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for StmtElsIfContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stmtElsIf(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_stmtElsIf(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for StmtElsIfContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_stmtElsIf(self);
	}
}

impl<'input> CustomRuleContext<'input> for StmtElsIfContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmtElsIf }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmtElsIf }
}
antlr_rust::tid!{StmtElsIfContextExt<'a>}

impl<'input> StmtElsIfContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StmtElsIfContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StmtElsIfContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StmtElsIfContextAttrs<'input>: aslParserContext<'input> + BorrowMut<StmtElsIfContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn blockOrEmbed1(&self) -> Option<Rc<BlockOrEmbed1ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StmtElsIfContextAttrs<'input> for StmtElsIfContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stmtElsIf(&mut self,)
	-> Result<Rc<StmtElsIfContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StmtElsIfContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_stmtElsIf);
        let mut _localctx: Rc<StmtElsIfContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(507);
			recog.base.match_token(T__58,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(508);
			recog.expr_rec(0)?;

			recog.base.set_state(509);
			recog.base.match_token(T__40,&mut recog.err_handler)?;

			/*InvokeRule blockOrEmbed1*/
			recog.base.set_state(510);
			recog.blockOrEmbed1()?;

			}
			Ok(())
		})();
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
//------------------- catchAlt ----------------
#[derive(Debug)]
pub enum CatchAltContextAll<'input>{
	CatchAltWhenContext(CatchAltWhenContext<'input>),
	CatchAltOtherwiseContext(CatchAltOtherwiseContext<'input>),
Error(CatchAltContext<'input>)
}
antlr_rust::tid!{CatchAltContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for CatchAltContextAll<'input>{}

impl<'input> aslParserContext<'input> for CatchAltContextAll<'input>{}

impl<'input> Deref for CatchAltContextAll<'input>{
	type Target = dyn CatchAltContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use CatchAltContextAll::*;
		match self{
			CatchAltWhenContext(inner) => inner,
			CatchAltOtherwiseContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CatchAltContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CatchAltContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type CatchAltContext<'input> = BaseParserRuleContext<'input,CatchAltContextExt<'input>>;

#[derive(Clone)]
pub struct CatchAltContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for CatchAltContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CatchAltContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CatchAltContext<'input>{
}

impl<'input> CustomRuleContext<'input> for CatchAltContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_catchAlt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_catchAlt }
}
antlr_rust::tid!{CatchAltContextExt<'a>}

impl<'input> CatchAltContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CatchAltContextAll<'input>> {
		Rc::new(
		CatchAltContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CatchAltContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait CatchAltContextAttrs<'input>: aslParserContext<'input> + BorrowMut<CatchAltContextExt<'input>>{


}

impl<'input> CatchAltContextAttrs<'input> for CatchAltContext<'input>{}

pub type CatchAltWhenContext<'input> = BaseParserRuleContext<'input,CatchAltWhenContextExt<'input>>;

pub trait CatchAltWhenContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn blockOrEmbed1(&self) -> Option<Rc<BlockOrEmbed1ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> CatchAltWhenContextAttrs<'input> for CatchAltWhenContext<'input>{}

pub struct CatchAltWhenContextExt<'input>{
	base:CatchAltContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CatchAltWhenContextExt<'a>}

impl<'input> aslParserContext<'input> for CatchAltWhenContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CatchAltWhenContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_CatchAltWhen(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_CatchAltWhen(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CatchAltWhenContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_CatchAltWhen(self);
	}
}

impl<'input> CustomRuleContext<'input> for CatchAltWhenContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_catchAlt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_catchAlt }
}

impl<'input> Borrow<CatchAltContextExt<'input>> for CatchAltWhenContext<'input>{
	fn borrow(&self) -> &CatchAltContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CatchAltContextExt<'input>> for CatchAltWhenContext<'input>{
	fn borrow_mut(&mut self) -> &mut CatchAltContextExt<'input> { &mut self.base }
}

impl<'input> CatchAltContextAttrs<'input> for CatchAltWhenContext<'input> {}

impl<'input> CatchAltWhenContextExt<'input>{
	fn new(ctx: &dyn CatchAltContextAttrs<'input>) -> Rc<CatchAltContextAll<'input>>  {
		Rc::new(
			CatchAltContextAll::CatchAltWhenContext(
				BaseParserRuleContext::copy_from(ctx,CatchAltWhenContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CatchAltOtherwiseContext<'input> = BaseParserRuleContext<'input,CatchAltOtherwiseContextExt<'input>>;

pub trait CatchAltOtherwiseContextAttrs<'input>: aslParserContext<'input>{
	fn blockOrEmbed1(&self) -> Option<Rc<BlockOrEmbed1ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> CatchAltOtherwiseContextAttrs<'input> for CatchAltOtherwiseContext<'input>{}

pub struct CatchAltOtherwiseContextExt<'input>{
	base:CatchAltContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CatchAltOtherwiseContextExt<'a>}

impl<'input> aslParserContext<'input> for CatchAltOtherwiseContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CatchAltOtherwiseContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_CatchAltOtherwise(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_CatchAltOtherwise(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CatchAltOtherwiseContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_CatchAltOtherwise(self);
	}
}

impl<'input> CustomRuleContext<'input> for CatchAltOtherwiseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_catchAlt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_catchAlt }
}

impl<'input> Borrow<CatchAltContextExt<'input>> for CatchAltOtherwiseContext<'input>{
	fn borrow(&self) -> &CatchAltContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CatchAltContextExt<'input>> for CatchAltOtherwiseContext<'input>{
	fn borrow_mut(&mut self) -> &mut CatchAltContextExt<'input> { &mut self.base }
}

impl<'input> CatchAltContextAttrs<'input> for CatchAltOtherwiseContext<'input> {}

impl<'input> CatchAltOtherwiseContextExt<'input>{
	fn new(ctx: &dyn CatchAltContextAttrs<'input>) -> Rc<CatchAltContextAll<'input>>  {
		Rc::new(
			CatchAltContextAll::CatchAltOtherwiseContext(
				BaseParserRuleContext::copy_from(ctx,CatchAltOtherwiseContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn catchAlt(&mut self,)
	-> Result<Rc<CatchAltContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CatchAltContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_catchAlt);
        let mut _localctx: Rc<CatchAltContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(518);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__59 
				=> {
					let tmp = CatchAltWhenContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(512);
					recog.base.match_token(T__59,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(513);
					recog.expr_rec(0)?;

					/*InvokeRule blockOrEmbed1*/
					recog.base.set_state(514);
					recog.blockOrEmbed1()?;

					}
				}

			 T__60 
				=> {
					let tmp = CatchAltOtherwiseContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(516);
					recog.base.match_token(T__60,&mut recog.err_handler)?;

					/*InvokeRule blockOrEmbed1*/
					recog.base.set_state(517);
					recog.blockOrEmbed1()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
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
//------------------- caseAlt ----------------
#[derive(Debug)]
pub enum CaseAltContextAll<'input>{
	CaseAltWhenContext(CaseAltWhenContext<'input>),
	CaseAltOtherwiseContext(CaseAltOtherwiseContext<'input>),
Error(CaseAltContext<'input>)
}
antlr_rust::tid!{CaseAltContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for CaseAltContextAll<'input>{}

impl<'input> aslParserContext<'input> for CaseAltContextAll<'input>{}

impl<'input> Deref for CaseAltContextAll<'input>{
	type Target = dyn CaseAltContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use CaseAltContextAll::*;
		match self{
			CaseAltWhenContext(inner) => inner,
			CaseAltOtherwiseContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CaseAltContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CaseAltContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type CaseAltContext<'input> = BaseParserRuleContext<'input,CaseAltContextExt<'input>>;

#[derive(Clone)]
pub struct CaseAltContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for CaseAltContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CaseAltContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CaseAltContext<'input>{
}

impl<'input> CustomRuleContext<'input> for CaseAltContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_caseAlt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_caseAlt }
}
antlr_rust::tid!{CaseAltContextExt<'a>}

impl<'input> CaseAltContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CaseAltContextAll<'input>> {
		Rc::new(
		CaseAltContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CaseAltContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait CaseAltContextAttrs<'input>: aslParserContext<'input> + BorrowMut<CaseAltContextExt<'input>>{


}

impl<'input> CaseAltContextAttrs<'input> for CaseAltContext<'input>{}

pub type CaseAltWhenContext<'input> = BaseParserRuleContext<'input,CaseAltWhenContextExt<'input>>;

pub trait CaseAltWhenContextAttrs<'input>: aslParserContext<'input>{
	fn casePattern_all(&self) ->  Vec<Rc<CasePatternContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn casePattern(&self, i: usize) -> Option<Rc<CasePatternContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn blockOrEmbed0(&self) -> Option<Rc<BlockOrEmbed0ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> CaseAltWhenContextAttrs<'input> for CaseAltWhenContext<'input>{}

pub struct CaseAltWhenContextExt<'input>{
	base:CaseAltContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CaseAltWhenContextExt<'a>}

impl<'input> aslParserContext<'input> for CaseAltWhenContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CaseAltWhenContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_CaseAltWhen(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_CaseAltWhen(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CaseAltWhenContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_CaseAltWhen(self);
	}
}

impl<'input> CustomRuleContext<'input> for CaseAltWhenContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_caseAlt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_caseAlt }
}

impl<'input> Borrow<CaseAltContextExt<'input>> for CaseAltWhenContext<'input>{
	fn borrow(&self) -> &CaseAltContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CaseAltContextExt<'input>> for CaseAltWhenContext<'input>{
	fn borrow_mut(&mut self) -> &mut CaseAltContextExt<'input> { &mut self.base }
}

impl<'input> CaseAltContextAttrs<'input> for CaseAltWhenContext<'input> {}

impl<'input> CaseAltWhenContextExt<'input>{
	fn new(ctx: &dyn CaseAltContextAttrs<'input>) -> Rc<CaseAltContextAll<'input>>  {
		Rc::new(
			CaseAltContextAll::CaseAltWhenContext(
				BaseParserRuleContext::copy_from(ctx,CaseAltWhenContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CaseAltOtherwiseContext<'input> = BaseParserRuleContext<'input,CaseAltOtherwiseContextExt<'input>>;

pub trait CaseAltOtherwiseContextAttrs<'input>: aslParserContext<'input>{
	fn blockOrEmbed0(&self) -> Option<Rc<BlockOrEmbed0ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> CaseAltOtherwiseContextAttrs<'input> for CaseAltOtherwiseContext<'input>{}

pub struct CaseAltOtherwiseContextExt<'input>{
	base:CaseAltContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CaseAltOtherwiseContextExt<'a>}

impl<'input> aslParserContext<'input> for CaseAltOtherwiseContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CaseAltOtherwiseContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_CaseAltOtherwise(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_CaseAltOtherwise(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CaseAltOtherwiseContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_CaseAltOtherwise(self);
	}
}

impl<'input> CustomRuleContext<'input> for CaseAltOtherwiseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_caseAlt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_caseAlt }
}

impl<'input> Borrow<CaseAltContextExt<'input>> for CaseAltOtherwiseContext<'input>{
	fn borrow(&self) -> &CaseAltContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CaseAltContextExt<'input>> for CaseAltOtherwiseContext<'input>{
	fn borrow_mut(&mut self) -> &mut CaseAltContextExt<'input> { &mut self.base }
}

impl<'input> CaseAltContextAttrs<'input> for CaseAltOtherwiseContext<'input> {}

impl<'input> CaseAltOtherwiseContextExt<'input>{
	fn new(ctx: &dyn CaseAltContextAttrs<'input>) -> Rc<CaseAltContextAll<'input>>  {
		Rc::new(
			CaseAltContextAll::CaseAltOtherwiseContext(
				BaseParserRuleContext::copy_from(ctx,CaseAltOtherwiseContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn caseAlt(&mut self,)
	-> Result<Rc<CaseAltContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CaseAltContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_caseAlt);
        let mut _localctx: Rc<CaseAltContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(537);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__59 
				=> {
					let tmp = CaseAltWhenContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(520);
					recog.base.match_token(T__59,&mut recog.err_handler)?;

					/*InvokeRule casePattern*/
					recog.base.set_state(521);
					recog.casePattern()?;

					recog.base.set_state(526);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__9 {
						{
						{
						recog.base.set_state(522);
						recog.base.match_token(T__9,&mut recog.err_handler)?;

						/*InvokeRule casePattern*/
						recog.base.set_state(523);
						recog.casePattern()?;

						}
						}
						recog.base.set_state(528);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(531);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__61 {
						{
						recog.base.set_state(529);
						recog.base.match_token(T__61,&mut recog.err_handler)?;

						/*InvokeRule expr*/
						recog.base.set_state(530);
						recog.expr_rec(0)?;

						}
					}

					/*InvokeRule blockOrEmbed0*/
					recog.base.set_state(533);
					recog.blockOrEmbed0()?;

					}
				}

			 T__60 
				=> {
					let tmp = CaseAltOtherwiseContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(535);
					recog.base.match_token(T__60,&mut recog.err_handler)?;

					/*InvokeRule blockOrEmbed0*/
					recog.base.set_state(536);
					recog.blockOrEmbed0()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
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
//------------------- casePattern ----------------
#[derive(Debug)]
pub enum CasePatternContextAll<'input>{
	CasePatternHexContext(CasePatternHexContext<'input>),
	CasePatternMaskContext(CasePatternMaskContext<'input>),
	CasePatternBindContext(CasePatternBindContext<'input>),
	CasePatternNatContext(CasePatternNatContext<'input>),
	CasePatternTupleContext(CasePatternTupleContext<'input>),
	CasePatternIgnoreContext(CasePatternIgnoreContext<'input>),
	CasePatternBinContext(CasePatternBinContext<'input>),
Error(CasePatternContext<'input>)
}
antlr_rust::tid!{CasePatternContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for CasePatternContextAll<'input>{}

impl<'input> aslParserContext<'input> for CasePatternContextAll<'input>{}

impl<'input> Deref for CasePatternContextAll<'input>{
	type Target = dyn CasePatternContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use CasePatternContextAll::*;
		match self{
			CasePatternHexContext(inner) => inner,
			CasePatternMaskContext(inner) => inner,
			CasePatternBindContext(inner) => inner,
			CasePatternNatContext(inner) => inner,
			CasePatternTupleContext(inner) => inner,
			CasePatternIgnoreContext(inner) => inner,
			CasePatternBinContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CasePatternContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CasePatternContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type CasePatternContext<'input> = BaseParserRuleContext<'input,CasePatternContextExt<'input>>;

#[derive(Clone)]
pub struct CasePatternContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for CasePatternContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CasePatternContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CasePatternContext<'input>{
}

impl<'input> CustomRuleContext<'input> for CasePatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_casePattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_casePattern }
}
antlr_rust::tid!{CasePatternContextExt<'a>}

impl<'input> CasePatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CasePatternContextAll<'input>> {
		Rc::new(
		CasePatternContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CasePatternContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait CasePatternContextAttrs<'input>: aslParserContext<'input> + BorrowMut<CasePatternContextExt<'input>>{


}

impl<'input> CasePatternContextAttrs<'input> for CasePatternContext<'input>{}

pub type CasePatternHexContext<'input> = BaseParserRuleContext<'input,CasePatternHexContextExt<'input>>;

pub trait CasePatternHexContextAttrs<'input>: aslParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token HEX_LIT
	/// Returns `None` if there is no child corresponding to token HEX_LIT
	fn HEX_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(HEX_LIT, 0)
	}
}

impl<'input> CasePatternHexContextAttrs<'input> for CasePatternHexContext<'input>{}

pub struct CasePatternHexContextExt<'input>{
	base:CasePatternContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CasePatternHexContextExt<'a>}

impl<'input> aslParserContext<'input> for CasePatternHexContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CasePatternHexContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_CasePatternHex(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_CasePatternHex(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CasePatternHexContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_CasePatternHex(self);
	}
}

impl<'input> CustomRuleContext<'input> for CasePatternHexContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_casePattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_casePattern }
}

impl<'input> Borrow<CasePatternContextExt<'input>> for CasePatternHexContext<'input>{
	fn borrow(&self) -> &CasePatternContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CasePatternContextExt<'input>> for CasePatternHexContext<'input>{
	fn borrow_mut(&mut self) -> &mut CasePatternContextExt<'input> { &mut self.base }
}

impl<'input> CasePatternContextAttrs<'input> for CasePatternHexContext<'input> {}

impl<'input> CasePatternHexContextExt<'input>{
	fn new(ctx: &dyn CasePatternContextAttrs<'input>) -> Rc<CasePatternContextAll<'input>>  {
		Rc::new(
			CasePatternContextAll::CasePatternHexContext(
				BaseParserRuleContext::copy_from(ctx,CasePatternHexContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CasePatternMaskContext<'input> = BaseParserRuleContext<'input,CasePatternMaskContextExt<'input>>;

pub trait CasePatternMaskContextAttrs<'input>: aslParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token MASK_LIT
	/// Returns `None` if there is no child corresponding to token MASK_LIT
	fn MASK_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(MASK_LIT, 0)
	}
}

impl<'input> CasePatternMaskContextAttrs<'input> for CasePatternMaskContext<'input>{}

pub struct CasePatternMaskContextExt<'input>{
	base:CasePatternContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CasePatternMaskContextExt<'a>}

impl<'input> aslParserContext<'input> for CasePatternMaskContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CasePatternMaskContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_CasePatternMask(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_CasePatternMask(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CasePatternMaskContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_CasePatternMask(self);
	}
}

impl<'input> CustomRuleContext<'input> for CasePatternMaskContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_casePattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_casePattern }
}

impl<'input> Borrow<CasePatternContextExt<'input>> for CasePatternMaskContext<'input>{
	fn borrow(&self) -> &CasePatternContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CasePatternContextExt<'input>> for CasePatternMaskContext<'input>{
	fn borrow_mut(&mut self) -> &mut CasePatternContextExt<'input> { &mut self.base }
}

impl<'input> CasePatternContextAttrs<'input> for CasePatternMaskContext<'input> {}

impl<'input> CasePatternMaskContextExt<'input>{
	fn new(ctx: &dyn CasePatternContextAttrs<'input>) -> Rc<CasePatternContextAll<'input>>  {
		Rc::new(
			CasePatternContextAll::CasePatternMaskContext(
				BaseParserRuleContext::copy_from(ctx,CasePatternMaskContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CasePatternBindContext<'input> = BaseParserRuleContext<'input,CasePatternBindContextExt<'input>>;

pub trait CasePatternBindContextAttrs<'input>: aslParserContext<'input>{
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> CasePatternBindContextAttrs<'input> for CasePatternBindContext<'input>{}

pub struct CasePatternBindContextExt<'input>{
	base:CasePatternContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CasePatternBindContextExt<'a>}

impl<'input> aslParserContext<'input> for CasePatternBindContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CasePatternBindContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_CasePatternBind(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_CasePatternBind(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CasePatternBindContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_CasePatternBind(self);
	}
}

impl<'input> CustomRuleContext<'input> for CasePatternBindContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_casePattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_casePattern }
}

impl<'input> Borrow<CasePatternContextExt<'input>> for CasePatternBindContext<'input>{
	fn borrow(&self) -> &CasePatternContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CasePatternContextExt<'input>> for CasePatternBindContext<'input>{
	fn borrow_mut(&mut self) -> &mut CasePatternContextExt<'input> { &mut self.base }
}

impl<'input> CasePatternContextAttrs<'input> for CasePatternBindContext<'input> {}

impl<'input> CasePatternBindContextExt<'input>{
	fn new(ctx: &dyn CasePatternContextAttrs<'input>) -> Rc<CasePatternContextAll<'input>>  {
		Rc::new(
			CasePatternContextAll::CasePatternBindContext(
				BaseParserRuleContext::copy_from(ctx,CasePatternBindContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CasePatternNatContext<'input> = BaseParserRuleContext<'input,CasePatternNatContextExt<'input>>;

pub trait CasePatternNatContextAttrs<'input>: aslParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NAT_LIT
	/// Returns `None` if there is no child corresponding to token NAT_LIT
	fn NAT_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(NAT_LIT, 0)
	}
}

impl<'input> CasePatternNatContextAttrs<'input> for CasePatternNatContext<'input>{}

pub struct CasePatternNatContextExt<'input>{
	base:CasePatternContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CasePatternNatContextExt<'a>}

impl<'input> aslParserContext<'input> for CasePatternNatContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CasePatternNatContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_CasePatternNat(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_CasePatternNat(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CasePatternNatContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_CasePatternNat(self);
	}
}

impl<'input> CustomRuleContext<'input> for CasePatternNatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_casePattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_casePattern }
}

impl<'input> Borrow<CasePatternContextExt<'input>> for CasePatternNatContext<'input>{
	fn borrow(&self) -> &CasePatternContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CasePatternContextExt<'input>> for CasePatternNatContext<'input>{
	fn borrow_mut(&mut self) -> &mut CasePatternContextExt<'input> { &mut self.base }
}

impl<'input> CasePatternContextAttrs<'input> for CasePatternNatContext<'input> {}

impl<'input> CasePatternNatContextExt<'input>{
	fn new(ctx: &dyn CasePatternContextAttrs<'input>) -> Rc<CasePatternContextAll<'input>>  {
		Rc::new(
			CasePatternContextAll::CasePatternNatContext(
				BaseParserRuleContext::copy_from(ctx,CasePatternNatContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CasePatternTupleContext<'input> = BaseParserRuleContext<'input,CasePatternTupleContextExt<'input>>;

pub trait CasePatternTupleContextAttrs<'input>: aslParserContext<'input>{
	fn casePattern_all(&self) ->  Vec<Rc<CasePatternContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn casePattern(&self, i: usize) -> Option<Rc<CasePatternContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> CasePatternTupleContextAttrs<'input> for CasePatternTupleContext<'input>{}

pub struct CasePatternTupleContextExt<'input>{
	base:CasePatternContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CasePatternTupleContextExt<'a>}

impl<'input> aslParserContext<'input> for CasePatternTupleContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CasePatternTupleContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_CasePatternTuple(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_CasePatternTuple(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CasePatternTupleContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_CasePatternTuple(self);
	}
}

impl<'input> CustomRuleContext<'input> for CasePatternTupleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_casePattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_casePattern }
}

impl<'input> Borrow<CasePatternContextExt<'input>> for CasePatternTupleContext<'input>{
	fn borrow(&self) -> &CasePatternContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CasePatternContextExt<'input>> for CasePatternTupleContext<'input>{
	fn borrow_mut(&mut self) -> &mut CasePatternContextExt<'input> { &mut self.base }
}

impl<'input> CasePatternContextAttrs<'input> for CasePatternTupleContext<'input> {}

impl<'input> CasePatternTupleContextExt<'input>{
	fn new(ctx: &dyn CasePatternContextAttrs<'input>) -> Rc<CasePatternContextAll<'input>>  {
		Rc::new(
			CasePatternContextAll::CasePatternTupleContext(
				BaseParserRuleContext::copy_from(ctx,CasePatternTupleContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CasePatternIgnoreContext<'input> = BaseParserRuleContext<'input,CasePatternIgnoreContextExt<'input>>;

pub trait CasePatternIgnoreContextAttrs<'input>: aslParserContext<'input>{
}

impl<'input> CasePatternIgnoreContextAttrs<'input> for CasePatternIgnoreContext<'input>{}

pub struct CasePatternIgnoreContextExt<'input>{
	base:CasePatternContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CasePatternIgnoreContextExt<'a>}

impl<'input> aslParserContext<'input> for CasePatternIgnoreContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CasePatternIgnoreContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_CasePatternIgnore(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_CasePatternIgnore(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CasePatternIgnoreContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_CasePatternIgnore(self);
	}
}

impl<'input> CustomRuleContext<'input> for CasePatternIgnoreContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_casePattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_casePattern }
}

impl<'input> Borrow<CasePatternContextExt<'input>> for CasePatternIgnoreContext<'input>{
	fn borrow(&self) -> &CasePatternContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CasePatternContextExt<'input>> for CasePatternIgnoreContext<'input>{
	fn borrow_mut(&mut self) -> &mut CasePatternContextExt<'input> { &mut self.base }
}

impl<'input> CasePatternContextAttrs<'input> for CasePatternIgnoreContext<'input> {}

impl<'input> CasePatternIgnoreContextExt<'input>{
	fn new(ctx: &dyn CasePatternContextAttrs<'input>) -> Rc<CasePatternContextAll<'input>>  {
		Rc::new(
			CasePatternContextAll::CasePatternIgnoreContext(
				BaseParserRuleContext::copy_from(ctx,CasePatternIgnoreContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CasePatternBinContext<'input> = BaseParserRuleContext<'input,CasePatternBinContextExt<'input>>;

pub trait CasePatternBinContextAttrs<'input>: aslParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token BIN_LIT
	/// Returns `None` if there is no child corresponding to token BIN_LIT
	fn BIN_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(BIN_LIT, 0)
	}
}

impl<'input> CasePatternBinContextAttrs<'input> for CasePatternBinContext<'input>{}

pub struct CasePatternBinContextExt<'input>{
	base:CasePatternContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CasePatternBinContextExt<'a>}

impl<'input> aslParserContext<'input> for CasePatternBinContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for CasePatternBinContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_CasePatternBin(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_CasePatternBin(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for CasePatternBinContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_CasePatternBin(self);
	}
}

impl<'input> CustomRuleContext<'input> for CasePatternBinContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_casePattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_casePattern }
}

impl<'input> Borrow<CasePatternContextExt<'input>> for CasePatternBinContext<'input>{
	fn borrow(&self) -> &CasePatternContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CasePatternContextExt<'input>> for CasePatternBinContext<'input>{
	fn borrow_mut(&mut self) -> &mut CasePatternContextExt<'input> { &mut self.base }
}

impl<'input> CasePatternContextAttrs<'input> for CasePatternBinContext<'input> {}

impl<'input> CasePatternBinContextExt<'input>{
	fn new(ctx: &dyn CasePatternContextAttrs<'input>) -> Rc<CasePatternContextAll<'input>>  {
		Rc::new(
			CasePatternContextAll::CasePatternBinContext(
				BaseParserRuleContext::copy_from(ctx,CasePatternBinContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn casePattern(&mut self,)
	-> Result<Rc<CasePatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CasePatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_casePattern);
        let mut _localctx: Rc<CasePatternContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(556);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 NAT_LIT 
				=> {
					let tmp = CasePatternNatContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(539);
					recog.base.match_token(NAT_LIT,&mut recog.err_handler)?;

					}
				}

			 HEX_LIT 
				=> {
					let tmp = CasePatternHexContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(540);
					recog.base.match_token(HEX_LIT,&mut recog.err_handler)?;

					}
				}

			 BIN_LIT 
				=> {
					let tmp = CasePatternBinContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(541);
					recog.base.match_token(BIN_LIT,&mut recog.err_handler)?;

					}
				}

			 MASK_LIT 
				=> {
					let tmp = CasePatternMaskContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(542);
					recog.base.match_token(MASK_LIT,&mut recog.err_handler)?;

					}
				}

			 T__33 | T__37 | T__67 | IDENTIFIER 
				=> {
					let tmp = CasePatternBindContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					/*InvokeRule id*/
					recog.base.set_state(543);
					recog.id()?;

					}
				}

			 T__62 
				=> {
					let tmp = CasePatternIgnoreContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6);
					_localctx = tmp;
					{
					recog.base.set_state(544);
					recog.base.match_token(T__62,&mut recog.err_handler)?;

					}
				}

			 T__31 
				=> {
					let tmp = CasePatternTupleContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7);
					_localctx = tmp;
					{
					recog.base.set_state(545);
					recog.base.match_token(T__31,&mut recog.err_handler)?;

					/*InvokeRule casePattern*/
					recog.base.set_state(546);
					recog.casePattern()?;

					recog.base.set_state(551);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__9 {
						{
						{
						recog.base.set_state(547);
						recog.base.match_token(T__9,&mut recog.err_handler)?;

						/*InvokeRule casePattern*/
						recog.base.set_state(548);
						recog.casePattern()?;

						}
						}
						recog.base.set_state(553);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(554);
					recog.base.match_token(T__32,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
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
//------------------- lValExpr ----------------
#[derive(Debug)]
pub enum LValExprContextAll<'input>{
	LValSliceOfContext(LValSliceOfContext<'input>),
	LValArrayContext(LValArrayContext<'input>),
	LValArrayIndexContext(LValArrayIndexContext<'input>),
	LValMemberBitsContext(LValMemberBitsContext<'input>),
	LValTupleContext(LValTupleContext<'input>),
	LValMemberArrayContext(LValMemberArrayContext<'input>),
	LValIgnoreContext(LValIgnoreContext<'input>),
	LValMemberContext(LValMemberContext<'input>),
	LValVarRefContext(LValVarRefContext<'input>),
	LValSliceContext(LValSliceContext<'input>),
Error(LValExprContext<'input>)
}
antlr_rust::tid!{LValExprContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for LValExprContextAll<'input>{}

impl<'input> aslParserContext<'input> for LValExprContextAll<'input>{}

impl<'input> Deref for LValExprContextAll<'input>{
	type Target = dyn LValExprContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use LValExprContextAll::*;
		match self{
			LValSliceOfContext(inner) => inner,
			LValArrayContext(inner) => inner,
			LValArrayIndexContext(inner) => inner,
			LValMemberBitsContext(inner) => inner,
			LValTupleContext(inner) => inner,
			LValMemberArrayContext(inner) => inner,
			LValIgnoreContext(inner) => inner,
			LValMemberContext(inner) => inner,
			LValVarRefContext(inner) => inner,
			LValSliceContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for LValExprContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for LValExprContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type LValExprContext<'input> = BaseParserRuleContext<'input,LValExprContextExt<'input>>;

#[derive(Clone)]
pub struct LValExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for LValExprContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for LValExprContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for LValExprContext<'input>{
}

impl<'input> CustomRuleContext<'input> for LValExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lValExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lValExpr }
}
antlr_rust::tid!{LValExprContextExt<'a>}

impl<'input> LValExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LValExprContextAll<'input>> {
		Rc::new(
		LValExprContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LValExprContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait LValExprContextAttrs<'input>: aslParserContext<'input> + BorrowMut<LValExprContextExt<'input>>{


}

impl<'input> LValExprContextAttrs<'input> for LValExprContext<'input>{}

pub type LValSliceOfContext<'input> = BaseParserRuleContext<'input,LValSliceOfContextExt<'input>>;

pub trait LValSliceOfContextAttrs<'input>: aslParserContext<'input>{
	fn lValExpr(&self) -> Option<Rc<LValExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn sliceCommaList1(&self) -> Option<Rc<SliceCommaList1ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> LValSliceOfContextAttrs<'input> for LValSliceOfContext<'input>{}

pub struct LValSliceOfContextExt<'input>{
	base:LValExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LValSliceOfContextExt<'a>}

impl<'input> aslParserContext<'input> for LValSliceOfContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for LValSliceOfContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LValSliceOf(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_LValSliceOf(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for LValSliceOfContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_LValSliceOf(self);
	}
}

impl<'input> CustomRuleContext<'input> for LValSliceOfContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lValExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lValExpr }
}

impl<'input> Borrow<LValExprContextExt<'input>> for LValSliceOfContext<'input>{
	fn borrow(&self) -> &LValExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LValExprContextExt<'input>> for LValSliceOfContext<'input>{
	fn borrow_mut(&mut self) -> &mut LValExprContextExt<'input> { &mut self.base }
}

impl<'input> LValExprContextAttrs<'input> for LValSliceOfContext<'input> {}

impl<'input> LValSliceOfContextExt<'input>{
	fn new(ctx: &dyn LValExprContextAttrs<'input>) -> Rc<LValExprContextAll<'input>>  {
		Rc::new(
			LValExprContextAll::LValSliceOfContext(
				BaseParserRuleContext::copy_from(ctx,LValSliceOfContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LValArrayContext<'input> = BaseParserRuleContext<'input,LValArrayContextExt<'input>>;

pub trait LValArrayContextAttrs<'input>: aslParserContext<'input>{
	fn lValExpr_all(&self) ->  Vec<Rc<LValExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn lValExpr(&self, i: usize) -> Option<Rc<LValExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> LValArrayContextAttrs<'input> for LValArrayContext<'input>{}

pub struct LValArrayContextExt<'input>{
	base:LValExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LValArrayContextExt<'a>}

impl<'input> aslParserContext<'input> for LValArrayContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for LValArrayContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LValArray(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_LValArray(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for LValArrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_LValArray(self);
	}
}

impl<'input> CustomRuleContext<'input> for LValArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lValExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lValExpr }
}

impl<'input> Borrow<LValExprContextExt<'input>> for LValArrayContext<'input>{
	fn borrow(&self) -> &LValExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LValExprContextExt<'input>> for LValArrayContext<'input>{
	fn borrow_mut(&mut self) -> &mut LValExprContextExt<'input> { &mut self.base }
}

impl<'input> LValExprContextAttrs<'input> for LValArrayContext<'input> {}

impl<'input> LValArrayContextExt<'input>{
	fn new(ctx: &dyn LValExprContextAttrs<'input>) -> Rc<LValExprContextAll<'input>>  {
		Rc::new(
			LValExprContextAll::LValArrayContext(
				BaseParserRuleContext::copy_from(ctx,LValArrayContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LValArrayIndexContext<'input> = BaseParserRuleContext<'input,LValArrayIndexContextExt<'input>>;

pub trait LValArrayIndexContextAttrs<'input>: aslParserContext<'input>{
	fn lValExpr(&self) -> Option<Rc<LValExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn slice_all(&self) ->  Vec<Rc<SliceContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn slice(&self, i: usize) -> Option<Rc<SliceContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> LValArrayIndexContextAttrs<'input> for LValArrayIndexContext<'input>{}

pub struct LValArrayIndexContextExt<'input>{
	base:LValExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LValArrayIndexContextExt<'a>}

impl<'input> aslParserContext<'input> for LValArrayIndexContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for LValArrayIndexContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LValArrayIndex(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_LValArrayIndex(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for LValArrayIndexContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_LValArrayIndex(self);
	}
}

impl<'input> CustomRuleContext<'input> for LValArrayIndexContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lValExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lValExpr }
}

impl<'input> Borrow<LValExprContextExt<'input>> for LValArrayIndexContext<'input>{
	fn borrow(&self) -> &LValExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LValExprContextExt<'input>> for LValArrayIndexContext<'input>{
	fn borrow_mut(&mut self) -> &mut LValExprContextExt<'input> { &mut self.base }
}

impl<'input> LValExprContextAttrs<'input> for LValArrayIndexContext<'input> {}

impl<'input> LValArrayIndexContextExt<'input>{
	fn new(ctx: &dyn LValExprContextAttrs<'input>) -> Rc<LValExprContextAll<'input>>  {
		Rc::new(
			LValExprContextAll::LValArrayIndexContext(
				BaseParserRuleContext::copy_from(ctx,LValArrayIndexContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LValMemberBitsContext<'input> = BaseParserRuleContext<'input,LValMemberBitsContextExt<'input>>;

pub trait LValMemberBitsContextAttrs<'input>: aslParserContext<'input>{
	fn lValExpr(&self) -> Option<Rc<LValExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn identifierCommaList1(&self) -> Option<Rc<IdentifierCommaList1ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> LValMemberBitsContextAttrs<'input> for LValMemberBitsContext<'input>{}

pub struct LValMemberBitsContextExt<'input>{
	base:LValExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LValMemberBitsContextExt<'a>}

impl<'input> aslParserContext<'input> for LValMemberBitsContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for LValMemberBitsContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LValMemberBits(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_LValMemberBits(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for LValMemberBitsContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_LValMemberBits(self);
	}
}

impl<'input> CustomRuleContext<'input> for LValMemberBitsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lValExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lValExpr }
}

impl<'input> Borrow<LValExprContextExt<'input>> for LValMemberBitsContext<'input>{
	fn borrow(&self) -> &LValExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LValExprContextExt<'input>> for LValMemberBitsContext<'input>{
	fn borrow_mut(&mut self) -> &mut LValExprContextExt<'input> { &mut self.base }
}

impl<'input> LValExprContextAttrs<'input> for LValMemberBitsContext<'input> {}

impl<'input> LValMemberBitsContextExt<'input>{
	fn new(ctx: &dyn LValExprContextAttrs<'input>) -> Rc<LValExprContextAll<'input>>  {
		Rc::new(
			LValExprContextAll::LValMemberBitsContext(
				BaseParserRuleContext::copy_from(ctx,LValMemberBitsContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LValTupleContext<'input> = BaseParserRuleContext<'input,LValTupleContextExt<'input>>;

pub trait LValTupleContextAttrs<'input>: aslParserContext<'input>{
	fn lValExpr_all(&self) ->  Vec<Rc<LValExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn lValExpr(&self, i: usize) -> Option<Rc<LValExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> LValTupleContextAttrs<'input> for LValTupleContext<'input>{}

pub struct LValTupleContextExt<'input>{
	base:LValExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LValTupleContextExt<'a>}

impl<'input> aslParserContext<'input> for LValTupleContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for LValTupleContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LValTuple(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_LValTuple(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for LValTupleContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_LValTuple(self);
	}
}

impl<'input> CustomRuleContext<'input> for LValTupleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lValExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lValExpr }
}

impl<'input> Borrow<LValExprContextExt<'input>> for LValTupleContext<'input>{
	fn borrow(&self) -> &LValExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LValExprContextExt<'input>> for LValTupleContext<'input>{
	fn borrow_mut(&mut self) -> &mut LValExprContextExt<'input> { &mut self.base }
}

impl<'input> LValExprContextAttrs<'input> for LValTupleContext<'input> {}

impl<'input> LValTupleContextExt<'input>{
	fn new(ctx: &dyn LValExprContextAttrs<'input>) -> Rc<LValExprContextAll<'input>>  {
		Rc::new(
			LValExprContextAll::LValTupleContext(
				BaseParserRuleContext::copy_from(ctx,LValTupleContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LValMemberArrayContext<'input> = BaseParserRuleContext<'input,LValMemberArrayContextExt<'input>>;

pub trait LValMemberArrayContextAttrs<'input>: aslParserContext<'input>{
	fn lValExpr(&self) -> Option<Rc<LValExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn identifierCommaList1(&self) -> Option<Rc<IdentifierCommaList1ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> LValMemberArrayContextAttrs<'input> for LValMemberArrayContext<'input>{}

pub struct LValMemberArrayContextExt<'input>{
	base:LValExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LValMemberArrayContextExt<'a>}

impl<'input> aslParserContext<'input> for LValMemberArrayContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for LValMemberArrayContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LValMemberArray(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_LValMemberArray(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for LValMemberArrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_LValMemberArray(self);
	}
}

impl<'input> CustomRuleContext<'input> for LValMemberArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lValExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lValExpr }
}

impl<'input> Borrow<LValExprContextExt<'input>> for LValMemberArrayContext<'input>{
	fn borrow(&self) -> &LValExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LValExprContextExt<'input>> for LValMemberArrayContext<'input>{
	fn borrow_mut(&mut self) -> &mut LValExprContextExt<'input> { &mut self.base }
}

impl<'input> LValExprContextAttrs<'input> for LValMemberArrayContext<'input> {}

impl<'input> LValMemberArrayContextExt<'input>{
	fn new(ctx: &dyn LValExprContextAttrs<'input>) -> Rc<LValExprContextAll<'input>>  {
		Rc::new(
			LValExprContextAll::LValMemberArrayContext(
				BaseParserRuleContext::copy_from(ctx,LValMemberArrayContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LValIgnoreContext<'input> = BaseParserRuleContext<'input,LValIgnoreContextExt<'input>>;

pub trait LValIgnoreContextAttrs<'input>: aslParserContext<'input>{
}

impl<'input> LValIgnoreContextAttrs<'input> for LValIgnoreContext<'input>{}

pub struct LValIgnoreContextExt<'input>{
	base:LValExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LValIgnoreContextExt<'a>}

impl<'input> aslParserContext<'input> for LValIgnoreContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for LValIgnoreContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LValIgnore(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_LValIgnore(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for LValIgnoreContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_LValIgnore(self);
	}
}

impl<'input> CustomRuleContext<'input> for LValIgnoreContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lValExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lValExpr }
}

impl<'input> Borrow<LValExprContextExt<'input>> for LValIgnoreContext<'input>{
	fn borrow(&self) -> &LValExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LValExprContextExt<'input>> for LValIgnoreContext<'input>{
	fn borrow_mut(&mut self) -> &mut LValExprContextExt<'input> { &mut self.base }
}

impl<'input> LValExprContextAttrs<'input> for LValIgnoreContext<'input> {}

impl<'input> LValIgnoreContextExt<'input>{
	fn new(ctx: &dyn LValExprContextAttrs<'input>) -> Rc<LValExprContextAll<'input>>  {
		Rc::new(
			LValExprContextAll::LValIgnoreContext(
				BaseParserRuleContext::copy_from(ctx,LValIgnoreContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LValMemberContext<'input> = BaseParserRuleContext<'input,LValMemberContextExt<'input>>;

pub trait LValMemberContextAttrs<'input>: aslParserContext<'input>{
	fn lValExpr(&self) -> Option<Rc<LValExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> LValMemberContextAttrs<'input> for LValMemberContext<'input>{}

pub struct LValMemberContextExt<'input>{
	base:LValExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LValMemberContextExt<'a>}

impl<'input> aslParserContext<'input> for LValMemberContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for LValMemberContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LValMember(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_LValMember(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for LValMemberContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_LValMember(self);
	}
}

impl<'input> CustomRuleContext<'input> for LValMemberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lValExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lValExpr }
}

impl<'input> Borrow<LValExprContextExt<'input>> for LValMemberContext<'input>{
	fn borrow(&self) -> &LValExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LValExprContextExt<'input>> for LValMemberContext<'input>{
	fn borrow_mut(&mut self) -> &mut LValExprContextExt<'input> { &mut self.base }
}

impl<'input> LValExprContextAttrs<'input> for LValMemberContext<'input> {}

impl<'input> LValMemberContextExt<'input>{
	fn new(ctx: &dyn LValExprContextAttrs<'input>) -> Rc<LValExprContextAll<'input>>  {
		Rc::new(
			LValExprContextAll::LValMemberContext(
				BaseParserRuleContext::copy_from(ctx,LValMemberContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LValVarRefContext<'input> = BaseParserRuleContext<'input,LValVarRefContextExt<'input>>;

pub trait LValVarRefContextAttrs<'input>: aslParserContext<'input>{
	fn qualId(&self) -> Option<Rc<QualIdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> LValVarRefContextAttrs<'input> for LValVarRefContext<'input>{}

pub struct LValVarRefContextExt<'input>{
	base:LValExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LValVarRefContextExt<'a>}

impl<'input> aslParserContext<'input> for LValVarRefContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for LValVarRefContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LValVarRef(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_LValVarRef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for LValVarRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_LValVarRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for LValVarRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lValExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lValExpr }
}

impl<'input> Borrow<LValExprContextExt<'input>> for LValVarRefContext<'input>{
	fn borrow(&self) -> &LValExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LValExprContextExt<'input>> for LValVarRefContext<'input>{
	fn borrow_mut(&mut self) -> &mut LValExprContextExt<'input> { &mut self.base }
}

impl<'input> LValExprContextAttrs<'input> for LValVarRefContext<'input> {}

impl<'input> LValVarRefContextExt<'input>{
	fn new(ctx: &dyn LValExprContextAttrs<'input>) -> Rc<LValExprContextAll<'input>>  {
		Rc::new(
			LValExprContextAll::LValVarRefContext(
				BaseParserRuleContext::copy_from(ctx,LValVarRefContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LValSliceContext<'input> = BaseParserRuleContext<'input,LValSliceContextExt<'input>>;

pub trait LValSliceContextAttrs<'input>: aslParserContext<'input>{
	fn lValExpr_all(&self) ->  Vec<Rc<LValExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn lValExpr(&self, i: usize) -> Option<Rc<LValExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> LValSliceContextAttrs<'input> for LValSliceContext<'input>{}

pub struct LValSliceContextExt<'input>{
	base:LValExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LValSliceContextExt<'a>}

impl<'input> aslParserContext<'input> for LValSliceContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for LValSliceContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LValSlice(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_LValSlice(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for LValSliceContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_LValSlice(self);
	}
}

impl<'input> CustomRuleContext<'input> for LValSliceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lValExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lValExpr }
}

impl<'input> Borrow<LValExprContextExt<'input>> for LValSliceContext<'input>{
	fn borrow(&self) -> &LValExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LValExprContextExt<'input>> for LValSliceContext<'input>{
	fn borrow_mut(&mut self) -> &mut LValExprContextExt<'input> { &mut self.base }
}

impl<'input> LValExprContextAttrs<'input> for LValSliceContext<'input> {}

impl<'input> LValSliceContextExt<'input>{
	fn new(ctx: &dyn LValExprContextAttrs<'input>) -> Rc<LValExprContextAll<'input>>  {
		Rc::new(
			LValExprContextAll::LValSliceContext(
				BaseParserRuleContext::copy_from(ctx,LValSliceContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  lValExpr(&mut self,)
	-> Result<Rc<LValExprContextAll<'input>>,ANTLRError> {
		self.lValExpr_rec(0)
	}

	fn lValExpr_rec(&mut self, _p: isize)
	-> Result<Rc<LValExprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = LValExprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 56, RULE_lValExpr, _p);
	    let mut _localctx: Rc<LValExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 56;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(594);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__4 
				=> {
					{
					let mut tmp = LValArrayContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					recog.base.set_state(559);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					/*InvokeRule lValExpr*/
					recog.base.set_state(560);
					recog.lValExpr_rec(0)?;

					recog.base.set_state(565);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__9 {
						{
						{
						recog.base.set_state(561);
						recog.base.match_token(T__9,&mut recog.err_handler)?;

						/*InvokeRule lValExpr*/
						recog.base.set_state(562);
						recog.lValExpr_rec(0)?;

						}
						}
						recog.base.set_state(567);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(568);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					}
				}

			 T__31 
				=> {
					{
					let mut tmp = LValTupleContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(570);
					recog.base.match_token(T__31,&mut recog.err_handler)?;

					/*InvokeRule lValExpr*/
					recog.base.set_state(571);
					recog.lValExpr_rec(0)?;

					recog.base.set_state(576);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__9 {
						{
						{
						recog.base.set_state(572);
						recog.base.match_token(T__9,&mut recog.err_handler)?;

						/*InvokeRule lValExpr*/
						recog.base.set_state(573);
						recog.lValExpr_rec(0)?;

						}
						}
						recog.base.set_state(578);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(579);
					recog.base.match_token(T__32,&mut recog.err_handler)?;

					}
				}

			 T__64 
				=> {
					{
					let mut tmp = LValSliceContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(581);
					recog.base.match_token(T__64,&mut recog.err_handler)?;

					/*InvokeRule lValExpr*/
					recog.base.set_state(582);
					recog.lValExpr_rec(0)?;

					recog.base.set_state(587);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__9 {
						{
						{
						recog.base.set_state(583);
						recog.base.match_token(T__9,&mut recog.err_handler)?;

						/*InvokeRule lValExpr*/
						recog.base.set_state(584);
						recog.lValExpr_rec(0)?;

						}
						}
						recog.base.set_state(589);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(590);
					recog.base.match_token(T__65,&mut recog.err_handler)?;

					}
				}

			 T__33 | T__37 | T__67 | T__88 | T__89 | IDENTIFIER 
				=> {
					{
					let mut tmp = LValVarRefContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule qualId*/
					recog.base.set_state(592);
					recog.qualId()?;

					}
				}

			 T__62 
				=> {
					{
					let mut tmp = LValIgnoreContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(593);
					recog.base.match_token(T__62,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(631);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(54,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(629);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(53,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = LValMemberContextExt::new(&**LValExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_lValExpr);
							_localctx = tmp;
							recog.base.set_state(596);
							if !({recog.precpred(None, 10)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 10)".to_owned()), None))?;
							}
							recog.base.set_state(597);
							recog.base.match_token(T__63,&mut recog.err_handler)?;

							/*InvokeRule id*/
							recog.base.set_state(598);
							recog.id()?;

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = LValMemberArrayContextExt::new(&**LValExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_lValExpr);
							_localctx = tmp;
							recog.base.set_state(599);
							if !({recog.precpred(None, 9)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 9)".to_owned()), None))?;
							}
							recog.base.set_state(600);
							recog.base.match_token(T__63,&mut recog.err_handler)?;

							recog.base.set_state(601);
							recog.base.match_token(T__4,&mut recog.err_handler)?;

							/*InvokeRule identifierCommaList1*/
							recog.base.set_state(602);
							recog.identifierCommaList1()?;

							recog.base.set_state(603);
							recog.base.match_token(T__6,&mut recog.err_handler)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = LValArrayIndexContextExt::new(&**LValExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_lValExpr);
							_localctx = tmp;
							recog.base.set_state(605);
							if !({recog.precpred(None, 8)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
							}
							recog.base.set_state(606);
							recog.base.match_token(T__4,&mut recog.err_handler)?;

							recog.base.set_state(615);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==T__3 || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (T__31 - 32)) | (1usize << (T__33 - 32)) | (1usize << (T__36 - 32)) | (1usize << (T__37 - 32)) | (1usize << (T__39 - 32)) | (1usize << (T__62 - 32)))) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (T__66 - 67)) | (1usize << (T__67 - 67)) | (1usize << (T__88 - 67)) | (1usize << (T__89 - 67)) | (1usize << (IDENTIFIER - 67)) | (1usize << (NAT_LIT - 67)) | (1usize << (HEX_LIT - 67)) | (1usize << (BIN_LIT - 67)) | (1usize << (MASK_LIT - 67)) | (1usize << (REAL_LIT - 67)) | (1usize << (STRING_LIT - 67)))) != 0) {
								{
								/*InvokeRule slice*/
								recog.base.set_state(607);
								recog.slice()?;

								recog.base.set_state(612);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								while _la==T__9 {
									{
									{
									recog.base.set_state(608);
									recog.base.match_token(T__9,&mut recog.err_handler)?;

									/*InvokeRule slice*/
									recog.base.set_state(609);
									recog.slice()?;

									}
									}
									recog.base.set_state(614);
									recog.err_handler.sync(&mut recog.base)?;
									_la = recog.base.input.la(1);
								}
								}
							}

							recog.base.set_state(617);
							recog.base.match_token(T__6,&mut recog.err_handler)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = LValSliceOfContextExt::new(&**LValExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_lValExpr);
							_localctx = tmp;
							recog.base.set_state(618);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(619);
							recog.base.match_token(T__64,&mut recog.err_handler)?;

							/*InvokeRule sliceCommaList1*/
							recog.base.set_state(620);
							recog.sliceCommaList1()?;

							recog.base.set_state(621);
							recog.base.match_token(T__65,&mut recog.err_handler)?;

							}
						}
					,
						5 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = LValMemberBitsContextExt::new(&**LValExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_lValExpr);
							_localctx = tmp;
							recog.base.set_state(623);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(624);
							recog.base.match_token(T__63,&mut recog.err_handler)?;

							recog.base.set_state(625);
							recog.base.match_token(T__64,&mut recog.err_handler)?;

							/*InvokeRule identifierCommaList1*/
							recog.base.set_state(626);
							recog.identifierCommaList1()?;

							recog.base.set_state(627);
							recog.base.match_token(T__65,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(633);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(54,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- expr ----------------
#[derive(Debug)]
pub enum ExprContextAll<'input>{
	ExprLitStringContext(ExprLitStringContext<'input>),
	ExprImpDefContext(ExprImpDefContext<'input>),
	ExprParenContext(ExprParenContext<'input>),
	ExprTupleContext(ExprTupleContext<'input>),
	ExprIndexContext(ExprIndexContext<'input>),
	ExprUnOpContext(ExprUnOpContext<'input>),
	ExprBinOpContext(ExprBinOpContext<'input>),
	ExprLitNatContext(ExprLitNatContext<'input>),
	ExprMembersContext(ExprMembersContext<'input>),
	ExprInMaskContext(ExprInMaskContext<'input>),
	ExprLitHexContext(ExprLitHexContext<'input>),
	ExprMemberBitsContext(ExprMemberBitsContext<'input>),
	ExprCallContext(ExprCallContext<'input>),
	ExprInSetContext(ExprInSetContext<'input>),
	ExprLitBinContext(ExprLitBinContext<'input>),
	ExprUnknownContext(ExprUnknownContext<'input>),
	ExprLitRealContext(ExprLitRealContext<'input>),
	ExprVarRefContext(ExprVarRefContext<'input>),
	ExprSliceContext(ExprSliceContext<'input>),
	ExprLitMaskContext(ExprLitMaskContext<'input>),
	ExprIfContext(ExprIfContext<'input>),
	ExprMemberContext(ExprMemberContext<'input>),
Error(ExprContext<'input>)
}
antlr_rust::tid!{ExprContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExprContextAll<'input>{}

impl<'input> aslParserContext<'input> for ExprContextAll<'input>{}

impl<'input> Deref for ExprContextAll<'input>{
	type Target = dyn ExprContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ExprContextAll::*;
		match self{
			ExprLitStringContext(inner) => inner,
			ExprImpDefContext(inner) => inner,
			ExprParenContext(inner) => inner,
			ExprTupleContext(inner) => inner,
			ExprIndexContext(inner) => inner,
			ExprUnOpContext(inner) => inner,
			ExprBinOpContext(inner) => inner,
			ExprLitNatContext(inner) => inner,
			ExprMembersContext(inner) => inner,
			ExprInMaskContext(inner) => inner,
			ExprLitHexContext(inner) => inner,
			ExprMemberBitsContext(inner) => inner,
			ExprCallContext(inner) => inner,
			ExprInSetContext(inner) => inner,
			ExprLitBinContext(inner) => inner,
			ExprUnknownContext(inner) => inner,
			ExprLitRealContext(inner) => inner,
			ExprVarRefContext(inner) => inner,
			ExprSliceContext(inner) => inner,
			ExprLitMaskContext(inner) => inner,
			ExprIfContext(inner) => inner,
			ExprMemberContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr_rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprContextAll<'input>> {
		Rc::new(
		ExprContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ExprContextAttrs<'input>: aslParserContext<'input> + BorrowMut<ExprContextExt<'input>>{


}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

pub type ExprLitStringContext<'input> = BaseParserRuleContext<'input,ExprLitStringContextExt<'input>>;

pub trait ExprLitStringContextAttrs<'input>: aslParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STRING_LIT
	/// Returns `None` if there is no child corresponding to token STRING_LIT
	fn STRING_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(STRING_LIT, 0)
	}
}

impl<'input> ExprLitStringContextAttrs<'input> for ExprLitStringContext<'input>{}

pub struct ExprLitStringContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprLitStringContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprLitStringContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprLitStringContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprLitString(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprLitString(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprLitStringContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprLitString(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprLitStringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprLitStringContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprLitStringContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprLitStringContext<'input> {}

impl<'input> ExprLitStringContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprLitStringContext(
				BaseParserRuleContext::copy_from(ctx,ExprLitStringContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprImpDefContext<'input> = BaseParserRuleContext<'input,ExprImpDefContextExt<'input>>;

pub trait ExprImpDefContextAttrs<'input>: aslParserContext<'input>{
	fn typeSpec(&self) -> Option<Rc<TypeSpecContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token STRING_LIT
	/// Returns `None` if there is no child corresponding to token STRING_LIT
	fn STRING_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(STRING_LIT, 0)
	}
}

impl<'input> ExprImpDefContextAttrs<'input> for ExprImpDefContext<'input>{}

pub struct ExprImpDefContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprImpDefContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprImpDefContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprImpDefContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprImpDef(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprImpDef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprImpDefContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprImpDef(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprImpDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprImpDefContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprImpDefContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprImpDefContext<'input> {}

impl<'input> ExprImpDefContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprImpDefContext(
				BaseParserRuleContext::copy_from(ctx,ExprImpDefContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprParenContext<'input> = BaseParserRuleContext<'input,ExprParenContextExt<'input>>;

pub trait ExprParenContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprParenContextAttrs<'input> for ExprParenContext<'input>{}

pub struct ExprParenContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprParenContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprParenContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprParenContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprParen(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprParen(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprParenContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprParen(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprParenContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprParenContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprParenContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprParenContext<'input> {}

impl<'input> ExprParenContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprParenContext(
				BaseParserRuleContext::copy_from(ctx,ExprParenContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprTupleContext<'input> = BaseParserRuleContext<'input,ExprTupleContextExt<'input>>;

pub trait ExprTupleContextAttrs<'input>: aslParserContext<'input>{
	fn exprCommaList1(&self) -> Option<Rc<ExprCommaList1ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprTupleContextAttrs<'input> for ExprTupleContext<'input>{}

pub struct ExprTupleContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprTupleContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprTupleContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprTupleContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprTuple(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprTuple(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprTupleContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprTuple(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprTupleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprTupleContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprTupleContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprTupleContext<'input> {}

impl<'input> ExprTupleContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprTupleContext(
				BaseParserRuleContext::copy_from(ctx,ExprTupleContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprIndexContext<'input> = BaseParserRuleContext<'input,ExprIndexContextExt<'input>>;

pub trait ExprIndexContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn sliceCommaList0(&self) -> Option<Rc<SliceCommaList0ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprIndexContextAttrs<'input> for ExprIndexContext<'input>{}

pub struct ExprIndexContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprIndexContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprIndexContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprIndexContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprIndex(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprIndex(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprIndexContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprIndex(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprIndexContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprIndexContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprIndexContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprIndexContext<'input> {}

impl<'input> ExprIndexContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprIndexContext(
				BaseParserRuleContext::copy_from(ctx,ExprIndexContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprUnOpContext<'input> = BaseParserRuleContext<'input,ExprUnOpContextExt<'input>>;

pub trait ExprUnOpContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprUnOpContextAttrs<'input> for ExprUnOpContext<'input>{}

pub struct ExprUnOpContextExt<'input>{
	base:ExprContextExt<'input>,
	pub operator: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprUnOpContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprUnOpContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprUnOpContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprUnOp(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprUnOp(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprUnOpContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprUnOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprUnOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprUnOpContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprUnOpContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprUnOpContext<'input> {}

impl<'input> ExprUnOpContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprUnOpContext(
				BaseParserRuleContext::copy_from(ctx,ExprUnOpContextExt{
					operator:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprBinOpContext<'input> = BaseParserRuleContext<'input,ExprBinOpContextExt<'input>>;

pub trait ExprBinOpContextAttrs<'input>: aslParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> ExprBinOpContextAttrs<'input> for ExprBinOpContext<'input>{}

pub struct ExprBinOpContextExt<'input>{
	base:ExprContextExt<'input>,
	pub operand1: Option<Rc<ExprContextAll<'input>>>,
	pub operator: Option<TokenType<'input>>,
	pub operand2: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprBinOpContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprBinOpContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprBinOpContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprBinOp(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprBinOp(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprBinOpContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprBinOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprBinOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprBinOpContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprBinOpContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprBinOpContext<'input> {}

impl<'input> ExprBinOpContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprBinOpContext(
				BaseParserRuleContext::copy_from(ctx,ExprBinOpContextExt{
					operator:None, 
        			operand1:None, operand2:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprLitNatContext<'input> = BaseParserRuleContext<'input,ExprLitNatContextExt<'input>>;

pub trait ExprLitNatContextAttrs<'input>: aslParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NAT_LIT
	/// Returns `None` if there is no child corresponding to token NAT_LIT
	fn NAT_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(NAT_LIT, 0)
	}
}

impl<'input> ExprLitNatContextAttrs<'input> for ExprLitNatContext<'input>{}

pub struct ExprLitNatContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprLitNatContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprLitNatContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprLitNatContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprLitNat(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprLitNat(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprLitNatContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprLitNat(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprLitNatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprLitNatContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprLitNatContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprLitNatContext<'input> {}

impl<'input> ExprLitNatContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprLitNatContext(
				BaseParserRuleContext::copy_from(ctx,ExprLitNatContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprMembersContext<'input> = BaseParserRuleContext<'input,ExprMembersContextExt<'input>>;

pub trait ExprMembersContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn identifierCommaList1(&self) -> Option<Rc<IdentifierCommaList1ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprMembersContextAttrs<'input> for ExprMembersContext<'input>{}

pub struct ExprMembersContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprMembersContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprMembersContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprMembersContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprMembers(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprMembers(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprMembersContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprMembers(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprMembersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprMembersContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprMembersContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprMembersContext<'input> {}

impl<'input> ExprMembersContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprMembersContext(
				BaseParserRuleContext::copy_from(ctx,ExprMembersContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprInMaskContext<'input> = BaseParserRuleContext<'input,ExprInMaskContextExt<'input>>;

pub trait ExprInMaskContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token MASK_LIT
	/// Returns `None` if there is no child corresponding to token MASK_LIT
	fn MASK_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(MASK_LIT, 0)
	}
}

impl<'input> ExprInMaskContextAttrs<'input> for ExprInMaskContext<'input>{}

pub struct ExprInMaskContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprInMaskContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprInMaskContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprInMaskContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprInMask(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprInMask(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprInMaskContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprInMask(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprInMaskContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprInMaskContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprInMaskContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprInMaskContext<'input> {}

impl<'input> ExprInMaskContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprInMaskContext(
				BaseParserRuleContext::copy_from(ctx,ExprInMaskContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprLitHexContext<'input> = BaseParserRuleContext<'input,ExprLitHexContextExt<'input>>;

pub trait ExprLitHexContextAttrs<'input>: aslParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token HEX_LIT
	/// Returns `None` if there is no child corresponding to token HEX_LIT
	fn HEX_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(HEX_LIT, 0)
	}
}

impl<'input> ExprLitHexContextAttrs<'input> for ExprLitHexContext<'input>{}

pub struct ExprLitHexContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprLitHexContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprLitHexContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprLitHexContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprLitHex(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprLitHex(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprLitHexContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprLitHex(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprLitHexContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprLitHexContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprLitHexContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprLitHexContext<'input> {}

impl<'input> ExprLitHexContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprLitHexContext(
				BaseParserRuleContext::copy_from(ctx,ExprLitHexContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprMemberBitsContext<'input> = BaseParserRuleContext<'input,ExprMemberBitsContextExt<'input>>;

pub trait ExprMemberBitsContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn identifierCommaList1(&self) -> Option<Rc<IdentifierCommaList1ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprMemberBitsContextAttrs<'input> for ExprMemberBitsContext<'input>{}

pub struct ExprMemberBitsContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprMemberBitsContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprMemberBitsContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprMemberBitsContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprMemberBits(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprMemberBits(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprMemberBitsContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprMemberBits(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprMemberBitsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprMemberBitsContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprMemberBitsContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprMemberBitsContext<'input> {}

impl<'input> ExprMemberBitsContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprMemberBitsContext(
				BaseParserRuleContext::copy_from(ctx,ExprMemberBitsContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprCallContext<'input> = BaseParserRuleContext<'input,ExprCallContextExt<'input>>;

pub trait ExprCallContextAttrs<'input>: aslParserContext<'input>{
	fn qualId(&self) -> Option<Rc<QualIdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn exprCommaList0(&self) -> Option<Rc<ExprCommaList0ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprCallContextAttrs<'input> for ExprCallContext<'input>{}

pub struct ExprCallContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprCallContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprCallContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprCallContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprCall(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprCall(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprCallContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprCallContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprCallContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprCallContext<'input> {}

impl<'input> ExprCallContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprCallContext(
				BaseParserRuleContext::copy_from(ctx,ExprCallContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprInSetContext<'input> = BaseParserRuleContext<'input,ExprInSetContextExt<'input>>;

pub trait ExprInSetContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn set(&self) -> Option<Rc<SetContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprInSetContextAttrs<'input> for ExprInSetContext<'input>{}

pub struct ExprInSetContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprInSetContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprInSetContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprInSetContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprInSet(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprInSet(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprInSetContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprInSet(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprInSetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprInSetContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprInSetContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprInSetContext<'input> {}

impl<'input> ExprInSetContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprInSetContext(
				BaseParserRuleContext::copy_from(ctx,ExprInSetContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprLitBinContext<'input> = BaseParserRuleContext<'input,ExprLitBinContextExt<'input>>;

pub trait ExprLitBinContextAttrs<'input>: aslParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token BIN_LIT
	/// Returns `None` if there is no child corresponding to token BIN_LIT
	fn BIN_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(BIN_LIT, 0)
	}
}

impl<'input> ExprLitBinContextAttrs<'input> for ExprLitBinContext<'input>{}

pub struct ExprLitBinContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprLitBinContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprLitBinContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprLitBinContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprLitBin(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprLitBin(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprLitBinContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprLitBin(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprLitBinContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprLitBinContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprLitBinContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprLitBinContext<'input> {}

impl<'input> ExprLitBinContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprLitBinContext(
				BaseParserRuleContext::copy_from(ctx,ExprLitBinContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprUnknownContext<'input> = BaseParserRuleContext<'input,ExprUnknownContextExt<'input>>;

pub trait ExprUnknownContextAttrs<'input>: aslParserContext<'input>{
	fn typeSpec(&self) -> Option<Rc<TypeSpecContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprUnknownContextAttrs<'input> for ExprUnknownContext<'input>{}

pub struct ExprUnknownContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprUnknownContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprUnknownContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprUnknownContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprUnknown(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprUnknown(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprUnknownContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprUnknown(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprUnknownContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprUnknownContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprUnknownContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprUnknownContext<'input> {}

impl<'input> ExprUnknownContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprUnknownContext(
				BaseParserRuleContext::copy_from(ctx,ExprUnknownContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprLitRealContext<'input> = BaseParserRuleContext<'input,ExprLitRealContextExt<'input>>;

pub trait ExprLitRealContextAttrs<'input>: aslParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token REAL_LIT
	/// Returns `None` if there is no child corresponding to token REAL_LIT
	fn REAL_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(REAL_LIT, 0)
	}
}

impl<'input> ExprLitRealContextAttrs<'input> for ExprLitRealContext<'input>{}

pub struct ExprLitRealContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprLitRealContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprLitRealContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprLitRealContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprLitReal(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprLitReal(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprLitRealContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprLitReal(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprLitRealContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprLitRealContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprLitRealContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprLitRealContext<'input> {}

impl<'input> ExprLitRealContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprLitRealContext(
				BaseParserRuleContext::copy_from(ctx,ExprLitRealContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprVarRefContext<'input> = BaseParserRuleContext<'input,ExprVarRefContextExt<'input>>;

pub trait ExprVarRefContextAttrs<'input>: aslParserContext<'input>{
	fn qualId(&self) -> Option<Rc<QualIdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprVarRefContextAttrs<'input> for ExprVarRefContext<'input>{}

pub struct ExprVarRefContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprVarRefContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprVarRefContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprVarRefContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprVarRef(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprVarRef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprVarRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprVarRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprVarRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprVarRefContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprVarRefContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprVarRefContext<'input> {}

impl<'input> ExprVarRefContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprVarRefContext(
				BaseParserRuleContext::copy_from(ctx,ExprVarRefContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprSliceContext<'input> = BaseParserRuleContext<'input,ExprSliceContextExt<'input>>;

pub trait ExprSliceContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn sliceCommaList1(&self) -> Option<Rc<SliceCommaList1ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprSliceContextAttrs<'input> for ExprSliceContext<'input>{}

pub struct ExprSliceContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprSliceContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprSliceContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprSliceContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprSlice(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprSlice(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprSliceContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprSlice(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprSliceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprSliceContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprSliceContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprSliceContext<'input> {}

impl<'input> ExprSliceContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprSliceContext(
				BaseParserRuleContext::copy_from(ctx,ExprSliceContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprLitMaskContext<'input> = BaseParserRuleContext<'input,ExprLitMaskContextExt<'input>>;

pub trait ExprLitMaskContextAttrs<'input>: aslParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token MASK_LIT
	/// Returns `None` if there is no child corresponding to token MASK_LIT
	fn MASK_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(MASK_LIT, 0)
	}
}

impl<'input> ExprLitMaskContextAttrs<'input> for ExprLitMaskContext<'input>{}

pub struct ExprLitMaskContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprLitMaskContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprLitMaskContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprLitMaskContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprLitMask(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprLitMask(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprLitMaskContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprLitMask(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprLitMaskContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprLitMaskContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprLitMaskContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprLitMaskContext<'input> {}

impl<'input> ExprLitMaskContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprLitMaskContext(
				BaseParserRuleContext::copy_from(ctx,ExprLitMaskContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprIfContext<'input> = BaseParserRuleContext<'input,ExprIfContextExt<'input>>;

pub trait ExprIfContextAttrs<'input>: aslParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn exprElsIf_all(&self) ->  Vec<Rc<ExprElsIfContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn exprElsIf(&self, i: usize) -> Option<Rc<ExprElsIfContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> ExprIfContextAttrs<'input> for ExprIfContext<'input>{}

pub struct ExprIfContextExt<'input>{
	base:ExprContextExt<'input>,
	pub test: Option<Rc<ExprContextAll<'input>>>,
	pub thenExpr: Option<Rc<ExprContextAll<'input>>>,
	pub elseExpr: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprIfContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprIfContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprIfContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprIf(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprIf(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprIfContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprIf(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprIfContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprIfContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprIfContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprIfContext<'input> {}

impl<'input> ExprIfContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprIfContext(
				BaseParserRuleContext::copy_from(ctx,ExprIfContextExt{
        			test:None, thenExpr:None, elseExpr:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprMemberContext<'input> = BaseParserRuleContext<'input,ExprMemberContextExt<'input>>;

pub trait ExprMemberContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprMemberContextAttrs<'input> for ExprMemberContext<'input>{}

pub struct ExprMemberContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprMemberContextExt<'a>}

impl<'input> aslParserContext<'input> for ExprMemberContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprMemberContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprMember(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_ExprMember(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprMemberContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_ExprMember(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprMemberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprMemberContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprMemberContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprMemberContext<'input> {}

impl<'input> ExprMemberContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprMemberContext(
				BaseParserRuleContext::copy_from(ctx,ExprMemberContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		self.expr_rec(0)
	}

	fn expr_rec(&mut self, _p: isize)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 58, RULE_expr, _p);
	    let mut _localctx: Rc<ExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 58;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(678);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(57,&mut recog.base)? {
				1 =>{
					{
					let mut tmp = ExprLitNatContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					recog.base.set_state(635);
					recog.base.match_token(NAT_LIT,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					{
					let mut tmp = ExprLitHexContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(636);
					recog.base.match_token(HEX_LIT,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					{
					let mut tmp = ExprLitRealContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(637);
					recog.base.match_token(REAL_LIT,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					{
					let mut tmp = ExprLitBinContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(638);
					recog.base.match_token(BIN_LIT,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					{
					let mut tmp = ExprLitMaskContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(639);
					recog.base.match_token(MASK_LIT,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					{
					let mut tmp = ExprLitStringContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(640);
					recog.base.match_token(STRING_LIT,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					{
					let mut tmp = ExprCallContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule qualId*/
					recog.base.set_state(641);
					recog.qualId()?;

					recog.base.set_state(642);
					recog.base.match_token(T__31,&mut recog.err_handler)?;

					/*InvokeRule exprCommaList0*/
					recog.base.set_state(643);
					recog.exprCommaList0()?;

					recog.base.set_state(644);
					recog.base.match_token(T__32,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					{
					let mut tmp = ExprVarRefContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule qualId*/
					recog.base.set_state(646);
					recog.qualId()?;

					}
				}
			,
				9 =>{
					{
					let mut tmp = ExprParenContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(647);
					recog.base.match_token(T__31,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(648);
					recog.expr_rec(0)?;

					recog.base.set_state(649);
					recog.base.match_token(T__32,&mut recog.err_handler)?;

					}
				}
			,
				10 =>{
					{
					let mut tmp = ExprTupleContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(651);
					recog.base.match_token(T__31,&mut recog.err_handler)?;

					/*InvokeRule exprCommaList1*/
					recog.base.set_state(652);
					recog.exprCommaList1()?;

					recog.base.set_state(653);
					recog.base.match_token(T__32,&mut recog.err_handler)?;

					}
				}
			,
				11 =>{
					{
					let mut tmp = ExprUnOpContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(655);
					if let ExprContextAll::ExprUnOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
					_la = recog.base.input.la(1);
					if { !(((((_la - 63)) & !0x3f) == 0 && ((1usize << (_la - 63)) & ((1usize << (T__62 - 63)) | (1usize << (T__66 - 63)) | (1usize << (T__67 - 63)))) != 0)) } {
						let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
						if let ExprContextAll::ExprUnOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
						ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule expr*/
					recog.base.set_state(656);
					recog.expr_rec(16)?;

					}
				}
			,
				12 =>{
					{
					let mut tmp = ExprUnknownContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule typeSpec*/
					recog.base.set_state(657);
					recog.typeSpec()?;

					recog.base.set_state(658);
					recog.base.match_token(T__68,&mut recog.err_handler)?;

					}
				}
			,
				13 =>{
					{
					let mut tmp = ExprImpDefContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule typeSpec*/
					recog.base.set_state(660);
					recog.typeSpec()?;

					recog.base.set_state(661);
					recog.base.match_token(T__53,&mut recog.err_handler)?;

					recog.base.set_state(663);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(55,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(662);
							recog.base.match_token(STRING_LIT,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				14 =>{
					{
					let mut tmp = ExprIfContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(665);
					recog.base.match_token(T__39,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(666);
					let tmp = recog.expr_rec(0)?;
					if let ExprContextAll::ExprIfContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.test = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(667);
					recog.base.match_token(T__40,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(668);
					let tmp = recog.expr_rec(0)?;
					if let ExprContextAll::ExprIfContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.thenExpr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(672);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__58 {
						{
						{
						/*InvokeRule exprElsIf*/
						recog.base.set_state(669);
						recog.exprElsIf()?;

						}
						}
						recog.base.set_state(674);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(675);
					recog.base.match_token(T__41,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(676);
					let tmp = recog.expr_rec(1)?;
					if let ExprContextAll::ExprIfContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.elseExpr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(731);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(59,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(729);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(58,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprBinOpContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.operand1 = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(680);
							if !({recog.precpred(None, 7)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
							}
							recog.base.set_state(681);
							let tmp = recog.base.match_token(T__70,&mut recog.err_handler)?;
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							/*InvokeRule expr*/
							recog.base.set_state(682);
							let tmp = recog.expr_rec(8)?;
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.operand2 = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprBinOpContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.operand1 = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(683);
							if !({recog.precpred(None, 6)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
							}
							recog.base.set_state(684);
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==T__71 || _la==T__72) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
								ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(685);
							let tmp = recog.expr_rec(7)?;
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.operand2 = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						3 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprBinOpContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.operand1 = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(686);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(687);
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==T__62 || _la==T__73) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
								ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(688);
							let tmp = recog.expr_rec(6)?;
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.operand2 = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						4 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprBinOpContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.operand1 = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(689);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(690);
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==T__8 || ((((_la - 75)) & !0x3f) == 0 && ((1usize << (_la - 75)) & ((1usize << (T__74 - 75)) | (1usize << (T__75 - 75)) | (1usize << (T__76 - 75)) | (1usize << (T__77 - 75)) | (1usize << (T__78 - 75)) | (1usize << (T__79 - 75)) | (1usize << (T__80 - 75)) | (1usize << (T__81 - 75)) | (1usize << (T__82 - 75)) | (1usize << (T__83 - 75)))) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
								ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(691);
							let tmp = recog.expr_rec(5)?;
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.operand2 = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						5 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprBinOpContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.operand1 = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(692);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(693);
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==T__26 || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (T__64 - 65)) | (1usize << (T__65 - 65)) | (1usize << (T__84 - 65)) | (1usize << (T__85 - 65)) | (1usize << (T__86 - 65)))) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
								ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(694);
							let tmp = recog.expr_rec(4)?;
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.operand2 = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						6 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprBinOpContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.operand1 = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(695);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(696);
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==T__61 || _la==T__87) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
								ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(697);
							let tmp = recog.expr_rec(3)?;
							if let ExprContextAll::ExprBinOpContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.operand2 = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						7 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprMemberContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(698);
							if !({recog.precpred(None, 17)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 17)".to_owned()), None))?;
							}
							recog.base.set_state(699);
							recog.base.match_token(T__63,&mut recog.err_handler)?;

							/*InvokeRule id*/
							recog.base.set_state(700);
							recog.id()?;

							}
						}
					,
						8 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprMembersContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(701);
							if !({recog.precpred(None, 13)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 13)".to_owned()), None))?;
							}
							recog.base.set_state(702);
							recog.base.match_token(T__63,&mut recog.err_handler)?;

							recog.base.set_state(703);
							recog.base.match_token(T__4,&mut recog.err_handler)?;

							/*InvokeRule identifierCommaList1*/
							recog.base.set_state(704);
							recog.identifierCommaList1()?;

							recog.base.set_state(705);
							recog.base.match_token(T__6,&mut recog.err_handler)?;

							}
						}
					,
						9 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprMemberBitsContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(707);
							if !({recog.precpred(None, 12)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 12)".to_owned()), None))?;
							}
							recog.base.set_state(708);
							recog.base.match_token(T__63,&mut recog.err_handler)?;

							recog.base.set_state(709);
							recog.base.match_token(T__64,&mut recog.err_handler)?;

							/*InvokeRule identifierCommaList1*/
							recog.base.set_state(710);
							recog.identifierCommaList1()?;

							recog.base.set_state(711);
							recog.base.match_token(T__65,&mut recog.err_handler)?;

							}
						}
					,
						10 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprIndexContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(713);
							if !({recog.precpred(None, 11)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 11)".to_owned()), None))?;
							}
							recog.base.set_state(714);
							recog.base.match_token(T__4,&mut recog.err_handler)?;

							/*InvokeRule sliceCommaList0*/
							recog.base.set_state(715);
							recog.sliceCommaList0()?;

							recog.base.set_state(716);
							recog.base.match_token(T__6,&mut recog.err_handler)?;

							}
						}
					,
						11 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprInSetContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(718);
							if !({recog.precpred(None, 10)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 10)".to_owned()), None))?;
							}
							recog.base.set_state(719);
							recog.base.match_token(T__69,&mut recog.err_handler)?;

							/*InvokeRule set*/
							recog.base.set_state(720);
							recog.set()?;

							}
						}
					,
						12 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprInMaskContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(721);
							if !({recog.precpred(None, 9)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 9)".to_owned()), None))?;
							}
							recog.base.set_state(722);
							recog.base.match_token(T__69,&mut recog.err_handler)?;

							recog.base.set_state(723);
							recog.base.match_token(MASK_LIT,&mut recog.err_handler)?;

							}
						}
					,
						13 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprSliceContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(724);
							if !({recog.precpred(None, 8)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
							}
							recog.base.set_state(725);
							recog.base.match_token(T__64,&mut recog.err_handler)?;

							/*InvokeRule sliceCommaList1*/
							recog.base.set_state(726);
							recog.sliceCommaList1()?;

							recog.base.set_state(727);
							recog.base.match_token(T__65,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(733);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(59,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- sliceExpr ----------------
#[derive(Debug)]
pub enum SliceExprContextAll<'input>{
	SliceExprMemberContext(SliceExprMemberContext<'input>),
	SliceExprLitNatContext(SliceExprLitNatContext<'input>),
	SliceExprVarRefContext(SliceExprVarRefContext<'input>),
	SliceExprCallContext(SliceExprCallContext<'input>),
	SliceExprUnOpContext(SliceExprUnOpContext<'input>),
	SliceExprBinOpContext(SliceExprBinOpContext<'input>),
	SliceExprLitHexContext(SliceExprLitHexContext<'input>),
	SliceExprParenContext(SliceExprParenContext<'input>),
	SliceExprIfContext(SliceExprIfContext<'input>),
Error(SliceExprContext<'input>)
}
antlr_rust::tid!{SliceExprContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for SliceExprContextAll<'input>{}

impl<'input> aslParserContext<'input> for SliceExprContextAll<'input>{}

impl<'input> Deref for SliceExprContextAll<'input>{
	type Target = dyn SliceExprContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use SliceExprContextAll::*;
		match self{
			SliceExprMemberContext(inner) => inner,
			SliceExprLitNatContext(inner) => inner,
			SliceExprVarRefContext(inner) => inner,
			SliceExprCallContext(inner) => inner,
			SliceExprUnOpContext(inner) => inner,
			SliceExprBinOpContext(inner) => inner,
			SliceExprLitHexContext(inner) => inner,
			SliceExprParenContext(inner) => inner,
			SliceExprIfContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceExprContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceExprContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type SliceExprContext<'input> = BaseParserRuleContext<'input,SliceExprContextExt<'input>>;

#[derive(Clone)]
pub struct SliceExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for SliceExprContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceExprContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceExprContext<'input>{
}

impl<'input> CustomRuleContext<'input> for SliceExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sliceExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sliceExpr }
}
antlr_rust::tid!{SliceExprContextExt<'a>}

impl<'input> SliceExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SliceExprContextAll<'input>> {
		Rc::new(
		SliceExprContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SliceExprContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait SliceExprContextAttrs<'input>: aslParserContext<'input> + BorrowMut<SliceExprContextExt<'input>>{


}

impl<'input> SliceExprContextAttrs<'input> for SliceExprContext<'input>{}

pub type SliceExprMemberContext<'input> = BaseParserRuleContext<'input,SliceExprMemberContextExt<'input>>;

pub trait SliceExprMemberContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SliceExprMemberContextAttrs<'input> for SliceExprMemberContext<'input>{}

pub struct SliceExprMemberContextExt<'input>{
	base:SliceExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SliceExprMemberContextExt<'a>}

impl<'input> aslParserContext<'input> for SliceExprMemberContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceExprMemberContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SliceExprMember(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SliceExprMember(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceExprMemberContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SliceExprMember(self);
	}
}

impl<'input> CustomRuleContext<'input> for SliceExprMemberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sliceExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sliceExpr }
}

impl<'input> Borrow<SliceExprContextExt<'input>> for SliceExprMemberContext<'input>{
	fn borrow(&self) -> &SliceExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SliceExprContextExt<'input>> for SliceExprMemberContext<'input>{
	fn borrow_mut(&mut self) -> &mut SliceExprContextExt<'input> { &mut self.base }
}

impl<'input> SliceExprContextAttrs<'input> for SliceExprMemberContext<'input> {}

impl<'input> SliceExprMemberContextExt<'input>{
	fn new(ctx: &dyn SliceExprContextAttrs<'input>) -> Rc<SliceExprContextAll<'input>>  {
		Rc::new(
			SliceExprContextAll::SliceExprMemberContext(
				BaseParserRuleContext::copy_from(ctx,SliceExprMemberContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SliceExprLitNatContext<'input> = BaseParserRuleContext<'input,SliceExprLitNatContextExt<'input>>;

pub trait SliceExprLitNatContextAttrs<'input>: aslParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NAT_LIT
	/// Returns `None` if there is no child corresponding to token NAT_LIT
	fn NAT_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(NAT_LIT, 0)
	}
}

impl<'input> SliceExprLitNatContextAttrs<'input> for SliceExprLitNatContext<'input>{}

pub struct SliceExprLitNatContextExt<'input>{
	base:SliceExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SliceExprLitNatContextExt<'a>}

impl<'input> aslParserContext<'input> for SliceExprLitNatContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceExprLitNatContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SliceExprLitNat(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SliceExprLitNat(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceExprLitNatContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SliceExprLitNat(self);
	}
}

impl<'input> CustomRuleContext<'input> for SliceExprLitNatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sliceExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sliceExpr }
}

impl<'input> Borrow<SliceExprContextExt<'input>> for SliceExprLitNatContext<'input>{
	fn borrow(&self) -> &SliceExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SliceExprContextExt<'input>> for SliceExprLitNatContext<'input>{
	fn borrow_mut(&mut self) -> &mut SliceExprContextExt<'input> { &mut self.base }
}

impl<'input> SliceExprContextAttrs<'input> for SliceExprLitNatContext<'input> {}

impl<'input> SliceExprLitNatContextExt<'input>{
	fn new(ctx: &dyn SliceExprContextAttrs<'input>) -> Rc<SliceExprContextAll<'input>>  {
		Rc::new(
			SliceExprContextAll::SliceExprLitNatContext(
				BaseParserRuleContext::copy_from(ctx,SliceExprLitNatContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SliceExprVarRefContext<'input> = BaseParserRuleContext<'input,SliceExprVarRefContextExt<'input>>;

pub trait SliceExprVarRefContextAttrs<'input>: aslParserContext<'input>{
	fn qualId(&self) -> Option<Rc<QualIdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SliceExprVarRefContextAttrs<'input> for SliceExprVarRefContext<'input>{}

pub struct SliceExprVarRefContextExt<'input>{
	base:SliceExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SliceExprVarRefContextExt<'a>}

impl<'input> aslParserContext<'input> for SliceExprVarRefContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceExprVarRefContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SliceExprVarRef(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SliceExprVarRef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceExprVarRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SliceExprVarRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for SliceExprVarRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sliceExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sliceExpr }
}

impl<'input> Borrow<SliceExprContextExt<'input>> for SliceExprVarRefContext<'input>{
	fn borrow(&self) -> &SliceExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SliceExprContextExt<'input>> for SliceExprVarRefContext<'input>{
	fn borrow_mut(&mut self) -> &mut SliceExprContextExt<'input> { &mut self.base }
}

impl<'input> SliceExprContextAttrs<'input> for SliceExprVarRefContext<'input> {}

impl<'input> SliceExprVarRefContextExt<'input>{
	fn new(ctx: &dyn SliceExprContextAttrs<'input>) -> Rc<SliceExprContextAll<'input>>  {
		Rc::new(
			SliceExprContextAll::SliceExprVarRefContext(
				BaseParserRuleContext::copy_from(ctx,SliceExprVarRefContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SliceExprCallContext<'input> = BaseParserRuleContext<'input,SliceExprCallContextExt<'input>>;

pub trait SliceExprCallContextAttrs<'input>: aslParserContext<'input>{
	fn qualId(&self) -> Option<Rc<QualIdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn exprCommaList0(&self) -> Option<Rc<ExprCommaList0ContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SliceExprCallContextAttrs<'input> for SliceExprCallContext<'input>{}

pub struct SliceExprCallContextExt<'input>{
	base:SliceExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SliceExprCallContextExt<'a>}

impl<'input> aslParserContext<'input> for SliceExprCallContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceExprCallContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SliceExprCall(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SliceExprCall(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceExprCallContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SliceExprCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for SliceExprCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sliceExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sliceExpr }
}

impl<'input> Borrow<SliceExprContextExt<'input>> for SliceExprCallContext<'input>{
	fn borrow(&self) -> &SliceExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SliceExprContextExt<'input>> for SliceExprCallContext<'input>{
	fn borrow_mut(&mut self) -> &mut SliceExprContextExt<'input> { &mut self.base }
}

impl<'input> SliceExprContextAttrs<'input> for SliceExprCallContext<'input> {}

impl<'input> SliceExprCallContextExt<'input>{
	fn new(ctx: &dyn SliceExprContextAttrs<'input>) -> Rc<SliceExprContextAll<'input>>  {
		Rc::new(
			SliceExprContextAll::SliceExprCallContext(
				BaseParserRuleContext::copy_from(ctx,SliceExprCallContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SliceExprUnOpContext<'input> = BaseParserRuleContext<'input,SliceExprUnOpContextExt<'input>>;

pub trait SliceExprUnOpContextAttrs<'input>: aslParserContext<'input>{
	fn sliceExpr(&self) -> Option<Rc<SliceExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SliceExprUnOpContextAttrs<'input> for SliceExprUnOpContext<'input>{}

pub struct SliceExprUnOpContextExt<'input>{
	base:SliceExprContextExt<'input>,
	pub operator: Option<TokenType<'input>>,
	pub operand: Option<Rc<SliceExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SliceExprUnOpContextExt<'a>}

impl<'input> aslParserContext<'input> for SliceExprUnOpContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceExprUnOpContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SliceExprUnOp(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SliceExprUnOp(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceExprUnOpContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SliceExprUnOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for SliceExprUnOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sliceExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sliceExpr }
}

impl<'input> Borrow<SliceExprContextExt<'input>> for SliceExprUnOpContext<'input>{
	fn borrow(&self) -> &SliceExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SliceExprContextExt<'input>> for SliceExprUnOpContext<'input>{
	fn borrow_mut(&mut self) -> &mut SliceExprContextExt<'input> { &mut self.base }
}

impl<'input> SliceExprContextAttrs<'input> for SliceExprUnOpContext<'input> {}

impl<'input> SliceExprUnOpContextExt<'input>{
	fn new(ctx: &dyn SliceExprContextAttrs<'input>) -> Rc<SliceExprContextAll<'input>>  {
		Rc::new(
			SliceExprContextAll::SliceExprUnOpContext(
				BaseParserRuleContext::copy_from(ctx,SliceExprUnOpContextExt{
					operator:None, 
        			operand:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SliceExprBinOpContext<'input> = BaseParserRuleContext<'input,SliceExprBinOpContextExt<'input>>;

pub trait SliceExprBinOpContextAttrs<'input>: aslParserContext<'input>{
	fn sliceExpr_all(&self) ->  Vec<Rc<SliceExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn sliceExpr(&self, i: usize) -> Option<Rc<SliceExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> SliceExprBinOpContextAttrs<'input> for SliceExprBinOpContext<'input>{}

pub struct SliceExprBinOpContextExt<'input>{
	base:SliceExprContextExt<'input>,
	pub operand1: Option<Rc<SliceExprContextAll<'input>>>,
	pub operator: Option<TokenType<'input>>,
	pub operand2: Option<Rc<SliceExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SliceExprBinOpContextExt<'a>}

impl<'input> aslParserContext<'input> for SliceExprBinOpContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceExprBinOpContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SliceExprBinOp(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SliceExprBinOp(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceExprBinOpContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SliceExprBinOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for SliceExprBinOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sliceExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sliceExpr }
}

impl<'input> Borrow<SliceExprContextExt<'input>> for SliceExprBinOpContext<'input>{
	fn borrow(&self) -> &SliceExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SliceExprContextExt<'input>> for SliceExprBinOpContext<'input>{
	fn borrow_mut(&mut self) -> &mut SliceExprContextExt<'input> { &mut self.base }
}

impl<'input> SliceExprContextAttrs<'input> for SliceExprBinOpContext<'input> {}

impl<'input> SliceExprBinOpContextExt<'input>{
	fn new(ctx: &dyn SliceExprContextAttrs<'input>) -> Rc<SliceExprContextAll<'input>>  {
		Rc::new(
			SliceExprContextAll::SliceExprBinOpContext(
				BaseParserRuleContext::copy_from(ctx,SliceExprBinOpContextExt{
					operator:None, 
        			operand1:None, operand2:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SliceExprLitHexContext<'input> = BaseParserRuleContext<'input,SliceExprLitHexContextExt<'input>>;

pub trait SliceExprLitHexContextAttrs<'input>: aslParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token HEX_LIT
	/// Returns `None` if there is no child corresponding to token HEX_LIT
	fn HEX_LIT(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
		self.get_token(HEX_LIT, 0)
	}
}

impl<'input> SliceExprLitHexContextAttrs<'input> for SliceExprLitHexContext<'input>{}

pub struct SliceExprLitHexContextExt<'input>{
	base:SliceExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SliceExprLitHexContextExt<'a>}

impl<'input> aslParserContext<'input> for SliceExprLitHexContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceExprLitHexContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SliceExprLitHex(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SliceExprLitHex(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceExprLitHexContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SliceExprLitHex(self);
	}
}

impl<'input> CustomRuleContext<'input> for SliceExprLitHexContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sliceExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sliceExpr }
}

impl<'input> Borrow<SliceExprContextExt<'input>> for SliceExprLitHexContext<'input>{
	fn borrow(&self) -> &SliceExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SliceExprContextExt<'input>> for SliceExprLitHexContext<'input>{
	fn borrow_mut(&mut self) -> &mut SliceExprContextExt<'input> { &mut self.base }
}

impl<'input> SliceExprContextAttrs<'input> for SliceExprLitHexContext<'input> {}

impl<'input> SliceExprLitHexContextExt<'input>{
	fn new(ctx: &dyn SliceExprContextAttrs<'input>) -> Rc<SliceExprContextAll<'input>>  {
		Rc::new(
			SliceExprContextAll::SliceExprLitHexContext(
				BaseParserRuleContext::copy_from(ctx,SliceExprLitHexContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SliceExprParenContext<'input> = BaseParserRuleContext<'input,SliceExprParenContextExt<'input>>;

pub trait SliceExprParenContextAttrs<'input>: aslParserContext<'input>{
	fn sliceExpr(&self) -> Option<Rc<SliceExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SliceExprParenContextAttrs<'input> for SliceExprParenContext<'input>{}

pub struct SliceExprParenContextExt<'input>{
	base:SliceExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SliceExprParenContextExt<'a>}

impl<'input> aslParserContext<'input> for SliceExprParenContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceExprParenContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SliceExprParen(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SliceExprParen(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceExprParenContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SliceExprParen(self);
	}
}

impl<'input> CustomRuleContext<'input> for SliceExprParenContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sliceExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sliceExpr }
}

impl<'input> Borrow<SliceExprContextExt<'input>> for SliceExprParenContext<'input>{
	fn borrow(&self) -> &SliceExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SliceExprContextExt<'input>> for SliceExprParenContext<'input>{
	fn borrow_mut(&mut self) -> &mut SliceExprContextExt<'input> { &mut self.base }
}

impl<'input> SliceExprContextAttrs<'input> for SliceExprParenContext<'input> {}

impl<'input> SliceExprParenContextExt<'input>{
	fn new(ctx: &dyn SliceExprContextAttrs<'input>) -> Rc<SliceExprContextAll<'input>>  {
		Rc::new(
			SliceExprContextAll::SliceExprParenContext(
				BaseParserRuleContext::copy_from(ctx,SliceExprParenContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SliceExprIfContext<'input> = BaseParserRuleContext<'input,SliceExprIfContextExt<'input>>;

pub trait SliceExprIfContextAttrs<'input>: aslParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn exprElsIf_all(&self) ->  Vec<Rc<ExprElsIfContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn exprElsIf(&self, i: usize) -> Option<Rc<ExprElsIfContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> SliceExprIfContextAttrs<'input> for SliceExprIfContext<'input>{}

pub struct SliceExprIfContextExt<'input>{
	base:SliceExprContextExt<'input>,
	pub test: Option<Rc<ExprContextAll<'input>>>,
	pub thenExpr: Option<Rc<ExprContextAll<'input>>>,
	pub elseExpr: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SliceExprIfContextExt<'a>}

impl<'input> aslParserContext<'input> for SliceExprIfContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceExprIfContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SliceExprIf(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SliceExprIf(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceExprIfContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SliceExprIf(self);
	}
}

impl<'input> CustomRuleContext<'input> for SliceExprIfContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sliceExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sliceExpr }
}

impl<'input> Borrow<SliceExprContextExt<'input>> for SliceExprIfContext<'input>{
	fn borrow(&self) -> &SliceExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SliceExprContextExt<'input>> for SliceExprIfContext<'input>{
	fn borrow_mut(&mut self) -> &mut SliceExprContextExt<'input> { &mut self.base }
}

impl<'input> SliceExprContextAttrs<'input> for SliceExprIfContext<'input> {}

impl<'input> SliceExprIfContextExt<'input>{
	fn new(ctx: &dyn SliceExprContextAttrs<'input>) -> Rc<SliceExprContextAll<'input>>  {
		Rc::new(
			SliceExprContextAll::SliceExprIfContext(
				BaseParserRuleContext::copy_from(ctx,SliceExprIfContextExt{
        			test:None, thenExpr:None, elseExpr:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  sliceExpr(&mut self,)
	-> Result<Rc<SliceExprContextAll<'input>>,ANTLRError> {
		self.sliceExpr_rec(0)
	}

	fn sliceExpr_rec(&mut self, _p: isize)
	-> Result<Rc<SliceExprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = SliceExprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 60, RULE_sliceExpr, _p);
	    let mut _localctx: Rc<SliceExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 60;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(766);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(61,&mut recog.base)? {
				1 =>{
					{
					let mut tmp = SliceExprLitNatContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					recog.base.set_state(735);
					recog.base.match_token(NAT_LIT,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					{
					let mut tmp = SliceExprLitHexContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(736);
					recog.base.match_token(HEX_LIT,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					{
					let mut tmp = SliceExprParenContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(737);
					recog.base.match_token(T__31,&mut recog.err_handler)?;

					/*InvokeRule sliceExpr*/
					recog.base.set_state(738);
					recog.sliceExpr_rec(0)?;

					recog.base.set_state(739);
					recog.base.match_token(T__32,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					{
					let mut tmp = SliceExprVarRefContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule qualId*/
					recog.base.set_state(741);
					recog.qualId()?;

					}
				}
			,
				5 =>{
					{
					let mut tmp = SliceExprCallContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule qualId*/
					recog.base.set_state(742);
					recog.qualId()?;

					recog.base.set_state(743);
					recog.base.match_token(T__31,&mut recog.err_handler)?;

					/*InvokeRule exprCommaList0*/
					recog.base.set_state(744);
					recog.exprCommaList0()?;

					recog.base.set_state(745);
					recog.base.match_token(T__32,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					{
					let mut tmp = SliceExprUnOpContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(747);
					if let SliceExprContextAll::SliceExprUnOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
					ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
					_la = recog.base.input.la(1);
					if { !(_la==T__62 || _la==T__66) } {
						let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
						if let SliceExprContextAll::SliceExprUnOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
						ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule sliceExpr*/
					recog.base.set_state(748);
					let tmp = recog.sliceExpr_rec(7)?;
					if let SliceExprContextAll::SliceExprUnOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
					ctx.operand = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				7 =>{
					{
					let mut tmp = SliceExprMemberContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule expr*/
					recog.base.set_state(749);
					recog.expr_rec(0)?;

					recog.base.set_state(750);
					recog.base.match_token(T__63,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(751);
					recog.id()?;

					}
				}
			,
				8 =>{
					{
					let mut tmp = SliceExprIfContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(753);
					recog.base.match_token(T__39,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(754);
					let tmp = recog.expr_rec(0)?;
					if let SliceExprContextAll::SliceExprIfContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
					ctx.test = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(755);
					recog.base.match_token(T__40,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(756);
					let tmp = recog.expr_rec(0)?;
					if let SliceExprContextAll::SliceExprIfContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
					ctx.thenExpr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(760);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__58 {
						{
						{
						/*InvokeRule exprElsIf*/
						recog.base.set_state(757);
						recog.exprElsIf()?;

						}
						}
						recog.base.set_state(762);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(763);
					recog.base.match_token(T__41,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(764);
					let tmp = recog.expr_rec(0)?;
					if let SliceExprContextAll::SliceExprIfContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
					ctx.elseExpr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(782);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(63,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(780);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(62,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = SliceExprBinOpContextExt::new(&**SliceExprContextExt::new(_parentctx.clone(), _parentState));
							if let SliceExprContextAll::SliceExprBinOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut tmp){
								ctx.operand1 = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_sliceExpr);
							_localctx = tmp;
							recog.base.set_state(768);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(769);
							let tmp = recog.base.match_token(T__70,&mut recog.err_handler)?;
							if let SliceExprContextAll::SliceExprBinOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
							ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							/*InvokeRule sliceExpr*/
							recog.base.set_state(770);
							let tmp = recog.sliceExpr_rec(6)?;
							if let SliceExprContextAll::SliceExprBinOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
							ctx.operand2 = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = SliceExprBinOpContextExt::new(&**SliceExprContextExt::new(_parentctx.clone(), _parentState));
							if let SliceExprContextAll::SliceExprBinOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut tmp){
								ctx.operand1 = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_sliceExpr);
							_localctx = tmp;
							recog.base.set_state(771);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(772);
							if let SliceExprContextAll::SliceExprBinOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
							ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==T__71 || _la==T__72) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let SliceExprContextAll::SliceExprBinOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
								ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule sliceExpr*/
							recog.base.set_state(773);
							let tmp = recog.sliceExpr_rec(5)?;
							if let SliceExprContextAll::SliceExprBinOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
							ctx.operand2 = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						3 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = SliceExprBinOpContextExt::new(&**SliceExprContextExt::new(_parentctx.clone(), _parentState));
							if let SliceExprContextAll::SliceExprBinOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut tmp){
								ctx.operand1 = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_sliceExpr);
							_localctx = tmp;
							recog.base.set_state(774);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(775);
							if let SliceExprContextAll::SliceExprBinOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
							ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==T__62 || _la==T__73) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let SliceExprContextAll::SliceExprBinOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
								ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule sliceExpr*/
							recog.base.set_state(776);
							let tmp = recog.sliceExpr_rec(4)?;
							if let SliceExprContextAll::SliceExprBinOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
							ctx.operand2 = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						4 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = SliceExprBinOpContextExt::new(&**SliceExprContextExt::new(_parentctx.clone(), _parentState));
							if let SliceExprContextAll::SliceExprBinOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut tmp){
								ctx.operand1 = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_sliceExpr);
							_localctx = tmp;
							recog.base.set_state(777);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(778);
							if let SliceExprContextAll::SliceExprBinOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
							ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(((((_la - 75)) & !0x3f) == 0 && ((1usize << (_la - 75)) & ((1usize << (T__74 - 75)) | (1usize << (T__75 - 75)) | (1usize << (T__76 - 75)) | (1usize << (T__77 - 75)) | (1usize << (T__78 - 75)) | (1usize << (T__79 - 75)) | (1usize << (T__80 - 75)) | (1usize << (T__81 - 75)) | (1usize << (T__82 - 75)) | (1usize << (T__83 - 75)))) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let SliceExprContextAll::SliceExprBinOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
								ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule sliceExpr*/
							recog.base.set_state(779);
							let tmp = recog.sliceExpr_rec(3)?;
							if let SliceExprContextAll::SliceExprBinOpContext(ctx) = cast_mut::<_,SliceExprContextAll >(&mut _localctx){
							ctx.operand2 = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(784);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(63,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- slice ----------------
#[derive(Debug)]
pub enum SliceContextAll<'input>{
	SliceSingleContext(SliceSingleContext<'input>),
	SliceRangeContext(SliceRangeContext<'input>),
	SliceOffsetContext(SliceOffsetContext<'input>),
Error(SliceContext<'input>)
}
antlr_rust::tid!{SliceContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for SliceContextAll<'input>{}

impl<'input> aslParserContext<'input> for SliceContextAll<'input>{}

impl<'input> Deref for SliceContextAll<'input>{
	type Target = dyn SliceContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use SliceContextAll::*;
		match self{
			SliceSingleContext(inner) => inner,
			SliceRangeContext(inner) => inner,
			SliceOffsetContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type SliceContext<'input> = BaseParserRuleContext<'input,SliceContextExt<'input>>;

#[derive(Clone)]
pub struct SliceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for SliceContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceContext<'input>{
}

impl<'input> CustomRuleContext<'input> for SliceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_slice }
	//fn type_rule_index() -> usize where Self: Sized { RULE_slice }
}
antlr_rust::tid!{SliceContextExt<'a>}

impl<'input> SliceContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SliceContextAll<'input>> {
		Rc::new(
		SliceContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SliceContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait SliceContextAttrs<'input>: aslParserContext<'input> + BorrowMut<SliceContextExt<'input>>{


}

impl<'input> SliceContextAttrs<'input> for SliceContext<'input>{}

pub type SliceSingleContext<'input> = BaseParserRuleContext<'input,SliceSingleContextExt<'input>>;

pub trait SliceSingleContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SliceSingleContextAttrs<'input> for SliceSingleContext<'input>{}

pub struct SliceSingleContextExt<'input>{
	base:SliceContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SliceSingleContextExt<'a>}

impl<'input> aslParserContext<'input> for SliceSingleContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceSingleContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SliceSingle(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SliceSingle(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceSingleContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SliceSingle(self);
	}
}

impl<'input> CustomRuleContext<'input> for SliceSingleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_slice }
	//fn type_rule_index() -> usize where Self: Sized { RULE_slice }
}

impl<'input> Borrow<SliceContextExt<'input>> for SliceSingleContext<'input>{
	fn borrow(&self) -> &SliceContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SliceContextExt<'input>> for SliceSingleContext<'input>{
	fn borrow_mut(&mut self) -> &mut SliceContextExt<'input> { &mut self.base }
}

impl<'input> SliceContextAttrs<'input> for SliceSingleContext<'input> {}

impl<'input> SliceSingleContextExt<'input>{
	fn new(ctx: &dyn SliceContextAttrs<'input>) -> Rc<SliceContextAll<'input>>  {
		Rc::new(
			SliceContextAll::SliceSingleContext(
				BaseParserRuleContext::copy_from(ctx,SliceSingleContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SliceRangeContext<'input> = BaseParserRuleContext<'input,SliceRangeContextExt<'input>>;

pub trait SliceRangeContextAttrs<'input>: aslParserContext<'input>{
	fn sliceExpr_all(&self) ->  Vec<Rc<SliceExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn sliceExpr(&self, i: usize) -> Option<Rc<SliceExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> SliceRangeContextAttrs<'input> for SliceRangeContext<'input>{}

pub struct SliceRangeContextExt<'input>{
	base:SliceContextExt<'input>,
	pub begin: Option<Rc<SliceExprContextAll<'input>>>,
	pub end: Option<Rc<SliceExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SliceRangeContextExt<'a>}

impl<'input> aslParserContext<'input> for SliceRangeContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceRangeContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SliceRange(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SliceRange(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceRangeContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SliceRange(self);
	}
}

impl<'input> CustomRuleContext<'input> for SliceRangeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_slice }
	//fn type_rule_index() -> usize where Self: Sized { RULE_slice }
}

impl<'input> Borrow<SliceContextExt<'input>> for SliceRangeContext<'input>{
	fn borrow(&self) -> &SliceContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SliceContextExt<'input>> for SliceRangeContext<'input>{
	fn borrow_mut(&mut self) -> &mut SliceContextExt<'input> { &mut self.base }
}

impl<'input> SliceContextAttrs<'input> for SliceRangeContext<'input> {}

impl<'input> SliceRangeContextExt<'input>{
	fn new(ctx: &dyn SliceContextAttrs<'input>) -> Rc<SliceContextAll<'input>>  {
		Rc::new(
			SliceContextAll::SliceRangeContext(
				BaseParserRuleContext::copy_from(ctx,SliceRangeContextExt{
        			begin:None, end:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SliceOffsetContext<'input> = BaseParserRuleContext<'input,SliceOffsetContextExt<'input>>;

pub trait SliceOffsetContextAttrs<'input>: aslParserContext<'input>{
	fn sliceExpr_all(&self) ->  Vec<Rc<SliceExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn sliceExpr(&self, i: usize) -> Option<Rc<SliceExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> SliceOffsetContextAttrs<'input> for SliceOffsetContext<'input>{}

pub struct SliceOffsetContextExt<'input>{
	base:SliceContextExt<'input>,
	pub sliceBase: Option<Rc<SliceExprContextAll<'input>>>,
	pub count: Option<Rc<SliceExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SliceOffsetContextExt<'a>}

impl<'input> aslParserContext<'input> for SliceOffsetContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceOffsetContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SliceOffset(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SliceOffset(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceOffsetContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SliceOffset(self);
	}
}

impl<'input> CustomRuleContext<'input> for SliceOffsetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_slice }
	//fn type_rule_index() -> usize where Self: Sized { RULE_slice }
}

impl<'input> Borrow<SliceContextExt<'input>> for SliceOffsetContext<'input>{
	fn borrow(&self) -> &SliceContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SliceContextExt<'input>> for SliceOffsetContext<'input>{
	fn borrow_mut(&mut self) -> &mut SliceContextExt<'input> { &mut self.base }
}

impl<'input> SliceContextAttrs<'input> for SliceOffsetContext<'input> {}

impl<'input> SliceOffsetContextExt<'input>{
	fn new(ctx: &dyn SliceContextAttrs<'input>) -> Rc<SliceContextAll<'input>>  {
		Rc::new(
			SliceContextAll::SliceOffsetContext(
				BaseParserRuleContext::copy_from(ctx,SliceOffsetContextExt{
        			sliceBase:None, count:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn slice(&mut self,)
	-> Result<Rc<SliceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SliceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_slice);
        let mut _localctx: Rc<SliceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(794);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(64,&mut recog.base)? {
				1 =>{
					let tmp = SliceRangeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule sliceExpr*/
					recog.base.set_state(785);
					let tmp = recog.sliceExpr_rec(0)?;
					if let SliceContextAll::SliceRangeContext(ctx) = cast_mut::<_,SliceContextAll >(&mut _localctx){
					ctx.begin = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(786);
					recog.base.match_token(T__8,&mut recog.err_handler)?;

					/*InvokeRule sliceExpr*/
					recog.base.set_state(787);
					let tmp = recog.sliceExpr_rec(0)?;
					if let SliceContextAll::SliceRangeContext(ctx) = cast_mut::<_,SliceContextAll >(&mut _localctx){
					ctx.end = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				2 =>{
					let tmp = SliceOffsetContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule sliceExpr*/
					recog.base.set_state(789);
					let tmp = recog.sliceExpr_rec(0)?;
					if let SliceContextAll::SliceOffsetContext(ctx) = cast_mut::<_,SliceContextAll >(&mut _localctx){
					ctx.sliceBase = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(790);
					recog.base.match_token(T__24,&mut recog.err_handler)?;

					/*InvokeRule sliceExpr*/
					recog.base.set_state(791);
					let tmp = recog.sliceExpr_rec(0)?;
					if let SliceContextAll::SliceOffsetContext(ctx) = cast_mut::<_,SliceContextAll >(&mut _localctx){
					ctx.count = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				3 =>{
					let tmp = SliceSingleContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					/*InvokeRule expr*/
					recog.base.set_state(793);
					recog.expr_rec(0)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
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
//------------------- exprElsIf ----------------
pub type ExprElsIfContextAll<'input> = ExprElsIfContext<'input>;


pub type ExprElsIfContext<'input> = BaseParserRuleContext<'input,ExprElsIfContextExt<'input>>;

#[derive(Clone)]
pub struct ExprElsIfContextExt<'input>{
	pub test: Option<Rc<ExprContextAll<'input>>>,
	pub result: Option<Rc<ExprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for ExprElsIfContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprElsIfContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_exprElsIf(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_exprElsIf(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprElsIfContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_exprElsIf(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprElsIfContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprElsIf }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprElsIf }
}
antlr_rust::tid!{ExprElsIfContextExt<'a>}

impl<'input> ExprElsIfContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprElsIfContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprElsIfContextExt{
				test: None, result: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ExprElsIfContextAttrs<'input>: aslParserContext<'input> + BorrowMut<ExprElsIfContextExt<'input>>{

fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExprElsIfContextAttrs<'input> for ExprElsIfContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn exprElsIf(&mut self,)
	-> Result<Rc<ExprElsIfContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExprElsIfContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_exprElsIf);
        let mut _localctx: Rc<ExprElsIfContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(796);
			recog.base.match_token(T__58,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(797);
			let tmp = recog.expr_rec(0)?;
			 cast_mut::<_,ExprElsIfContext >(&mut _localctx).test = Some(tmp.clone());
			  

			recog.base.set_state(798);
			recog.base.match_token(T__40,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(799);
			let tmp = recog.expr_rec(0)?;
			 cast_mut::<_,ExprElsIfContext >(&mut _localctx).result = Some(tmp.clone());
			  

			}
			Ok(())
		})();
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
//------------------- setElement ----------------
#[derive(Debug)]
pub enum SetElementContextAll<'input>{
	SetElementRangeContext(SetElementRangeContext<'input>),
	SetElementSingleContext(SetElementSingleContext<'input>),
Error(SetElementContext<'input>)
}
antlr_rust::tid!{SetElementContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for SetElementContextAll<'input>{}

impl<'input> aslParserContext<'input> for SetElementContextAll<'input>{}

impl<'input> Deref for SetElementContextAll<'input>{
	type Target = dyn SetElementContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use SetElementContextAll::*;
		match self{
			SetElementRangeContext(inner) => inner,
			SetElementSingleContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SetElementContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SetElementContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type SetElementContext<'input> = BaseParserRuleContext<'input,SetElementContextExt<'input>>;

#[derive(Clone)]
pub struct SetElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for SetElementContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SetElementContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SetElementContext<'input>{
}

impl<'input> CustomRuleContext<'input> for SetElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_setElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_setElement }
}
antlr_rust::tid!{SetElementContextExt<'a>}

impl<'input> SetElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SetElementContextAll<'input>> {
		Rc::new(
		SetElementContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SetElementContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait SetElementContextAttrs<'input>: aslParserContext<'input> + BorrowMut<SetElementContextExt<'input>>{


}

impl<'input> SetElementContextAttrs<'input> for SetElementContext<'input>{}

pub type SetElementRangeContext<'input> = BaseParserRuleContext<'input,SetElementRangeContextExt<'input>>;

pub trait SetElementRangeContextAttrs<'input>: aslParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> SetElementRangeContextAttrs<'input> for SetElementRangeContext<'input>{}

pub struct SetElementRangeContextExt<'input>{
	base:SetElementContextExt<'input>,
	pub begin: Option<Rc<ExprContextAll<'input>>>,
	pub end: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SetElementRangeContextExt<'a>}

impl<'input> aslParserContext<'input> for SetElementRangeContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SetElementRangeContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SetElementRange(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SetElementRange(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SetElementRangeContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SetElementRange(self);
	}
}

impl<'input> CustomRuleContext<'input> for SetElementRangeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_setElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_setElement }
}

impl<'input> Borrow<SetElementContextExt<'input>> for SetElementRangeContext<'input>{
	fn borrow(&self) -> &SetElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SetElementContextExt<'input>> for SetElementRangeContext<'input>{
	fn borrow_mut(&mut self) -> &mut SetElementContextExt<'input> { &mut self.base }
}

impl<'input> SetElementContextAttrs<'input> for SetElementRangeContext<'input> {}

impl<'input> SetElementRangeContextExt<'input>{
	fn new(ctx: &dyn SetElementContextAttrs<'input>) -> Rc<SetElementContextAll<'input>>  {
		Rc::new(
			SetElementContextAll::SetElementRangeContext(
				BaseParserRuleContext::copy_from(ctx,SetElementRangeContextExt{
        			begin:None, end:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SetElementSingleContext<'input> = BaseParserRuleContext<'input,SetElementSingleContextExt<'input>>;

pub trait SetElementSingleContextAttrs<'input>: aslParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SetElementSingleContextAttrs<'input> for SetElementSingleContext<'input>{}

pub struct SetElementSingleContextExt<'input>{
	base:SetElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SetElementSingleContextExt<'a>}

impl<'input> aslParserContext<'input> for SetElementSingleContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SetElementSingleContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SetElementSingle(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_SetElementSingle(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SetElementSingleContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_SetElementSingle(self);
	}
}

impl<'input> CustomRuleContext<'input> for SetElementSingleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_setElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_setElement }
}

impl<'input> Borrow<SetElementContextExt<'input>> for SetElementSingleContext<'input>{
	fn borrow(&self) -> &SetElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<SetElementContextExt<'input>> for SetElementSingleContext<'input>{
	fn borrow_mut(&mut self) -> &mut SetElementContextExt<'input> { &mut self.base }
}

impl<'input> SetElementContextAttrs<'input> for SetElementSingleContext<'input> {}

impl<'input> SetElementSingleContextExt<'input>{
	fn new(ctx: &dyn SetElementContextAttrs<'input>) -> Rc<SetElementContextAll<'input>>  {
		Rc::new(
			SetElementContextAll::SetElementSingleContext(
				BaseParserRuleContext::copy_from(ctx,SetElementSingleContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn setElement(&mut self,)
	-> Result<Rc<SetElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SetElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_setElement);
        let mut _localctx: Rc<SetElementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(806);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(65,&mut recog.base)? {
				1 =>{
					let tmp = SetElementRangeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule expr*/
					recog.base.set_state(801);
					let tmp = recog.expr_rec(0)?;
					if let SetElementContextAll::SetElementRangeContext(ctx) = cast_mut::<_,SetElementContextAll >(&mut _localctx){
					ctx.begin = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(802);
					recog.base.match_token(T__5,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(803);
					let tmp = recog.expr_rec(0)?;
					if let SetElementContextAll::SetElementRangeContext(ctx) = cast_mut::<_,SetElementContextAll >(&mut _localctx){
					ctx.end = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				2 =>{
					let tmp = SetElementSingleContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule expr*/
					recog.base.set_state(805);
					recog.expr_rec(0)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
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
//------------------- set ----------------
pub type SetContextAll<'input> = SetContext<'input>;


pub type SetContext<'input> = BaseParserRuleContext<'input,SetContextExt<'input>>;

#[derive(Clone)]
pub struct SetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for SetContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SetContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_set(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_set(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SetContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_set(self);
	}
}

impl<'input> CustomRuleContext<'input> for SetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_set }
	//fn type_rule_index() -> usize where Self: Sized { RULE_set }
}
antlr_rust::tid!{SetContextExt<'a>}

impl<'input> SetContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SetContextAttrs<'input>: aslParserContext<'input> + BorrowMut<SetContextExt<'input>>{

fn setElement_all(&self) ->  Vec<Rc<SetElementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn setElement(&self, i: usize) -> Option<Rc<SetElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SetContextAttrs<'input> for SetContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn set(&mut self,)
	-> Result<Rc<SetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_set);
        let mut _localctx: Rc<SetContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(808);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			recog.base.set_state(817);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__3 || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (T__31 - 32)) | (1usize << (T__33 - 32)) | (1usize << (T__36 - 32)) | (1usize << (T__37 - 32)) | (1usize << (T__39 - 32)) | (1usize << (T__62 - 32)))) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (T__66 - 67)) | (1usize << (T__67 - 67)) | (1usize << (T__88 - 67)) | (1usize << (T__89 - 67)) | (1usize << (IDENTIFIER - 67)) | (1usize << (NAT_LIT - 67)) | (1usize << (HEX_LIT - 67)) | (1usize << (BIN_LIT - 67)) | (1usize << (MASK_LIT - 67)) | (1usize << (REAL_LIT - 67)) | (1usize << (STRING_LIT - 67)))) != 0) {
				{
				/*InvokeRule setElement*/
				recog.base.set_state(809);
				recog.setElement()?;

				recog.base.set_state(814);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__9 {
					{
					{
					recog.base.set_state(810);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule setElement*/
					recog.base.set_state(811);
					recog.setElement()?;

					}
					}
					recog.base.set_state(816);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(819);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- sliceCommaList0 ----------------
pub type SliceCommaList0ContextAll<'input> = SliceCommaList0Context<'input>;


pub type SliceCommaList0Context<'input> = BaseParserRuleContext<'input,SliceCommaList0ContextExt<'input>>;

#[derive(Clone)]
pub struct SliceCommaList0ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for SliceCommaList0Context<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceCommaList0Context<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_sliceCommaList0(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_sliceCommaList0(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceCommaList0Context<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_sliceCommaList0(self);
	}
}

impl<'input> CustomRuleContext<'input> for SliceCommaList0ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sliceCommaList0 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sliceCommaList0 }
}
antlr_rust::tid!{SliceCommaList0ContextExt<'a>}

impl<'input> SliceCommaList0ContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SliceCommaList0ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SliceCommaList0ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SliceCommaList0ContextAttrs<'input>: aslParserContext<'input> + BorrowMut<SliceCommaList0ContextExt<'input>>{

fn slice_all(&self) ->  Vec<Rc<SliceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn slice(&self, i: usize) -> Option<Rc<SliceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SliceCommaList0ContextAttrs<'input> for SliceCommaList0Context<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sliceCommaList0(&mut self,)
	-> Result<Rc<SliceCommaList0ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SliceCommaList0ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_sliceCommaList0);
        let mut _localctx: Rc<SliceCommaList0ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(829);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__3 || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (T__31 - 32)) | (1usize << (T__33 - 32)) | (1usize << (T__36 - 32)) | (1usize << (T__37 - 32)) | (1usize << (T__39 - 32)) | (1usize << (T__62 - 32)))) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (T__66 - 67)) | (1usize << (T__67 - 67)) | (1usize << (T__88 - 67)) | (1usize << (T__89 - 67)) | (1usize << (IDENTIFIER - 67)) | (1usize << (NAT_LIT - 67)) | (1usize << (HEX_LIT - 67)) | (1usize << (BIN_LIT - 67)) | (1usize << (MASK_LIT - 67)) | (1usize << (REAL_LIT - 67)) | (1usize << (STRING_LIT - 67)))) != 0) {
				{
				/*InvokeRule slice*/
				recog.base.set_state(821);
				recog.slice()?;

				recog.base.set_state(826);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__9 {
					{
					{
					recog.base.set_state(822);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule slice*/
					recog.base.set_state(823);
					recog.slice()?;

					}
					}
					recog.base.set_state(828);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			}
			Ok(())
		})();
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
//------------------- sliceCommaList1 ----------------
pub type SliceCommaList1ContextAll<'input> = SliceCommaList1Context<'input>;


pub type SliceCommaList1Context<'input> = BaseParserRuleContext<'input,SliceCommaList1ContextExt<'input>>;

#[derive(Clone)]
pub struct SliceCommaList1ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for SliceCommaList1Context<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SliceCommaList1Context<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_sliceCommaList1(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_sliceCommaList1(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SliceCommaList1Context<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_sliceCommaList1(self);
	}
}

impl<'input> CustomRuleContext<'input> for SliceCommaList1ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sliceCommaList1 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sliceCommaList1 }
}
antlr_rust::tid!{SliceCommaList1ContextExt<'a>}

impl<'input> SliceCommaList1ContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SliceCommaList1ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SliceCommaList1ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SliceCommaList1ContextAttrs<'input>: aslParserContext<'input> + BorrowMut<SliceCommaList1ContextExt<'input>>{

fn slice_all(&self) ->  Vec<Rc<SliceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn slice(&self, i: usize) -> Option<Rc<SliceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SliceCommaList1ContextAttrs<'input> for SliceCommaList1Context<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sliceCommaList1(&mut self,)
	-> Result<Rc<SliceCommaList1ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SliceCommaList1ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_sliceCommaList1);
        let mut _localctx: Rc<SliceCommaList1ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule slice*/
			recog.base.set_state(831);
			recog.slice()?;

			recog.base.set_state(836);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__9 {
				{
				{
				recog.base.set_state(832);
				recog.base.match_token(T__9,&mut recog.err_handler)?;

				/*InvokeRule slice*/
				recog.base.set_state(833);
				recog.slice()?;

				}
				}
				recog.base.set_state(838);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			}
			Ok(())
		})();
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
//------------------- exprCommaList1 ----------------
pub type ExprCommaList1ContextAll<'input> = ExprCommaList1Context<'input>;


pub type ExprCommaList1Context<'input> = BaseParserRuleContext<'input,ExprCommaList1ContextExt<'input>>;

#[derive(Clone)]
pub struct ExprCommaList1ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for ExprCommaList1Context<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprCommaList1Context<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_exprCommaList1(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_exprCommaList1(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprCommaList1Context<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_exprCommaList1(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprCommaList1ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprCommaList1 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprCommaList1 }
}
antlr_rust::tid!{ExprCommaList1ContextExt<'a>}

impl<'input> ExprCommaList1ContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprCommaList1ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprCommaList1ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExprCommaList1ContextAttrs<'input>: aslParserContext<'input> + BorrowMut<ExprCommaList1ContextExt<'input>>{

fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExprCommaList1ContextAttrs<'input> for ExprCommaList1Context<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn exprCommaList1(&mut self,)
	-> Result<Rc<ExprCommaList1ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExprCommaList1ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_exprCommaList1);
        let mut _localctx: Rc<ExprCommaList1ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expr*/
			recog.base.set_state(839);
			recog.expr_rec(0)?;

			recog.base.set_state(844);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__9 {
				{
				{
				recog.base.set_state(840);
				recog.base.match_token(T__9,&mut recog.err_handler)?;

				/*InvokeRule expr*/
				recog.base.set_state(841);
				recog.expr_rec(0)?;

				}
				}
				recog.base.set_state(846);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
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
//------------------- exprCommaList0 ----------------
pub type ExprCommaList0ContextAll<'input> = ExprCommaList0Context<'input>;


pub type ExprCommaList0Context<'input> = BaseParserRuleContext<'input,ExprCommaList0ContextExt<'input>>;

#[derive(Clone)]
pub struct ExprCommaList0ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for ExprCommaList0Context<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for ExprCommaList0Context<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_exprCommaList0(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_exprCommaList0(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for ExprCommaList0Context<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_exprCommaList0(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprCommaList0ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprCommaList0 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprCommaList0 }
}
antlr_rust::tid!{ExprCommaList0ContextExt<'a>}

impl<'input> ExprCommaList0ContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprCommaList0ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprCommaList0ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExprCommaList0ContextAttrs<'input>: aslParserContext<'input> + BorrowMut<ExprCommaList0ContextExt<'input>>{

fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExprCommaList0ContextAttrs<'input> for ExprCommaList0Context<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn exprCommaList0(&mut self,)
	-> Result<Rc<ExprCommaList0ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExprCommaList0ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_exprCommaList0);
        let mut _localctx: Rc<ExprCommaList0ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(855);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__3 || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (T__31 - 32)) | (1usize << (T__33 - 32)) | (1usize << (T__36 - 32)) | (1usize << (T__37 - 32)) | (1usize << (T__39 - 32)) | (1usize << (T__62 - 32)))) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (T__66 - 67)) | (1usize << (T__67 - 67)) | (1usize << (T__88 - 67)) | (1usize << (T__89 - 67)) | (1usize << (IDENTIFIER - 67)) | (1usize << (NAT_LIT - 67)) | (1usize << (HEX_LIT - 67)) | (1usize << (BIN_LIT - 67)) | (1usize << (MASK_LIT - 67)) | (1usize << (REAL_LIT - 67)) | (1usize << (STRING_LIT - 67)))) != 0) {
				{
				/*InvokeRule expr*/
				recog.base.set_state(847);
				recog.expr_rec(0)?;

				recog.base.set_state(852);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__9 {
					{
					{
					recog.base.set_state(848);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(849);
					recog.expr_rec(0)?;

					}
					}
					recog.base.set_state(854);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			}
			Ok(())
		})();
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
//------------------- symDecl ----------------
pub type SymDeclContextAll<'input> = SymDeclContext<'input>;


pub type SymDeclContext<'input> = BaseParserRuleContext<'input,SymDeclContextExt<'input>>;

#[derive(Clone)]
pub struct SymDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for SymDeclContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SymDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_symDecl(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_symDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SymDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_symDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for SymDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_symDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_symDecl }
}
antlr_rust::tid!{SymDeclContextExt<'a>}

impl<'input> SymDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SymDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SymDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SymDeclContextAttrs<'input>: aslParserContext<'input> + BorrowMut<SymDeclContextExt<'input>>{

fn typeSpec(&self) -> Option<Rc<TypeSpecContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SymDeclContextAttrs<'input> for SymDeclContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn symDecl(&mut self,)
	-> Result<Rc<SymDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SymDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_symDecl);
        let mut _localctx: Rc<SymDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeSpec*/
			recog.base.set_state(857);
			recog.typeSpec()?;

			/*InvokeRule id*/
			recog.base.set_state(858);
			recog.id()?;

			}
			Ok(())
		})();
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
//------------------- identifierCommaList0 ----------------
pub type IdentifierCommaList0ContextAll<'input> = IdentifierCommaList0Context<'input>;


pub type IdentifierCommaList0Context<'input> = BaseParserRuleContext<'input,IdentifierCommaList0ContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierCommaList0ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for IdentifierCommaList0Context<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for IdentifierCommaList0Context<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_identifierCommaList0(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_identifierCommaList0(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for IdentifierCommaList0Context<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_identifierCommaList0(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdentifierCommaList0ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifierCommaList0 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifierCommaList0 }
}
antlr_rust::tid!{IdentifierCommaList0ContextExt<'a>}

impl<'input> IdentifierCommaList0ContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdentifierCommaList0ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentifierCommaList0ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdentifierCommaList0ContextAttrs<'input>: aslParserContext<'input> + BorrowMut<IdentifierCommaList0ContextExt<'input>>{

fn id_all(&self) ->  Vec<Rc<IdContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn id(&self, i: usize) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> IdentifierCommaList0ContextAttrs<'input> for IdentifierCommaList0Context<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn identifierCommaList0(&mut self,)
	-> Result<Rc<IdentifierCommaList0ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierCommaList0ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_identifierCommaList0);
        let mut _localctx: Rc<IdentifierCommaList0ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(868);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__33 || _la==T__37 || _la==T__67 || _la==IDENTIFIER {
				{
				/*InvokeRule id*/
				recog.base.set_state(860);
				recog.id()?;

				recog.base.set_state(865);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__9 {
					{
					{
					recog.base.set_state(861);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(862);
					recog.id()?;

					}
					}
					recog.base.set_state(867);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			}
			Ok(())
		})();
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
//------------------- identifierCommaList1 ----------------
pub type IdentifierCommaList1ContextAll<'input> = IdentifierCommaList1Context<'input>;


pub type IdentifierCommaList1Context<'input> = BaseParserRuleContext<'input,IdentifierCommaList1ContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierCommaList1ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for IdentifierCommaList1Context<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for IdentifierCommaList1Context<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_identifierCommaList1(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_identifierCommaList1(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for IdentifierCommaList1Context<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_identifierCommaList1(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdentifierCommaList1ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifierCommaList1 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifierCommaList1 }
}
antlr_rust::tid!{IdentifierCommaList1ContextExt<'a>}

impl<'input> IdentifierCommaList1ContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdentifierCommaList1ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentifierCommaList1ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdentifierCommaList1ContextAttrs<'input>: aslParserContext<'input> + BorrowMut<IdentifierCommaList1ContextExt<'input>>{

fn id_all(&self) ->  Vec<Rc<IdContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn id(&self, i: usize) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> IdentifierCommaList1ContextAttrs<'input> for IdentifierCommaList1Context<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn identifierCommaList1(&mut self,)
	-> Result<Rc<IdentifierCommaList1ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierCommaList1ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_identifierCommaList1);
        let mut _localctx: Rc<IdentifierCommaList1ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule id*/
			recog.base.set_state(870);
			recog.id()?;

			recog.base.set_state(875);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__9 {
				{
				{
				recog.base.set_state(871);
				recog.base.match_token(T__9,&mut recog.err_handler)?;

				/*InvokeRule id*/
				recog.base.set_state(872);
				recog.id()?;

				}
				}
				recog.base.set_state(877);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
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
//------------------- symDeclCommaList ----------------
pub type SymDeclCommaListContextAll<'input> = SymDeclCommaListContext<'input>;


pub type SymDeclCommaListContext<'input> = BaseParserRuleContext<'input,SymDeclCommaListContextExt<'input>>;

#[derive(Clone)]
pub struct SymDeclCommaListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for SymDeclCommaListContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for SymDeclCommaListContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_symDeclCommaList(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_symDeclCommaList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for SymDeclCommaListContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_symDeclCommaList(self);
	}
}

impl<'input> CustomRuleContext<'input> for SymDeclCommaListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_symDeclCommaList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_symDeclCommaList }
}
antlr_rust::tid!{SymDeclCommaListContextExt<'a>}

impl<'input> SymDeclCommaListContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SymDeclCommaListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SymDeclCommaListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SymDeclCommaListContextAttrs<'input>: aslParserContext<'input> + BorrowMut<SymDeclCommaListContextExt<'input>>{

fn symDecl_all(&self) ->  Vec<Rc<SymDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn symDecl(&self, i: usize) -> Option<Rc<SymDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SymDeclCommaListContextAttrs<'input> for SymDeclCommaListContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn symDeclCommaList(&mut self,)
	-> Result<Rc<SymDeclCommaListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SymDeclCommaListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_symDeclCommaList);
        let mut _localctx: Rc<SymDeclCommaListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(886);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__3 || ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (T__33 - 34)) | (1usize << (T__36 - 34)) | (1usize << (T__37 - 34)))) != 0) || ((((_la - 68)) & !0x3f) == 0 && ((1usize << (_la - 68)) & ((1usize << (T__67 - 68)) | (1usize << (T__88 - 68)) | (1usize << (T__89 - 68)) | (1usize << (IDENTIFIER - 68)))) != 0) {
				{
				/*InvokeRule symDecl*/
				recog.base.set_state(878);
				recog.symDecl()?;

				recog.base.set_state(883);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__9 {
					{
					{
					recog.base.set_state(879);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule symDecl*/
					recog.base.set_state(880);
					recog.symDecl()?;

					}
					}
					recog.base.set_state(885);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			}
			Ok(())
		})();
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
//------------------- qualId ----------------
#[derive(Debug)]
pub enum QualIdContextAll<'input>{
	QualIdUnqualifiedContext(QualIdUnqualifiedContext<'input>),
	QualIdAArch32Context(QualIdAArch32Context<'input>),
	QualIdAArch64Context(QualIdAArch64Context<'input>),
Error(QualIdContext<'input>)
}
antlr_rust::tid!{QualIdContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for QualIdContextAll<'input>{}

impl<'input> aslParserContext<'input> for QualIdContextAll<'input>{}

impl<'input> Deref for QualIdContextAll<'input>{
	type Target = dyn QualIdContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use QualIdContextAll::*;
		match self{
			QualIdUnqualifiedContext(inner) => inner,
			QualIdAArch32Context(inner) => inner,
			QualIdAArch64Context(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for QualIdContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn aslVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for QualIdContextAll<'input>{
    fn enter(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn aslListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type QualIdContext<'input> = BaseParserRuleContext<'input,QualIdContextExt<'input>>;

#[derive(Clone)]
pub struct QualIdContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for QualIdContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for QualIdContext<'input>{
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for QualIdContext<'input>{
}

impl<'input> CustomRuleContext<'input> for QualIdContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualId }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualId }
}
antlr_rust::tid!{QualIdContextExt<'a>}

impl<'input> QualIdContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QualIdContextAll<'input>> {
		Rc::new(
		QualIdContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QualIdContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait QualIdContextAttrs<'input>: aslParserContext<'input> + BorrowMut<QualIdContextExt<'input>>{


}

impl<'input> QualIdContextAttrs<'input> for QualIdContext<'input>{}

pub type QualIdUnqualifiedContext<'input> = BaseParserRuleContext<'input,QualIdUnqualifiedContextExt<'input>>;

pub trait QualIdUnqualifiedContextAttrs<'input>: aslParserContext<'input>{
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> QualIdUnqualifiedContextAttrs<'input> for QualIdUnqualifiedContext<'input>{}

pub struct QualIdUnqualifiedContextExt<'input>{
	base:QualIdContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{QualIdUnqualifiedContextExt<'a>}

impl<'input> aslParserContext<'input> for QualIdUnqualifiedContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for QualIdUnqualifiedContext<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_QualIdUnqualified(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_QualIdUnqualified(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for QualIdUnqualifiedContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_QualIdUnqualified(self);
	}
}

impl<'input> CustomRuleContext<'input> for QualIdUnqualifiedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualId }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualId }
}

impl<'input> Borrow<QualIdContextExt<'input>> for QualIdUnqualifiedContext<'input>{
	fn borrow(&self) -> &QualIdContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<QualIdContextExt<'input>> for QualIdUnqualifiedContext<'input>{
	fn borrow_mut(&mut self) -> &mut QualIdContextExt<'input> { &mut self.base }
}

impl<'input> QualIdContextAttrs<'input> for QualIdUnqualifiedContext<'input> {}

impl<'input> QualIdUnqualifiedContextExt<'input>{
	fn new(ctx: &dyn QualIdContextAttrs<'input>) -> Rc<QualIdContextAll<'input>>  {
		Rc::new(
			QualIdContextAll::QualIdUnqualifiedContext(
				BaseParserRuleContext::copy_from(ctx,QualIdUnqualifiedContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type QualIdAArch32Context<'input> = BaseParserRuleContext<'input,QualIdAArch32ContextExt<'input>>;

pub trait QualIdAArch32ContextAttrs<'input>: aslParserContext<'input>{
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> QualIdAArch32ContextAttrs<'input> for QualIdAArch32Context<'input>{}

pub struct QualIdAArch32ContextExt<'input>{
	base:QualIdContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{QualIdAArch32ContextExt<'a>}

impl<'input> aslParserContext<'input> for QualIdAArch32Context<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for QualIdAArch32Context<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_QualIdAArch32(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_QualIdAArch32(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for QualIdAArch32Context<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_QualIdAArch32(self);
	}
}

impl<'input> CustomRuleContext<'input> for QualIdAArch32ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualId }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualId }
}

impl<'input> Borrow<QualIdContextExt<'input>> for QualIdAArch32Context<'input>{
	fn borrow(&self) -> &QualIdContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<QualIdContextExt<'input>> for QualIdAArch32Context<'input>{
	fn borrow_mut(&mut self) -> &mut QualIdContextExt<'input> { &mut self.base }
}

impl<'input> QualIdContextAttrs<'input> for QualIdAArch32Context<'input> {}

impl<'input> QualIdAArch32ContextExt<'input>{
	fn new(ctx: &dyn QualIdContextAttrs<'input>) -> Rc<QualIdContextAll<'input>>  {
		Rc::new(
			QualIdContextAll::QualIdAArch32Context(
				BaseParserRuleContext::copy_from(ctx,QualIdAArch32ContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type QualIdAArch64Context<'input> = BaseParserRuleContext<'input,QualIdAArch64ContextExt<'input>>;

pub trait QualIdAArch64ContextAttrs<'input>: aslParserContext<'input>{
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> QualIdAArch64ContextAttrs<'input> for QualIdAArch64Context<'input>{}

pub struct QualIdAArch64ContextExt<'input>{
	base:QualIdContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{QualIdAArch64ContextExt<'a>}

impl<'input> aslParserContext<'input> for QualIdAArch64Context<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for QualIdAArch64Context<'input>{
	fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_QualIdAArch64(self);
	}
	fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
		listener.exit_QualIdAArch64(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for QualIdAArch64Context<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_QualIdAArch64(self);
	}
}

impl<'input> CustomRuleContext<'input> for QualIdAArch64ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualId }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualId }
}

impl<'input> Borrow<QualIdContextExt<'input>> for QualIdAArch64Context<'input>{
	fn borrow(&self) -> &QualIdContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<QualIdContextExt<'input>> for QualIdAArch64Context<'input>{
	fn borrow_mut(&mut self) -> &mut QualIdContextExt<'input> { &mut self.base }
}

impl<'input> QualIdContextAttrs<'input> for QualIdAArch64Context<'input> {}

impl<'input> QualIdAArch64ContextExt<'input>{
	fn new(ctx: &dyn QualIdContextAttrs<'input>) -> Rc<QualIdContextAll<'input>>  {
		Rc::new(
			QualIdContextAll::QualIdAArch64Context(
				BaseParserRuleContext::copy_from(ctx,QualIdAArch64ContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn qualId(&mut self,)
	-> Result<Rc<QualIdContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QualIdContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_qualId);
        let mut _localctx: Rc<QualIdContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(895);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__33 | T__37 | T__67 | IDENTIFIER 
				=> {
					let tmp = QualIdUnqualifiedContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule id*/
					recog.base.set_state(888);
					recog.id()?;

					}
				}

			 T__88 
				=> {
					let tmp = QualIdAArch32ContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(889);
					recog.base.match_token(T__88,&mut recog.err_handler)?;

					recog.base.set_state(890);
					recog.base.match_token(T__63,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(891);
					recog.id()?;

					}
				}

			 T__89 
				=> {
					let tmp = QualIdAArch64ContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(892);
					recog.base.match_token(T__89,&mut recog.err_handler)?;

					recog.base.set_state(893);
					recog.base.match_token(T__63,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(894);
					recog.id()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
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
//------------------- idWithDots ----------------
pub type IdWithDotsContextAll<'input> = IdWithDotsContext<'input>;


pub type IdWithDotsContext<'input> = BaseParserRuleContext<'input,IdWithDotsContextExt<'input>>;

#[derive(Clone)]
pub struct IdWithDotsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for IdWithDotsContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for IdWithDotsContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_idWithDots(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_idWithDots(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for IdWithDotsContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_idWithDots(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdWithDotsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_idWithDots }
	//fn type_rule_index() -> usize where Self: Sized { RULE_idWithDots }
}
antlr_rust::tid!{IdWithDotsContextExt<'a>}

impl<'input> IdWithDotsContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdWithDotsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdWithDotsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdWithDotsContextAttrs<'input>: aslParserContext<'input> + BorrowMut<IdWithDotsContextExt<'input>>{

fn id_all(&self) ->  Vec<Rc<IdContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn id(&self, i: usize) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token NAT_LIT in current rule
fn NAT_LIT_all(&self) -> Vec<Rc<TerminalNode<'input,aslParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NAT_LIT, starting from 0.
/// Returns `None` if number of children corresponding to token NAT_LIT is less or equal than `i`.
fn NAT_LIT(&self, i: usize) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(NAT_LIT, i)
}

}

impl<'input> IdWithDotsContextAttrs<'input> for IdWithDotsContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn idWithDots(&mut self,)
	-> Result<Rc<IdWithDotsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdWithDotsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_idWithDots);
        let mut _localctx: Rc<IdWithDotsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule id*/
			recog.base.set_state(897);
			recog.id()?;

			recog.base.set_state(905);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__63 {
				{
				{
				recog.base.set_state(898);
				recog.base.match_token(T__63,&mut recog.err_handler)?;

				recog.base.set_state(901);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 T__33 | T__37 | T__67 | IDENTIFIER 
					=> {
						{
						/*InvokeRule id*/
						recog.base.set_state(899);
						recog.id()?;

						}
					}

				 NAT_LIT 
					=> {
						{
						recog.base.set_state(900);
						recog.base.match_token(NAT_LIT,&mut recog.err_handler)?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				}
				recog.base.set_state(907);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
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
//------------------- id ----------------
pub type IdContextAll<'input> = IdContext<'input>;


pub type IdContext<'input> = BaseParserRuleContext<'input,IdContextExt<'input>>;

#[derive(Clone)]
pub struct IdContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> aslParserContext<'input> for IdContext<'input>{}

impl<'input,'a> Listenable<dyn aslListener<'input> + 'a> for IdContext<'input>{
		fn enter(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_id(self);
		}
		fn exit(&self,listener: &mut (dyn aslListener<'input> + 'a)) {
			listener.exit_id(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn aslVisitor<'input> + 'a> for IdContext<'input>{
	fn accept(&self,visitor: &mut (dyn aslVisitor<'input> + 'a)) {
		visitor.visit_id(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = aslParserContextType;
	fn get_rule_index(&self) -> usize { RULE_id }
	//fn type_rule_index() -> usize where Self: Sized { RULE_id }
}
antlr_rust::tid!{IdContextExt<'a>}

impl<'input> IdContextExt<'input>{
	fn new(parent: Option<Rc<dyn aslParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdContextAttrs<'input>: aslParserContext<'input> + BorrowMut<IdContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,aslParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> IdContextAttrs<'input> for IdContext<'input>{}

impl<'input, I, H> aslParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn id(&mut self,)
	-> Result<Rc<IdContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_id);
        let mut _localctx: Rc<IdContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(908);
			_la = recog.base.input.la(1);
			if { !(_la==T__33 || _la==T__37 || _la==T__67 || _la==IDENTIFIER) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
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
	\x6b\u{391}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x03\x02\x07\x02\x60\x0a\x02\x0c\x02\x0e\x02\
	\x63\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\x05\x03\x69\x0a\x03\x03\x04\
	\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x06\x03\x06\
	\x03\x06\x03\x06\x05\x06\u{80}\x0a\x06\x03\x07\x03\x07\x03\x07\x07\x07\u{85}\
	\x0a\x07\x0c\x07\x0e\x07\u{88}\x0b\x07\x05\x07\u{8a}\x0a\x07\x03\x08\x07\
	\x08\u{8d}\x0a\x08\x0c\x08\x0e\x08\u{90}\x0b\x08\x03\x08\x03\x08\x03\x09\
	\x03\x09\x03\x09\x03\x09\x06\x09\u{98}\x0a\x09\x0d\x09\x0e\x09\u{99}\x03\
	\x09\x03\x09\x05\x09\u{9e}\x0a\x09\x05\x09\u{a0}\x0a\x09\x03\x09\x03\x09\
	\x05\x09\u{a4}\x0a\x09\x03\x09\x05\x09\u{a7}\x0a\x09\x03\x09\x03\x09\x03\
	\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x07\x0a\u{b1}\x0a\x0a\x0c\x0a\
	\x0e\x0a\u{b4}\x0b\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x07\x0a\u{bb}\
	\x0a\x0a\x0c\x0a\x0e\x0a\u{be}\x0b\x0a\x03\x0a\x03\x0a\x05\x0a\u{c2}\x0a\
	\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\
	\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0d\x07\x0d\u{d2}\x0a\x0d\x0c\x0d\
	\x0e\x0d\u{d5}\x0b\x0d\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x05\
	\x0e\u{10a}\x0a\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x05\
	\x0e\u{112}\x0a\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x05\x0e\u{11f}\x0a\x0e\x03\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x05\x0e\u{129}\x0a\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x05\x0e\u{130}\x0a\x0e\x05\
	\x0e\u{132}\x0a\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\
	\x0f\x05\x0f\u{13b}\x0a\x0f\x03\x10\x03\x10\x03\x10\x07\x10\u{140}\x0a\x10\
	\x0c\x10\x0e\x10\u{143}\x0b\x10\x05\x10\u{145}\x0a\x10\x03\x11\x03\x11\x03\
	\x11\x03\x11\x03\x11\x07\x11\u{14c}\x0a\x11\x0c\x11\x0e\x11\u{14f}\x0b\x11\
	\x03\x11\x03\x11\x05\x11\u{153}\x0a\x11\x03\x12\x03\x12\x03\x12\x03\x12\
	\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\
	\x03\x12\x03\x12\x03\x12\x03\x12\x07\x12\u{166}\x0a\x12\x0c\x12\x0e\x12\
	\u{169}\x0b\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\
	\x03\x12\x03\x12\x05\x12\u{174}\x0a\x12\x03\x13\x03\x13\x03\x13\x03\x13\
	\x03\x13\x05\x13\u{17b}\x0a\x13\x03\x14\x03\x14\x03\x14\x07\x14\u{180}\x0a\
	\x14\x0c\x14\x0e\x14\u{183}\x0b\x14\x03\x14\x03\x14\x03\x15\x06\x15\u{188}\
	\x0a\x15\x0d\x15\x0e\x15\u{189}\x03\x16\x05\x16\u{18d}\x0a\x16\x03\x17\x03\
	\x17\x03\x17\x07\x17\u{192}\x0a\x17\x0c\x17\x0e\x17\u{195}\x0b\x17\x03\x17\
	\x03\x17\x05\x17\u{199}\x0a\x17\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\
	\x03\x18\x03\x18\x03\x18\x07\x18\u{1a3}\x0a\x18\x0c\x18\x0e\x18\u{1a6}\x0b\
	\x18\x03\x18\x03\x18\x05\x18\u{1aa}\x0a\x18\x03\x18\x03\x18\x03\x18\x03\
	\x18\x06\x18\u{1b0}\x0a\x18\x0d\x18\x0e\x18\u{1b1}\x03\x18\x03\x18\x03\x18\
	\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\
	\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x06\x18\u{1c7}\
	\x0a\x18\x0d\x18\x0e\x18\u{1c8}\x03\x18\x03\x18\x05\x18\u{1cd}\x0a\x18\x03\
	\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\
	\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\
	\x19\x03\x19\x03\x19\x03\x19\x03\x19\x05\x19\u{1e6}\x0a\x19\x03\x19\x03\
	\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\
	\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\
	\x19\x05\x19\u{1fc}\x0a\x19\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\
	\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x05\x1b\u{209}\x0a\x1b\x03\
	\x1c\x03\x1c\x03\x1c\x03\x1c\x07\x1c\u{20f}\x0a\x1c\x0c\x1c\x0e\x1c\u{212}\
	\x0b\x1c\x03\x1c\x03\x1c\x05\x1c\u{216}\x0a\x1c\x03\x1c\x03\x1c\x03\x1c\
	\x03\x1c\x05\x1c\u{21c}\x0a\x1c\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\
	\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x07\x1d\u{228}\x0a\x1d\x0c\x1d\
	\x0e\x1d\u{22b}\x0b\x1d\x03\x1d\x03\x1d\x05\x1d\u{22f}\x0a\x1d\x03\x1e\x03\
	\x1e\x03\x1e\x03\x1e\x03\x1e\x07\x1e\u{236}\x0a\x1e\x0c\x1e\x0e\x1e\u{239}\
	\x0b\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x07\x1e\u{241}\
	\x0a\x1e\x0c\x1e\x0e\x1e\u{244}\x0b\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\
	\x03\x1e\x03\x1e\x07\x1e\u{24c}\x0a\x1e\x0c\x1e\x0e\x1e\u{24f}\x0b\x1e\x03\
	\x1e\x03\x1e\x03\x1e\x03\x1e\x05\x1e\u{255}\x0a\x1e\x03\x1e\x03\x1e\x03\
	\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\
	\x1e\x03\x1e\x03\x1e\x07\x1e\u{265}\x0a\x1e\x0c\x1e\x0e\x1e\u{268}\x0b\x1e\
	\x05\x1e\u{26a}\x0a\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\
	\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x07\x1e\u{278}\x0a\x1e\
	\x0c\x1e\x0e\x1e\u{27b}\x0b\x1e\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\
	\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\
	\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\
	\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x05\x1f\u{29a}\x0a\x1f\
	\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x07\x1f\u{2a1}\x0a\x1f\x0c\x1f\
	\x0e\x1f\u{2a4}\x0b\x1f\x03\x1f\x03\x1f\x03\x1f\x05\x1f\u{2a9}\x0a\x1f\x03\
	\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\
	\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\
	\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\
	\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\
	\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\
	\x1f\x03\x1f\x03\x1f\x03\x1f\x07\x1f\u{2dc}\x0a\x1f\x0c\x1f\x0e\x1f\u{2df}\
	\x0b\x1f\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\
	\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\
	\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x07\x20\u{2f9}\
	\x0a\x20\x0c\x20\x0e\x20\u{2fc}\x0b\x20\x03\x20\x03\x20\x03\x20\x05\x20\
	\u{301}\x0a\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\
	\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x07\x20\u{30f}\x0a\x20\x0c\x20\
	\x0e\x20\u{312}\x0b\x20\x03\x21\x03\x21\x03\x21\x03\x21\x03\x21\x03\x21\
	\x03\x21\x03\x21\x03\x21\x05\x21\u{31d}\x0a\x21\x03\x22\x03\x22\x03\x22\
	\x03\x22\x03\x22\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\x05\x23\u{329}\
	\x0a\x23\x03\x24\x03\x24\x03\x24\x03\x24\x07\x24\u{32f}\x0a\x24\x0c\x24\
	\x0e\x24\u{332}\x0b\x24\x05\x24\u{334}\x0a\x24\x03\x24\x03\x24\x03\x25\x03\
	\x25\x03\x25\x07\x25\u{33b}\x0a\x25\x0c\x25\x0e\x25\u{33e}\x0b\x25\x05\x25\
	\u{340}\x0a\x25\x03\x26\x03\x26\x03\x26\x07\x26\u{345}\x0a\x26\x0c\x26\x0e\
	\x26\u{348}\x0b\x26\x03\x27\x03\x27\x03\x27\x07\x27\u{34d}\x0a\x27\x0c\x27\
	\x0e\x27\u{350}\x0b\x27\x03\x28\x03\x28\x03\x28\x07\x28\u{355}\x0a\x28\x0c\
	\x28\x0e\x28\u{358}\x0b\x28\x05\x28\u{35a}\x0a\x28\x03\x29\x03\x29\x03\x29\
	\x03\x2a\x03\x2a\x03\x2a\x07\x2a\u{362}\x0a\x2a\x0c\x2a\x0e\x2a\u{365}\x0b\
	\x2a\x05\x2a\u{367}\x0a\x2a\x03\x2b\x03\x2b\x03\x2b\x07\x2b\u{36c}\x0a\x2b\
	\x0c\x2b\x0e\x2b\u{36f}\x0b\x2b\x03\x2c\x03\x2c\x03\x2c\x07\x2c\u{374}\x0a\
	\x2c\x0c\x2c\x0e\x2c\u{377}\x0b\x2c\x05\x2c\u{379}\x0a\x2c\x03\x2d\x03\x2d\
	\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x05\x2d\u{382}\x0a\x2d\x03\x2e\
	\x03\x2e\x03\x2e\x03\x2e\x05\x2e\u{388}\x0a\x2e\x07\x2e\u{38a}\x0a\x2e\x0c\
	\x2e\x0e\x2e\u{38d}\x0b\x2e\x03\x2f\x03\x2f\x03\x2f\x02\x05\x3a\x3c\x3e\
	\x30\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\
	\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\
	\x48\x4a\x4c\x4e\x50\x52\x54\x56\x58\x5a\x5c\x02\x0f\x03\x02\x13\x16\x03\
	\x02\x61\x62\x04\x02\x29\x29\x5d\x5d\x03\x02\x2f\x30\x04\x02\x41\x41\x45\
	\x46\x03\x02\x4a\x4b\x04\x02\x41\x41\x4c\x4c\x04\x02\x0b\x0b\x4d\x56\x05\
	\x02\x1d\x1d\x43\x44\x57\x59\x04\x02\x40\x40\x5a\x5a\x04\x02\x41\x41\x45\
	\x45\x03\x02\x4d\x56\x06\x02\x24\x24\x28\x28\x46\x46\x5e\x5e\x02\u{3fe}\
	\x02\x61\x03\x02\x02\x02\x04\x68\x03\x02\x02\x02\x06\x6a\x03\x02\x02\x02\
	\x08\x72\x03\x02\x02\x02\x0a\x7b\x03\x02\x02\x02\x0c\u{89}\x03\x02\x02\x02\
	\x0e\u{8e}\x03\x02\x02\x02\x10\u{93}\x03\x02\x02\x02\x12\u{aa}\x03\x02\x02\
	\x02\x14\u{c5}\x03\x02\x02\x02\x16\u{cb}\x03\x02\x02\x02\x18\u{d3}\x03\x02\
	\x02\x02\x1a\u{131}\x03\x02\x02\x02\x1c\u{13a}\x03\x02\x02\x02\x1e\u{144}\
	\x03\x02\x02\x02\x20\u{152}\x03\x02\x02\x02\x22\u{173}\x03\x02\x02\x02\x24\
	\u{17a}\x03\x02\x02\x02\x26\u{17c}\x03\x02\x02\x02\x28\u{187}\x03\x02\x02\
	\x02\x2a\u{18c}\x03\x02\x02\x02\x2c\u{198}\x03\x02\x02\x02\x2e\u{1cc}\x03\
	\x02\x02\x02\x30\u{1fb}\x03\x02\x02\x02\x32\u{1fd}\x03\x02\x02\x02\x34\u{208}\
	\x03\x02\x02\x02\x36\u{21b}\x03\x02\x02\x02\x38\u{22e}\x03\x02\x02\x02\x3a\
	\u{254}\x03\x02\x02\x02\x3c\u{2a8}\x03\x02\x02\x02\x3e\u{300}\x03\x02\x02\
	\x02\x40\u{31c}\x03\x02\x02\x02\x42\u{31e}\x03\x02\x02\x02\x44\u{328}\x03\
	\x02\x02\x02\x46\u{32a}\x03\x02\x02\x02\x48\u{33f}\x03\x02\x02\x02\x4a\u{341}\
	\x03\x02\x02\x02\x4c\u{349}\x03\x02\x02\x02\x4e\u{359}\x03\x02\x02\x02\x50\
	\u{35b}\x03\x02\x02\x02\x52\u{366}\x03\x02\x02\x02\x54\u{368}\x03\x02\x02\
	\x02\x56\u{378}\x03\x02\x02\x02\x58\u{381}\x03\x02\x02\x02\x5a\u{383}\x03\
	\x02\x02\x02\x5c\u{38e}\x03\x02\x02\x02\x5e\x60\x05\x04\x03\x02\x5f\x5e\
	\x03\x02\x02\x02\x60\x63\x03\x02\x02\x02\x61\x5f\x03\x02\x02\x02\x61\x62\
	\x03\x02\x02\x02\x62\x64\x03\x02\x02\x02\x63\x61\x03\x02\x02\x02\x64\x65\
	\x07\x02\x02\x03\x65\x03\x03\x02\x02\x02\x66\x69\x05\x08\x05\x02\x67\x69\
	\x05\x06\x04\x02\x68\x66\x03\x02\x02\x02\x68\x67\x03\x02\x02\x02\x69\x05\
	\x03\x02\x02\x02\x6a\x6b\x07\x03\x02\x02\x6b\x6c\x07\x5f\x02\x02\x6c\x6d\
	\x07\x04\x02\x02\x6d\x6e\x05\x0c\x07\x02\x6e\x6f\x07\x05\x02\x02\x6f\x70\
	\x05\x5c\x2f\x02\x70\x71\x07\x5d\x02\x02\x71\x07\x03\x02\x02\x02\x72\x73\
	\x07\x06\x02\x02\x73\x74\x07\x07\x02\x02\x74\x75\x07\x5f\x02\x02\x75\x76\
	\x07\x08\x02\x02\x76\x77\x07\x5f\x02\x02\x77\x78\x07\x09\x02\x02\x78\x79\
	\x07\x0a\x02\x02\x79\x7a\x05\x06\x04\x02\x7a\x09\x03\x02\x02\x02\x7b\x7c\
	\x07\x5f\x02\x02\x7c\x7d\x07\x0b\x02\x02\x7d\x7f\x07\x5f\x02\x02\x7e\u{80}\
	\x05\x5c\x2f\x02\x7f\x7e\x03\x02\x02\x02\x7f\u{80}\x03\x02\x02\x02\u{80}\
	\x0b\x03\x02\x02\x02\u{81}\u{86}\x05\x0a\x06\x02\u{82}\u{83}\x07\x0c\x02\
	\x02\u{83}\u{85}\x05\x0a\x06\x02\u{84}\u{82}\x03\x02\x02\x02\u{85}\u{88}\
	\x03\x02\x02\x02\u{86}\u{84}\x03\x02\x02\x02\u{86}\u{87}\x03\x02\x02\x02\
	\u{87}\u{8a}\x03\x02\x02\x02\u{88}\u{86}\x03\x02\x02\x02\u{89}\u{81}\x03\
	\x02\x02\x02\u{89}\u{8a}\x03\x02\x02\x02\u{8a}\x0d\x03\x02\x02\x02\u{8b}\
	\u{8d}\x05\x10\x09\x02\u{8c}\u{8b}\x03\x02\x02\x02\u{8d}\u{90}\x03\x02\x02\
	\x02\u{8e}\u{8c}\x03\x02\x02\x02\u{8e}\u{8f}\x03\x02\x02\x02\u{8f}\u{91}\
	\x03\x02\x02\x02\u{90}\u{8e}\x03\x02\x02\x02\u{91}\u{92}\x07\x02\x02\x03\
	\u{92}\x0f\x03\x02\x02\x02\u{93}\u{94}\x07\x0d\x02\x02\u{94}\u{95}\x05\x5a\
	\x2e\x02\u{95}\u{97}\x07\x6a\x02\x02\u{96}\u{98}\x05\x12\x0a\x02\u{97}\u{96}\
	\x03\x02\x02\x02\u{98}\u{99}\x03\x02\x02\x02\u{99}\u{97}\x03\x02\x02\x02\
	\u{99}\u{9a}\x03\x02\x02\x02\u{9a}\u{9f}\x03\x02\x02\x02\u{9b}\u{9d}\x07\
	\x0e\x02\x02\u{9c}\u{9e}\x05\x28\x15\x02\u{9d}\u{9c}\x03\x02\x02\x02\u{9d}\
	\u{9e}\x03\x02\x02\x02\u{9e}\u{a0}\x03\x02\x02\x02\u{9f}\u{9b}\x03\x02\x02\
	\x02\u{9f}\u{a0}\x03\x02\x02\x02\u{a0}\u{a1}\x03\x02\x02\x02\u{a1}\u{a3}\
	\x07\x0f\x02\x02\u{a2}\u{a4}\x07\x10\x02\x02\u{a3}\u{a2}\x03\x02\x02\x02\
	\u{a3}\u{a4}\x03\x02\x02\x02\u{a4}\u{a6}\x03\x02\x02\x02\u{a5}\u{a7}\x05\
	\x28\x15\x02\u{a6}\u{a5}\x03\x02\x02\x02\u{a6}\u{a7}\x03\x02\x02\x02\u{a7}\
	\u{a8}\x03\x02\x02\x02\u{a8}\u{a9}\x07\x6b\x02\x02\u{a9}\x11\x03\x02\x02\
	\x02\u{aa}\u{ab}\x07\x11\x02\x02\u{ab}\u{ac}\x05\x5a\x2e\x02\u{ac}\u{ad}\
	\x07\x6a\x02\x02\u{ad}\u{ae}\x07\x12\x02\x02\u{ae}\u{b2}\x09\x02\x02\x02\
	\u{af}\u{b1}\x05\x14\x0b\x02\u{b0}\u{af}\x03\x02\x02\x02\u{b1}\u{b4}\x03\
	\x02\x02\x02\u{b2}\u{b0}\x03\x02\x02\x02\u{b2}\u{b3}\x03\x02\x02\x02\u{b3}\
	\u{b5}\x03\x02\x02\x02\u{b4}\u{b2}\x03\x02\x02\x02\u{b5}\u{b6}\x07\x17\x02\
	\x02\u{b6}\u{b7}\x09\x03\x02\x02\u{b7}\u{b8}\x07\x18\x02\x02\u{b8}\u{bc}\
	\x05\x3c\x1f\x02\u{b9}\u{bb}\x05\x16\x0c\x02\u{ba}\u{b9}\x03\x02\x02\x02\
	\u{bb}\u{be}\x03\x02\x02\x02\u{bc}\u{ba}\x03\x02\x02\x02\u{bc}\u{bd}\x03\
	\x02\x02\x02\u{bd}\u{bf}\x03\x02\x02\x02\u{be}\u{bc}\x03\x02\x02\x02\u{bf}\
	\u{c1}\x07\x19\x02\x02\u{c0}\u{c2}\x05\x28\x15\x02\u{c1}\u{c0}\x03\x02\x02\
	\x02\u{c1}\u{c2}\x03\x02\x02\x02\u{c2}\u{c3}\x03\x02\x02\x02\u{c3}\u{c4}\
	\x07\x6b\x02\x02\u{c4}\x13\x03\x02\x02\x02\u{c5}\u{c6}\x07\x1a\x02\x02\u{c6}\
	\u{c7}\x05\x5c\x2f\x02\u{c7}\u{c8}\x07\x5f\x02\x02\u{c8}\u{c9}\x07\x1b\x02\
	\x02\u{c9}\u{ca}\x07\x5f\x02\x02\u{ca}\x15\x03\x02\x02\x02\u{cb}\u{cc}\x07\
	\x1c\x02\x02\u{cc}\u{cd}\x07\x5f\x02\x02\u{cd}\u{ce}\x07\x1d\x02\x02\u{ce}\
	\u{cf}\x07\x61\x02\x02\u{cf}\x17\x03\x02\x02\x02\u{d0}\u{d2}\x05\x1a\x0e\
	\x02\u{d1}\u{d0}\x03\x02\x02\x02\u{d2}\u{d5}\x03\x02\x02\x02\u{d3}\u{d1}\
	\x03\x02\x02\x02\u{d3}\u{d4}\x03\x02\x02\x02\u{d4}\u{d6}\x03\x02\x02\x02\
	\u{d5}\u{d3}\x03\x02\x02\x02\u{d6}\u{d7}\x07\x02\x02\x03\u{d7}\x19\x03\x02\
	\x02\x02\u{d8}\u{d9}\x07\x1e\x02\x02\u{d9}\u{da}\x07\x1f\x02\x02\u{da}\u{db}\
	\x05\x5c\x2f\x02\u{db}\u{dc}\x07\x5d\x02\x02\u{dc}\u{132}\x03\x02\x02\x02\
	\u{dd}\u{de}\x07\x1f\x02\x02\u{de}\u{df}\x05\x5c\x2f\x02\u{df}\u{e0}\x07\
	\x5d\x02\x02\u{e0}\u{132}\x03\x02\x02\x02\u{e1}\u{e2}\x07\x1f\x02\x02\u{e2}\
	\u{e3}\x05\x5c\x2f\x02\u{e3}\u{e4}\x07\x20\x02\x02\u{e4}\u{e5}\x05\x22\x12\
	\x02\u{e5}\u{e6}\x07\x5d\x02\x02\u{e6}\u{132}\x03\x02\x02\x02\u{e7}\u{e8}\
	\x07\x1f\x02\x02\u{e8}\u{e9}\x05\x58\x2d\x02\u{e9}\u{ea}\x07\x21\x02\x02\
	\u{ea}\u{eb}\x07\x22\x02\x02\u{eb}\u{ec}\x05\x56\x2c\x02\u{ec}\u{ed}\x07\
	\x23\x02\x02\u{ed}\u{132}\x03\x02\x02\x02\u{ee}\u{ef}\x07\x24\x02\x02\u{ef}\
	\u{f0}\x05\x5c\x2f\x02\u{f0}\u{f1}\x07\x04\x02\x02\u{f1}\u{f2}\x05\x52\x2a\
	\x02\u{f2}\u{f3}\x07\x05\x02\x02\u{f3}\u{f4}\x07\x5d\x02\x02\u{f4}\u{132}\
	\x03\x02\x02\x02\u{f5}\u{f6}\x05\x22\x12\x02\u{f6}\u{f7}\x05\x58\x2d\x02\
	\u{f7}\u{f8}\x07\x5d\x02\x02\u{f8}\u{132}\x03\x02\x02\x02\u{f9}\u{fa}\x07\
	\x25\x02\x02\u{fa}\u{fb}\x05\x22\x12\x02\u{fb}\u{fc}\x05\x5c\x2f\x02\u{fc}\
	\u{fd}\x07\x20\x02\x02\u{fd}\u{fe}\x05\x3c\x1f\x02\u{fe}\u{ff}\x07\x5d\x02\
	\x02\u{ff}\u{132}\x03\x02\x02\x02\u{100}\u{101}\x07\x06\x02\x02\u{101}\u{102}\
	\x05\x22\x12\x02\u{102}\u{103}\x05\x5c\x2f\x02\u{103}\u{104}\x07\x07\x02\
	\x02\u{104}\u{105}\x05\x24\x13\x02\u{105}\u{106}\x07\x09\x02\x02\u{106}\
	\u{107}\x07\x5d\x02\x02\u{107}\u{132}\x03\x02\x02\x02\u{108}\u{10a}\x05\
	\x20\x11\x02\u{109}\u{108}\x03\x02\x02\x02\u{109}\u{10a}\x03\x02\x02\x02\
	\u{10a}\u{10b}\x03\x02\x02\x02\u{10b}\u{10c}\x05\x58\x2d\x02\u{10c}\u{10d}\
	\x07\x22\x02\x02\u{10d}\u{10e}\x05\x56\x2c\x02\u{10e}\u{111}\x07\x23\x02\
	\x02\u{10f}\u{112}\x05\x28\x15\x02\u{110}\u{112}\x07\x5d\x02\x02\u{111}\
	\u{10f}\x03\x02\x02\x02\u{111}\u{110}\x03\x02\x02\x02\u{112}\u{132}\x03\
	\x02\x02\x02\u{113}\u{114}\x05\x20\x11\x02\u{114}\u{115}\x05\x58\x2d\x02\
	\u{115}\u{116}\x05\x28\x15\x02\u{116}\u{132}\x03\x02\x02\x02\u{117}\u{118}\
	\x05\x20\x11\x02\u{118}\u{119}\x05\x58\x2d\x02\u{119}\u{11a}\x07\x07\x02\
	\x02\u{11a}\u{11b}\x05\x56\x2c\x02\u{11b}\u{11e}\x07\x09\x02\x02\u{11c}\
	\u{11f}\x05\x28\x15\x02\u{11d}\u{11f}\x07\x5d\x02\x02\u{11e}\u{11c}\x03\
	\x02\x02\x02\u{11e}\u{11d}\x03\x02\x02\x02\u{11f}\u{132}\x03\x02\x02\x02\
	\u{120}\u{121}\x05\x58\x2d\x02\u{121}\u{122}\x07\x07\x02\x02\u{122}\u{123}\
	\x05\x1e\x10\x02\u{123}\u{124}\x07\x09\x02\x02\u{124}\u{125}\x07\x20\x02\
	\x02\u{125}\u{128}\x05\x50\x29\x02\u{126}\u{129}\x05\x28\x15\x02\u{127}\
	\u{129}\x07\x5d\x02\x02\u{128}\u{126}\x03\x02\x02\x02\u{128}\u{127}\x03\
	\x02\x02\x02\u{129}\u{132}\x03\x02\x02\x02\u{12a}\u{12b}\x05\x58\x2d\x02\
	\u{12b}\u{12c}\x07\x20\x02\x02\u{12c}\u{12f}\x05\x50\x29\x02\u{12d}\u{130}\
	\x05\x28\x15\x02\u{12e}\u{130}\x07\x5d\x02\x02\u{12f}\u{12d}\x03\x02\x02\
	\x02\u{12f}\u{12e}\x03\x02\x02\x02\u{130}\u{132}\x03\x02\x02\x02\u{131}\
	\u{d8}\x03\x02\x02\x02\u{131}\u{dd}\x03\x02\x02\x02\u{131}\u{e1}\x03\x02\
	\x02\x02\u{131}\u{e7}\x03\x02\x02\x02\u{131}\u{ee}\x03\x02\x02\x02\u{131}\
	\u{f5}\x03\x02\x02\x02\u{131}\u{f9}\x03\x02\x02\x02\u{131}\u{100}\x03\x02\
	\x02\x02\u{131}\u{109}\x03\x02\x02\x02\u{131}\u{113}\x03\x02\x02\x02\u{131}\
	\u{117}\x03\x02\x02\x02\u{131}\u{120}\x03\x02\x02\x02\u{131}\u{12a}\x03\
	\x02\x02\x02\u{132}\x1b\x03\x02\x02\x02\u{133}\u{134}\x05\x22\x12\x02\u{134}\
	\u{135}\x07\x26\x02\x02\u{135}\u{136}\x05\x5c\x2f\x02\u{136}\u{13b}\x03\
	\x02\x02\x02\u{137}\u{138}\x05\x22\x12\x02\u{138}\u{139}\x05\x5c\x2f\x02\
	\u{139}\u{13b}\x03\x02\x02\x02\u{13a}\u{133}\x03\x02\x02\x02\u{13a}\u{137}\
	\x03\x02\x02\x02\u{13b}\x1d\x03\x02\x02\x02\u{13c}\u{141}\x05\x1c\x0f\x02\
	\u{13d}\u{13e}\x07\x0c\x02\x02\u{13e}\u{140}\x05\x1c\x0f\x02\u{13f}\u{13d}\
	\x03\x02\x02\x02\u{140}\u{143}\x03\x02\x02\x02\u{141}\u{13f}\x03\x02\x02\
	\x02\u{141}\u{142}\x03\x02\x02\x02\u{142}\u{145}\x03\x02\x02\x02\u{143}\
	\u{141}\x03\x02\x02\x02\u{144}\u{13c}\x03\x02\x02\x02\u{144}\u{145}\x03\
	\x02\x02\x02\u{145}\x1f\x03\x02\x02\x02\u{146}\u{153}\x05\x22\x12\x02\u{147}\
	\u{148}\x07\x22\x02\x02\u{148}\u{14d}\x05\x22\x12\x02\u{149}\u{14a}\x07\
	\x0c\x02\x02\u{14a}\u{14c}\x05\x22\x12\x02\u{14b}\u{149}\x03\x02\x02\x02\
	\u{14c}\u{14f}\x03\x02\x02\x02\u{14d}\u{14b}\x03\x02\x02\x02\u{14d}\u{14e}\
	\x03\x02\x02\x02\u{14e}\u{150}\x03\x02\x02\x02\u{14f}\u{14d}\x03\x02\x02\
	\x02\u{150}\u{151}\x07\x23\x02\x02\u{151}\u{153}\x03\x02\x02\x02\u{152}\
	\u{146}\x03\x02\x02\x02\u{152}\u{147}\x03\x02\x02\x02\u{153}\x21\x03\x02\
	\x02\x02\u{154}\u{174}\x05\x58\x2d\x02\u{155}\u{156}\x05\x5c\x2f\x02\u{156}\
	\u{157}\x07\x22\x02\x02\u{157}\u{158}\x05\x3c\x1f\x02\u{158}\u{159}\x07\
	\x23\x02\x02\u{159}\u{174}\x03\x02\x02\x02\u{15a}\u{15b}\x07\x27\x02\x02\
	\u{15b}\u{15c}\x07\x22\x02\x02\u{15c}\u{15d}\x05\x3c\x1f\x02\u{15d}\u{15e}\
	\x07\x23\x02\x02\u{15e}\u{174}\x03\x02\x02\x02\u{15f}\u{160}\x07\x28\x02\
	\x02\u{160}\u{161}\x07\x5f\x02\x02\u{161}\u{162}\x07\x04\x02\x02\u{162}\
	\u{167}\x05\x26\x14\x02\u{163}\u{164}\x07\x0c\x02\x02\u{164}\u{166}\x05\
	\x26\x14\x02\u{165}\u{163}\x03\x02\x02\x02\u{166}\u{169}\x03\x02\x02\x02\
	\u{167}\u{165}\x03\x02\x02\x02\u{167}\u{168}\x03\x02\x02\x02\u{168}\u{16a}\
	\x03\x02\x02\x02\u{169}\u{167}\x03\x02\x02\x02\u{16a}\u{16b}\x07\x05\x02\
	\x02\u{16b}\u{174}\x03\x02\x02\x02\u{16c}\u{16d}\x07\x06\x02\x02\u{16d}\
	\u{16e}\x07\x07\x02\x02\u{16e}\u{16f}\x05\x24\x13\x02\u{16f}\u{170}\x07\
	\x09\x02\x02\u{170}\u{171}\x07\x0a\x02\x02\u{171}\u{172}\x05\x22\x12\x02\
	\u{172}\u{174}\x03\x02\x02\x02\u{173}\u{154}\x03\x02\x02\x02\u{173}\u{155}\
	\x03\x02\x02\x02\u{173}\u{15a}\x03\x02\x02\x02\u{173}\u{15f}\x03\x02\x02\
	\x02\u{173}\u{16c}\x03\x02\x02\x02\u{174}\x23\x03\x02\x02\x02\u{175}\u{17b}\
	\x05\x5c\x2f\x02\u{176}\u{177}\x05\x3c\x1f\x02\u{177}\u{178}\x07\x08\x02\
	\x02\u{178}\u{179}\x05\x3c\x1f\x02\u{179}\u{17b}\x03\x02\x02\x02\u{17a}\
	\u{175}\x03\x02\x02\x02\u{17a}\u{176}\x03\x02\x02\x02\u{17b}\x25\x03\x02\
	\x02\x02\u{17c}\u{181}\x05\x40\x21\x02\u{17d}\u{17e}\x07\x0c\x02\x02\u{17e}\
	\u{180}\x05\x40\x21\x02\u{17f}\u{17d}\x03\x02\x02\x02\u{180}\u{183}\x03\
	\x02\x02\x02\u{181}\u{17f}\x03\x02\x02\x02\u{181}\u{182}\x03\x02\x02\x02\
	\u{182}\u{184}\x03\x02\x02\x02\u{183}\u{181}\x03\x02\x02\x02\u{184}\u{185}\
	\x05\x5c\x2f\x02\u{185}\x27\x03\x02\x02\x02\u{186}\u{188}\x05\x2e\x18\x02\
	\u{187}\u{186}\x03\x02\x02\x02\u{188}\u{189}\x03\x02\x02\x02\u{189}\u{187}\
	\x03\x02\x02\x02\u{189}\u{18a}\x03\x02\x02\x02\u{18a}\x29\x03\x02\x02\x02\
	\u{18b}\u{18d}\x05\x2c\x17\x02\u{18c}\u{18b}\x03\x02\x02\x02\u{18c}\u{18d}\
	\x03\x02\x02\x02\u{18d}\x2b\x03\x02\x02\x02\u{18e}\u{18f}\x05\x30\x19\x02\
	\u{18f}\u{190}\x07\x29\x02\x02\u{190}\u{192}\x03\x02\x02\x02\u{191}\u{18e}\
	\x03\x02\x02\x02\u{192}\u{195}\x03\x02\x02\x02\u{193}\u{191}\x03\x02\x02\
	\x02\u{193}\u{194}\x03\x02\x02\x02\u{194}\u{196}\x03\x02\x02\x02\u{195}\
	\u{193}\x03\x02\x02\x02\u{196}\u{199}\x05\x2e\x18\x02\u{197}\u{199}\x05\
	\x28\x15\x02\u{198}\u{193}\x03\x02\x02\x02\u{198}\u{197}\x03\x02\x02\x02\
	\u{199}\x2d\x03\x02\x02\x02\u{19a}\u{19b}\x05\x30\x19\x02\u{19b}\u{19c}\
	\x09\x04\x02\x02\u{19c}\u{1cd}\x03\x02\x02\x02\u{19d}\u{19e}\x07\x2a\x02\
	\x02\u{19e}\u{19f}\x05\x3c\x1f\x02\u{19f}\u{1a0}\x07\x2b\x02\x02\u{1a0}\
	\u{1a4}\x05\x2c\x17\x02\u{1a1}\u{1a3}\x05\x32\x1a\x02\u{1a2}\u{1a1}\x03\
	\x02\x02\x02\u{1a3}\u{1a6}\x03\x02\x02\x02\u{1a4}\u{1a2}\x03\x02\x02\x02\
	\u{1a4}\u{1a5}\x03\x02\x02\x02\u{1a5}\u{1a9}\x03\x02\x02\x02\u{1a6}\u{1a4}\
	\x03\x02\x02\x02\u{1a7}\u{1a8}\x07\x2c\x02\x02\u{1a8}\u{1aa}\x05\x2c\x17\
	\x02\u{1a9}\u{1a7}\x03\x02\x02\x02\u{1a9}\u{1aa}\x03\x02\x02\x02\u{1aa}\
	\u{1cd}\x03\x02\x02\x02\u{1ab}\u{1ac}\x07\x2d\x02\x02\u{1ac}\u{1ad}\x05\
	\x3c\x1f\x02\u{1ad}\u{1af}\x07\x0a\x02\x02\u{1ae}\u{1b0}\x05\x36\x1c\x02\
	\u{1af}\u{1ae}\x03\x02\x02\x02\u{1b0}\u{1b1}\x03\x02\x02\x02\u{1b1}\u{1af}\
	\x03\x02\x02\x02\u{1b1}\u{1b2}\x03\x02\x02\x02\u{1b2}\u{1cd}\x03\x02\x02\
	\x02\u{1b3}\u{1b4}\x07\x2e\x02\x02\u{1b4}\u{1b5}\x05\x5c\x2f\x02\u{1b5}\
	\u{1b6}\x07\x20\x02\x02\u{1b6}\u{1b7}\x05\x3c\x1f\x02\u{1b7}\u{1b8}\x09\
	\x05\x02\x02\u{1b8}\u{1b9}\x05\x3c\x1f\x02\u{1b9}\u{1ba}\x05\x28\x15\x02\
	\u{1ba}\u{1cd}\x03\x02\x02\x02\u{1bb}\u{1bc}\x07\x31\x02\x02\u{1bc}\u{1bd}\
	\x05\x3c\x1f\x02\u{1bd}\u{1be}\x07\x32\x02\x02\u{1be}\u{1bf}\x05\x28\x15\
	\x02\u{1bf}\u{1cd}\x03\x02\x02\x02\u{1c0}\u{1c1}\x07\x33\x02\x02\u{1c1}\
	\u{1c2}\x05\x28\x15\x02\u{1c2}\u{1c3}\x07\x34\x02\x02\u{1c3}\u{1c4}\x05\
	\x5c\x2f\x02\u{1c4}\u{1c6}\x07\x6a\x02\x02\u{1c5}\u{1c7}\x05\x34\x1b\x02\
	\u{1c6}\u{1c5}\x03\x02\x02\x02\u{1c7}\u{1c8}\x03\x02\x02\x02\u{1c8}\u{1c6}\
	\x03\x02\x02\x02\u{1c8}\u{1c9}\x03\x02\x02\x02\u{1c9}\u{1ca}\x03\x02\x02\
	\x02\u{1ca}\u{1cb}\x07\x6b\x02\x02\u{1cb}\u{1cd}\x03\x02\x02\x02\u{1cc}\
	\u{19a}\x03\x02\x02\x02\u{1cc}\u{19d}\x03\x02\x02\x02\u{1cc}\u{1ab}\x03\
	\x02\x02\x02\u{1cc}\u{1b3}\x03\x02\x02\x02\u{1cc}\u{1bb}\x03\x02\x02\x02\
	\u{1cc}\u{1c0}\x03\x02\x02\x02\u{1cd}\x2f\x03\x02\x02\x02\u{1ce}\u{1cf}\
	\x05\x22\x12\x02\u{1cf}\u{1d0}\x05\x54\x2b\x02\u{1d0}\u{1fc}\x03\x02\x02\
	\x02\u{1d1}\u{1d2}\x05\x50\x29\x02\u{1d2}\u{1d3}\x07\x20\x02\x02\u{1d3}\
	\u{1d4}\x05\x3c\x1f\x02\u{1d4}\u{1fc}\x03\x02\x02\x02\u{1d5}\u{1d6}\x07\
	\x25\x02\x02\u{1d6}\u{1d7}\x05\x50\x29\x02\u{1d7}\u{1d8}\x07\x20\x02\x02\
	\u{1d8}\u{1d9}\x05\x3c\x1f\x02\u{1d9}\u{1fc}\x03\x02\x02\x02\u{1da}\u{1db}\
	\x05\x3a\x1e\x02\u{1db}\u{1dc}\x07\x20\x02\x02\u{1dc}\u{1dd}\x05\x3c\x1f\
	\x02\u{1dd}\u{1fc}\x03\x02\x02\x02\u{1de}\u{1df}\x05\x58\x2d\x02\u{1df}\
	\u{1e0}\x07\x22\x02\x02\u{1e0}\u{1e1}\x05\x4e\x28\x02\u{1e1}\u{1e2}\x07\
	\x23\x02\x02\u{1e2}\u{1fc}\x03\x02\x02\x02\u{1e3}\u{1e5}\x07\x35\x02\x02\
	\u{1e4}\u{1e6}\x05\x3c\x1f\x02\u{1e5}\u{1e4}\x03\x02\x02\x02\u{1e5}\u{1e6}\
	\x03\x02\x02\x02\u{1e6}\u{1fc}\x03\x02\x02\x02\u{1e7}\u{1e8}\x07\x36\x02\
	\x02\u{1e8}\u{1fc}\x05\x3c\x1f\x02\u{1e9}\u{1fc}\x07\x37\x02\x02\u{1ea}\
	\u{1eb}\x07\x38\x02\x02\u{1eb}\u{1fc}\x07\x64\x02\x02\u{1ec}\u{1ed}\x07\
	\x39\x02\x02\u{1ed}\u{1ee}\x05\x28\x15\x02\u{1ee}\u{1ef}\x07\x3a\x02\x02\
	\u{1ef}\u{1f0}\x05\x3c\x1f\x02\u{1f0}\u{1fc}\x03\x02\x02\x02\u{1f1}\u{1f2}\
	\x07\x3b\x02\x02\u{1f2}\u{1fc}\x05\x5c\x2f\x02\u{1f3}\u{1fc}\x07\x3c\x02\
	\x02\u{1f4}\u{1fc}\x07\x65\x02\x02\u{1f5}\u{1f6}\x07\x24\x02\x02\u{1f6}\
	\u{1f7}\x05\x5c\x2f\x02\u{1f7}\u{1f8}\x07\x04\x02\x02\u{1f8}\u{1f9}\x05\
	\x52\x2a\x02\u{1f9}\u{1fa}\x07\x05\x02\x02\u{1fa}\u{1fc}\x03\x02\x02\x02\
	\u{1fb}\u{1ce}\x03\x02\x02\x02\u{1fb}\u{1d1}\x03\x02\x02\x02\u{1fb}\u{1d5}\
	\x03\x02\x02\x02\u{1fb}\u{1da}\x03\x02\x02\x02\u{1fb}\u{1de}\x03\x02\x02\
	\x02\u{1fb}\u{1e3}\x03\x02\x02\x02\u{1fb}\u{1e7}\x03\x02\x02\x02\u{1fb}\
	\u{1e9}\x03\x02\x02\x02\u{1fb}\u{1ea}\x03\x02\x02\x02\u{1fb}\u{1ec}\x03\
	\x02\x02\x02\u{1fb}\u{1f1}\x03\x02\x02\x02\u{1fb}\u{1f3}\x03\x02\x02\x02\
	\u{1fb}\u{1f4}\x03\x02\x02\x02\u{1fb}\u{1f5}\x03\x02\x02\x02\u{1fc}\x31\
	\x03\x02\x02\x02\u{1fd}\u{1fe}\x07\x3d\x02\x02\u{1fe}\u{1ff}\x05\x3c\x1f\
	\x02\u{1ff}\u{200}\x07\x2b\x02\x02\u{200}\u{201}\x05\x2c\x17\x02\u{201}\
	\x33\x03\x02\x02\x02\u{202}\u{203}\x07\x3e\x02\x02\u{203}\u{204}\x05\x3c\
	\x1f\x02\u{204}\u{205}\x05\x2c\x17\x02\u{205}\u{209}\x03\x02\x02\x02\u{206}\
	\u{207}\x07\x3f\x02\x02\u{207}\u{209}\x05\x2c\x17\x02\u{208}\u{202}\x03\
	\x02\x02\x02\u{208}\u{206}\x03\x02\x02\x02\u{209}\x35\x03\x02\x02\x02\u{20a}\
	\u{20b}\x07\x3e\x02\x02\u{20b}\u{210}\x05\x38\x1d\x02\u{20c}\u{20d}\x07\
	\x0c\x02\x02\u{20d}\u{20f}\x05\x38\x1d\x02\u{20e}\u{20c}\x03\x02\x02\x02\
	\u{20f}\u{212}\x03\x02\x02\x02\u{210}\u{20e}\x03\x02\x02\x02\u{210}\u{211}\
	\x03\x02\x02\x02\u{211}\u{215}\x03\x02\x02\x02\u{212}\u{210}\x03\x02\x02\
	\x02\u{213}\u{214}\x07\x40\x02\x02\u{214}\u{216}\x05\x3c\x1f\x02\u{215}\
	\u{213}\x03\x02\x02\x02\u{215}\u{216}\x03\x02\x02\x02\u{216}\u{217}\x03\
	\x02\x02\x02\u{217}\u{218}\x05\x2a\x16\x02\u{218}\u{21c}\x03\x02\x02\x02\
	\u{219}\u{21a}\x07\x3f\x02\x02\u{21a}\u{21c}\x05\x2a\x16\x02\u{21b}\u{20a}\
	\x03\x02\x02\x02\u{21b}\u{219}\x03\x02\x02\x02\u{21c}\x37\x03\x02\x02\x02\
	\u{21d}\u{22f}\x07\x5f\x02\x02\u{21e}\u{22f}\x07\x60\x02\x02\u{21f}\u{22f}\
	\x07\x61\x02\x02\u{220}\u{22f}\x07\x62\x02\x02\u{221}\u{22f}\x05\x5c\x2f\
	\x02\u{222}\u{22f}\x07\x41\x02\x02\u{223}\u{224}\x07\x22\x02\x02\u{224}\
	\u{229}\x05\x38\x1d\x02\u{225}\u{226}\x07\x0c\x02\x02\u{226}\u{228}\x05\
	\x38\x1d\x02\u{227}\u{225}\x03\x02\x02\x02\u{228}\u{22b}\x03\x02\x02\x02\
	\u{229}\u{227}\x03\x02\x02\x02\u{229}\u{22a}\x03\x02\x02\x02\u{22a}\u{22c}\
	\x03\x02\x02\x02\u{22b}\u{229}\x03\x02\x02\x02\u{22c}\u{22d}\x07\x23\x02\
	\x02\u{22d}\u{22f}\x03\x02\x02\x02\u{22e}\u{21d}\x03\x02\x02\x02\u{22e}\
	\u{21e}\x03\x02\x02\x02\u{22e}\u{21f}\x03\x02\x02\x02\u{22e}\u{220}\x03\
	\x02\x02\x02\u{22e}\u{221}\x03\x02\x02\x02\u{22e}\u{222}\x03\x02\x02\x02\
	\u{22e}\u{223}\x03\x02\x02\x02\u{22f}\x39\x03\x02\x02\x02\u{230}\u{231}\
	\x08\x1e\x01\x02\u{231}\u{232}\x07\x07\x02\x02\u{232}\u{237}\x05\x3a\x1e\
	\x02\u{233}\u{234}\x07\x0c\x02\x02\u{234}\u{236}\x05\x3a\x1e\x02\u{235}\
	\u{233}\x03\x02\x02\x02\u{236}\u{239}\x03\x02\x02\x02\u{237}\u{235}\x03\
	\x02\x02\x02\u{237}\u{238}\x03\x02\x02\x02\u{238}\u{23a}\x03\x02\x02\x02\
	\u{239}\u{237}\x03\x02\x02\x02\u{23a}\u{23b}\x07\x09\x02\x02\u{23b}\u{255}\
	\x03\x02\x02\x02\u{23c}\u{23d}\x07\x22\x02\x02\u{23d}\u{242}\x05\x3a\x1e\
	\x02\u{23e}\u{23f}\x07\x0c\x02\x02\u{23f}\u{241}\x05\x3a\x1e\x02\u{240}\
	\u{23e}\x03\x02\x02\x02\u{241}\u{244}\x03\x02\x02\x02\u{242}\u{240}\x03\
	\x02\x02\x02\u{242}\u{243}\x03\x02\x02\x02\u{243}\u{245}\x03\x02\x02\x02\
	\u{244}\u{242}\x03\x02\x02\x02\u{245}\u{246}\x07\x23\x02\x02\u{246}\u{255}\
	\x03\x02\x02\x02\u{247}\u{248}\x07\x43\x02\x02\u{248}\u{24d}\x05\x3a\x1e\
	\x02\u{249}\u{24a}\x07\x0c\x02\x02\u{24a}\u{24c}\x05\x3a\x1e\x02\u{24b}\
	\u{249}\x03\x02\x02\x02\u{24c}\u{24f}\x03\x02\x02\x02\u{24d}\u{24b}\x03\
	\x02\x02\x02\u{24d}\u{24e}\x03\x02\x02\x02\u{24e}\u{250}\x03\x02\x02\x02\
	\u{24f}\u{24d}\x03\x02\x02\x02\u{250}\u{251}\x07\x44\x02\x02\u{251}\u{255}\
	\x03\x02\x02\x02\u{252}\u{255}\x05\x58\x2d\x02\u{253}\u{255}\x07\x41\x02\
	\x02\u{254}\u{230}\x03\x02\x02\x02\u{254}\u{23c}\x03\x02\x02\x02\u{254}\
	\u{247}\x03\x02\x02\x02\u{254}\u{252}\x03\x02\x02\x02\u{254}\u{253}\x03\
	\x02\x02\x02\u{255}\u{279}\x03\x02\x02\x02\u{256}\u{257}\x0c\x0c\x02\x02\
	\u{257}\u{258}\x07\x42\x02\x02\u{258}\u{278}\x05\x5c\x2f\x02\u{259}\u{25a}\
	\x0c\x0b\x02\x02\u{25a}\u{25b}\x07\x42\x02\x02\u{25b}\u{25c}\x07\x07\x02\
	\x02\u{25c}\u{25d}\x05\x54\x2b\x02\u{25d}\u{25e}\x07\x09\x02\x02\u{25e}\
	\u{278}\x03\x02\x02\x02\u{25f}\u{260}\x0c\x0a\x02\x02\u{260}\u{269}\x07\
	\x07\x02\x02\u{261}\u{266}\x05\x40\x21\x02\u{262}\u{263}\x07\x0c\x02\x02\
	\u{263}\u{265}\x05\x40\x21\x02\u{264}\u{262}\x03\x02\x02\x02\u{265}\u{268}\
	\x03\x02\x02\x02\u{266}\u{264}\x03\x02\x02\x02\u{266}\u{267}\x03\x02\x02\
	\x02\u{267}\u{26a}\x03\x02\x02\x02\u{268}\u{266}\x03\x02\x02\x02\u{269}\
	\u{261}\x03\x02\x02\x02\u{269}\u{26a}\x03\x02\x02\x02\u{26a}\u{26b}\x03\
	\x02\x02\x02\u{26b}\u{278}\x07\x09\x02\x02\u{26c}\u{26d}\x0c\x07\x02\x02\
	\u{26d}\u{26e}\x07\x43\x02\x02\u{26e}\u{26f}\x05\x4a\x26\x02\u{26f}\u{270}\
	\x07\x44\x02\x02\u{270}\u{278}\x03\x02\x02\x02\u{271}\u{272}\x0c\x06\x02\
	\x02\u{272}\u{273}\x07\x42\x02\x02\u{273}\u{274}\x07\x43\x02\x02\u{274}\
	\u{275}\x05\x54\x2b\x02\u{275}\u{276}\x07\x44\x02\x02\u{276}\u{278}\x03\
	\x02\x02\x02\u{277}\u{256}\x03\x02\x02\x02\u{277}\u{259}\x03\x02\x02\x02\
	\u{277}\u{25f}\x03\x02\x02\x02\u{277}\u{26c}\x03\x02\x02\x02\u{277}\u{271}\
	\x03\x02\x02\x02\u{278}\u{27b}\x03\x02\x02\x02\u{279}\u{277}\x03\x02\x02\
	\x02\u{279}\u{27a}\x03\x02\x02\x02\u{27a}\x3b\x03\x02\x02\x02\u{27b}\u{279}\
	\x03\x02\x02\x02\u{27c}\u{27d}\x08\x1f\x01\x02\u{27d}\u{2a9}\x07\x5f\x02\
	\x02\u{27e}\u{2a9}\x07\x60\x02\x02\u{27f}\u{2a9}\x07\x63\x02\x02\u{280}\
	\u{2a9}\x07\x61\x02\x02\u{281}\u{2a9}\x07\x62\x02\x02\u{282}\u{2a9}\x07\
	\x64\x02\x02\u{283}\u{284}\x05\x58\x2d\x02\u{284}\u{285}\x07\x22\x02\x02\
	\u{285}\u{286}\x05\x4e\x28\x02\u{286}\u{287}\x07\x23\x02\x02\u{287}\u{2a9}\
	\x03\x02\x02\x02\u{288}\u{2a9}\x05\x58\x2d\x02\u{289}\u{28a}\x07\x22\x02\
	\x02\u{28a}\u{28b}\x05\x3c\x1f\x02\u{28b}\u{28c}\x07\x23\x02\x02\u{28c}\
	\u{2a9}\x03\x02\x02\x02\u{28d}\u{28e}\x07\x22\x02\x02\u{28e}\u{28f}\x05\
	\x4c\x27\x02\u{28f}\u{290}\x07\x23\x02\x02\u{290}\u{2a9}\x03\x02\x02\x02\
	\u{291}\u{292}\x09\x06\x02\x02\u{292}\u{2a9}\x05\x3c\x1f\x12\u{293}\u{294}\
	\x05\x22\x12\x02\u{294}\u{295}\x07\x47\x02\x02\u{295}\u{2a9}\x03\x02\x02\
	\x02\u{296}\u{297}\x05\x22\x12\x02\u{297}\u{299}\x07\x38\x02\x02\u{298}\
	\u{29a}\x07\x64\x02\x02\u{299}\u{298}\x03\x02\x02\x02\u{299}\u{29a}\x03\
	\x02\x02\x02\u{29a}\u{2a9}\x03\x02\x02\x02\u{29b}\u{29c}\x07\x2a\x02\x02\
	\u{29c}\u{29d}\x05\x3c\x1f\x02\u{29d}\u{29e}\x07\x2b\x02\x02\u{29e}\u{2a2}\
	\x05\x3c\x1f\x02\u{29f}\u{2a1}\x05\x42\x22\x02\u{2a0}\u{29f}\x03\x02\x02\
	\x02\u{2a1}\u{2a4}\x03\x02\x02\x02\u{2a2}\u{2a0}\x03\x02\x02\x02\u{2a2}\
	\u{2a3}\x03\x02\x02\x02\u{2a3}\u{2a5}\x03\x02\x02\x02\u{2a4}\u{2a2}\x03\
	\x02\x02\x02\u{2a5}\u{2a6}\x07\x2c\x02\x02\u{2a6}\u{2a7}\x05\x3c\x1f\x03\
	\u{2a7}\u{2a9}\x03\x02\x02\x02\u{2a8}\u{27c}\x03\x02\x02\x02\u{2a8}\u{27e}\
	\x03\x02\x02\x02\u{2a8}\u{27f}\x03\x02\x02\x02\u{2a8}\u{280}\x03\x02\x02\
	\x02\u{2a8}\u{281}\x03\x02\x02\x02\u{2a8}\u{282}\x03\x02\x02\x02\u{2a8}\
	\u{283}\x03\x02\x02\x02\u{2a8}\u{288}\x03\x02\x02\x02\u{2a8}\u{289}\x03\
	\x02\x02\x02\u{2a8}\u{28d}\x03\x02\x02\x02\u{2a8}\u{291}\x03\x02\x02\x02\
	\u{2a8}\u{293}\x03\x02\x02\x02\u{2a8}\u{296}\x03\x02\x02\x02\u{2a8}\u{29b}\
	\x03\x02\x02\x02\u{2a9}\u{2dd}\x03\x02\x02\x02\u{2aa}\u{2ab}\x0c\x09\x02\
	\x02\u{2ab}\u{2ac}\x07\x49\x02\x02\u{2ac}\u{2dc}\x05\x3c\x1f\x0a\u{2ad}\
	\u{2ae}\x0c\x08\x02\x02\u{2ae}\u{2af}\x09\x07\x02\x02\u{2af}\u{2dc}\x05\
	\x3c\x1f\x09\u{2b0}\u{2b1}\x0c\x07\x02\x02\u{2b1}\u{2b2}\x09\x08\x02\x02\
	\u{2b2}\u{2dc}\x05\x3c\x1f\x08\u{2b3}\u{2b4}\x0c\x06\x02\x02\u{2b4}\u{2b5}\
	\x09\x09\x02\x02\u{2b5}\u{2dc}\x05\x3c\x1f\x07\u{2b6}\u{2b7}\x0c\x05\x02\
	\x02\u{2b7}\u{2b8}\x09\x0a\x02\x02\u{2b8}\u{2dc}\x05\x3c\x1f\x06\u{2b9}\
	\u{2ba}\x0c\x04\x02\x02\u{2ba}\u{2bb}\x09\x0b\x02\x02\u{2bb}\u{2dc}\x05\
	\x3c\x1f\x05\u{2bc}\u{2bd}\x0c\x13\x02\x02\u{2bd}\u{2be}\x07\x42\x02\x02\
	\u{2be}\u{2dc}\x05\x5c\x2f\x02\u{2bf}\u{2c0}\x0c\x0f\x02\x02\u{2c0}\u{2c1}\
	\x07\x42\x02\x02\u{2c1}\u{2c2}\x07\x07\x02\x02\u{2c2}\u{2c3}\x05\x54\x2b\
	\x02\u{2c3}\u{2c4}\x07\x09\x02\x02\u{2c4}\u{2dc}\x03\x02\x02\x02\u{2c5}\
	\u{2c6}\x0c\x0e\x02\x02\u{2c6}\u{2c7}\x07\x42\x02\x02\u{2c7}\u{2c8}\x07\
	\x43\x02\x02\u{2c8}\u{2c9}\x05\x54\x2b\x02\u{2c9}\u{2ca}\x07\x44\x02\x02\
	\u{2ca}\u{2dc}\x03\x02\x02\x02\u{2cb}\u{2cc}\x0c\x0d\x02\x02\u{2cc}\u{2cd}\
	\x07\x07\x02\x02\u{2cd}\u{2ce}\x05\x48\x25\x02\u{2ce}\u{2cf}\x07\x09\x02\
	\x02\u{2cf}\u{2dc}\x03\x02\x02\x02\u{2d0}\u{2d1}\x0c\x0c\x02\x02\u{2d1}\
	\u{2d2}\x07\x48\x02\x02\u{2d2}\u{2dc}\x05\x46\x24\x02\u{2d3}\u{2d4}\x0c\
	\x0b\x02\x02\u{2d4}\u{2d5}\x07\x48\x02\x02\u{2d5}\u{2dc}\x07\x62\x02\x02\
	\u{2d6}\u{2d7}\x0c\x0a\x02\x02\u{2d7}\u{2d8}\x07\x43\x02\x02\u{2d8}\u{2d9}\
	\x05\x4a\x26\x02\u{2d9}\u{2da}\x07\x44\x02\x02\u{2da}\u{2dc}\x03\x02\x02\
	\x02\u{2db}\u{2aa}\x03\x02\x02\x02\u{2db}\u{2ad}\x03\x02\x02\x02\u{2db}\
	\u{2b0}\x03\x02\x02\x02\u{2db}\u{2b3}\x03\x02\x02\x02\u{2db}\u{2b6}\x03\
	\x02\x02\x02\u{2db}\u{2b9}\x03\x02\x02\x02\u{2db}\u{2bc}\x03\x02\x02\x02\
	\u{2db}\u{2bf}\x03\x02\x02\x02\u{2db}\u{2c5}\x03\x02\x02\x02\u{2db}\u{2cb}\
	\x03\x02\x02\x02\u{2db}\u{2d0}\x03\x02\x02\x02\u{2db}\u{2d3}\x03\x02\x02\
	\x02\u{2db}\u{2d6}\x03\x02\x02\x02\u{2dc}\u{2df}\x03\x02\x02\x02\u{2dd}\
	\u{2db}\x03\x02\x02\x02\u{2dd}\u{2de}\x03\x02\x02\x02\u{2de}\x3d\x03\x02\
	\x02\x02\u{2df}\u{2dd}\x03\x02\x02\x02\u{2e0}\u{2e1}\x08\x20\x01\x02\u{2e1}\
	\u{301}\x07\x5f\x02\x02\u{2e2}\u{301}\x07\x60\x02\x02\u{2e3}\u{2e4}\x07\
	\x22\x02\x02\u{2e4}\u{2e5}\x05\x3e\x20\x02\u{2e5}\u{2e6}\x07\x23\x02\x02\
	\u{2e6}\u{301}\x03\x02\x02\x02\u{2e7}\u{301}\x05\x58\x2d\x02\u{2e8}\u{2e9}\
	\x05\x58\x2d\x02\u{2e9}\u{2ea}\x07\x22\x02\x02\u{2ea}\u{2eb}\x05\x4e\x28\
	\x02\u{2eb}\u{2ec}\x07\x23\x02\x02\u{2ec}\u{301}\x03\x02\x02\x02\u{2ed}\
	\u{2ee}\x09\x0c\x02\x02\u{2ee}\u{301}\x05\x3e\x20\x09\u{2ef}\u{2f0}\x05\
	\x3c\x1f\x02\u{2f0}\u{2f1}\x07\x42\x02\x02\u{2f1}\u{2f2}\x05\x5c\x2f\x02\
	\u{2f2}\u{301}\x03\x02\x02\x02\u{2f3}\u{2f4}\x07\x2a\x02\x02\u{2f4}\u{2f5}\
	\x05\x3c\x1f\x02\u{2f5}\u{2f6}\x07\x2b\x02\x02\u{2f6}\u{2fa}\x05\x3c\x1f\
	\x02\u{2f7}\u{2f9}\x05\x42\x22\x02\u{2f8}\u{2f7}\x03\x02\x02\x02\u{2f9}\
	\u{2fc}\x03\x02\x02\x02\u{2fa}\u{2f8}\x03\x02\x02\x02\u{2fa}\u{2fb}\x03\
	\x02\x02\x02\u{2fb}\u{2fd}\x03\x02\x02\x02\u{2fc}\u{2fa}\x03\x02\x02\x02\
	\u{2fd}\u{2fe}\x07\x2c\x02\x02\u{2fe}\u{2ff}\x05\x3c\x1f\x02\u{2ff}\u{301}\
	\x03\x02\x02\x02\u{300}\u{2e0}\x03\x02\x02\x02\u{300}\u{2e2}\x03\x02\x02\
	\x02\u{300}\u{2e3}\x03\x02\x02\x02\u{300}\u{2e7}\x03\x02\x02\x02\u{300}\
	\u{2e8}\x03\x02\x02\x02\u{300}\u{2ed}\x03\x02\x02\x02\u{300}\u{2ef}\x03\
	\x02\x02\x02\u{300}\u{2f3}\x03\x02\x02\x02\u{301}\u{310}\x03\x02\x02\x02\
	\u{302}\u{303}\x0c\x07\x02\x02\u{303}\u{304}\x07\x49\x02\x02\u{304}\u{30f}\
	\x05\x3e\x20\x08\u{305}\u{306}\x0c\x06\x02\x02\u{306}\u{307}\x09\x07\x02\
	\x02\u{307}\u{30f}\x05\x3e\x20\x07\u{308}\u{309}\x0c\x05\x02\x02\u{309}\
	\u{30a}\x09\x08\x02\x02\u{30a}\u{30f}\x05\x3e\x20\x06\u{30b}\u{30c}\x0c\
	\x04\x02\x02\u{30c}\u{30d}\x09\x0d\x02\x02\u{30d}\u{30f}\x05\x3e\x20\x05\
	\u{30e}\u{302}\x03\x02\x02\x02\u{30e}\u{305}\x03\x02\x02\x02\u{30e}\u{308}\
	\x03\x02\x02\x02\u{30e}\u{30b}\x03\x02\x02\x02\u{30f}\u{312}\x03\x02\x02\
	\x02\u{310}\u{30e}\x03\x02\x02\x02\u{310}\u{311}\x03\x02\x02\x02\u{311}\
	\x3f\x03\x02\x02\x02\u{312}\u{310}\x03\x02\x02\x02\u{313}\u{314}\x05\x3e\
	\x20\x02\u{314}\u{315}\x07\x0b\x02\x02\u{315}\u{316}\x05\x3e\x20\x02\u{316}\
	\u{31d}\x03\x02\x02\x02\u{317}\u{318}\x05\x3e\x20\x02\u{318}\u{319}\x07\
	\x1b\x02\x02\u{319}\u{31a}\x05\x3e\x20\x02\u{31a}\u{31d}\x03\x02\x02\x02\
	\u{31b}\u{31d}\x05\x3c\x1f\x02\u{31c}\u{313}\x03\x02\x02\x02\u{31c}\u{317}\
	\x03\x02\x02\x02\u{31c}\u{31b}\x03\x02\x02\x02\u{31d}\x41\x03\x02\x02\x02\
	\u{31e}\u{31f}\x07\x3d\x02\x02\u{31f}\u{320}\x05\x3c\x1f\x02\u{320}\u{321}\
	\x07\x2b\x02\x02\u{321}\u{322}\x05\x3c\x1f\x02\u{322}\x43\x03\x02\x02\x02\
	\u{323}\u{324}\x05\x3c\x1f\x02\u{324}\u{325}\x07\x08\x02\x02\u{325}\u{326}\
	\x05\x3c\x1f\x02\u{326}\u{329}\x03\x02\x02\x02\u{327}\u{329}\x05\x3c\x1f\
	\x02\u{328}\u{323}\x03\x02\x02\x02\u{328}\u{327}\x03\x02\x02\x02\u{329}\
	\x45\x03\x02\x02\x02\u{32a}\u{333}\x07\x04\x02\x02\u{32b}\u{330}\x05\x44\
	\x23\x02\u{32c}\u{32d}\x07\x0c\x02\x02\u{32d}\u{32f}\x05\x44\x23\x02\u{32e}\
	\u{32c}\x03\x02\x02\x02\u{32f}\u{332}\x03\x02\x02\x02\u{330}\u{32e}\x03\
	\x02\x02\x02\u{330}\u{331}\x03\x02\x02\x02\u{331}\u{334}\x03\x02\x02\x02\
	\u{332}\u{330}\x03\x02\x02\x02\u{333}\u{32b}\x03\x02\x02\x02\u{333}\u{334}\
	\x03\x02\x02\x02\u{334}\u{335}\x03\x02\x02\x02\u{335}\u{336}\x07\x05\x02\
	\x02\u{336}\x47\x03\x02\x02\x02\u{337}\u{33c}\x05\x40\x21\x02\u{338}\u{339}\
	\x07\x0c\x02\x02\u{339}\u{33b}\x05\x40\x21\x02\u{33a}\u{338}\x03\x02\x02\
	\x02\u{33b}\u{33e}\x03\x02\x02\x02\u{33c}\u{33a}\x03\x02\x02\x02\u{33c}\
	\u{33d}\x03\x02\x02\x02\u{33d}\u{340}\x03\x02\x02\x02\u{33e}\u{33c}\x03\
	\x02\x02\x02\u{33f}\u{337}\x03\x02\x02\x02\u{33f}\u{340}\x03\x02\x02\x02\
	\u{340}\x49\x03\x02\x02\x02\u{341}\u{346}\x05\x40\x21\x02\u{342}\u{343}\
	\x07\x0c\x02\x02\u{343}\u{345}\x05\x40\x21\x02\u{344}\u{342}\x03\x02\x02\
	\x02\u{345}\u{348}\x03\x02\x02\x02\u{346}\u{344}\x03\x02\x02\x02\u{346}\
	\u{347}\x03\x02\x02\x02\u{347}\x4b\x03\x02\x02\x02\u{348}\u{346}\x03\x02\
	\x02\x02\u{349}\u{34e}\x05\x3c\x1f\x02\u{34a}\u{34b}\x07\x0c\x02\x02\u{34b}\
	\u{34d}\x05\x3c\x1f\x02\u{34c}\u{34a}\x03\x02\x02\x02\u{34d}\u{350}\x03\
	\x02\x02\x02\u{34e}\u{34c}\x03\x02\x02\x02\u{34e}\u{34f}\x03\x02\x02\x02\
	\u{34f}\x4d\x03\x02\x02\x02\u{350}\u{34e}\x03\x02\x02\x02\u{351}\u{356}\
	\x05\x3c\x1f\x02\u{352}\u{353}\x07\x0c\x02\x02\u{353}\u{355}\x05\x3c\x1f\
	\x02\u{354}\u{352}\x03\x02\x02\x02\u{355}\u{358}\x03\x02\x02\x02\u{356}\
	\u{354}\x03\x02\x02\x02\u{356}\u{357}\x03\x02\x02\x02\u{357}\u{35a}\x03\
	\x02\x02\x02\u{358}\u{356}\x03\x02\x02\x02\u{359}\u{351}\x03\x02\x02\x02\
	\u{359}\u{35a}\x03\x02\x02\x02\u{35a}\x4f\x03\x02\x02\x02\u{35b}\u{35c}\
	\x05\x22\x12\x02\u{35c}\u{35d}\x05\x5c\x2f\x02\u{35d}\x51\x03\x02\x02\x02\
	\u{35e}\u{363}\x05\x5c\x2f\x02\u{35f}\u{360}\x07\x0c\x02\x02\u{360}\u{362}\
	\x05\x5c\x2f\x02\u{361}\u{35f}\x03\x02\x02\x02\u{362}\u{365}\x03\x02\x02\
	\x02\u{363}\u{361}\x03\x02\x02\x02\u{363}\u{364}\x03\x02\x02\x02\u{364}\
	\u{367}\x03\x02\x02\x02\u{365}\u{363}\x03\x02\x02\x02\u{366}\u{35e}\x03\
	\x02\x02\x02\u{366}\u{367}\x03\x02\x02\x02\u{367}\x53\x03\x02\x02\x02\u{368}\
	\u{36d}\x05\x5c\x2f\x02\u{369}\u{36a}\x07\x0c\x02\x02\u{36a}\u{36c}\x05\
	\x5c\x2f\x02\u{36b}\u{369}\x03\x02\x02\x02\u{36c}\u{36f}\x03\x02\x02\x02\
	\u{36d}\u{36b}\x03\x02\x02\x02\u{36d}\u{36e}\x03\x02\x02\x02\u{36e}\x55\
	\x03\x02\x02\x02\u{36f}\u{36d}\x03\x02\x02\x02\u{370}\u{375}\x05\x50\x29\
	\x02\u{371}\u{372}\x07\x0c\x02\x02\u{372}\u{374}\x05\x50\x29\x02\u{373}\
	\u{371}\x03\x02\x02\x02\u{374}\u{377}\x03\x02\x02\x02\u{375}\u{373}\x03\
	\x02\x02\x02\u{375}\u{376}\x03\x02\x02\x02\u{376}\u{379}\x03\x02\x02\x02\
	\u{377}\u{375}\x03\x02\x02\x02\u{378}\u{370}\x03\x02\x02\x02\u{378}\u{379}\
	\x03\x02\x02\x02\u{379}\x57\x03\x02\x02\x02\u{37a}\u{382}\x05\x5c\x2f\x02\
	\u{37b}\u{37c}\x07\x5b\x02\x02\u{37c}\u{37d}\x07\x42\x02\x02\u{37d}\u{382}\
	\x05\x5c\x2f\x02\u{37e}\u{37f}\x07\x5c\x02\x02\u{37f}\u{380}\x07\x42\x02\
	\x02\u{380}\u{382}\x05\x5c\x2f\x02\u{381}\u{37a}\x03\x02\x02\x02\u{381}\
	\u{37b}\x03\x02\x02\x02\u{381}\u{37e}\x03\x02\x02\x02\u{382}\x59\x03\x02\
	\x02\x02\u{383}\u{38b}\x05\x5c\x2f\x02\u{384}\u{387}\x07\x42\x02\x02\u{385}\
	\u{388}\x05\x5c\x2f\x02\u{386}\u{388}\x07\x5f\x02\x02\u{387}\u{385}\x03\
	\x02\x02\x02\u{387}\u{386}\x03\x02\x02\x02\u{388}\u{38a}\x03\x02\x02\x02\
	\u{389}\u{384}\x03\x02\x02\x02\u{38a}\u{38d}\x03\x02\x02\x02\u{38b}\u{389}\
	\x03\x02\x02\x02\u{38b}\u{38c}\x03\x02\x02\x02\u{38c}\x5b\x03\x02\x02\x02\
	\u{38d}\u{38b}\x03\x02\x02\x02\u{38e}\u{38f}\x09\x0e\x02\x02\u{38f}\x5d\
	\x03\x02\x02\x02\x54\x61\x68\x7f\u{86}\u{89}\u{8e}\u{99}\u{9d}\u{9f}\u{a3}\
	\u{a6}\u{b2}\u{bc}\u{c1}\u{d3}\u{109}\u{111}\u{11e}\u{128}\u{12f}\u{131}\
	\u{13a}\u{141}\u{144}\u{14d}\u{152}\u{167}\u{173}\u{17a}\u{181}\u{189}\u{18c}\
	\u{193}\u{198}\u{1a4}\u{1a9}\u{1b1}\u{1c8}\u{1cc}\u{1e5}\u{1fb}\u{208}\u{210}\
	\u{215}\u{21b}\u{229}\u{22e}\u{237}\u{242}\u{24d}\u{254}\u{266}\u{269}\u{277}\
	\u{279}\u{299}\u{2a2}\u{2a8}\u{2db}\u{2dd}\u{2fa}\u{300}\u{30e}\u{310}\u{31c}\
	\u{328}\u{330}\u{333}\u{33c}\u{33f}\u{346}\u{34e}\u{356}\u{359}\u{363}\u{366}\
	\u{36d}\u{375}\u{378}\u{381}\u{387}\u{38b}";

