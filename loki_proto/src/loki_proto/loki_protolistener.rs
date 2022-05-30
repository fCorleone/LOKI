#![allow(nonstandard_style)]
// Generated from Loki_proto.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::loki_protoparser::*;

pub trait Loki_protoListener<'input> : ParseTreeListener<'input,Loki_protoParserContextType>{

/**
 * Enter a parse tree produced by {@link Loki_protoParser#proto}.
 * @param ctx the parse tree
 */
fn enter_proto(&mut self, _ctx: &ProtoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#proto}.
 * @param ctx the parse tree
 */
fn exit_proto(&mut self, _ctx: &ProtoContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#syntax}.
 * @param ctx the parse tree
 */
fn enter_syntax(&mut self, _ctx: &SyntaxContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#syntax}.
 * @param ctx the parse tree
 */
fn exit_syntax(&mut self, _ctx: &SyntaxContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#importStatement}.
 * @param ctx the parse tree
 */
fn enter_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#importStatement}.
 * @param ctx the parse tree
 */
fn exit_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#packageStatement}.
 * @param ctx the parse tree
 */
fn enter_packageStatement(&mut self, _ctx: &PackageStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#packageStatement}.
 * @param ctx the parse tree
 */
fn exit_packageStatement(&mut self, _ctx: &PackageStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#option}.
 * @param ctx the parse tree
 */
fn enter_option(&mut self, _ctx: &OptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#option}.
 * @param ctx the parse tree
 */
fn exit_option(&mut self, _ctx: &OptionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#optionName}.
 * @param ctx the parse tree
 */
fn enter_optionName(&mut self, _ctx: &OptionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#optionName}.
 * @param ctx the parse tree
 */
fn exit_optionName(&mut self, _ctx: &OptionNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#topLevelDef}.
 * @param ctx the parse tree
 */
fn enter_topLevelDef(&mut self, _ctx: &TopLevelDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#topLevelDef}.
 * @param ctx the parse tree
 */
fn exit_topLevelDef(&mut self, _ctx: &TopLevelDefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#message}.
 * @param ctx the parse tree
 */
fn enter_message(&mut self, _ctx: &MessageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#message}.
 * @param ctx the parse tree
 */
fn exit_message(&mut self, _ctx: &MessageContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#messageBody}.
 * @param ctx the parse tree
 */
fn enter_messageBody(&mut self, _ctx: &MessageBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#messageBody}.
 * @param ctx the parse tree
 */
fn exit_messageBody(&mut self, _ctx: &MessageBodyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#enumDefinition}.
 * @param ctx the parse tree
 */
fn enter_enumDefinition(&mut self, _ctx: &EnumDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#enumDefinition}.
 * @param ctx the parse tree
 */
fn exit_enumDefinition(&mut self, _ctx: &EnumDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#enumBody}.
 * @param ctx the parse tree
 */
fn enter_enumBody(&mut self, _ctx: &EnumBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#enumBody}.
 * @param ctx the parse tree
 */
fn exit_enumBody(&mut self, _ctx: &EnumBodyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#enumField}.
 * @param ctx the parse tree
 */
fn enter_enumField(&mut self, _ctx: &EnumFieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#enumField}.
 * @param ctx the parse tree
 */
fn exit_enumField(&mut self, _ctx: &EnumFieldContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#enumValueOption}.
 * @param ctx the parse tree
 */
fn enter_enumValueOption(&mut self, _ctx: &EnumValueOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#enumValueOption}.
 * @param ctx the parse tree
 */
fn exit_enumValueOption(&mut self, _ctx: &EnumValueOptionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#service}.
 * @param ctx the parse tree
 */
fn enter_service(&mut self, _ctx: &ServiceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#service}.
 * @param ctx the parse tree
 */
fn exit_service(&mut self, _ctx: &ServiceContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#rpc}.
 * @param ctx the parse tree
 */
fn enter_rpc(&mut self, _ctx: &RpcContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#rpc}.
 * @param ctx the parse tree
 */
fn exit_rpc(&mut self, _ctx: &RpcContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#reserved}.
 * @param ctx the parse tree
 */
fn enter_reserved(&mut self, _ctx: &ReservedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#reserved}.
 * @param ctx the parse tree
 */
fn exit_reserved(&mut self, _ctx: &ReservedContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#ranges}.
 * @param ctx the parse tree
 */
fn enter_ranges(&mut self, _ctx: &RangesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#ranges}.
 * @param ctx the parse tree
 */
fn exit_ranges(&mut self, _ctx: &RangesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#range}.
 * @param ctx the parse tree
 */
fn enter_range(&mut self, _ctx: &RangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#range}.
 * @param ctx the parse tree
 */
fn exit_range(&mut self, _ctx: &RangeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#fieldNames}.
 * @param ctx the parse tree
 */
fn enter_fieldNames(&mut self, _ctx: &FieldNamesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#fieldNames}.
 * @param ctx the parse tree
 */
fn exit_fieldNames(&mut self, _ctx: &FieldNamesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#eleType}.
 * @param ctx the parse tree
 */
fn enter_eleType(&mut self, _ctx: &EleTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#eleType}.
 * @param ctx the parse tree
 */
fn exit_eleType(&mut self, _ctx: &EleTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#fieldNumber}.
 * @param ctx the parse tree
 */
fn enter_fieldNumber(&mut self, _ctx: &FieldNumberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#fieldNumber}.
 * @param ctx the parse tree
 */
fn exit_fieldNumber(&mut self, _ctx: &FieldNumberContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#field}.
 * @param ctx the parse tree
 */
fn enter_field(&mut self, _ctx: &FieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#field}.
 * @param ctx the parse tree
 */
fn exit_field(&mut self, _ctx: &FieldContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#fieldOptions}.
 * @param ctx the parse tree
 */
fn enter_fieldOptions(&mut self, _ctx: &FieldOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#fieldOptions}.
 * @param ctx the parse tree
 */
fn exit_fieldOptions(&mut self, _ctx: &FieldOptionsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#fieldOption}.
 * @param ctx the parse tree
 */
fn enter_fieldOption(&mut self, _ctx: &FieldOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#fieldOption}.
 * @param ctx the parse tree
 */
fn exit_fieldOption(&mut self, _ctx: &FieldOptionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#oneof}.
 * @param ctx the parse tree
 */
fn enter_oneof(&mut self, _ctx: &OneofContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#oneof}.
 * @param ctx the parse tree
 */
fn exit_oneof(&mut self, _ctx: &OneofContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#oneofField}.
 * @param ctx the parse tree
 */
fn enter_oneofField(&mut self, _ctx: &OneofFieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#oneofField}.
 * @param ctx the parse tree
 */
fn exit_oneofField(&mut self, _ctx: &OneofFieldContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#mapField}.
 * @param ctx the parse tree
 */
fn enter_mapField(&mut self, _ctx: &MapFieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#mapField}.
 * @param ctx the parse tree
 */
fn exit_mapField(&mut self, _ctx: &MapFieldContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#keyType}.
 * @param ctx the parse tree
 */
fn enter_keyType(&mut self, _ctx: &KeyTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#keyType}.
 * @param ctx the parse tree
 */
fn exit_keyType(&mut self, _ctx: &KeyTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#fullIdent}.
 * @param ctx the parse tree
 */
fn enter_fullIdent(&mut self, _ctx: &FullIdentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#fullIdent}.
 * @param ctx the parse tree
 */
fn exit_fullIdent(&mut self, _ctx: &FullIdentContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#messageName}.
 * @param ctx the parse tree
 */
fn enter_messageName(&mut self, _ctx: &MessageNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#messageName}.
 * @param ctx the parse tree
 */
fn exit_messageName(&mut self, _ctx: &MessageNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#enumName}.
 * @param ctx the parse tree
 */
fn enter_enumName(&mut self, _ctx: &EnumNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#enumName}.
 * @param ctx the parse tree
 */
fn exit_enumName(&mut self, _ctx: &EnumNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#messageOrEnumName}.
 * @param ctx the parse tree
 */
fn enter_messageOrEnumName(&mut self, _ctx: &MessageOrEnumNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#messageOrEnumName}.
 * @param ctx the parse tree
 */
fn exit_messageOrEnumName(&mut self, _ctx: &MessageOrEnumNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#fieldName}.
 * @param ctx the parse tree
 */
fn enter_fieldName(&mut self, _ctx: &FieldNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#fieldName}.
 * @param ctx the parse tree
 */
fn exit_fieldName(&mut self, _ctx: &FieldNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#oneofName}.
 * @param ctx the parse tree
 */
fn enter_oneofName(&mut self, _ctx: &OneofNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#oneofName}.
 * @param ctx the parse tree
 */
fn exit_oneofName(&mut self, _ctx: &OneofNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#mapName}.
 * @param ctx the parse tree
 */
fn enter_mapName(&mut self, _ctx: &MapNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#mapName}.
 * @param ctx the parse tree
 */
fn exit_mapName(&mut self, _ctx: &MapNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#serviceName}.
 * @param ctx the parse tree
 */
fn enter_serviceName(&mut self, _ctx: &ServiceNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#serviceName}.
 * @param ctx the parse tree
 */
fn exit_serviceName(&mut self, _ctx: &ServiceNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#rpcName}.
 * @param ctx the parse tree
 */
fn enter_rpcName(&mut self, _ctx: &RpcNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#rpcName}.
 * @param ctx the parse tree
 */
fn exit_rpcName(&mut self, _ctx: &RpcNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#messageType}.
 * @param ctx the parse tree
 */
fn enter_messageType(&mut self, _ctx: &MessageTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#messageType}.
 * @param ctx the parse tree
 */
fn exit_messageType(&mut self, _ctx: &MessageTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#messageOrEnumType}.
 * @param ctx the parse tree
 */
fn enter_messageOrEnumType(&mut self, _ctx: &MessageOrEnumTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#messageOrEnumType}.
 * @param ctx the parse tree
 */
fn exit_messageOrEnumType(&mut self, _ctx: &MessageOrEnumTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#emptyStatement}.
 * @param ctx the parse tree
 */
fn enter_emptyStatement(&mut self, _ctx: &EmptyStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#emptyStatement}.
 * @param ctx the parse tree
 */
fn exit_emptyStatement(&mut self, _ctx: &EmptyStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link Loki_protoParser#constant}.
 * @param ctx the parse tree
 */
fn enter_constant(&mut self, _ctx: &ConstantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Loki_protoParser#constant}.
 * @param ctx the parse tree
 */
fn exit_constant(&mut self, _ctx: &ConstantContext<'input>) { }

}
