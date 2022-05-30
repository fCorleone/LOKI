// #![feature(try_blocks)]
// #[macro_use]
// use antlr_rust::common_token_stream::CommonTokenStream;
// use antlr_rust::input_stream::InputStream;
// use antlr_rust::token_factory::{ArenaCommonFactory, CommonTokenFactory, OwningTokenFactory};
// use antlr_rust::tree::Visitable;
use antlr_rust::tree::{ParseTree, ParseTreeVisitor};

use anyhow::{anyhow, Result};
use std::rc::Rc;

// use super::loki_speclexer::*;
// use super::loki_speclistener::*;
use super::loki_specparser::*;
use super::loki_specvisitor::*;
use super::message::*;

// mod loki_speclexer;
// mod loki_speclistener;
// mod loki_specparser;
// mod loki_specvisitor;
// mod message;

#[derive(Default)]
pub struct Specvisitor<'input> {
    specname: String,
    message_ctxs: Vec<Rc<MessageContextAll<'input>>>,
    message_list: Vec<Message>,
    msg_size: i32,
    current_msg: Message,
    current_index: i32,
}

#[allow(dead_code)]
impl<'input> Specvisitor<'input> {
    pub fn get_structure_from_msg_type(&self, _msg_type: String) -> Result<&Message> {
        for msg in &self.message_list {
            // println!("msg's name is : {}, _msg_type is {}", msg.get_name(), _msg_type);
            if msg.get_name() == _msg_type {
                return Ok(msg);
            }
        }
        return Err(anyhow!(
            "cannot find the message with _msg_type {}",
            _msg_type
        ));
    }

    pub fn get_clone_structure_from_msg_type(&self, _msg_type: String) -> Result<Message> {
        for msg in &self.message_list {
            // println!("msg's name is : {}, _msg_type is {}", msg.get_name(), _msg_type);
            if msg.get_name() == _msg_type {
                let new_msg = copy_message(msg);
                return Ok(new_msg);
            }
        }
        return Err(anyhow!(
            "cannot find the message with _msg_type {}",
            _msg_type
        ));
    }

    pub fn print_status(&self) {
        println!("spec messages len is {}", self.message_list.len());
    }

    pub fn get_spec_messages(&self) -> &Vec<Message> {
        &self.message_list
    }

    pub fn get_mut_spec_messages(&mut self) -> &mut Vec<Message> {
        &mut self.message_list
    }

    pub fn get_spec_messages_len(&self) -> usize {
        self.message_list.len()
    }

    pub fn set_spec_messages(&mut self, message_list: Vec<Message>) -> Result<bool> {
        self.message_list = message_list;
        Ok(true)
    }

    pub fn push_spec_message(&mut self, message_list: Message) -> Result<bool> {
        self.message_list.push(message_list);
        Ok(true)
    }

    pub fn push_spec_messages(&mut self, message_list: Vec<Message>) -> Result<bool> {
        for message in message_list {
            self.message_list.push(message);
        }
        Ok(true)
    }
}

impl<'i> ParseTreeVisitor<'i, Loki_specParserContextType> for Specvisitor<'i> {}

impl<'input> Loki_specVisitor<'input> for Specvisitor<'input> {
    fn visit_document(&mut self, ctx: &DocumentContext<'input>) {
        // println!("Hello, antlr4 visitor!");
        let _begin_ctx = ctx.loki_begin().expect("document parsed unsuccessfully");
        // println!("content = {}", begin_ctx.get_text());

        self.visit_children(ctx)
    }

    fn visit_loki_begin(&mut self, ctx: &Loki_beginContext<'input>) {
        let spec_name = ctx.STRING().expect("loki_begin parsed unsuccessfully");
        let s1 = spec_name.get_text();
        self.specname = (&s1[1..s1.len() - 1]).to_string();
        // println!("specname={}", self.specname);

        self.visit_children(ctx)
    }

    fn visit_loki_end(&mut self, ctx: &Loki_endContext<'input>) {
        // There's nothing to do right now
        self.visit_children(ctx)
    }

    fn visit_spec_content(&mut self, ctx: &Spec_contentContext<'input>) {
        let message_vec = ctx.message_all();
        // println!("number of size = {}", message_vec.len());
        self.msg_size = message_vec.len() as i32;
        self.current_index = 0;

        for message in message_vec {
            self.message_ctxs.push(message);
        }
        // println!("number of message_ctxs = {}", self.message_ctxs.len());
        // println!("number of message_list = {}", self.message_list.len());

        self.visit_children(ctx)
    }

    fn visit_message(&mut self, ctx: &MessageContext<'input>) {
        // current_index should never be equal or larger than msg_size
        if self.current_index >= self.msg_size {
            println!("Error in parsering messages: wrong message number!");
            todo!() //TODO
        }

        // println!("current_index is: {}", self.current_index);

        // 1. construct a new message
        self.current_msg = construct_message();

        // dfs visit other attribute
        self.visit_children(ctx);

        // finally, undate current_index
        self.current_index += 1;
    }

    fn visit_msg_begin(&mut self, ctx: &Msg_beginContext<'input>) {
        // 2. parse the message name
        let message_name = ctx.STRING().expect("msg_begin parsed unsuccessfully");
        let s1 = message_name.get_text();
        self.current_msg
            .set_name((&s1[1..s1.len() - 1]).to_string());
        // println!("message name = {}", self.current_msg.get_name());

        self.visit_children(ctx)
    }

    fn visit_msg_content(&mut self, ctx: &Msg_contentContext<'input>) {
        // 3. parse the message content
        // println!("current msg's ");
        self.visit_children(ctx)
    }

    fn visit_msg_end(&mut self, ctx: &Msg_endContext<'input>) {
        // 4. copy the current message and push it into message_list
        let msg = copy_message(&self.current_msg);
        // println!(
        //     "len of attributes is {}",
        //     self.current_msg.get_attrs().len()
        // );
        // println!(
        //     "len of expected message is {}",
        //     self.current_msg.get_expect_msgs().len()
        // );
        self.message_list.push(msg);

        self.visit_children(ctx)
    }

    fn visit_attribute(&mut self, ctx: &AttributeContext<'input>) {
        // 3.1 parse the attribute first

        self.visit_children(ctx)
    }

    fn visit_attr_begin(&mut self, ctx: &Attr_beginContext<'input>) {
        let mut attr_tmp = construct_attribute();

        // 3.1.1 parse the  Name Euqal STRING Type Euqal attr_type reff* size* mutator* param* algo*
        let name_str = ctx.STRING().expect("attr_begin name parsed unsuccessfully");
        let s1 = name_str.get_text();
        attr_tmp.set_attr_name((&s1[1..s1.len() - 1]).to_string());
        // println!("attr_name = {}", attr_tmp.get_attr_name());

        let type_str = ctx
            .attr_type()
            .expect("attr_begin type parsed unsuccessfully");
        attr_tmp.set_attr_type(type_str.get_text());
        // println!("attr_type = {}", attr_tmp.get_attr_type());

        let ele_types = ctx.ele_type_all(); //TODO ele_type should be 0 or 1, not a vec
        if ele_types.len() > 0 {
            let ele_type_str = ctx
                .ele_type(0)
                .expect("attr_begin ele_type parsed unsuccessfully");

            let s1 = ele_type_str.get_text();
            attr_tmp.set_attr_ele_type((&s1[10..s1.len() - 1]).to_string());
            // println!("attr_ele_type = {}", attr_tmp.get_attr_ele_type());
        }

        let reffs = ctx.reff_all(); //TODO ref should be 0 or 1, not a vec
        if reffs.len() > 0 {
            let ref_str = ctx.reff(0).expect("attr_begin ref parsed unsuccessfully");

            let s1 = ref_str.get_text();
            attr_tmp.set_attr_reff((&s1[5..s1.len() - 1]).to_string());
            // println!("attr_ref = {}", attr_tmp.get_attr_reff());
        }

        let size = ctx.size_all(); //TODO size should be 0 or 1, not a vec
        if size.len() > 0 {
            let size_str = ctx.size(0).expect("attr_begin ref parsed unsuccessfully");

            let s1 = size_str.get_text();
            attr_tmp.set_attr_size((&s1[6..s1.len() - 1]).to_string());
            // println!("attr_size = {}", attr_tmp.get_attr_size());
        }

        let option = ctx.option_all(); //TODO ref should be 0 or 1, not a vec
        if option.len() > 0 {
            let option_str = ctx.option(0).expect("attr_begin ref parsed unsuccessfully");

            let s1 = option_str.get_text();
            attr_tmp.set_attr_option((&s1[8..s1.len() - 1]).to_string());
            // println!("attr_option = {}", attr_tmp.get_attr_option());
        }

        let key_type = ctx.key_type_all(); //TODO ref should be 0 or 1, not a vec
        if key_type.len() > 0 {
            let key_type_str = ctx
                .key_type(0)
                .expect("attr_begin ref parsed unsuccessfully");

            let s1 = key_type_str.get_text();
            attr_tmp.set_attr_key_type((&s1[10..s1.len() - 1]).to_string());
            // println!("attr_key_type = {}", attr_tmp.get_attr_key_type());
        }

        let value_type = ctx.value_type_all(); //TODO ref should be 0 or 1, not a vec
        if value_type.len() > 0 {
            let value_type_str = ctx
                .value_type(0)
                .expect("attr_begin ref parsed unsuccessfully");

            let s1 = value_type_str.get_text();
            attr_tmp.set_attr_value_type((&s1[12..s1.len() - 1]).to_string());
            // println!("attr_value_type = {}", attr_tmp.get_attr_value_type());
        }

        let mutator = ctx.mutator_all(); //TODO ref should be 0 or 1, not a vec
        if mutator.len() > 0 {
            let mutator_str = ctx
                .mutator(0)
                .expect("attr_begin ref parsed unsuccessfully");
            // attr_tmp.set_attr_mutator(mutator_str.get_text());

            let s1 = mutator_str.get_text();
            attr_tmp.set_attr_mutator((&s1[9..s1.len() - 1]).to_string());

            // println!("attr_mutator = {}", attr_tmp.get_attr_mutator());
        }

        let param = ctx.param_all(); //TODO ref should be 0 or 1, not a vec
        if param.len() > 0 {
            let param_str = ctx.param(0).expect("attr_begin ref parsed unsuccessfully");
            // attr_tmp.set_attr_param(param_str.get_text());

            let s1 = param_str.get_text();
            attr_tmp.set_attr_param((&s1[7..s1.len() - 1]).to_string());
            // println!("attr_param = {}", attr_tmp.get_attr_param());
        }

        let algo = ctx.algo_all(); //TODO ref should be 0 or 1, not a vec
        if algo.len() > 0 {
            let algo_str = ctx.algo(0).expect("attr_begin ref parsed unsuccessfully");
            // attr_tmp.set_attr_algo(algo_str.get_text());

            let s1 = algo_str.get_text();
            attr_tmp.set_attr_algo((&s1[6..s1.len() - 1]).to_string());
            // println!("attr_algo = {}", attr_tmp.get_attr_algo());
        }

        // 3.1.2 copy the  attribute and push it into current_msg.attrs
        self.current_msg.get_mut_attrs().push(attr_tmp);

        // self.visit_children(ctx)
    }

    fn visit_attr_end(&mut self, ctx: &Attr_endContext<'input>) {
        // There's nothing to do right now

        self.visit_children(ctx)
    }

    fn visit_attr_type(&mut self, ctx: &Attr_typeContext<'input>) {
        // There's nothing to do right now

        self.visit_children(ctx)
    }

    fn visit_ele_type(&mut self, ctx: &Ele_typeContext<'input>) {
        // There's nothing to do right now

        self.visit_children(ctx)
    }

    fn visit_reff(&mut self, ctx: &ReffContext<'input>) {
        // There's nothing to do right now

        self.visit_children(ctx)
    }

    fn visit_size(&mut self, ctx: &SizeContext<'input>) {
        // There's nothing to do right now

        self.visit_children(ctx)
    }

    fn visit_mutator(&mut self, ctx: &MutatorContext<'input>) {
        // There's nothing to do right now

        self.visit_children(ctx)
    }

    fn visit_mutation(&mut self, ctx: &MutationContext<'input>) {
        // There's nothing to do right now

        self.visit_children(ctx)
    }

    fn visit_param(&mut self, ctx: &ParamContext<'input>) {
        // There's nothing to do right now

        self.visit_children(ctx)
    }

    fn visit_algo(&mut self, ctx: &AlgoContext<'input>) {
        // There's nothing to do right now

        self.visit_children(ctx)
    }

    fn visit_expectedMsg(&mut self, ctx: &ExpectedMsgContext<'input>) {
        // 3.2 parse the expectedMsg then

        self.visit_children(ctx)
    }

    fn visit_exc_begin(&mut self, ctx: &Exc_beginContext<'input>) {
        let mut expect_msg = construct_expected_msg();

        // 3.1.1 parse the  Name Euqal STRING 'identify' Euqal STRING 'excptedIdentify' Euqal STRING;

        if ctx.STRING_all().len() < 3 {
            //TODO
            println!(
                "Error in parsing expected message! len of ctx.STRING_all() is: {}",
                ctx.STRING_all().len()
            );

            println!(
                "{}",
                ctx.STRING(0)
                    .expect("exc_begin expect_name parsed unsuccessfully")
                    .get_text()
            );
            println!(
                "{}",
                ctx.STRING(1)
                    .expect("exc_begin expect_name parsed unsuccessfully")
                    .get_text()
            );
            println!(
                "{}",
                ctx.STRING(2)
                    .expect("exc_begin expect_name parsed unsuccessfully")
                    .get_text()
            );
            // println!("{}", ctx.STRING(3).expect("exc_begin expect_name parsed unsuccessfully").get_text());
        } else {
            let expect_name = ctx
                .STRING(0)
                .expect("exc_begin expect_name parsed unsuccessfully");

            let s1 = expect_name.get_text();
            expect_msg.set_expect_name((&s1[1..s1.len() - 1]).to_string());
            // println!("expected name = {}", expect_msg.get_expect_name());

            let identify = ctx
                .STRING(1)
                .expect("attr_begin identify parsed unsuccessfully");

            let s1 = identify.get_text();
            expect_msg.set_identify((&s1[1..s1.len() - 1]).to_string());
            // println!("identify = {}", expect_msg.get_identify());

            let expect_identify = ctx
                .STRING(2)
                .expect("attr_begin expect_identify parsed unsuccessfully");

            let s1 = expect_identify.get_text();
            expect_msg.set_expect_identify((&s1[1..s1.len() - 1]).to_string());
            // println!("expected identify = {}", expect_msg.get_expect_identify());

            // 3.1.2 copy the expectedMsg and push it into current_msg.attrs
            self.current_msg.get_mut_expect_msgs().push(expect_msg);
        }

        self.visit_children(ctx)
    }

    fn visit_exc_end(&mut self, ctx: &Exc_endContext<'input>) {
        // There's nothing to do right now

        self.visit_children(ctx)
    }
}
