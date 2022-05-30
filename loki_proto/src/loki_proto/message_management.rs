

use super::proto_message::*;
use super::proto_visitor::*;


#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct MsgManagement {
    proto_names: Vec<String>,
    proto_enums: Vec<ProtoEnum>,
    proto_messages: Vec<ProtoMessage>,
    visitors: Vec<Protovisitor>,
}

#[allow(dead_code)]
impl MsgManagement {


    pub fn get_proto_names(&self) -> &Vec<String> {
        &self.proto_names
    }

    pub fn get_mut_proto_names(&mut self) -> &mut Vec<String> {
        &mut self.proto_names
    }

    pub fn get_proto_names_len(&self) -> usize {
        self.proto_names.len()
    }

    pub fn set_proto_names(&mut self, proto_names: Vec<String>) -> bool {
        self.proto_names = proto_names;
        true
    }

    pub fn push_proto_name(&mut self, proto_name: String) -> bool {
        self.proto_names.push(proto_name);
        true
    }

    pub fn push_proto_names(&mut self, proto_names: Vec<String>) -> bool {
        for proto_name in proto_names{
            self.proto_names.push(proto_name);
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
        for proto_enum in proto_enums{
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
        for proto_message in proto_messages{
            self.proto_messages.push(proto_message);
        }
        true
    }

    pub fn push_proto_messages_with_path(&mut self, proto_messages: Vec<ProtoMessage>, path: String) -> bool {
        for mut proto_message in proto_messages{
            proto_message.set_path(path.clone());
            self.check_same_message_name(&mut proto_message);
            self.proto_messages.push(proto_message);
        }
        true
    }

    pub fn check_same_message_name(&mut self, msg: &mut ProtoMessage){
        for proto_message in self.get_mut_proto_messages(){
            if proto_message.get_name() == msg.get_name(){
                proto_message.set_same_name(true);
                msg.set_same_name(true);
            }
        }
    }

    pub fn get_visitors(&self) -> &Vec<Protovisitor> {
        &self.visitors
    }

    pub fn push_visitor(&mut self, visitor: Protovisitor) -> bool {

        self.push_proto_name(visitor.get_name());
        self.push_proto_enums(visitor.get_proto_enums().to_vec());
        self.push_proto_messages(visitor.get_proto_messages().to_vec());

        self.visitors.push(visitor);

        true
    }

    pub fn push_visitor_with_path(&mut self, visitor: Protovisitor, path: String) -> bool {

        self.push_proto_name(visitor.get_name());
        self.push_proto_enums(visitor.get_proto_enums().to_vec());
        self.push_proto_messages_with_path(visitor.get_proto_messages().to_vec(), path);

        self.visitors.push(visitor);

        true
    }


    pub fn print_status(&self) {
        print!("messages len is {}", self.proto_messages.len());
        println!("\tenum len is {}", self.proto_enums.len());
    }

}