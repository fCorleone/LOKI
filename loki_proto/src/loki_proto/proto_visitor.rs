use antlr_rust::tree::{ParseTree, ParseTreeVisitor};

// use std::rc::Rc;
use anyhow::{anyhow, Result};

use super::loki_protoparser::*;
use super::loki_protovisitor::*;
use super::proto_message::*;

#[allow(dead_code)]
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Protovisitor {
    proto_name: String,
    proto_enums: Vec<ProtoEnum>,
    proto_messages: Vec<ProtoMessage>,
    prefix: String,
    imports: Vec<String>,
}

#[allow(dead_code)]
impl Protovisitor {
    pub fn get_structure_from_msg_type(&self, _msg_type: String) -> Result<&ProtoMessage> {
        for msg in &self.proto_messages {
            if msg.get_name() == _msg_type {
                return Ok(msg);
            }
        }
        return Err(anyhow!(
            "cannot find the message with _msg_type {}",
            _msg_type
        ));
    }

    pub fn get_clone_structure_from_msg_type(&self, _msg_type: String) -> Result<ProtoMessage> {
        for msg in &self.proto_messages {
            if msg.get_name() == _msg_type {
                let new_msg = copy_protomessage(msg);
                return Ok(new_msg);
            }
        }
        return Err(anyhow!(
            "cannot find the message with _msg_type {}",
            _msg_type
        ));
    }

    pub fn print_status(&self) {
        print!("messages len is {}", self.proto_messages.len());
        // println!("\t attr len is {}", self.proto_messages[self.proto_messages.len()-1].get_attrs().len());

        // print!("enum len is {}", self.proto_enums.len());
        // println!("\t attr len is {}", self.proto_enums[self.proto_enums.len()-1].get_attrs().len());
        println!("\tenum len is {}", self.proto_enums.len());
    }

    pub fn get_name(&self) -> String {
        self.proto_name.clone()
    }

    pub fn set_name(&mut self, name: String) -> bool {
        self.proto_name = name;
        true
    }

    pub fn get_imports(&self) -> &Vec<String> {
        &self.imports
    }

    pub fn get_mut_imports(&mut self) -> &mut Vec<String> {
        &mut self.imports
    }

    pub fn set_imports(&mut self, imports: Vec<String>) -> bool {
        self.imports = imports;
        true
    }

    pub fn push_import(&mut self, import: String) -> bool {
        self.imports.push(import);
        true
    }

    pub fn get_proto_enums(&self) -> &Vec<ProtoEnum> {
        &self.proto_enums
    }

    pub fn get_mut_proto_enums(&mut self) -> &mut Vec<ProtoEnum> {
        &mut self.proto_enums
    }

    pub fn get_proto_names_len(&self) -> usize {
        self.proto_enums.len()
    }

    pub fn set_proto_enums(&mut self, proto_enums: Vec<ProtoEnum>) -> Result<bool> {
        self.proto_enums = proto_enums;
        Ok(true)
    }

    pub fn push_proto_enum(&mut self, proto_enum: ProtoEnum) -> Result<bool> {
        self.proto_enums.push(proto_enum);
        Ok(true)
    }

    pub fn push_proto_enums(&mut self, proto_enums: Vec<ProtoEnum>) -> Result<bool> {
        for proto_enum in proto_enums {
            self.proto_enums.push(proto_enum);
        }
        Ok(true)
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

    pub fn set_proto_messages(&mut self, proto_messages: Vec<ProtoMessage>) -> Result<bool> {
        self.proto_messages = proto_messages;
        Ok(true)
    }

    pub fn push_proto_message(&mut self, proto_message: ProtoMessage) -> Result<bool> {
        self.proto_messages.push(proto_message);
        Ok(true)
    }

    pub fn push_proto_messages(&mut self, proto_messages: Vec<ProtoMessage>) -> Result<bool> {
        for proto_message in proto_messages {
            self.proto_messages.push(proto_message);
        }
        Ok(true)
    }
}

impl<'i> ParseTreeVisitor<'i, Loki_protoParserContextType> for Protovisitor {}

#[allow(unused_must_use)]
impl<'input> Loki_protoVisitor<'input> for Protovisitor {
    fn visit_message(&mut self, ctx: &MessageContext<'input>) {
        let mut tmp_msg = construct_protomessage();

        let ctx_msg_name = ctx
            .messageName()
            .expect("messageName parsed unsuccessfully");
        if self.prefix == "" {
            // println!("message name is {}", ctx_msg_name.get_text());
            tmp_msg.set_name(ctx_msg_name.get_text());
        } else {
            let mut s = self.prefix.clone();
            s += ".";
            s += &ctx_msg_name.get_text();
            // println!("message name is {}", s);
            tmp_msg.set_name(s);
        }

        let ctx_msg_body = ctx
            .messageBody()
            .expect("messageBody parsed unsuccessfully");

        //1. first deal with field_all
        // println!("Attribute list is: ");
        let ctx_field_all = ctx_msg_body.field_all();
        for ctx_field in ctx_field_all {
            // println!("field is : {}", ctx_field.get_text());

            let mut attr_tmp = construct_proto_attribute();

            let ctx_field_name = ctx_field
                .fieldName()
                .expect("fieldName parsed unsuccessfully");
            attr_tmp.set_attr_name(ctx_field_name.get_text());
            // print!("\t attrName is {}", ctx_field_name.get_text());

            let ctx_field_number = ctx_field
                .fieldNumber()
                .expect("fieldNumber parsed unsuccessfully");
            attr_tmp.set_attr_value(ctx_field_number.get_text());
            // print!("\t attrValue is {}", ctx_field_number.get_text());

            let ctx_ele_type = ctx_field.eleType().expect("eletype parsed unsuccessfully");

            let ctx_repeated = ctx_field.REPEATED();
            match ctx_repeated {
                None => {
                    match ctx_ele_type.messageOrEnumType() {
                        None => {
                            // println!("\t attrType is {}", ctx_ele_type.get_text());
                            attr_tmp.set_attr_type(ctx_ele_type.get_text());
                        }
                        Some(ctx_message) => {
                            // println!("\t attrType is Struct \t ref is {}", ctx_message.get_text());
                            attr_tmp.set_attr_type("Struct".to_string());
                            attr_tmp.set_attr_reff(ctx_message.get_text());
                        }
                    }
                }
                Some(_ctx_repeated) => {
                    // print!("\t attrType is Array");
                    attr_tmp.set_attr_type("Array".to_string());

                    match ctx_ele_type.messageOrEnumType() {
                        None => {
                            // println!("\t eletype is {}", ctx_ele_type.get_text());
                            attr_tmp.set_attr_ele_type(ctx_ele_type.get_text());
                        }
                        Some(ctx_message) => {
                            // println!("\t eletype is Struct \t ref is {}", ctx_message.get_text());
                            attr_tmp.set_attr_ele_type("Struct".to_string());
                            attr_tmp.set_attr_reff(ctx_message.get_text());
                        }
                    }
                }
            }

            tmp_msg.push_attr(attr_tmp);
        }

        //2. secondly, deal with enumDefinition message by prefix
        let ctx_message_all = ctx_msg_body.message_all();
        for _ctx_message in ctx_message_all {
            //TODO{};
        }

        //3. thirdly, ignore option, parse oneof
        let ctx_oneof_all = ctx_msg_body.oneof_all();
        for ctx_oneof in ctx_oneof_all {
            // println!("oneof is : {}", ctx_oneof.get_text());

            let mut attr_tmp = construct_proto_attribute();

            attr_tmp.set_attr_type("Oneof".to_string());

            let ctx_oneof_name = ctx_oneof
                .oneofName()
                .expect("oneofName parsed unsuccessfully");
            attr_tmp.set_attr_name(ctx_oneof_name.get_text());
            // print!("\t oneofName is {}", ctx_oneof_name.get_text());

            let mut s = "".to_string();

            let ctx_oneof_field_all = ctx_oneof.oneofField_all();
            for ctx_oneof_field in ctx_oneof_field_all {
                let ctx_ele_type = ctx_oneof_field
                    .eleType()
                    .expect("eletype parsed unsuccessfully");
                s += &ctx_ele_type.get_text();

                let ctx_field_name = ctx_oneof_field
                    .fieldName()
                    .expect("fieldName parsed unsuccessfully");
                s += " ";
                s += &ctx_field_name.get_text();

                let ctx_field_number = ctx_oneof_field
                    .fieldNumber()
                    .expect("fieldNumber parsed unsuccessfully");
                s += " ";
                s += &ctx_field_number.get_text();
                s += ",";
            }

            attr_tmp.set_options((s[0..s.len() - 1]).to_string());
            // println!("\t ref is {}", (s[0..s.len()-1]).to_string());

            tmp_msg.push_attr(attr_tmp);
        }

        //4. fourthly, parse mapField, ignore reserved and emptyStatement
        let ctx_mapfield_all = ctx_msg_body.mapField_all();
        for ctx_map in ctx_mapfield_all {
            // println!("map is : {}", ctx_map.get_text());

            let mut attr_tmp = construct_proto_attribute();
            attr_tmp.set_attr_type("Map".to_string());

            let ctx_map_name = ctx_map.mapName().expect("mapName parsed unsuccessfully");
            attr_tmp.set_attr_name(ctx_map_name.get_text());

            let ctx_key_type = ctx_map.keyType().expect("keyType parsed unsuccessfully");
            attr_tmp.set_key_type(ctx_key_type.get_text());

            let ctx_ele_type = ctx_map.eleType().expect("eleType parsed unsuccessfully");
            attr_tmp.set_value_type(ctx_ele_type.get_text());

            let ctx_field_number = ctx_map
                .fieldNumber()
                .expect("keyType parsed unsuccessfully");
            attr_tmp.set_attr_value(ctx_field_number.get_text());

            // println!("attrName is {}\t attrValue is {}\t attrType is Map\t key_type is {}\t value_type is{}", ctx_map_name.get_text(), ctx_field_number.get_text(), ctx_key_type.get_text(), ctx_ele_type.get_text());

            tmp_msg.push_attr(attr_tmp);
        }

        self.proto_messages.push(tmp_msg);
        // print!("messages len is {}", self.proto_messages.len());
        // println!("\t attr len is {}", self.proto_messages[self.proto_messages.len()-1].get_attrs().len());

        self.prefix = ctx_msg_name.get_text();
        self.visit_children(ctx);
        self.prefix = "".to_string();
    }

    fn visit_enumDefinition(&mut self, ctx: &EnumDefinitionContext<'input>) {
        let mut tmp_enum = construct_protoenum();

        let ctx_enum_name = ctx.enumName().expect("enumName parsed unsuccessfully");
        if self.prefix == "" {
            // println!("enum name is {}", ctx_enum_name.get_text());
            tmp_enum.set_name(ctx_enum_name.get_text());
        } else {
            let mut s = self.prefix.clone();
            s += ".";
            s += &ctx_enum_name.get_text();
            // println!("enum name is {}", s);
            tmp_enum.set_name(s);
        }

        let ctx_enum_body = ctx.enumBody().expect("enumBody parsed unsuccessfully");
        // TODD(): ignore option & emptyStatement, deal with enumField
        let ctx_enum_field_all = ctx_enum_body.enumField_all();
        for ctx_enum_field in ctx_enum_field_all {
            // println!("enum field is : {}", ctx_enum_field.get_text());

            // TODD(): ignore enumValueOption
            // let ctx_enumValueOption_all = ctx_enum_field.enumValueOption_all();
            // println!("enumValueOption size is : {}", ctx_enumValueOption_all.len());
            // for ctx_enumValueOption in ctx_enumValueOption_all{

            // }

            let mut attr_tmp = construct_proto_attribute();

            let attr_name = ctx_enum_field
                .Ident()
                .expect("enumField Ident parsed unsuccessfully");
            // println!("enum field name is : {}", attr_name.get_text());

            let attr_value = ctx_enum_field
                .IntLit()
                .expect("enumField Ident parsed unsuccessfully");
            // println!("enum field name is : {}", attr_value.get_text());

            attr_tmp.set_attr_name(attr_name.get_text());
            attr_tmp.set_attr_value(attr_value.get_text());
            tmp_enum.push_attr(attr_tmp);
        }

        self.proto_enums.push(tmp_enum);
        // println!("enum len is {}", self.proto_enums.len());
        // println!("attr len is {}", self.proto_enums[self.proto_enums.len()-1].get_attrs().len());

        self.prefix = ctx_enum_name.get_text();
        self.visit_children(ctx);
        self.prefix = "".to_string();
    }

    fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) {
        let ctx_str = ctx.StrLit().expect("Import statement parse error!");

        println!("{}", ctx_str.get_text());

        let import = ctx_str.get_text().to_string();
        self.push_import((&import[1..import.len() - 1]).to_string());

        self.visit_children(ctx)
    }
}
