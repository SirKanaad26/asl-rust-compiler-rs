#![allow(nonstandard_style)]
// Generated from grammar/asl.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::aslparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link aslParser}.
 */
pub trait aslVisitor<'input>: ParseTreeVisitor<'input,aslParserContextType>{
	/**
	 * Visit a parse tree produced by {@link aslParser#registers}.
	 * @param ctx the parse tree
	 */
	fn visit_registers(&mut self, ctx: &RegistersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code RegDefArray}
	 * labeled alternative in {@link aslParser#registerDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_RegDefArray(&mut self, ctx: &RegDefArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code RegDefBasic}
	 * labeled alternative in {@link aslParser#registerDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_RegDefBasic(&mut self, ctx: &RegDefBasicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#register}.
	 * @param ctx the parse tree
	 */
	fn visit_register(&mut self, ctx: &RegisterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#arrayRegister}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayRegister(&mut self, ctx: &ArrayRegisterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#registerField}.
	 * @param ctx the parse tree
	 */
	fn visit_registerField(&mut self, ctx: &RegisterFieldContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#registerFieldCommaList}.
	 * @param ctx the parse tree
	 */
	fn visit_registerFieldCommaList(&mut self, ctx: &RegisterFieldCommaListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#instructions}.
	 * @param ctx the parse tree
	 */
	fn visit_instructions(&mut self, ctx: &InstructionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#instruction}.
	 * @param ctx the parse tree
	 */
	fn visit_instruction(&mut self, ctx: &InstructionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#encoding}.
	 * @param ctx the parse tree
	 */
	fn visit_encoding(&mut self, ctx: &EncodingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#instructionField}.
	 * @param ctx the parse tree
	 */
	fn visit_instructionField(&mut self, ctx: &InstructionFieldContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#instrUnpredictableUnless}.
	 * @param ctx the parse tree
	 */
	fn visit_instrUnpredictableUnless(&mut self, ctx: &InstrUnpredictableUnlessContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#definitions}.
	 * @param ctx the parse tree
	 */
	fn visit_definitions(&mut self, ctx: &DefinitionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code DefTypeBuiltin}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
	fn visit_DefTypeBuiltin(&mut self, ctx: &DefTypeBuiltinContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code DefTypeAbstract}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
	fn visit_DefTypeAbstract(&mut self, ctx: &DefTypeAbstractContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code DefTypeAlias}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
	fn visit_DefTypeAlias(&mut self, ctx: &DefTypeAliasContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code DefTypeStruct}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
	fn visit_DefTypeStruct(&mut self, ctx: &DefTypeStructContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code DefTypeEnum}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
	fn visit_DefTypeEnum(&mut self, ctx: &DefTypeEnumContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code DefVariable}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
	fn visit_DefVariable(&mut self, ctx: &DefVariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code DefConstant}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
	fn visit_DefConstant(&mut self, ctx: &DefConstantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code DefArray}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
	fn visit_DefArray(&mut self, ctx: &DefArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code DefCallable}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
	fn visit_DefCallable(&mut self, ctx: &DefCallableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code DefGetter}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
	fn visit_DefGetter(&mut self, ctx: &DefGetterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code DefSetter}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
	fn visit_DefSetter(&mut self, ctx: &DefSetterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SetterRefArg}
	 * labeled alternative in {@link aslParser#setterArg}.
	 * @param ctx the parse tree
	 */
	fn visit_SetterRefArg(&mut self, ctx: &SetterRefArgContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SetterValArg}
	 * labeled alternative in {@link aslParser#setterArg}.
	 * @param ctx the parse tree
	 */
	fn visit_SetterValArg(&mut self, ctx: &SetterValArgContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#setterArgCommaList}.
	 * @param ctx the parse tree
	 */
	fn visit_setterArgCommaList(&mut self, ctx: &SetterArgCommaListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#returnType}.
	 * @param ctx the parse tree
	 */
	fn visit_returnType(&mut self, ctx: &ReturnTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code TypeRef}
	 * labeled alternative in {@link aslParser#typeSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_TypeRef(&mut self, ctx: &TypeRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code TypeIndexed}
	 * labeled alternative in {@link aslParser#typeSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_TypeIndexed(&mut self, ctx: &TypeIndexedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code TypeOf}
	 * labeled alternative in {@link aslParser#typeSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_TypeOf(&mut self, ctx: &TypeOfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code TypeRegister}
	 * labeled alternative in {@link aslParser#typeSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_TypeRegister(&mut self, ctx: &TypeRegisterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code TypeArray}
	 * labeled alternative in {@link aslParser#typeSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_TypeArray(&mut self, ctx: &TypeArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code IxTypeRef}
	 * labeled alternative in {@link aslParser#ixType}.
	 * @param ctx the parse tree
	 */
	fn visit_IxTypeRef(&mut self, ctx: &IxTypeRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code IxTypeRange}
	 * labeled alternative in {@link aslParser#ixType}.
	 * @param ctx the parse tree
	 */
	fn visit_IxTypeRange(&mut self, ctx: &IxTypeRangeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#regField}.
	 * @param ctx the parse tree
	 */
	fn visit_regField(&mut self, ctx: &RegFieldContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#indentedBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_indentedBlock(&mut self, ctx: &IndentedBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#blockOrEmbed0}.
	 * @param ctx the parse tree
	 */
	fn visit_blockOrEmbed0(&mut self, ctx: &BlockOrEmbed0Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code BlockInline}
	 * labeled alternative in {@link aslParser#blockOrEmbed1}.
	 * @param ctx the parse tree
	 */
	fn visit_BlockInline(&mut self, ctx: &BlockInlineContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code BlockIndent}
	 * labeled alternative in {@link aslParser#blockOrEmbed1}.
	 * @param ctx the parse tree
	 */
	fn visit_BlockIndent(&mut self, ctx: &BlockIndentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtsInline}
	 * labeled alternative in {@link aslParser#stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtsInline(&mut self, ctx: &StmtsInlineContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtIf}
	 * labeled alternative in {@link aslParser#stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtIf(&mut self, ctx: &StmtIfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtCase}
	 * labeled alternative in {@link aslParser#stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtCase(&mut self, ctx: &StmtCaseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtFor}
	 * labeled alternative in {@link aslParser#stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtFor(&mut self, ctx: &StmtForContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtWhile}
	 * labeled alternative in {@link aslParser#stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtWhile(&mut self, ctx: &StmtWhileContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtTry}
	 * labeled alternative in {@link aslParser#stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtTry(&mut self, ctx: &StmtTryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtVarsDecl}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtVarsDecl(&mut self, ctx: &StmtVarsDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtVarDeclInit}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtVarDeclInit(&mut self, ctx: &StmtVarDeclInitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtConstDecl}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtConstDecl(&mut self, ctx: &StmtConstDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtAssign}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtAssign(&mut self, ctx: &StmtAssignContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtCall}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtCall(&mut self, ctx: &StmtCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtReturn}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtReturn(&mut self, ctx: &StmtReturnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtAssert}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtAssert(&mut self, ctx: &StmtAssertContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtUnpredictable}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtUnpredictable(&mut self, ctx: &StmtUnpredictableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtImpDef}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtImpDef(&mut self, ctx: &StmtImpDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtRepeat}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtRepeat(&mut self, ctx: &StmtRepeatContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtThrow}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtThrow(&mut self, ctx: &StmtThrowContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtUndefined}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtUndefined(&mut self, ctx: &StmtUndefinedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtSee}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtSee(&mut self, ctx: &StmtSeeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code StmtDefEnum}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_StmtDefEnum(&mut self, ctx: &StmtDefEnumContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#stmtElsIf}.
	 * @param ctx the parse tree
	 */
	fn visit_stmtElsIf(&mut self, ctx: &StmtElsIfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code CatchAltWhen}
	 * labeled alternative in {@link aslParser#catchAlt}.
	 * @param ctx the parse tree
	 */
	fn visit_CatchAltWhen(&mut self, ctx: &CatchAltWhenContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code CatchAltOtherwise}
	 * labeled alternative in {@link aslParser#catchAlt}.
	 * @param ctx the parse tree
	 */
	fn visit_CatchAltOtherwise(&mut self, ctx: &CatchAltOtherwiseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code CaseAltWhen}
	 * labeled alternative in {@link aslParser#caseAlt}.
	 * @param ctx the parse tree
	 */
	fn visit_CaseAltWhen(&mut self, ctx: &CaseAltWhenContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code CaseAltOtherwise}
	 * labeled alternative in {@link aslParser#caseAlt}.
	 * @param ctx the parse tree
	 */
	fn visit_CaseAltOtherwise(&mut self, ctx: &CaseAltOtherwiseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code CasePatternNat}
	 * labeled alternative in {@link aslParser#casePattern}.
	 * @param ctx the parse tree
	 */
	fn visit_CasePatternNat(&mut self, ctx: &CasePatternNatContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code CasePatternHex}
	 * labeled alternative in {@link aslParser#casePattern}.
	 * @param ctx the parse tree
	 */
	fn visit_CasePatternHex(&mut self, ctx: &CasePatternHexContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code CasePatternBin}
	 * labeled alternative in {@link aslParser#casePattern}.
	 * @param ctx the parse tree
	 */
	fn visit_CasePatternBin(&mut self, ctx: &CasePatternBinContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code CasePatternMask}
	 * labeled alternative in {@link aslParser#casePattern}.
	 * @param ctx the parse tree
	 */
	fn visit_CasePatternMask(&mut self, ctx: &CasePatternMaskContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code CasePatternBind}
	 * labeled alternative in {@link aslParser#casePattern}.
	 * @param ctx the parse tree
	 */
	fn visit_CasePatternBind(&mut self, ctx: &CasePatternBindContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code CasePatternIgnore}
	 * labeled alternative in {@link aslParser#casePattern}.
	 * @param ctx the parse tree
	 */
	fn visit_CasePatternIgnore(&mut self, ctx: &CasePatternIgnoreContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code CasePatternTuple}
	 * labeled alternative in {@link aslParser#casePattern}.
	 * @param ctx the parse tree
	 */
	fn visit_CasePatternTuple(&mut self, ctx: &CasePatternTupleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code LValSliceOf}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_LValSliceOf(&mut self, ctx: &LValSliceOfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code LValArray}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_LValArray(&mut self, ctx: &LValArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code LValArrayIndex}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_LValArrayIndex(&mut self, ctx: &LValArrayIndexContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code LValMemberBits}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_LValMemberBits(&mut self, ctx: &LValMemberBitsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code LValTuple}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_LValTuple(&mut self, ctx: &LValTupleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code LValMemberArray}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_LValMemberArray(&mut self, ctx: &LValMemberArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code LValIgnore}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_LValIgnore(&mut self, ctx: &LValIgnoreContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code LValMember}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_LValMember(&mut self, ctx: &LValMemberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code LValVarRef}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_LValVarRef(&mut self, ctx: &LValVarRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code LValSlice}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_LValSlice(&mut self, ctx: &LValSliceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprLitString}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprLitString(&mut self, ctx: &ExprLitStringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprImpDef}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprImpDef(&mut self, ctx: &ExprImpDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprParen}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprParen(&mut self, ctx: &ExprParenContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprTuple}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprTuple(&mut self, ctx: &ExprTupleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprIndex}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprIndex(&mut self, ctx: &ExprIndexContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprUnOp}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprUnOp(&mut self, ctx: &ExprUnOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprBinOp}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprBinOp(&mut self, ctx: &ExprBinOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprLitNat}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprLitNat(&mut self, ctx: &ExprLitNatContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprMembers}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprMembers(&mut self, ctx: &ExprMembersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprInMask}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprInMask(&mut self, ctx: &ExprInMaskContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprLitHex}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprLitHex(&mut self, ctx: &ExprLitHexContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprMemberBits}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprMemberBits(&mut self, ctx: &ExprMemberBitsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprCall}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprCall(&mut self, ctx: &ExprCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprInSet}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprInSet(&mut self, ctx: &ExprInSetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprLitBin}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprLitBin(&mut self, ctx: &ExprLitBinContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprUnknown}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprUnknown(&mut self, ctx: &ExprUnknownContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprLitReal}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprLitReal(&mut self, ctx: &ExprLitRealContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprVarRef}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprVarRef(&mut self, ctx: &ExprVarRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprSlice}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprSlice(&mut self, ctx: &ExprSliceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprLitMask}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprLitMask(&mut self, ctx: &ExprLitMaskContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprIf}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprIf(&mut self, ctx: &ExprIfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprMember}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprMember(&mut self, ctx: &ExprMemberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SliceExprMember}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_SliceExprMember(&mut self, ctx: &SliceExprMemberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SliceExprLitNat}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_SliceExprLitNat(&mut self, ctx: &SliceExprLitNatContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SliceExprVarRef}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_SliceExprVarRef(&mut self, ctx: &SliceExprVarRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SliceExprCall}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_SliceExprCall(&mut self, ctx: &SliceExprCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SliceExprUnOp}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_SliceExprUnOp(&mut self, ctx: &SliceExprUnOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SliceExprBinOp}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_SliceExprBinOp(&mut self, ctx: &SliceExprBinOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SliceExprLitHex}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_SliceExprLitHex(&mut self, ctx: &SliceExprLitHexContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SliceExprParen}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_SliceExprParen(&mut self, ctx: &SliceExprParenContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SliceExprIf}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_SliceExprIf(&mut self, ctx: &SliceExprIfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SliceRange}
	 * labeled alternative in {@link aslParser#slice}.
	 * @param ctx the parse tree
	 */
	fn visit_SliceRange(&mut self, ctx: &SliceRangeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SliceOffset}
	 * labeled alternative in {@link aslParser#slice}.
	 * @param ctx the parse tree
	 */
	fn visit_SliceOffset(&mut self, ctx: &SliceOffsetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SliceSingle}
	 * labeled alternative in {@link aslParser#slice}.
	 * @param ctx the parse tree
	 */
	fn visit_SliceSingle(&mut self, ctx: &SliceSingleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#exprElsIf}.
	 * @param ctx the parse tree
	 */
	fn visit_exprElsIf(&mut self, ctx: &ExprElsIfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SetElementRange}
	 * labeled alternative in {@link aslParser#setElement}.
	 * @param ctx the parse tree
	 */
	fn visit_SetElementRange(&mut self, ctx: &SetElementRangeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SetElementSingle}
	 * labeled alternative in {@link aslParser#setElement}.
	 * @param ctx the parse tree
	 */
	fn visit_SetElementSingle(&mut self, ctx: &SetElementSingleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#set}.
	 * @param ctx the parse tree
	 */
	fn visit_set(&mut self, ctx: &SetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#sliceCommaList0}.
	 * @param ctx the parse tree
	 */
	fn visit_sliceCommaList0(&mut self, ctx: &SliceCommaList0Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#sliceCommaList1}.
	 * @param ctx the parse tree
	 */
	fn visit_sliceCommaList1(&mut self, ctx: &SliceCommaList1Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#exprCommaList1}.
	 * @param ctx the parse tree
	 */
	fn visit_exprCommaList1(&mut self, ctx: &ExprCommaList1Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#exprCommaList0}.
	 * @param ctx the parse tree
	 */
	fn visit_exprCommaList0(&mut self, ctx: &ExprCommaList0Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#symDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_symDecl(&mut self, ctx: &SymDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#identifierCommaList0}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierCommaList0(&mut self, ctx: &IdentifierCommaList0Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#identifierCommaList1}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierCommaList1(&mut self, ctx: &IdentifierCommaList1Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#symDeclCommaList}.
	 * @param ctx the parse tree
	 */
	fn visit_symDeclCommaList(&mut self, ctx: &SymDeclCommaListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code QualIdUnqualified}
	 * labeled alternative in {@link aslParser#qualId}.
	 * @param ctx the parse tree
	 */
	fn visit_QualIdUnqualified(&mut self, ctx: &QualIdUnqualifiedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code QualIdAArch32}
	 * labeled alternative in {@link aslParser#qualId}.
	 * @param ctx the parse tree
	 */
	fn visit_QualIdAArch32(&mut self, ctx: &QualIdAArch32Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code QualIdAArch64}
	 * labeled alternative in {@link aslParser#qualId}.
	 * @param ctx the parse tree
	 */
	fn visit_QualIdAArch64(&mut self, ctx: &QualIdAArch64Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#idWithDots}.
	 * @param ctx the parse tree
	 */
	fn visit_idWithDots(&mut self, ctx: &IdWithDotsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link aslParser#id}.
	 * @param ctx the parse tree
	 */
	fn visit_id(&mut self, ctx: &IdContext<'input>) { self.visit_children(ctx) }

}

pub trait aslVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= aslParserContextType>{
	/**
	 * Visit a parse tree produced by {@link aslParser#registers}.
	 * @param ctx the parse tree
	 */
		fn visit_registers(&mut self, ctx: &RegistersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code RegDefArray}
	 * labeled alternative in {@link aslParser#registerDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_RegDefArray(&mut self, ctx: &RegDefArrayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code RegDefBasic}
	 * labeled alternative in {@link aslParser#registerDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_RegDefBasic(&mut self, ctx: &RegDefBasicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#register}.
	 * @param ctx the parse tree
	 */
		fn visit_register(&mut self, ctx: &RegisterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#arrayRegister}.
	 * @param ctx the parse tree
	 */
		fn visit_arrayRegister(&mut self, ctx: &ArrayRegisterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#registerField}.
	 * @param ctx the parse tree
	 */
		fn visit_registerField(&mut self, ctx: &RegisterFieldContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#registerFieldCommaList}.
	 * @param ctx the parse tree
	 */
		fn visit_registerFieldCommaList(&mut self, ctx: &RegisterFieldCommaListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#instructions}.
	 * @param ctx the parse tree
	 */
		fn visit_instructions(&mut self, ctx: &InstructionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#instruction}.
	 * @param ctx the parse tree
	 */
		fn visit_instruction(&mut self, ctx: &InstructionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#encoding}.
	 * @param ctx the parse tree
	 */
		fn visit_encoding(&mut self, ctx: &EncodingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#instructionField}.
	 * @param ctx the parse tree
	 */
		fn visit_instructionField(&mut self, ctx: &InstructionFieldContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#instrUnpredictableUnless}.
	 * @param ctx the parse tree
	 */
		fn visit_instrUnpredictableUnless(&mut self, ctx: &InstrUnpredictableUnlessContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#definitions}.
	 * @param ctx the parse tree
	 */
		fn visit_definitions(&mut self, ctx: &DefinitionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code DefTypeBuiltin}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
		fn visit_DefTypeBuiltin(&mut self, ctx: &DefTypeBuiltinContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code DefTypeAbstract}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
		fn visit_DefTypeAbstract(&mut self, ctx: &DefTypeAbstractContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code DefTypeAlias}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
		fn visit_DefTypeAlias(&mut self, ctx: &DefTypeAliasContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code DefTypeStruct}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
		fn visit_DefTypeStruct(&mut self, ctx: &DefTypeStructContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code DefTypeEnum}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
		fn visit_DefTypeEnum(&mut self, ctx: &DefTypeEnumContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code DefVariable}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
		fn visit_DefVariable(&mut self, ctx: &DefVariableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code DefConstant}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
		fn visit_DefConstant(&mut self, ctx: &DefConstantContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code DefArray}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
		fn visit_DefArray(&mut self, ctx: &DefArrayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code DefCallable}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
		fn visit_DefCallable(&mut self, ctx: &DefCallableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code DefGetter}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
		fn visit_DefGetter(&mut self, ctx: &DefGetterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code DefSetter}
	 * labeled alternative in {@link aslParser#definition}.
	 * @param ctx the parse tree
	 */
		fn visit_DefSetter(&mut self, ctx: &DefSetterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SetterRefArg}
	 * labeled alternative in {@link aslParser#setterArg}.
	 * @param ctx the parse tree
	 */
		fn visit_SetterRefArg(&mut self, ctx: &SetterRefArgContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SetterValArg}
	 * labeled alternative in {@link aslParser#setterArg}.
	 * @param ctx the parse tree
	 */
		fn visit_SetterValArg(&mut self, ctx: &SetterValArgContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#setterArgCommaList}.
	 * @param ctx the parse tree
	 */
		fn visit_setterArgCommaList(&mut self, ctx: &SetterArgCommaListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#returnType}.
	 * @param ctx the parse tree
	 */
		fn visit_returnType(&mut self, ctx: &ReturnTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code TypeRef}
	 * labeled alternative in {@link aslParser#typeSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_TypeRef(&mut self, ctx: &TypeRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code TypeIndexed}
	 * labeled alternative in {@link aslParser#typeSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_TypeIndexed(&mut self, ctx: &TypeIndexedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code TypeOf}
	 * labeled alternative in {@link aslParser#typeSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_TypeOf(&mut self, ctx: &TypeOfContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code TypeRegister}
	 * labeled alternative in {@link aslParser#typeSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_TypeRegister(&mut self, ctx: &TypeRegisterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code TypeArray}
	 * labeled alternative in {@link aslParser#typeSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_TypeArray(&mut self, ctx: &TypeArrayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code IxTypeRef}
	 * labeled alternative in {@link aslParser#ixType}.
	 * @param ctx the parse tree
	 */
		fn visit_IxTypeRef(&mut self, ctx: &IxTypeRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code IxTypeRange}
	 * labeled alternative in {@link aslParser#ixType}.
	 * @param ctx the parse tree
	 */
		fn visit_IxTypeRange(&mut self, ctx: &IxTypeRangeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#regField}.
	 * @param ctx the parse tree
	 */
		fn visit_regField(&mut self, ctx: &RegFieldContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#indentedBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_indentedBlock(&mut self, ctx: &IndentedBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#blockOrEmbed0}.
	 * @param ctx the parse tree
	 */
		fn visit_blockOrEmbed0(&mut self, ctx: &BlockOrEmbed0Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code BlockInline}
	 * labeled alternative in {@link aslParser#blockOrEmbed1}.
	 * @param ctx the parse tree
	 */
		fn visit_BlockInline(&mut self, ctx: &BlockInlineContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code BlockIndent}
	 * labeled alternative in {@link aslParser#blockOrEmbed1}.
	 * @param ctx the parse tree
	 */
		fn visit_BlockIndent(&mut self, ctx: &BlockIndentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtsInline}
	 * labeled alternative in {@link aslParser#stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtsInline(&mut self, ctx: &StmtsInlineContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtIf}
	 * labeled alternative in {@link aslParser#stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtIf(&mut self, ctx: &StmtIfContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtCase}
	 * labeled alternative in {@link aslParser#stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtCase(&mut self, ctx: &StmtCaseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtFor}
	 * labeled alternative in {@link aslParser#stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtFor(&mut self, ctx: &StmtForContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtWhile}
	 * labeled alternative in {@link aslParser#stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtWhile(&mut self, ctx: &StmtWhileContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtTry}
	 * labeled alternative in {@link aslParser#stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtTry(&mut self, ctx: &StmtTryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtVarsDecl}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtVarsDecl(&mut self, ctx: &StmtVarsDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtVarDeclInit}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtVarDeclInit(&mut self, ctx: &StmtVarDeclInitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtConstDecl}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtConstDecl(&mut self, ctx: &StmtConstDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtAssign}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtAssign(&mut self, ctx: &StmtAssignContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtCall}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtCall(&mut self, ctx: &StmtCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtReturn}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtReturn(&mut self, ctx: &StmtReturnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtAssert}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtAssert(&mut self, ctx: &StmtAssertContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtUnpredictable}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtUnpredictable(&mut self, ctx: &StmtUnpredictableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtImpDef}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtImpDef(&mut self, ctx: &StmtImpDefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtRepeat}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtRepeat(&mut self, ctx: &StmtRepeatContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtThrow}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtThrow(&mut self, ctx: &StmtThrowContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtUndefined}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtUndefined(&mut self, ctx: &StmtUndefinedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtSee}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtSee(&mut self, ctx: &StmtSeeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code StmtDefEnum}
	 * labeled alternative in {@link aslParser#inlineStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_StmtDefEnum(&mut self, ctx: &StmtDefEnumContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#stmtElsIf}.
	 * @param ctx the parse tree
	 */
		fn visit_stmtElsIf(&mut self, ctx: &StmtElsIfContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code CatchAltWhen}
	 * labeled alternative in {@link aslParser#catchAlt}.
	 * @param ctx the parse tree
	 */
		fn visit_CatchAltWhen(&mut self, ctx: &CatchAltWhenContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code CatchAltOtherwise}
	 * labeled alternative in {@link aslParser#catchAlt}.
	 * @param ctx the parse tree
	 */
		fn visit_CatchAltOtherwise(&mut self, ctx: &CatchAltOtherwiseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code CaseAltWhen}
	 * labeled alternative in {@link aslParser#caseAlt}.
	 * @param ctx the parse tree
	 */
		fn visit_CaseAltWhen(&mut self, ctx: &CaseAltWhenContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code CaseAltOtherwise}
	 * labeled alternative in {@link aslParser#caseAlt}.
	 * @param ctx the parse tree
	 */
		fn visit_CaseAltOtherwise(&mut self, ctx: &CaseAltOtherwiseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code CasePatternNat}
	 * labeled alternative in {@link aslParser#casePattern}.
	 * @param ctx the parse tree
	 */
		fn visit_CasePatternNat(&mut self, ctx: &CasePatternNatContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code CasePatternHex}
	 * labeled alternative in {@link aslParser#casePattern}.
	 * @param ctx the parse tree
	 */
		fn visit_CasePatternHex(&mut self, ctx: &CasePatternHexContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code CasePatternBin}
	 * labeled alternative in {@link aslParser#casePattern}.
	 * @param ctx the parse tree
	 */
		fn visit_CasePatternBin(&mut self, ctx: &CasePatternBinContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code CasePatternMask}
	 * labeled alternative in {@link aslParser#casePattern}.
	 * @param ctx the parse tree
	 */
		fn visit_CasePatternMask(&mut self, ctx: &CasePatternMaskContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code CasePatternBind}
	 * labeled alternative in {@link aslParser#casePattern}.
	 * @param ctx the parse tree
	 */
		fn visit_CasePatternBind(&mut self, ctx: &CasePatternBindContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code CasePatternIgnore}
	 * labeled alternative in {@link aslParser#casePattern}.
	 * @param ctx the parse tree
	 */
		fn visit_CasePatternIgnore(&mut self, ctx: &CasePatternIgnoreContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code CasePatternTuple}
	 * labeled alternative in {@link aslParser#casePattern}.
	 * @param ctx the parse tree
	 */
		fn visit_CasePatternTuple(&mut self, ctx: &CasePatternTupleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code LValSliceOf}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_LValSliceOf(&mut self, ctx: &LValSliceOfContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code LValArray}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_LValArray(&mut self, ctx: &LValArrayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code LValArrayIndex}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_LValArrayIndex(&mut self, ctx: &LValArrayIndexContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code LValMemberBits}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_LValMemberBits(&mut self, ctx: &LValMemberBitsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code LValTuple}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_LValTuple(&mut self, ctx: &LValTupleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code LValMemberArray}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_LValMemberArray(&mut self, ctx: &LValMemberArrayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code LValIgnore}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_LValIgnore(&mut self, ctx: &LValIgnoreContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code LValMember}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_LValMember(&mut self, ctx: &LValMemberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code LValVarRef}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_LValVarRef(&mut self, ctx: &LValVarRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code LValSlice}
	 * labeled alternative in {@link aslParser#lValExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_LValSlice(&mut self, ctx: &LValSliceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprLitString}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprLitString(&mut self, ctx: &ExprLitStringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprImpDef}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprImpDef(&mut self, ctx: &ExprImpDefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprParen}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprParen(&mut self, ctx: &ExprParenContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprTuple}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprTuple(&mut self, ctx: &ExprTupleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprIndex}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprIndex(&mut self, ctx: &ExprIndexContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprUnOp}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprUnOp(&mut self, ctx: &ExprUnOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprBinOp}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprBinOp(&mut self, ctx: &ExprBinOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprLitNat}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprLitNat(&mut self, ctx: &ExprLitNatContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprMembers}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprMembers(&mut self, ctx: &ExprMembersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprInMask}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprInMask(&mut self, ctx: &ExprInMaskContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprLitHex}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprLitHex(&mut self, ctx: &ExprLitHexContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprMemberBits}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprMemberBits(&mut self, ctx: &ExprMemberBitsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprCall}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprCall(&mut self, ctx: &ExprCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprInSet}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprInSet(&mut self, ctx: &ExprInSetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprLitBin}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprLitBin(&mut self, ctx: &ExprLitBinContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprUnknown}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprUnknown(&mut self, ctx: &ExprUnknownContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprLitReal}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprLitReal(&mut self, ctx: &ExprLitRealContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprVarRef}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprVarRef(&mut self, ctx: &ExprVarRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprSlice}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprSlice(&mut self, ctx: &ExprSliceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprLitMask}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprLitMask(&mut self, ctx: &ExprLitMaskContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprIf}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprIf(&mut self, ctx: &ExprIfContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprMember}
	 * labeled alternative in {@link aslParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprMember(&mut self, ctx: &ExprMemberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SliceExprMember}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_SliceExprMember(&mut self, ctx: &SliceExprMemberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SliceExprLitNat}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_SliceExprLitNat(&mut self, ctx: &SliceExprLitNatContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SliceExprVarRef}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_SliceExprVarRef(&mut self, ctx: &SliceExprVarRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SliceExprCall}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_SliceExprCall(&mut self, ctx: &SliceExprCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SliceExprUnOp}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_SliceExprUnOp(&mut self, ctx: &SliceExprUnOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SliceExprBinOp}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_SliceExprBinOp(&mut self, ctx: &SliceExprBinOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SliceExprLitHex}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_SliceExprLitHex(&mut self, ctx: &SliceExprLitHexContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SliceExprParen}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_SliceExprParen(&mut self, ctx: &SliceExprParenContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SliceExprIf}
	 * labeled alternative in {@link aslParser#sliceExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_SliceExprIf(&mut self, ctx: &SliceExprIfContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SliceRange}
	 * labeled alternative in {@link aslParser#slice}.
	 * @param ctx the parse tree
	 */
		fn visit_SliceRange(&mut self, ctx: &SliceRangeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SliceOffset}
	 * labeled alternative in {@link aslParser#slice}.
	 * @param ctx the parse tree
	 */
		fn visit_SliceOffset(&mut self, ctx: &SliceOffsetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SliceSingle}
	 * labeled alternative in {@link aslParser#slice}.
	 * @param ctx the parse tree
	 */
		fn visit_SliceSingle(&mut self, ctx: &SliceSingleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#exprElsIf}.
	 * @param ctx the parse tree
	 */
		fn visit_exprElsIf(&mut self, ctx: &ExprElsIfContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SetElementRange}
	 * labeled alternative in {@link aslParser#setElement}.
	 * @param ctx the parse tree
	 */
		fn visit_SetElementRange(&mut self, ctx: &SetElementRangeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SetElementSingle}
	 * labeled alternative in {@link aslParser#setElement}.
	 * @param ctx the parse tree
	 */
		fn visit_SetElementSingle(&mut self, ctx: &SetElementSingleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#set}.
	 * @param ctx the parse tree
	 */
		fn visit_set(&mut self, ctx: &SetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#sliceCommaList0}.
	 * @param ctx the parse tree
	 */
		fn visit_sliceCommaList0(&mut self, ctx: &SliceCommaList0Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#sliceCommaList1}.
	 * @param ctx the parse tree
	 */
		fn visit_sliceCommaList1(&mut self, ctx: &SliceCommaList1Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#exprCommaList1}.
	 * @param ctx the parse tree
	 */
		fn visit_exprCommaList1(&mut self, ctx: &ExprCommaList1Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#exprCommaList0}.
	 * @param ctx the parse tree
	 */
		fn visit_exprCommaList0(&mut self, ctx: &ExprCommaList0Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#symDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_symDecl(&mut self, ctx: &SymDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#identifierCommaList0}.
	 * @param ctx the parse tree
	 */
		fn visit_identifierCommaList0(&mut self, ctx: &IdentifierCommaList0Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#identifierCommaList1}.
	 * @param ctx the parse tree
	 */
		fn visit_identifierCommaList1(&mut self, ctx: &IdentifierCommaList1Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#symDeclCommaList}.
	 * @param ctx the parse tree
	 */
		fn visit_symDeclCommaList(&mut self, ctx: &SymDeclCommaListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code QualIdUnqualified}
	 * labeled alternative in {@link aslParser#qualId}.
	 * @param ctx the parse tree
	 */
		fn visit_QualIdUnqualified(&mut self, ctx: &QualIdUnqualifiedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code QualIdAArch32}
	 * labeled alternative in {@link aslParser#qualId}.
	 * @param ctx the parse tree
	 */
		fn visit_QualIdAArch32(&mut self, ctx: &QualIdAArch32Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code QualIdAArch64}
	 * labeled alternative in {@link aslParser#qualId}.
	 * @param ctx the parse tree
	 */
		fn visit_QualIdAArch64(&mut self, ctx: &QualIdAArch64Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#idWithDots}.
	 * @param ctx the parse tree
	 */
		fn visit_idWithDots(&mut self, ctx: &IdWithDotsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link aslParser#id}.
	 * @param ctx the parse tree
	 */
		fn visit_id(&mut self, ctx: &IdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> aslVisitor<'input> for T
where
	T: aslVisitorCompat<'input>
{
	fn visit_registers(&mut self, ctx: &RegistersContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_registers(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_RegDefArray(&mut self, ctx: &RegDefArrayContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_RegDefArray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_RegDefBasic(&mut self, ctx: &RegDefBasicContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_RegDefBasic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_register(&mut self, ctx: &RegisterContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_register(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arrayRegister(&mut self, ctx: &ArrayRegisterContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_arrayRegister(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_registerField(&mut self, ctx: &RegisterFieldContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_registerField(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_registerFieldCommaList(&mut self, ctx: &RegisterFieldCommaListContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_registerFieldCommaList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_instructions(&mut self, ctx: &InstructionsContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_instructions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_instruction(&mut self, ctx: &InstructionContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_instruction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_encoding(&mut self, ctx: &EncodingContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_encoding(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_instructionField(&mut self, ctx: &InstructionFieldContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_instructionField(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_instrUnpredictableUnless(&mut self, ctx: &InstrUnpredictableUnlessContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_instrUnpredictableUnless(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_definitions(&mut self, ctx: &DefinitionsContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_definitions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_DefTypeBuiltin(&mut self, ctx: &DefTypeBuiltinContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_DefTypeBuiltin(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_DefTypeAbstract(&mut self, ctx: &DefTypeAbstractContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_DefTypeAbstract(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_DefTypeAlias(&mut self, ctx: &DefTypeAliasContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_DefTypeAlias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_DefTypeStruct(&mut self, ctx: &DefTypeStructContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_DefTypeStruct(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_DefTypeEnum(&mut self, ctx: &DefTypeEnumContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_DefTypeEnum(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_DefVariable(&mut self, ctx: &DefVariableContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_DefVariable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_DefConstant(&mut self, ctx: &DefConstantContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_DefConstant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_DefArray(&mut self, ctx: &DefArrayContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_DefArray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_DefCallable(&mut self, ctx: &DefCallableContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_DefCallable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_DefGetter(&mut self, ctx: &DefGetterContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_DefGetter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_DefSetter(&mut self, ctx: &DefSetterContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_DefSetter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SetterRefArg(&mut self, ctx: &SetterRefArgContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SetterRefArg(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SetterValArg(&mut self, ctx: &SetterValArgContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SetterValArg(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setterArgCommaList(&mut self, ctx: &SetterArgCommaListContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_setterArgCommaList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_returnType(&mut self, ctx: &ReturnTypeContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_returnType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_TypeRef(&mut self, ctx: &TypeRefContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_TypeRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_TypeIndexed(&mut self, ctx: &TypeIndexedContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_TypeIndexed(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_TypeOf(&mut self, ctx: &TypeOfContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_TypeOf(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_TypeRegister(&mut self, ctx: &TypeRegisterContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_TypeRegister(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_TypeArray(&mut self, ctx: &TypeArrayContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_TypeArray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_IxTypeRef(&mut self, ctx: &IxTypeRefContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_IxTypeRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_IxTypeRange(&mut self, ctx: &IxTypeRangeContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_IxTypeRange(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_regField(&mut self, ctx: &RegFieldContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_regField(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_indentedBlock(&mut self, ctx: &IndentedBlockContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_indentedBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_blockOrEmbed0(&mut self, ctx: &BlockOrEmbed0Context<'input>){
		let result = <Self as aslVisitorCompat>::visit_blockOrEmbed0(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_BlockInline(&mut self, ctx: &BlockInlineContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_BlockInline(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_BlockIndent(&mut self, ctx: &BlockIndentContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_BlockIndent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtsInline(&mut self, ctx: &StmtsInlineContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtsInline(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtIf(&mut self, ctx: &StmtIfContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtIf(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtCase(&mut self, ctx: &StmtCaseContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtCase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtFor(&mut self, ctx: &StmtForContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtFor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtWhile(&mut self, ctx: &StmtWhileContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtWhile(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtTry(&mut self, ctx: &StmtTryContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtTry(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtVarsDecl(&mut self, ctx: &StmtVarsDeclContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtVarsDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtVarDeclInit(&mut self, ctx: &StmtVarDeclInitContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtVarDeclInit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtConstDecl(&mut self, ctx: &StmtConstDeclContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtConstDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtAssign(&mut self, ctx: &StmtAssignContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtAssign(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtCall(&mut self, ctx: &StmtCallContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtReturn(&mut self, ctx: &StmtReturnContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtReturn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtAssert(&mut self, ctx: &StmtAssertContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtAssert(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtUnpredictable(&mut self, ctx: &StmtUnpredictableContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtUnpredictable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtImpDef(&mut self, ctx: &StmtImpDefContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtImpDef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtRepeat(&mut self, ctx: &StmtRepeatContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtRepeat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtThrow(&mut self, ctx: &StmtThrowContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtThrow(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtUndefined(&mut self, ctx: &StmtUndefinedContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtUndefined(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtSee(&mut self, ctx: &StmtSeeContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtSee(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_StmtDefEnum(&mut self, ctx: &StmtDefEnumContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_StmtDefEnum(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stmtElsIf(&mut self, ctx: &StmtElsIfContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_stmtElsIf(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_CatchAltWhen(&mut self, ctx: &CatchAltWhenContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_CatchAltWhen(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_CatchAltOtherwise(&mut self, ctx: &CatchAltOtherwiseContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_CatchAltOtherwise(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_CaseAltWhen(&mut self, ctx: &CaseAltWhenContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_CaseAltWhen(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_CaseAltOtherwise(&mut self, ctx: &CaseAltOtherwiseContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_CaseAltOtherwise(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_CasePatternNat(&mut self, ctx: &CasePatternNatContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_CasePatternNat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_CasePatternHex(&mut self, ctx: &CasePatternHexContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_CasePatternHex(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_CasePatternBin(&mut self, ctx: &CasePatternBinContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_CasePatternBin(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_CasePatternMask(&mut self, ctx: &CasePatternMaskContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_CasePatternMask(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_CasePatternBind(&mut self, ctx: &CasePatternBindContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_CasePatternBind(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_CasePatternIgnore(&mut self, ctx: &CasePatternIgnoreContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_CasePatternIgnore(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_CasePatternTuple(&mut self, ctx: &CasePatternTupleContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_CasePatternTuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_LValSliceOf(&mut self, ctx: &LValSliceOfContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_LValSliceOf(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_LValArray(&mut self, ctx: &LValArrayContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_LValArray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_LValArrayIndex(&mut self, ctx: &LValArrayIndexContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_LValArrayIndex(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_LValMemberBits(&mut self, ctx: &LValMemberBitsContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_LValMemberBits(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_LValTuple(&mut self, ctx: &LValTupleContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_LValTuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_LValMemberArray(&mut self, ctx: &LValMemberArrayContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_LValMemberArray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_LValIgnore(&mut self, ctx: &LValIgnoreContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_LValIgnore(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_LValMember(&mut self, ctx: &LValMemberContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_LValMember(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_LValVarRef(&mut self, ctx: &LValVarRefContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_LValVarRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_LValSlice(&mut self, ctx: &LValSliceContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_LValSlice(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprLitString(&mut self, ctx: &ExprLitStringContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprLitString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprImpDef(&mut self, ctx: &ExprImpDefContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprImpDef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprParen(&mut self, ctx: &ExprParenContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprParen(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprTuple(&mut self, ctx: &ExprTupleContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprTuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprIndex(&mut self, ctx: &ExprIndexContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprIndex(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprUnOp(&mut self, ctx: &ExprUnOpContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprUnOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprBinOp(&mut self, ctx: &ExprBinOpContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprBinOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprLitNat(&mut self, ctx: &ExprLitNatContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprLitNat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprMembers(&mut self, ctx: &ExprMembersContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprMembers(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprInMask(&mut self, ctx: &ExprInMaskContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprInMask(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprLitHex(&mut self, ctx: &ExprLitHexContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprLitHex(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprMemberBits(&mut self, ctx: &ExprMemberBitsContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprMemberBits(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprCall(&mut self, ctx: &ExprCallContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprInSet(&mut self, ctx: &ExprInSetContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprInSet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprLitBin(&mut self, ctx: &ExprLitBinContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprLitBin(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprUnknown(&mut self, ctx: &ExprUnknownContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprUnknown(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprLitReal(&mut self, ctx: &ExprLitRealContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprLitReal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprVarRef(&mut self, ctx: &ExprVarRefContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprVarRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprSlice(&mut self, ctx: &ExprSliceContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprSlice(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprLitMask(&mut self, ctx: &ExprLitMaskContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprLitMask(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprIf(&mut self, ctx: &ExprIfContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprIf(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprMember(&mut self, ctx: &ExprMemberContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_ExprMember(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SliceExprMember(&mut self, ctx: &SliceExprMemberContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SliceExprMember(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SliceExprLitNat(&mut self, ctx: &SliceExprLitNatContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SliceExprLitNat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SliceExprVarRef(&mut self, ctx: &SliceExprVarRefContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SliceExprVarRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SliceExprCall(&mut self, ctx: &SliceExprCallContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SliceExprCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SliceExprUnOp(&mut self, ctx: &SliceExprUnOpContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SliceExprUnOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SliceExprBinOp(&mut self, ctx: &SliceExprBinOpContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SliceExprBinOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SliceExprLitHex(&mut self, ctx: &SliceExprLitHexContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SliceExprLitHex(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SliceExprParen(&mut self, ctx: &SliceExprParenContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SliceExprParen(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SliceExprIf(&mut self, ctx: &SliceExprIfContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SliceExprIf(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SliceRange(&mut self, ctx: &SliceRangeContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SliceRange(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SliceOffset(&mut self, ctx: &SliceOffsetContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SliceOffset(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SliceSingle(&mut self, ctx: &SliceSingleContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SliceSingle(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exprElsIf(&mut self, ctx: &ExprElsIfContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_exprElsIf(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SetElementRange(&mut self, ctx: &SetElementRangeContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SetElementRange(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SetElementSingle(&mut self, ctx: &SetElementSingleContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_SetElementSingle(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_set(&mut self, ctx: &SetContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_set(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sliceCommaList0(&mut self, ctx: &SliceCommaList0Context<'input>){
		let result = <Self as aslVisitorCompat>::visit_sliceCommaList0(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sliceCommaList1(&mut self, ctx: &SliceCommaList1Context<'input>){
		let result = <Self as aslVisitorCompat>::visit_sliceCommaList1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exprCommaList1(&mut self, ctx: &ExprCommaList1Context<'input>){
		let result = <Self as aslVisitorCompat>::visit_exprCommaList1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exprCommaList0(&mut self, ctx: &ExprCommaList0Context<'input>){
		let result = <Self as aslVisitorCompat>::visit_exprCommaList0(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_symDecl(&mut self, ctx: &SymDeclContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_symDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifierCommaList0(&mut self, ctx: &IdentifierCommaList0Context<'input>){
		let result = <Self as aslVisitorCompat>::visit_identifierCommaList0(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifierCommaList1(&mut self, ctx: &IdentifierCommaList1Context<'input>){
		let result = <Self as aslVisitorCompat>::visit_identifierCommaList1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_symDeclCommaList(&mut self, ctx: &SymDeclCommaListContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_symDeclCommaList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_QualIdUnqualified(&mut self, ctx: &QualIdUnqualifiedContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_QualIdUnqualified(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_QualIdAArch32(&mut self, ctx: &QualIdAArch32Context<'input>){
		let result = <Self as aslVisitorCompat>::visit_QualIdAArch32(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_QualIdAArch64(&mut self, ctx: &QualIdAArch64Context<'input>){
		let result = <Self as aslVisitorCompat>::visit_QualIdAArch64(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_idWithDots(&mut self, ctx: &IdWithDotsContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_idWithDots(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_id(&mut self, ctx: &IdContext<'input>){
		let result = <Self as aslVisitorCompat>::visit_id(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}