#![feature(try_blocks)]
extern crate lazy_static;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::tree::Visitable;
// use antlr_rust::tree::{ParseTree, ParseTreeVisitor};

use std::fs;
use std::path::PathBuf;
// use std::rc::Rc;

use crate::loki_spec::loki_speclexer::*;
use crate::loki_spec::loki_specparser::*;
use crate::loki_spec::spec_visitor::*;

mod loki_spec;

fn main() {
    // 1. test input from string
    testcase_1();
    // 2. test input from file
    testcase_2();

    //3. test fn get_structure_from_msg_type()
    testcase_3();
}

// /** TODOList:
//  *
//  * 1. refactor code struct
//  * 2. add print function
//  * 2. remove warning
//  * 3. run more test case
//  * 4. finish TODO
//  *
//  */
fn testcase_1() {
    println!("this is a test case 1!");

    let input_str = String::from("<LOKI specname=\"fabric_example\"> <Message name=\"p2p packet\"> <Attribute name=\"message type\" type=String /> </Message> <Message name=\"viewchange\">  <Attribute name=\"message type\" type=String /> <Attribute name=\"view\" type=Number /> <ExpectedMsg name=\"A\" identify=\"1,2\" excptedIdentify=\"A.1,A.2\" /> </Message> </LOKI>");
    // println!("{}",inputStr);

    let input = InputStream::new(&*input_str);
    let tf = CommonTokenFactory::default();
    let lexer = Loki_specLexer::new_with_token_factory(input, &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = Loki_specParser::new(token_source);
    let result = parser.document();

    // println!("{}",result.is_ok());

    let result_ctx = result.expect("input parsed unsuccessfully");

    let mut visitor = Specvisitor::default();
    result_ctx.accept(&mut visitor);
}

fn testcase_2() {
    println!("this is a test case 2!");

    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("testcase");
    config_path.push("simple.spec");
    println!("config file: {:?}", config_path);
    let input_str = fs::read_to_string(config_path).unwrap();
    // println!("{}", input_str);

    // let inputStr = String::from("<LOKI specname=\"fabric_example\"> <Message name=\"p2p packet\"> <Attribute name=\"message type\" type=String /> </Message> <Message name=\"viewchange\">  <Attribute name=\"message type\" type=String /> <Attribute name=\"view\" type=Number /> <ExpectedMsg name=\"A\" identify=\"1,2\" excptedIdentify=\"A.1,A.2\" /> </Message> </LOKI>");
    // println!("{}",inputStr);

    let input = InputStream::new(&*input_str);
    let tf = CommonTokenFactory::default();
    let lexer = Loki_specLexer::new_with_token_factory(input, &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = Loki_specParser::new(token_source);
    let result = parser.document();

    // println!("{}",result.is_ok());

    let result_ctx = result.expect("input parsed unsuccessfully");

    let mut visitor = Specvisitor::default();
    result_ctx.accept(&mut visitor);
}

fn testcase_3() {
    println!("this is a test case 3: test fn get_structure_from_msg_type()!");

    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("testcase");
    config_path.push("simple.spec");
    println!("config file: {:?}", config_path);
    let input_str = fs::read_to_string(config_path).unwrap();
    // println!("{}", input_str);

    // let inputStr = String::from("<LOKI specname=\"fabric_example\"> <Message name=\"p2p packet\"> <Attribute name=\"message type\" type=String /> </Message> <Message name=\"viewchange\">  <Attribute name=\"message type\" type=String /> <Attribute name=\"view\" type=Number /> <ExpectedMsg name=\"A\" identify=\"1,2\" excptedIdentify=\"A.1,A.2\" /> </Message> </LOKI>");
    // println!("{}",inputStr);

    let input = InputStream::new(&*input_str);
    let tf = CommonTokenFactory::default();
    let lexer = Loki_specLexer::new_with_token_factory(input, &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = Loki_specParser::new(token_source);
    let result = parser.document();

    // println!("{}",result.is_ok());

    let result_ctx = result.expect("input parsed unsuccessfully");

    let mut visitor = Specvisitor::default();
    result_ctx.accept(&mut visitor);
    let msg = visitor.get_structure_from_msg_type("viewchange".to_string());
    match msg {
        Ok(msg) => println!("msg's name is : {:?}", msg.get_name()),
        Err(e) => println!("error calling  get_structure_from_msg_type: {:?}", e),
    }

    let new_msg = visitor.get_structure_from_msg_type("viewchange".to_string());
    match new_msg {
        Ok(new_msg) => println!("msg's name is : {:?}", new_msg.get_name()),
        Err(e) => println!("error calling  get_structure_from_msg_type: {:?}", e),
    }
}
