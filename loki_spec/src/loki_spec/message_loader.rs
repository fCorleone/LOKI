use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::tree::Visitable;

use std::fs;
use std::path::PathBuf;

use walkdir::WalkDir;

use loki_proto::loki_proto::loki_protolexer::*;
use loki_proto::loki_proto::loki_protoparser::*;
use loki_proto::loki_proto::message_management::*;
use loki_proto::loki_proto::proto_message::*;
use loki_proto::loki_proto::proto_visitor::*;

use crate::loki_spec::loki_speclexer::*;
use crate::loki_spec::loki_specparser::*;
use crate::loki_spec::message::*;
use crate::loki_spec::spec_visitor::*;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct MsgLoader {
    all_messages: Vec<Message>,
    spec_messages: Vec<Message>,
    proto_enums: Vec<ProtoEnum>,
    proto_messages: Vec<ProtoMessage>,
    visitors: Vec<Protovisitor>,
}

#[allow(dead_code)]
impl MsgLoader {
    // 1. parse & load proto messages from proto_path(dir)
    pub fn load_proto_messages(&mut self, proto_path: PathBuf) {
        let mut msg_management = MsgManagement::default();

        for entry in WalkDir::new(proto_path.as_os_str())
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.file_name().to_string_lossy();
            let _f_path = entry.path().display();
            if f_name.ends_with(".proto") {
                // println!("{} : {}", f_name, _f_path);

                let input_str = fs::read_to_string(entry.path()).unwrap();
                let input = InputStream::new(&*input_str);
                let tf = CommonTokenFactory::default();
                let lexer = Loki_protoLexer::new_with_token_factory(input, &tf);
                let token_source = CommonTokenStream::new(lexer);
                let mut parser = Loki_protoParser::new(token_source);
                let result = parser.proto();

                let result_ctx = result.expect("input parsed unsuccessfully");

                let mut visitor = Protovisitor::default();
                result_ctx.accept(&mut visitor);

                msg_management.push_visitor(visitor);
            }
        }

        self.set_visitors(msg_management.get_visitors().to_vec());
        msg_management.print_status();

        self.set_proto_enums(msg_management.get_proto_enums().to_vec());
        self.set_proto_messages(msg_management.get_proto_messages().to_vec());
        self.print_status();
    }

    pub fn load_proto_messages_with_path(&mut self, proto_path: PathBuf) {
        let pre_index = proto_path.display().to_string().len();

        let mut msg_management = MsgManagement::default();

        for entry in WalkDir::new(proto_path.as_os_str())
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.file_name().to_string_lossy();
            let f_path = entry.path().display().to_string();
            if f_name.ends_with(".proto") {
                // println!("{} : {}", f_name, _f_path);

                let input_str = fs::read_to_string(entry.path()).unwrap();
                let input = InputStream::new(&*input_str);
                let tf = CommonTokenFactory::default();
                let lexer = Loki_protoLexer::new_with_token_factory(input, &tf);
                let token_source = CommonTokenStream::new(lexer);
                let mut parser = Loki_protoParser::new(token_source);
                let result = parser.proto();

                let result_ctx = result.expect("input parsed unsuccessfully");

                let mut visitor = Protovisitor::default();
                visitor.set_name((&f_path[pre_index + 1..f_path.len()]).to_string());
                result_ctx.accept(&mut visitor);

                msg_management.push_visitor_with_path(
                    visitor,
                    (&f_path[pre_index + 1..f_path.len() - 6]).to_string(),
                );
            }
        }
        self.set_visitors(msg_management.get_visitors().to_vec());
        msg_management.print_status();

        self.set_proto_enums(msg_management.get_proto_enums().to_vec());
        self.set_proto_messages(msg_management.get_proto_messages().to_vec());
        self.print_status();

        // for msg in msg_management.get_mut_proto_messages(){
        //     println!("name is {}, path is {}", msg.get_name(), msg.get_path());
        // }
    }

    // 2. parse & load loki spec from spec_path
    pub fn load_spec_message(&mut self, spec_path: PathBuf) {
        let input_str = fs::read_to_string(spec_path).unwrap();

        let input = InputStream::new(&*input_str);
        let tf = CommonTokenFactory::default();
        let lexer = Loki_specLexer::new_with_token_factory(input, &tf);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = Loki_specParser::new(token_source);
        let result = parser.document();

        let result_ctx = result.expect("input parsed unsuccessfully");

        let mut visitor = Specvisitor::default();
        result_ctx.accept(&mut visitor);

        visitor.print_status();

        self.set_spec_messages(visitor.get_spec_messages().to_vec());
        self.print_status();
    }

    // 3. merge syntax(proto_message) with senmatic(spec_message)
    pub fn merge(&mut self) {
        for proto_message in &self.proto_messages {
            //3.1 convert proto_message to spec_message
            let mut msg = message_convert_proto_to_spec(proto_message);
            msg.set_name(proto_message.get_name());

            // 3.2 add spec info into msg
            add_spec_info(&mut msg, &self.spec_messages);

            self.all_messages.push(msg);
        }

        self.print_status();
    }

    pub fn get_all_messages(&self) -> &Vec<Message> {
        &self.all_messages
    }

    pub fn get_mut_all_messages(&mut self) -> &mut Vec<Message> {
        &mut self.all_messages
    }

    pub fn get_all_messages_len(&self) -> usize {
        self.all_messages.len()
    }

    pub fn set_all_messages(&mut self, all_messages: Vec<Message>) -> bool {
        self.all_messages = all_messages;
        true
    }

    pub fn push_all_message(&mut self, all_message: Message) -> bool {
        self.all_messages.push(all_message);
        true
    }

    pub fn push_all_messages(&mut self, all_messages: Vec<Message>) -> bool {
        for all_message in all_messages {
            self.all_messages.push(all_message);
        }
        true
    }

    pub fn get_spec_messages(&self) -> &Vec<Message> {
        &self.spec_messages
    }

    pub fn get_mut_spec_messages(&mut self) -> &mut Vec<Message> {
        &mut self.spec_messages
    }

    pub fn get_spec_messages_len(&self) -> usize {
        self.spec_messages.len()
    }

    pub fn set_spec_messages(&mut self, spec_messages: Vec<Message>) -> bool {
        self.spec_messages = spec_messages;
        true
    }

    pub fn push_spec_message(&mut self, spec_message: Message) -> bool {
        self.spec_messages.push(spec_message);
        true
    }

    pub fn push_spec_messages(&mut self, spec_messages: Vec<Message>) -> bool {
        for spec_message in spec_messages {
            self.spec_messages.push(spec_message);
        }
        true
    }

    pub fn get_proto_enums(&self) -> &Vec<ProtoEnum> {
        &self.proto_enums
    }

    pub fn get_mut_proto_enums(&mut self) -> &mut Vec<ProtoEnum> {
        &mut self.proto_enums
    }

    pub fn get_proto_enums_len(&self) -> usize {
        self.proto_enums.len()
    }

    pub fn set_proto_enums(&mut self, proto_enums: Vec<ProtoEnum>) -> bool {
        self.proto_enums = proto_enums;
        true
    }

    pub fn push_proto_enum(&mut self, proto_enum: ProtoEnum) -> bool {
        self.proto_enums.push(proto_enum);
        true
    }

    pub fn push_proto_enums(&mut self, proto_enums: Vec<ProtoEnum>) -> bool {
        for proto_enum in proto_enums {
            self.proto_enums.push(proto_enum);
        }
        true
    }

    pub fn get_proto_messages(&self) -> &Vec<ProtoMessage> {
        &self.proto_messages
    }

    pub fn get_mut_proto_messages(&mut self) -> &mut Vec<ProtoMessage> {
        &mut self.proto_messages
    }

    pub fn get_proto_messages_len(&self) -> usize {
        self.proto_messages.len()
    }

    pub fn set_proto_messages(&mut self, proto_messages: Vec<ProtoMessage>) -> bool {
        self.proto_messages = proto_messages;
        true
    }

    pub fn push_proto_message(&mut self, proto_message: ProtoMessage) -> bool {
        self.proto_messages.push(proto_message);
        true
    }

    pub fn push_proto_messages(&mut self, proto_messages: Vec<ProtoMessage>) -> bool {
        for proto_message in proto_messages {
            self.proto_messages.push(proto_message);
        }
        true
    }

    pub fn get_visitors(&self) -> &Vec<Protovisitor> {
        &self.visitors
    }

    pub fn set_visitors(&mut self, visitors: Vec<Protovisitor>) -> bool {
        self.visitors = visitors;
        true
    }

    pub fn push_visitor(&mut self, visitor: Protovisitor) -> bool {
        self.visitors.push(visitor);
        true
    }

    pub fn print_status(&self) {
        print!("proto_messages len is {}", self.proto_messages.len());
        print!("\tproto_enums len is {}", self.proto_enums.len());
        print!("\tspec_messages len is {}", self.spec_messages.len());
        println!("\tall_messages len is {}", self.all_messages.len());
    }
}

#[allow(unused_must_use)]
pub fn message_convert_proto_to_spec(proto_message: &ProtoMessage) -> Message {
    let mut ret = Message::default();
    ret.set_name(proto_message.get_name());

    // for i in 0..proto_message.attrs.len() {
    //     let attr = if let Some(attr) = proto_message.attrs.get(i) {
    //         attr
    //     } else {
    //         todo!() //TODO
    //     };
    //     ret.attrs.push(attribute_convert_proto_to_spec(attr));
    // }

    for attr in proto_message.get_attrs() {
        let attr = attribute_convert_proto_to_spec(attr);

        ret.push_attr(attr);
    }

    return ret;
}

pub fn attribute_convert_proto_to_spec(proto_attr: &ProtoAttribute) -> Attribute {
    let mut ret = Attribute::default();
    ret.set_attr_name(proto_attr.get_attr_name());
    ret.set_attr_type(proto_attr.get_attr_type());
    ret.set_attr_ele_type(proto_attr.get_attr_ele_type());
    ret.set_attr_reff(proto_attr.get_attr_reff());
    ret.set_attr_size(proto_attr.get_attr_size());
    ret.set_attr_option(proto_attr.get_options());
    ret.set_attr_key_type(proto_attr.get_key_type());
    ret.set_attr_value_type(proto_attr.get_value_type());
    return ret;
}

#[allow(unused_must_use)]
pub fn add_spec_info(msg: &mut Message, messages: &Vec<Message>) {
    for spec_msg in messages {
        if spec_msg.get_name() == msg.get_name() {
            for attr in msg.get_mut_attrs() {
                for spec_attr in spec_msg.get_attrs() {
                    if attr.get_attr_name() == spec_attr.get_attr_name() {
                        attr.set_attr_mutator(spec_attr.get_attr_mutator());
                        attr.set_attr_param(spec_attr.get_attr_param());
                        attr.set_attr_algo(spec_attr.get_attr_algo());
                    }
                }
            }

            for expect_msg in spec_msg.get_expect_msgs() {
                msg.push_expect_msg(copy_expected_msg(expect_msg));
            }
        }
    }
}
