#![allow(nonstandard_style)]
// Generated from Loki_spec.g4 by ANTLR 4.8
use super::loki_specparser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait Loki_specListener<'input>: ParseTreeListener<'input, Loki_specParserContextType> {
    /**
     * Enter a parse tree produced by {@link Loki_specParser#document}.
     * @param ctx the parse tree
     */
    fn enter_document(&mut self, _ctx: &DocumentContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#document}.
     * @param ctx the parse tree
     */
    fn exit_document(&mut self, _ctx: &DocumentContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#loki_begin}.
     * @param ctx the parse tree
     */
    fn enter_loki_begin(&mut self, _ctx: &Loki_beginContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#loki_begin}.
     * @param ctx the parse tree
     */
    fn exit_loki_begin(&mut self, _ctx: &Loki_beginContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#loki_end}.
     * @param ctx the parse tree
     */
    fn enter_loki_end(&mut self, _ctx: &Loki_endContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#loki_end}.
     * @param ctx the parse tree
     */
    fn exit_loki_end(&mut self, _ctx: &Loki_endContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#spec_content}.
     * @param ctx the parse tree
     */
    fn enter_spec_content(&mut self, _ctx: &Spec_contentContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#spec_content}.
     * @param ctx the parse tree
     */
    fn exit_spec_content(&mut self, _ctx: &Spec_contentContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#message}.
     * @param ctx the parse tree
     */
    fn enter_message(&mut self, _ctx: &MessageContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#message}.
     * @param ctx the parse tree
     */
    fn exit_message(&mut self, _ctx: &MessageContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#msg_begin}.
     * @param ctx the parse tree
     */
    fn enter_msg_begin(&mut self, _ctx: &Msg_beginContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#msg_begin}.
     * @param ctx the parse tree
     */
    fn exit_msg_begin(&mut self, _ctx: &Msg_beginContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#msg_end}.
     * @param ctx the parse tree
     */
    fn enter_msg_end(&mut self, _ctx: &Msg_endContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#msg_end}.
     * @param ctx the parse tree
     */
    fn exit_msg_end(&mut self, _ctx: &Msg_endContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#msg_content}.
     * @param ctx the parse tree
     */
    fn enter_msg_content(&mut self, _ctx: &Msg_contentContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#msg_content}.
     * @param ctx the parse tree
     */
    fn exit_msg_content(&mut self, _ctx: &Msg_contentContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#attribute}.
     * @param ctx the parse tree
     */
    fn enter_attribute(&mut self, _ctx: &AttributeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#attribute}.
     * @param ctx the parse tree
     */
    fn exit_attribute(&mut self, _ctx: &AttributeContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#attr_begin}.
     * @param ctx the parse tree
     */
    fn enter_attr_begin(&mut self, _ctx: &Attr_beginContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#attr_begin}.
     * @param ctx the parse tree
     */
    fn exit_attr_begin(&mut self, _ctx: &Attr_beginContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#attr_end}.
     * @param ctx the parse tree
     */
    fn enter_attr_end(&mut self, _ctx: &Attr_endContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#attr_end}.
     * @param ctx the parse tree
     */
    fn exit_attr_end(&mut self, _ctx: &Attr_endContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#attr_type}.
     * @param ctx the parse tree
     */
    fn enter_attr_type(&mut self, _ctx: &Attr_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#attr_type}.
     * @param ctx the parse tree
     */
    fn exit_attr_type(&mut self, _ctx: &Attr_typeContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#ele_type}.
     * @param ctx the parse tree
     */
    fn enter_ele_type(&mut self, _ctx: &Ele_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#ele_type}.
     * @param ctx the parse tree
     */
    fn exit_ele_type(&mut self, _ctx: &Ele_typeContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#reff}.
     * @param ctx the parse tree
     */
    fn enter_reff(&mut self, _ctx: &ReffContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#reff}.
     * @param ctx the parse tree
     */
    fn exit_reff(&mut self, _ctx: &ReffContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#size}.
     * @param ctx the parse tree
     */
    fn enter_size(&mut self, _ctx: &SizeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#size}.
     * @param ctx the parse tree
     */
    fn exit_size(&mut self, _ctx: &SizeContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#option}.
     * @param ctx the parse tree
     */
    fn enter_option(&mut self, _ctx: &OptionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#option}.
     * @param ctx the parse tree
     */
    fn exit_option(&mut self, _ctx: &OptionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#key_type}.
     * @param ctx the parse tree
     */
    fn enter_key_type(&mut self, _ctx: &Key_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#key_type}.
     * @param ctx the parse tree
     */
    fn exit_key_type(&mut self, _ctx: &Key_typeContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#value_type}.
     * @param ctx the parse tree
     */
    fn enter_value_type(&mut self, _ctx: &Value_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#value_type}.
     * @param ctx the parse tree
     */
    fn exit_value_type(&mut self, _ctx: &Value_typeContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#mutator}.
     * @param ctx the parse tree
     */
    fn enter_mutator(&mut self, _ctx: &MutatorContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#mutator}.
     * @param ctx the parse tree
     */
    fn exit_mutator(&mut self, _ctx: &MutatorContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#mutation}.
     * @param ctx the parse tree
     */
    fn enter_mutation(&mut self, _ctx: &MutationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#mutation}.
     * @param ctx the parse tree
     */
    fn exit_mutation(&mut self, _ctx: &MutationContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#param}.
     * @param ctx the parse tree
     */
    fn enter_param(&mut self, _ctx: &ParamContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#param}.
     * @param ctx the parse tree
     */
    fn exit_param(&mut self, _ctx: &ParamContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#algo}.
     * @param ctx the parse tree
     */
    fn enter_algo(&mut self, _ctx: &AlgoContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#algo}.
     * @param ctx the parse tree
     */
    fn exit_algo(&mut self, _ctx: &AlgoContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#expectedMsg}.
     * @param ctx the parse tree
     */
    fn enter_expectedMsg(&mut self, _ctx: &ExpectedMsgContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#expectedMsg}.
     * @param ctx the parse tree
     */
    fn exit_expectedMsg(&mut self, _ctx: &ExpectedMsgContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#exc_begin}.
     * @param ctx the parse tree
     */
    fn enter_exc_begin(&mut self, _ctx: &Exc_beginContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#exc_begin}.
     * @param ctx the parse tree
     */
    fn exit_exc_begin(&mut self, _ctx: &Exc_beginContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link Loki_specParser#exc_end}.
     * @param ctx the parse tree
     */
    fn enter_exc_end(&mut self, _ctx: &Exc_endContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link Loki_specParser#exc_end}.
     * @param ctx the parse tree
     */
    fn exit_exc_end(&mut self, _ctx: &Exc_endContext<'input>) {}
}
