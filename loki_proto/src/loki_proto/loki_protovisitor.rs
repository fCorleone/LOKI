#![allow(nonstandard_style)]
// Generated from Loki_proto.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor};
use super::loki_protoparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link Loki_protoParser}.
 */
pub trait Loki_protoVisitor<'input>: ParseTreeVisitor<'input,Loki_protoParserContextType>{
	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#proto}.
	 * @param ctx the parse tree
	 */
	fn visit_proto(&mut self, ctx: &ProtoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#syntax}.
	 * @param ctx the parse tree
	 */
	fn visit_syntax(&mut self, ctx: &SyntaxContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#importStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#packageStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_packageStatement(&mut self, ctx: &PackageStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#option}.
	 * @param ctx the parse tree
	 */
	fn visit_option(&mut self, ctx: &OptionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#optionName}.
	 * @param ctx the parse tree
	 */
	fn visit_optionName(&mut self, ctx: &OptionNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#topLevelDef}.
	 * @param ctx the parse tree
	 */
	fn visit_topLevelDef(&mut self, ctx: &TopLevelDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#message}.
	 * @param ctx the parse tree
	 */
	fn visit_message(&mut self, ctx: &MessageContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#messageBody}.
	 * @param ctx the parse tree
	 */
	fn visit_messageBody(&mut self, ctx: &MessageBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#enumDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_enumDefinition(&mut self, ctx: &EnumDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#enumBody}.
	 * @param ctx the parse tree
	 */
	fn visit_enumBody(&mut self, ctx: &EnumBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#enumField}.
	 * @param ctx the parse tree
	 */
	fn visit_enumField(&mut self, ctx: &EnumFieldContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#enumValueOption}.
	 * @param ctx the parse tree
	 */
	fn visit_enumValueOption(&mut self, ctx: &EnumValueOptionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#service}.
	 * @param ctx the parse tree
	 */
	fn visit_service(&mut self, ctx: &ServiceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#rpc}.
	 * @param ctx the parse tree
	 */
	fn visit_rpc(&mut self, ctx: &RpcContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#reserved}.
	 * @param ctx the parse tree
	 */
	fn visit_reserved(&mut self, ctx: &ReservedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#ranges}.
	 * @param ctx the parse tree
	 */
	fn visit_ranges(&mut self, ctx: &RangesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#range}.
	 * @param ctx the parse tree
	 */
	fn visit_range(&mut self, ctx: &RangeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#fieldNames}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldNames(&mut self, ctx: &FieldNamesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#eleType}.
	 * @param ctx the parse tree
	 */
	fn visit_eleType(&mut self, ctx: &EleTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#fieldNumber}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldNumber(&mut self, ctx: &FieldNumberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#field}.
	 * @param ctx the parse tree
	 */
	fn visit_field(&mut self, ctx: &FieldContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#fieldOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldOptions(&mut self, ctx: &FieldOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#fieldOption}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldOption(&mut self, ctx: &FieldOptionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#oneof}.
	 * @param ctx the parse tree
	 */
	fn visit_oneof(&mut self, ctx: &OneofContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#oneofField}.
	 * @param ctx the parse tree
	 */
	fn visit_oneofField(&mut self, ctx: &OneofFieldContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#mapField}.
	 * @param ctx the parse tree
	 */
	fn visit_mapField(&mut self, ctx: &MapFieldContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#keyType}.
	 * @param ctx the parse tree
	 */
	fn visit_keyType(&mut self, ctx: &KeyTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#fullIdent}.
	 * @param ctx the parse tree
	 */
	fn visit_fullIdent(&mut self, ctx: &FullIdentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#messageName}.
	 * @param ctx the parse tree
	 */
	fn visit_messageName(&mut self, ctx: &MessageNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#enumName}.
	 * @param ctx the parse tree
	 */
	fn visit_enumName(&mut self, ctx: &EnumNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#messageOrEnumName}.
	 * @param ctx the parse tree
	 */
	fn visit_messageOrEnumName(&mut self, ctx: &MessageOrEnumNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#fieldName}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldName(&mut self, ctx: &FieldNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#oneofName}.
	 * @param ctx the parse tree
	 */
	fn visit_oneofName(&mut self, ctx: &OneofNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#mapName}.
	 * @param ctx the parse tree
	 */
	fn visit_mapName(&mut self, ctx: &MapNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#serviceName}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceName(&mut self, ctx: &ServiceNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#rpcName}.
	 * @param ctx the parse tree
	 */
	fn visit_rpcName(&mut self, ctx: &RpcNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#messageType}.
	 * @param ctx the parse tree
	 */
	fn visit_messageType(&mut self, ctx: &MessageTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#messageOrEnumType}.
	 * @param ctx the parse tree
	 */
	fn visit_messageOrEnumType(&mut self, ctx: &MessageOrEnumTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#emptyStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_emptyStatement(&mut self, ctx: &EmptyStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Loki_protoParser#constant}.
	 * @param ctx the parse tree
	 */
	fn visit_constant(&mut self, ctx: &ConstantContext<'input>) { self.visit_children(ctx) }


}