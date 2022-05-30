#![feature(try_blocks)]
extern crate lazy_static;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::tree::Visitable;

use std::path::PathBuf;
use std::{env, fs};
// use std::rc::Rc;

use walkdir::WalkDir;

use crate::loki_proto::loki_protolexer::*;
use crate::loki_proto::loki_protoparser::*;
use crate::loki_proto::message_management::*;
use crate::loki_proto::proto_visitor::*;

mod loki_proto;

fn main() {
    // 1. test input from file
    testcase_1();
    // 2. test inputs from file: oneof & message.message, message.enum
    testcase_2();
    // 3. test inputs from file: map
    testcase_3();
    // 4. test inputs from directory
    testcase_4();
    // 5. test inputs from directory
    testcase_5();
}

#[allow(dead_code)]
fn testcase_1() {
    println!("this is a test case 1!");

    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("testcase");
    config_path.push("gossip");
    config_path.push("message.proto");
    println!("testcase file: {:?}", config_path);
    let input_str = fs::read_to_string(config_path).unwrap();
    // println!("{}", input_str);

    let input = InputStream::new(&*input_str);
    let tf = CommonTokenFactory::default();
    let lexer = Loki_protoLexer::new_with_token_factory(input, &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = Loki_protoParser::new(token_source);
    let result = parser.proto();

    let result_ctx = result.expect("input parsed unsuccessfully");

    let mut visitor = Protovisitor::default();
    result_ctx.accept(&mut visitor);
    visitor.print_status();
}

#[allow(dead_code)]
fn testcase_2() {
    println!("this is a test case 2!");

    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("testcase");
    config_path.push("common");
    config_path.push("policies.proto");
    println!("testcase file: {:?}", config_path);
    let input_str = fs::read_to_string(config_path).unwrap();
    // println!("{}", input_str);

    let input = InputStream::new(&*input_str);
    let tf = CommonTokenFactory::default();
    let lexer = Loki_protoLexer::new_with_token_factory(input, &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = Loki_protoParser::new(token_source);
    let result = parser.proto();

    let result_ctx = result.expect("input parsed unsuccessfully");

    let mut visitor = Protovisitor::default();
    result_ctx.accept(&mut visitor);
    visitor.print_status();
}

#[allow(dead_code)]
fn testcase_3() {
    println!("this is a test case 3!");

    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("testcase");
    config_path.push("common");
    config_path.push("configtx.proto");
    println!("testcase file: {:?}", config_path);
    let input_str = fs::read_to_string(config_path).unwrap();
    // println!("{}", input_str);

    let input = InputStream::new(&*input_str);
    let tf = CommonTokenFactory::default();
    let lexer = Loki_protoLexer::new_with_token_factory(input, &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = Loki_protoParser::new(token_source);
    let result = parser.proto();

    let result_ctx = result.expect("input parsed unsuccessfully");

    let mut visitor = Protovisitor::default();
    result_ctx.accept(&mut visitor);
    visitor.print_status();
}

#[allow(dead_code)]
fn testcase_4() {
    println!("this is a test case 4: test parse protos files from dir");

    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("testcase");
    println!("testcases file: {:?}", config_path);

    let mut msg_management = MsgManagement::default();

    for entry in WalkDir::new(config_path.as_os_str())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let _f_path = entry.path().display();
        if f_name.ends_with(".proto") {
            // println!("{} : {}", f_name, f_path);

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

            // visitor.print_status();

            msg_management.push_visitor(visitor);
            msg_management.print_status();
        }
    }
}

#[allow(dead_code)]
fn testcase_5() {
    println!("this is a test case 5: test path info in message!");

    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let pre_index = config_path.display().to_string().len();
    config_path.push("testcase");
    println!("testcases file: {:?}", config_path);

    let mut msg_management = MsgManagement::default();

    for entry in WalkDir::new(config_path.as_os_str())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let f_path = entry.path().display().to_string();
        if f_name.ends_with(".proto") {
            println!("{} : {}", f_name, f_path);

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

            // visitor.print_status();

            msg_management.push_visitor_with_path(
                visitor,
                (&f_path[pre_index + 1..f_path.len() - f_name.len()]).to_string(),
            );
            // msg_management.print_status();
        }
    }
    for msg in msg_management.get_mut_proto_messages() {
        println!("name is {}, path is {}", msg.get_name(), msg.get_path());
    }
}
