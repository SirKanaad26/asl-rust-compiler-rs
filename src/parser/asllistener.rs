#![allow(nonstandard_style)]
// Generated from grammar/asl.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::aslparser::*;

pub trait aslListener<'input> : ParseTreeListener<'input,aslParserContextType>{
/**
 * Enter a parse tree produced by {@link aslParser#registers}.
 * @param ctx the parse tree
 */
fn enter_registers(&mut self, _ctx: &RegistersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#registers}.
 * @param ctx the parse tree
 */
fn exit_registers(&mut self, _ctx: &RegistersContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code RegDefArray}
 * labeled alternative in {@link aslParser#registerDefinition}.
 * @param ctx the parse tree
 */
fn enter_RegDefArray(&mut self, _ctx: &RegDefArrayContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code RegDefArray}
 * labeled alternative in {@link aslParser#registerDefinition}.
 * @param ctx the parse tree
 */
fn exit_RegDefArray(&mut self, _ctx: &RegDefArrayContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code RegDefBasic}
 * labeled alternative in {@link aslParser#registerDefinition}.
 * @param ctx the parse tree
 */
fn enter_RegDefBasic(&mut self, _ctx: &RegDefBasicContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code RegDefBasic}
 * labeled alternative in {@link aslParser#registerDefinition}.
 * @param ctx the parse tree
 */
fn exit_RegDefBasic(&mut self, _ctx: &RegDefBasicContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#register}.
 * @param ctx the parse tree
 */
fn enter_register(&mut self, _ctx: &RegisterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#register}.
 * @param ctx the parse tree
 */
fn exit_register(&mut self, _ctx: &RegisterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#arrayRegister}.
 * @param ctx the parse tree
 */
fn enter_arrayRegister(&mut self, _ctx: &ArrayRegisterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#arrayRegister}.
 * @param ctx the parse tree
 */
fn exit_arrayRegister(&mut self, _ctx: &ArrayRegisterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#registerField}.
 * @param ctx the parse tree
 */
fn enter_registerField(&mut self, _ctx: &RegisterFieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#registerField}.
 * @param ctx the parse tree
 */
fn exit_registerField(&mut self, _ctx: &RegisterFieldContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#registerFieldCommaList}.
 * @param ctx the parse tree
 */
fn enter_registerFieldCommaList(&mut self, _ctx: &RegisterFieldCommaListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#registerFieldCommaList}.
 * @param ctx the parse tree
 */
fn exit_registerFieldCommaList(&mut self, _ctx: &RegisterFieldCommaListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#instructions}.
 * @param ctx the parse tree
 */
fn enter_instructions(&mut self, _ctx: &InstructionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#instructions}.
 * @param ctx the parse tree
 */
fn exit_instructions(&mut self, _ctx: &InstructionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#instruction}.
 * @param ctx the parse tree
 */
fn enter_instruction(&mut self, _ctx: &InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#instruction}.
 * @param ctx the parse tree
 */
fn exit_instruction(&mut self, _ctx: &InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#encoding}.
 * @param ctx the parse tree
 */
fn enter_encoding(&mut self, _ctx: &EncodingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#encoding}.
 * @param ctx the parse tree
 */
fn exit_encoding(&mut self, _ctx: &EncodingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#instructionField}.
 * @param ctx the parse tree
 */
fn enter_instructionField(&mut self, _ctx: &InstructionFieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#instructionField}.
 * @param ctx the parse tree
 */
fn exit_instructionField(&mut self, _ctx: &InstructionFieldContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#instrUnpredictableUnless}.
 * @param ctx the parse tree
 */
fn enter_instrUnpredictableUnless(&mut self, _ctx: &InstrUnpredictableUnlessContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#instrUnpredictableUnless}.
 * @param ctx the parse tree
 */
fn exit_instrUnpredictableUnless(&mut self, _ctx: &InstrUnpredictableUnlessContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#definitions}.
 * @param ctx the parse tree
 */
fn enter_definitions(&mut self, _ctx: &DefinitionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#definitions}.
 * @param ctx the parse tree
 */
fn exit_definitions(&mut self, _ctx: &DefinitionsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code DefTypeBuiltin}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn enter_DefTypeBuiltin(&mut self, _ctx: &DefTypeBuiltinContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code DefTypeBuiltin}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn exit_DefTypeBuiltin(&mut self, _ctx: &DefTypeBuiltinContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code DefTypeAbstract}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn enter_DefTypeAbstract(&mut self, _ctx: &DefTypeAbstractContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code DefTypeAbstract}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn exit_DefTypeAbstract(&mut self, _ctx: &DefTypeAbstractContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code DefTypeAlias}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn enter_DefTypeAlias(&mut self, _ctx: &DefTypeAliasContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code DefTypeAlias}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn exit_DefTypeAlias(&mut self, _ctx: &DefTypeAliasContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code DefTypeStruct}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn enter_DefTypeStruct(&mut self, _ctx: &DefTypeStructContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code DefTypeStruct}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn exit_DefTypeStruct(&mut self, _ctx: &DefTypeStructContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code DefTypeEnum}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn enter_DefTypeEnum(&mut self, _ctx: &DefTypeEnumContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code DefTypeEnum}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn exit_DefTypeEnum(&mut self, _ctx: &DefTypeEnumContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code DefVariable}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn enter_DefVariable(&mut self, _ctx: &DefVariableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code DefVariable}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn exit_DefVariable(&mut self, _ctx: &DefVariableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code DefConstant}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn enter_DefConstant(&mut self, _ctx: &DefConstantContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code DefConstant}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn exit_DefConstant(&mut self, _ctx: &DefConstantContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code DefArray}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn enter_DefArray(&mut self, _ctx: &DefArrayContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code DefArray}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn exit_DefArray(&mut self, _ctx: &DefArrayContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code DefCallable}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn enter_DefCallable(&mut self, _ctx: &DefCallableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code DefCallable}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn exit_DefCallable(&mut self, _ctx: &DefCallableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code DefGetter}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn enter_DefGetter(&mut self, _ctx: &DefGetterContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code DefGetter}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn exit_DefGetter(&mut self, _ctx: &DefGetterContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code DefSetter}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn enter_DefSetter(&mut self, _ctx: &DefSetterContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code DefSetter}
 * labeled alternative in {@link aslParser#definition}.
 * @param ctx the parse tree
 */
fn exit_DefSetter(&mut self, _ctx: &DefSetterContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SetterRefArg}
 * labeled alternative in {@link aslParser#setterArg}.
 * @param ctx the parse tree
 */
fn enter_SetterRefArg(&mut self, _ctx: &SetterRefArgContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SetterRefArg}
 * labeled alternative in {@link aslParser#setterArg}.
 * @param ctx the parse tree
 */
fn exit_SetterRefArg(&mut self, _ctx: &SetterRefArgContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SetterValArg}
 * labeled alternative in {@link aslParser#setterArg}.
 * @param ctx the parse tree
 */
fn enter_SetterValArg(&mut self, _ctx: &SetterValArgContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SetterValArg}
 * labeled alternative in {@link aslParser#setterArg}.
 * @param ctx the parse tree
 */
fn exit_SetterValArg(&mut self, _ctx: &SetterValArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#setterArgCommaList}.
 * @param ctx the parse tree
 */
fn enter_setterArgCommaList(&mut self, _ctx: &SetterArgCommaListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#setterArgCommaList}.
 * @param ctx the parse tree
 */
fn exit_setterArgCommaList(&mut self, _ctx: &SetterArgCommaListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#returnType}.
 * @param ctx the parse tree
 */
fn enter_returnType(&mut self, _ctx: &ReturnTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#returnType}.
 * @param ctx the parse tree
 */
fn exit_returnType(&mut self, _ctx: &ReturnTypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code TypeRef}
 * labeled alternative in {@link aslParser#typeSpec}.
 * @param ctx the parse tree
 */
fn enter_TypeRef(&mut self, _ctx: &TypeRefContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code TypeRef}
 * labeled alternative in {@link aslParser#typeSpec}.
 * @param ctx the parse tree
 */
fn exit_TypeRef(&mut self, _ctx: &TypeRefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code TypeIndexed}
 * labeled alternative in {@link aslParser#typeSpec}.
 * @param ctx the parse tree
 */
fn enter_TypeIndexed(&mut self, _ctx: &TypeIndexedContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code TypeIndexed}
 * labeled alternative in {@link aslParser#typeSpec}.
 * @param ctx the parse tree
 */
fn exit_TypeIndexed(&mut self, _ctx: &TypeIndexedContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code TypeOf}
 * labeled alternative in {@link aslParser#typeSpec}.
 * @param ctx the parse tree
 */
fn enter_TypeOf(&mut self, _ctx: &TypeOfContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code TypeOf}
 * labeled alternative in {@link aslParser#typeSpec}.
 * @param ctx the parse tree
 */
fn exit_TypeOf(&mut self, _ctx: &TypeOfContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code TypeRegister}
 * labeled alternative in {@link aslParser#typeSpec}.
 * @param ctx the parse tree
 */
fn enter_TypeRegister(&mut self, _ctx: &TypeRegisterContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code TypeRegister}
 * labeled alternative in {@link aslParser#typeSpec}.
 * @param ctx the parse tree
 */
fn exit_TypeRegister(&mut self, _ctx: &TypeRegisterContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code TypeArray}
 * labeled alternative in {@link aslParser#typeSpec}.
 * @param ctx the parse tree
 */
fn enter_TypeArray(&mut self, _ctx: &TypeArrayContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code TypeArray}
 * labeled alternative in {@link aslParser#typeSpec}.
 * @param ctx the parse tree
 */
fn exit_TypeArray(&mut self, _ctx: &TypeArrayContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code IxTypeRef}
 * labeled alternative in {@link aslParser#ixType}.
 * @param ctx the parse tree
 */
fn enter_IxTypeRef(&mut self, _ctx: &IxTypeRefContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code IxTypeRef}
 * labeled alternative in {@link aslParser#ixType}.
 * @param ctx the parse tree
 */
fn exit_IxTypeRef(&mut self, _ctx: &IxTypeRefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code IxTypeRange}
 * labeled alternative in {@link aslParser#ixType}.
 * @param ctx the parse tree
 */
fn enter_IxTypeRange(&mut self, _ctx: &IxTypeRangeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code IxTypeRange}
 * labeled alternative in {@link aslParser#ixType}.
 * @param ctx the parse tree
 */
fn exit_IxTypeRange(&mut self, _ctx: &IxTypeRangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#regField}.
 * @param ctx the parse tree
 */
fn enter_regField(&mut self, _ctx: &RegFieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#regField}.
 * @param ctx the parse tree
 */
fn exit_regField(&mut self, _ctx: &RegFieldContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#indentedBlock}.
 * @param ctx the parse tree
 */
fn enter_indentedBlock(&mut self, _ctx: &IndentedBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#indentedBlock}.
 * @param ctx the parse tree
 */
fn exit_indentedBlock(&mut self, _ctx: &IndentedBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#blockOrEmbed0}.
 * @param ctx the parse tree
 */
fn enter_blockOrEmbed0(&mut self, _ctx: &BlockOrEmbed0Context<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#blockOrEmbed0}.
 * @param ctx the parse tree
 */
fn exit_blockOrEmbed0(&mut self, _ctx: &BlockOrEmbed0Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code BlockInline}
 * labeled alternative in {@link aslParser#blockOrEmbed1}.
 * @param ctx the parse tree
 */
fn enter_BlockInline(&mut self, _ctx: &BlockInlineContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code BlockInline}
 * labeled alternative in {@link aslParser#blockOrEmbed1}.
 * @param ctx the parse tree
 */
fn exit_BlockInline(&mut self, _ctx: &BlockInlineContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code BlockIndent}
 * labeled alternative in {@link aslParser#blockOrEmbed1}.
 * @param ctx the parse tree
 */
fn enter_BlockIndent(&mut self, _ctx: &BlockIndentContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code BlockIndent}
 * labeled alternative in {@link aslParser#blockOrEmbed1}.
 * @param ctx the parse tree
 */
fn exit_BlockIndent(&mut self, _ctx: &BlockIndentContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtsInline}
 * labeled alternative in {@link aslParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_StmtsInline(&mut self, _ctx: &StmtsInlineContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtsInline}
 * labeled alternative in {@link aslParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_StmtsInline(&mut self, _ctx: &StmtsInlineContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtIf}
 * labeled alternative in {@link aslParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_StmtIf(&mut self, _ctx: &StmtIfContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtIf}
 * labeled alternative in {@link aslParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_StmtIf(&mut self, _ctx: &StmtIfContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtCase}
 * labeled alternative in {@link aslParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_StmtCase(&mut self, _ctx: &StmtCaseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtCase}
 * labeled alternative in {@link aslParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_StmtCase(&mut self, _ctx: &StmtCaseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtFor}
 * labeled alternative in {@link aslParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_StmtFor(&mut self, _ctx: &StmtForContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtFor}
 * labeled alternative in {@link aslParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_StmtFor(&mut self, _ctx: &StmtForContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtWhile}
 * labeled alternative in {@link aslParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_StmtWhile(&mut self, _ctx: &StmtWhileContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtWhile}
 * labeled alternative in {@link aslParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_StmtWhile(&mut self, _ctx: &StmtWhileContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtTry}
 * labeled alternative in {@link aslParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_StmtTry(&mut self, _ctx: &StmtTryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtTry}
 * labeled alternative in {@link aslParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_StmtTry(&mut self, _ctx: &StmtTryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtVarsDecl}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn enter_StmtVarsDecl(&mut self, _ctx: &StmtVarsDeclContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtVarsDecl}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn exit_StmtVarsDecl(&mut self, _ctx: &StmtVarsDeclContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtVarDeclInit}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn enter_StmtVarDeclInit(&mut self, _ctx: &StmtVarDeclInitContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtVarDeclInit}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn exit_StmtVarDeclInit(&mut self, _ctx: &StmtVarDeclInitContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtConstDecl}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn enter_StmtConstDecl(&mut self, _ctx: &StmtConstDeclContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtConstDecl}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn exit_StmtConstDecl(&mut self, _ctx: &StmtConstDeclContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtAssign}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn enter_StmtAssign(&mut self, _ctx: &StmtAssignContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtAssign}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn exit_StmtAssign(&mut self, _ctx: &StmtAssignContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtCall}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn enter_StmtCall(&mut self, _ctx: &StmtCallContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtCall}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn exit_StmtCall(&mut self, _ctx: &StmtCallContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtReturn}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn enter_StmtReturn(&mut self, _ctx: &StmtReturnContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtReturn}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn exit_StmtReturn(&mut self, _ctx: &StmtReturnContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtAssert}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn enter_StmtAssert(&mut self, _ctx: &StmtAssertContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtAssert}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn exit_StmtAssert(&mut self, _ctx: &StmtAssertContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtUnpredictable}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn enter_StmtUnpredictable(&mut self, _ctx: &StmtUnpredictableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtUnpredictable}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn exit_StmtUnpredictable(&mut self, _ctx: &StmtUnpredictableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtImpDef}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn enter_StmtImpDef(&mut self, _ctx: &StmtImpDefContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtImpDef}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn exit_StmtImpDef(&mut self, _ctx: &StmtImpDefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtRepeat}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn enter_StmtRepeat(&mut self, _ctx: &StmtRepeatContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtRepeat}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn exit_StmtRepeat(&mut self, _ctx: &StmtRepeatContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtThrow}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn enter_StmtThrow(&mut self, _ctx: &StmtThrowContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtThrow}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn exit_StmtThrow(&mut self, _ctx: &StmtThrowContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtUndefined}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn enter_StmtUndefined(&mut self, _ctx: &StmtUndefinedContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtUndefined}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn exit_StmtUndefined(&mut self, _ctx: &StmtUndefinedContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtSee}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn enter_StmtSee(&mut self, _ctx: &StmtSeeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtSee}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn exit_StmtSee(&mut self, _ctx: &StmtSeeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StmtDefEnum}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn enter_StmtDefEnum(&mut self, _ctx: &StmtDefEnumContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StmtDefEnum}
 * labeled alternative in {@link aslParser#inlineStmt}.
 * @param ctx the parse tree
 */
fn exit_StmtDefEnum(&mut self, _ctx: &StmtDefEnumContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#stmtElsIf}.
 * @param ctx the parse tree
 */
fn enter_stmtElsIf(&mut self, _ctx: &StmtElsIfContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#stmtElsIf}.
 * @param ctx the parse tree
 */
fn exit_stmtElsIf(&mut self, _ctx: &StmtElsIfContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code CatchAltWhen}
 * labeled alternative in {@link aslParser#catchAlt}.
 * @param ctx the parse tree
 */
fn enter_CatchAltWhen(&mut self, _ctx: &CatchAltWhenContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code CatchAltWhen}
 * labeled alternative in {@link aslParser#catchAlt}.
 * @param ctx the parse tree
 */
fn exit_CatchAltWhen(&mut self, _ctx: &CatchAltWhenContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code CatchAltOtherwise}
 * labeled alternative in {@link aslParser#catchAlt}.
 * @param ctx the parse tree
 */
fn enter_CatchAltOtherwise(&mut self, _ctx: &CatchAltOtherwiseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code CatchAltOtherwise}
 * labeled alternative in {@link aslParser#catchAlt}.
 * @param ctx the parse tree
 */
fn exit_CatchAltOtherwise(&mut self, _ctx: &CatchAltOtherwiseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code CaseAltWhen}
 * labeled alternative in {@link aslParser#caseAlt}.
 * @param ctx the parse tree
 */
fn enter_CaseAltWhen(&mut self, _ctx: &CaseAltWhenContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code CaseAltWhen}
 * labeled alternative in {@link aslParser#caseAlt}.
 * @param ctx the parse tree
 */
fn exit_CaseAltWhen(&mut self, _ctx: &CaseAltWhenContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code CaseAltOtherwise}
 * labeled alternative in {@link aslParser#caseAlt}.
 * @param ctx the parse tree
 */
fn enter_CaseAltOtherwise(&mut self, _ctx: &CaseAltOtherwiseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code CaseAltOtherwise}
 * labeled alternative in {@link aslParser#caseAlt}.
 * @param ctx the parse tree
 */
fn exit_CaseAltOtherwise(&mut self, _ctx: &CaseAltOtherwiseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code CasePatternNat}
 * labeled alternative in {@link aslParser#casePattern}.
 * @param ctx the parse tree
 */
fn enter_CasePatternNat(&mut self, _ctx: &CasePatternNatContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code CasePatternNat}
 * labeled alternative in {@link aslParser#casePattern}.
 * @param ctx the parse tree
 */
fn exit_CasePatternNat(&mut self, _ctx: &CasePatternNatContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code CasePatternHex}
 * labeled alternative in {@link aslParser#casePattern}.
 * @param ctx the parse tree
 */
fn enter_CasePatternHex(&mut self, _ctx: &CasePatternHexContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code CasePatternHex}
 * labeled alternative in {@link aslParser#casePattern}.
 * @param ctx the parse tree
 */
fn exit_CasePatternHex(&mut self, _ctx: &CasePatternHexContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code CasePatternBin}
 * labeled alternative in {@link aslParser#casePattern}.
 * @param ctx the parse tree
 */
fn enter_CasePatternBin(&mut self, _ctx: &CasePatternBinContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code CasePatternBin}
 * labeled alternative in {@link aslParser#casePattern}.
 * @param ctx the parse tree
 */
fn exit_CasePatternBin(&mut self, _ctx: &CasePatternBinContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code CasePatternMask}
 * labeled alternative in {@link aslParser#casePattern}.
 * @param ctx the parse tree
 */
fn enter_CasePatternMask(&mut self, _ctx: &CasePatternMaskContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code CasePatternMask}
 * labeled alternative in {@link aslParser#casePattern}.
 * @param ctx the parse tree
 */
fn exit_CasePatternMask(&mut self, _ctx: &CasePatternMaskContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code CasePatternBind}
 * labeled alternative in {@link aslParser#casePattern}.
 * @param ctx the parse tree
 */
fn enter_CasePatternBind(&mut self, _ctx: &CasePatternBindContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code CasePatternBind}
 * labeled alternative in {@link aslParser#casePattern}.
 * @param ctx the parse tree
 */
fn exit_CasePatternBind(&mut self, _ctx: &CasePatternBindContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code CasePatternIgnore}
 * labeled alternative in {@link aslParser#casePattern}.
 * @param ctx the parse tree
 */
fn enter_CasePatternIgnore(&mut self, _ctx: &CasePatternIgnoreContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code CasePatternIgnore}
 * labeled alternative in {@link aslParser#casePattern}.
 * @param ctx the parse tree
 */
fn exit_CasePatternIgnore(&mut self, _ctx: &CasePatternIgnoreContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code CasePatternTuple}
 * labeled alternative in {@link aslParser#casePattern}.
 * @param ctx the parse tree
 */
fn enter_CasePatternTuple(&mut self, _ctx: &CasePatternTupleContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code CasePatternTuple}
 * labeled alternative in {@link aslParser#casePattern}.
 * @param ctx the parse tree
 */
fn exit_CasePatternTuple(&mut self, _ctx: &CasePatternTupleContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LValSliceOf}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn enter_LValSliceOf(&mut self, _ctx: &LValSliceOfContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LValSliceOf}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn exit_LValSliceOf(&mut self, _ctx: &LValSliceOfContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LValArray}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn enter_LValArray(&mut self, _ctx: &LValArrayContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LValArray}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn exit_LValArray(&mut self, _ctx: &LValArrayContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LValArrayIndex}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn enter_LValArrayIndex(&mut self, _ctx: &LValArrayIndexContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LValArrayIndex}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn exit_LValArrayIndex(&mut self, _ctx: &LValArrayIndexContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LValMemberBits}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn enter_LValMemberBits(&mut self, _ctx: &LValMemberBitsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LValMemberBits}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn exit_LValMemberBits(&mut self, _ctx: &LValMemberBitsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LValTuple}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn enter_LValTuple(&mut self, _ctx: &LValTupleContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LValTuple}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn exit_LValTuple(&mut self, _ctx: &LValTupleContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LValMemberArray}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn enter_LValMemberArray(&mut self, _ctx: &LValMemberArrayContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LValMemberArray}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn exit_LValMemberArray(&mut self, _ctx: &LValMemberArrayContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LValIgnore}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn enter_LValIgnore(&mut self, _ctx: &LValIgnoreContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LValIgnore}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn exit_LValIgnore(&mut self, _ctx: &LValIgnoreContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LValMember}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn enter_LValMember(&mut self, _ctx: &LValMemberContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LValMember}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn exit_LValMember(&mut self, _ctx: &LValMemberContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LValVarRef}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn enter_LValVarRef(&mut self, _ctx: &LValVarRefContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LValVarRef}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn exit_LValVarRef(&mut self, _ctx: &LValVarRefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LValSlice}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn enter_LValSlice(&mut self, _ctx: &LValSliceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LValSlice}
 * labeled alternative in {@link aslParser#lValExpr}.
 * @param ctx the parse tree
 */
fn exit_LValSlice(&mut self, _ctx: &LValSliceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprLitString}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprLitString(&mut self, _ctx: &ExprLitStringContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprLitString}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprLitString(&mut self, _ctx: &ExprLitStringContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprImpDef}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprImpDef(&mut self, _ctx: &ExprImpDefContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprImpDef}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprImpDef(&mut self, _ctx: &ExprImpDefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprParen}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprParen(&mut self, _ctx: &ExprParenContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprParen}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprParen(&mut self, _ctx: &ExprParenContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprTuple}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprTuple(&mut self, _ctx: &ExprTupleContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprTuple}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprTuple(&mut self, _ctx: &ExprTupleContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprIndex}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprIndex(&mut self, _ctx: &ExprIndexContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprIndex}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprIndex(&mut self, _ctx: &ExprIndexContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprUnOp}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprUnOp(&mut self, _ctx: &ExprUnOpContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprUnOp}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprUnOp(&mut self, _ctx: &ExprUnOpContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprBinOp}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprBinOp(&mut self, _ctx: &ExprBinOpContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprBinOp}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprBinOp(&mut self, _ctx: &ExprBinOpContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprLitNat}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprLitNat(&mut self, _ctx: &ExprLitNatContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprLitNat}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprLitNat(&mut self, _ctx: &ExprLitNatContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprMembers}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprMembers(&mut self, _ctx: &ExprMembersContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprMembers}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprMembers(&mut self, _ctx: &ExprMembersContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprInMask}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprInMask(&mut self, _ctx: &ExprInMaskContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprInMask}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprInMask(&mut self, _ctx: &ExprInMaskContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprLitHex}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprLitHex(&mut self, _ctx: &ExprLitHexContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprLitHex}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprLitHex(&mut self, _ctx: &ExprLitHexContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprMemberBits}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprMemberBits(&mut self, _ctx: &ExprMemberBitsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprMemberBits}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprMemberBits(&mut self, _ctx: &ExprMemberBitsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprCall}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprCall(&mut self, _ctx: &ExprCallContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprCall}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprCall(&mut self, _ctx: &ExprCallContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprInSet}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprInSet(&mut self, _ctx: &ExprInSetContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprInSet}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprInSet(&mut self, _ctx: &ExprInSetContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprLitBin}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprLitBin(&mut self, _ctx: &ExprLitBinContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprLitBin}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprLitBin(&mut self, _ctx: &ExprLitBinContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprUnknown}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprUnknown(&mut self, _ctx: &ExprUnknownContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprUnknown}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprUnknown(&mut self, _ctx: &ExprUnknownContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprLitReal}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprLitReal(&mut self, _ctx: &ExprLitRealContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprLitReal}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprLitReal(&mut self, _ctx: &ExprLitRealContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprVarRef}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprVarRef(&mut self, _ctx: &ExprVarRefContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprVarRef}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprVarRef(&mut self, _ctx: &ExprVarRefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprSlice}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprSlice(&mut self, _ctx: &ExprSliceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprSlice}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprSlice(&mut self, _ctx: &ExprSliceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprLitMask}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprLitMask(&mut self, _ctx: &ExprLitMaskContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprLitMask}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprLitMask(&mut self, _ctx: &ExprLitMaskContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprIf}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprIf(&mut self, _ctx: &ExprIfContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprIf}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprIf(&mut self, _ctx: &ExprIfContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprMember}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprMember(&mut self, _ctx: &ExprMemberContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprMember}
 * labeled alternative in {@link aslParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprMember(&mut self, _ctx: &ExprMemberContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SliceExprMember}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn enter_SliceExprMember(&mut self, _ctx: &SliceExprMemberContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SliceExprMember}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn exit_SliceExprMember(&mut self, _ctx: &SliceExprMemberContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SliceExprLitNat}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn enter_SliceExprLitNat(&mut self, _ctx: &SliceExprLitNatContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SliceExprLitNat}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn exit_SliceExprLitNat(&mut self, _ctx: &SliceExprLitNatContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SliceExprVarRef}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn enter_SliceExprVarRef(&mut self, _ctx: &SliceExprVarRefContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SliceExprVarRef}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn exit_SliceExprVarRef(&mut self, _ctx: &SliceExprVarRefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SliceExprCall}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn enter_SliceExprCall(&mut self, _ctx: &SliceExprCallContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SliceExprCall}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn exit_SliceExprCall(&mut self, _ctx: &SliceExprCallContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SliceExprUnOp}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn enter_SliceExprUnOp(&mut self, _ctx: &SliceExprUnOpContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SliceExprUnOp}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn exit_SliceExprUnOp(&mut self, _ctx: &SliceExprUnOpContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SliceExprBinOp}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn enter_SliceExprBinOp(&mut self, _ctx: &SliceExprBinOpContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SliceExprBinOp}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn exit_SliceExprBinOp(&mut self, _ctx: &SliceExprBinOpContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SliceExprLitHex}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn enter_SliceExprLitHex(&mut self, _ctx: &SliceExprLitHexContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SliceExprLitHex}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn exit_SliceExprLitHex(&mut self, _ctx: &SliceExprLitHexContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SliceExprParen}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn enter_SliceExprParen(&mut self, _ctx: &SliceExprParenContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SliceExprParen}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn exit_SliceExprParen(&mut self, _ctx: &SliceExprParenContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SliceExprIf}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn enter_SliceExprIf(&mut self, _ctx: &SliceExprIfContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SliceExprIf}
 * labeled alternative in {@link aslParser#sliceExpr}.
 * @param ctx the parse tree
 */
fn exit_SliceExprIf(&mut self, _ctx: &SliceExprIfContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SliceRange}
 * labeled alternative in {@link aslParser#slice}.
 * @param ctx the parse tree
 */
fn enter_SliceRange(&mut self, _ctx: &SliceRangeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SliceRange}
 * labeled alternative in {@link aslParser#slice}.
 * @param ctx the parse tree
 */
fn exit_SliceRange(&mut self, _ctx: &SliceRangeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SliceOffset}
 * labeled alternative in {@link aslParser#slice}.
 * @param ctx the parse tree
 */
fn enter_SliceOffset(&mut self, _ctx: &SliceOffsetContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SliceOffset}
 * labeled alternative in {@link aslParser#slice}.
 * @param ctx the parse tree
 */
fn exit_SliceOffset(&mut self, _ctx: &SliceOffsetContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SliceSingle}
 * labeled alternative in {@link aslParser#slice}.
 * @param ctx the parse tree
 */
fn enter_SliceSingle(&mut self, _ctx: &SliceSingleContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SliceSingle}
 * labeled alternative in {@link aslParser#slice}.
 * @param ctx the parse tree
 */
fn exit_SliceSingle(&mut self, _ctx: &SliceSingleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#exprElsIf}.
 * @param ctx the parse tree
 */
fn enter_exprElsIf(&mut self, _ctx: &ExprElsIfContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#exprElsIf}.
 * @param ctx the parse tree
 */
fn exit_exprElsIf(&mut self, _ctx: &ExprElsIfContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SetElementRange}
 * labeled alternative in {@link aslParser#setElement}.
 * @param ctx the parse tree
 */
fn enter_SetElementRange(&mut self, _ctx: &SetElementRangeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SetElementRange}
 * labeled alternative in {@link aslParser#setElement}.
 * @param ctx the parse tree
 */
fn exit_SetElementRange(&mut self, _ctx: &SetElementRangeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SetElementSingle}
 * labeled alternative in {@link aslParser#setElement}.
 * @param ctx the parse tree
 */
fn enter_SetElementSingle(&mut self, _ctx: &SetElementSingleContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SetElementSingle}
 * labeled alternative in {@link aslParser#setElement}.
 * @param ctx the parse tree
 */
fn exit_SetElementSingle(&mut self, _ctx: &SetElementSingleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#set}.
 * @param ctx the parse tree
 */
fn enter_set(&mut self, _ctx: &SetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#set}.
 * @param ctx the parse tree
 */
fn exit_set(&mut self, _ctx: &SetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#sliceCommaList0}.
 * @param ctx the parse tree
 */
fn enter_sliceCommaList0(&mut self, _ctx: &SliceCommaList0Context<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#sliceCommaList0}.
 * @param ctx the parse tree
 */
fn exit_sliceCommaList0(&mut self, _ctx: &SliceCommaList0Context<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#sliceCommaList1}.
 * @param ctx the parse tree
 */
fn enter_sliceCommaList1(&mut self, _ctx: &SliceCommaList1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#sliceCommaList1}.
 * @param ctx the parse tree
 */
fn exit_sliceCommaList1(&mut self, _ctx: &SliceCommaList1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#exprCommaList1}.
 * @param ctx the parse tree
 */
fn enter_exprCommaList1(&mut self, _ctx: &ExprCommaList1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#exprCommaList1}.
 * @param ctx the parse tree
 */
fn exit_exprCommaList1(&mut self, _ctx: &ExprCommaList1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#exprCommaList0}.
 * @param ctx the parse tree
 */
fn enter_exprCommaList0(&mut self, _ctx: &ExprCommaList0Context<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#exprCommaList0}.
 * @param ctx the parse tree
 */
fn exit_exprCommaList0(&mut self, _ctx: &ExprCommaList0Context<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#symDecl}.
 * @param ctx the parse tree
 */
fn enter_symDecl(&mut self, _ctx: &SymDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#symDecl}.
 * @param ctx the parse tree
 */
fn exit_symDecl(&mut self, _ctx: &SymDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#identifierCommaList0}.
 * @param ctx the parse tree
 */
fn enter_identifierCommaList0(&mut self, _ctx: &IdentifierCommaList0Context<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#identifierCommaList0}.
 * @param ctx the parse tree
 */
fn exit_identifierCommaList0(&mut self, _ctx: &IdentifierCommaList0Context<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#identifierCommaList1}.
 * @param ctx the parse tree
 */
fn enter_identifierCommaList1(&mut self, _ctx: &IdentifierCommaList1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#identifierCommaList1}.
 * @param ctx the parse tree
 */
fn exit_identifierCommaList1(&mut self, _ctx: &IdentifierCommaList1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#symDeclCommaList}.
 * @param ctx the parse tree
 */
fn enter_symDeclCommaList(&mut self, _ctx: &SymDeclCommaListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#symDeclCommaList}.
 * @param ctx the parse tree
 */
fn exit_symDeclCommaList(&mut self, _ctx: &SymDeclCommaListContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code QualIdUnqualified}
 * labeled alternative in {@link aslParser#qualId}.
 * @param ctx the parse tree
 */
fn enter_QualIdUnqualified(&mut self, _ctx: &QualIdUnqualifiedContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code QualIdUnqualified}
 * labeled alternative in {@link aslParser#qualId}.
 * @param ctx the parse tree
 */
fn exit_QualIdUnqualified(&mut self, _ctx: &QualIdUnqualifiedContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code QualIdAArch32}
 * labeled alternative in {@link aslParser#qualId}.
 * @param ctx the parse tree
 */
fn enter_QualIdAArch32(&mut self, _ctx: &QualIdAArch32Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code QualIdAArch32}
 * labeled alternative in {@link aslParser#qualId}.
 * @param ctx the parse tree
 */
fn exit_QualIdAArch32(&mut self, _ctx: &QualIdAArch32Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code QualIdAArch64}
 * labeled alternative in {@link aslParser#qualId}.
 * @param ctx the parse tree
 */
fn enter_QualIdAArch64(&mut self, _ctx: &QualIdAArch64Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code QualIdAArch64}
 * labeled alternative in {@link aslParser#qualId}.
 * @param ctx the parse tree
 */
fn exit_QualIdAArch64(&mut self, _ctx: &QualIdAArch64Context<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#idWithDots}.
 * @param ctx the parse tree
 */
fn enter_idWithDots(&mut self, _ctx: &IdWithDotsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#idWithDots}.
 * @param ctx the parse tree
 */
fn exit_idWithDots(&mut self, _ctx: &IdWithDotsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link aslParser#id}.
 * @param ctx the parse tree
 */
fn enter_id(&mut self, _ctx: &IdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link aslParser#id}.
 * @param ctx the parse tree
 */
fn exit_id(&mut self, _ctx: &IdContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : aslListener<'input> }


