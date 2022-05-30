#![allow(nonstandard_style)]
// Generated from Loki_spec.g4 by ANTLR 4.8
use super::loki_specparser::*;
use antlr_rust::tree::ParseTreeVisitor;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link Loki_specParser}.
 */
pub trait Loki_specVisitor<'input>: ParseTreeVisitor<'input, Loki_specParserContextType> {
    /**
     * Visit a parse tree produced by {@link Loki_specParser#document}.
     * @param ctx the parse tree
     */
    fn visit_document(&mut self, ctx: &DocumentContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#loki_begin}.
     * @param ctx the parse tree
     */
    fn visit_loki_begin(&mut self, ctx: &Loki_beginContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#loki_end}.
     * @param ctx the parse tree
     */
    fn visit_loki_end(&mut self, ctx: &Loki_endContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#spec_content}.
     * @param ctx the parse tree
     */
    fn visit_spec_content(&mut self, ctx: &Spec_contentContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#message}.
     * @param ctx the parse tree
     */
    fn visit_message(&mut self, ctx: &MessageContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#msg_begin}.
     * @param ctx the parse tree
     */
    fn visit_msg_begin(&mut self, ctx: &Msg_beginContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#msg_end}.
     * @param ctx the parse tree
     */
    fn visit_msg_end(&mut self, ctx: &Msg_endContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#msg_content}.
     * @param ctx the parse tree
     */
    fn visit_msg_content(&mut self, ctx: &Msg_contentContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#attribute}.
     * @param ctx the parse tree
     */
    fn visit_attribute(&mut self, ctx: &AttributeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#attr_begin}.
     * @param ctx the parse tree
     */
    fn visit_attr_begin(&mut self, ctx: &Attr_beginContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#attr_end}.
     * @param ctx the parse tree
     */
    fn visit_attr_end(&mut self, ctx: &Attr_endContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#attr_type}.
     * @param ctx the parse tree
     */
    fn visit_attr_type(&mut self, ctx: &Attr_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#ele_type}.
     * @param ctx the parse tree
     */
    fn visit_ele_type(&mut self, ctx: &Ele_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#reff}.
     * @param ctx the parse tree
     */
    fn visit_reff(&mut self, ctx: &ReffContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#size}.
     * @param ctx the parse tree
     */
    fn visit_size(&mut self, ctx: &SizeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#option}.
     * @param ctx the parse tree
     */
    fn visit_option(&mut self, ctx: &OptionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#key_type}.
     * @param ctx the parse tree
     */
    fn visit_key_type(&mut self, ctx: &Key_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#value_type}.
     * @param ctx the parse tree
     */
    fn visit_value_type(&mut self, ctx: &Value_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#mutator}.
     * @param ctx the parse tree
     */
    fn visit_mutator(&mut self, ctx: &MutatorContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#mutation}.
     * @param ctx the parse tree
     */
    fn visit_mutation(&mut self, ctx: &MutationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#param}.
     * @param ctx the parse tree
     */
    fn visit_param(&mut self, ctx: &ParamContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#algo}.
     * @param ctx the parse tree
     */
    fn visit_algo(&mut self, ctx: &AlgoContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#expectedMsg}.
     * @param ctx the parse tree
     */
    fn visit_expectedMsg(&mut self, ctx: &ExpectedMsgContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#exc_begin}.
     * @param ctx the parse tree
     */
    fn visit_exc_begin(&mut self, ctx: &Exc_beginContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Loki_specParser#exc_end}.
     * @param ctx the parse tree
     */
    fn visit_exc_end(&mut self, ctx: &Exc_endContext<'input>) {
        self.visit_children(ctx)
    }
}
