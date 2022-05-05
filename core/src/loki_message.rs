//! The normalization message representation in LOKI

use crate::loki_type::{get_current_language, Array, BasicType, TIMESTAMP_LENGTH};
use crate::mutator::*;
use anyhow::Result;
use lazy_static::lazy_static;
#[allow(unused_imports)]
use loki_spec::loki_spec::*;
use serde_json::{Map, Number, Value};
use std::sync::Mutex;

lazy_static! {
    /// all the message types supported by LOKI
    pub static ref MESSAGE_TYPE: Mutex<Vec<String>> = Mutex::new(vec!());
}

/// set all of the message types
pub fn add_message_type(new_type: String) {
    let mut l = MESSAGE_TYPE.lock().unwrap();
    (*l).push(new_type);
}

/// get all the message types
pub fn get_message_types() -> Vec<String> {
    let res = MESSAGE_TYPE.lock().unwrap();
    (*res).clone()
}

/// the message structure in LOKI
#[derive(Debug, Clone)]
/// the loki message struct
pub struct LokiMessage {
    /// the nodeid who sends the message
    from: String,
    // the structure of the message, represented as a json object
    // use json::JsonValue::new_object() to create an empty json object
    // structure: JsonValue,
    /// using the message struct defined in loki_spec
    structure: message::Message,
    /// the content of current message, represented as a hash map
    content: Map<String, Value>,
}

impl LokiMessage {
    /// construct a new message with structure
    pub fn new(
        from: String,
        structure: loki_spec::loki_spec::message::Message,
        content: Map<String, Value>,
    ) -> Self {
        Self {
            from,
            structure,
            content,
        }
    }

    /// construct a new message with only source node
    pub fn new_with_from(from: String) -> Self {
        let structure = loki_spec::loki_spec::message::Message::new("".to_string(), vec![], vec![]);
        let content = Map::new();
        Self {
            from,
            structure,
            content,
        }
    }

    /// get the content
    pub fn get_content(&self) -> Map<String, Value> {
        self.content.clone()
    }

    /// get the mutate content
    pub fn get_mut_content(&mut self) -> &mut Map<String, Value> {
        &mut self.content
    }

    /// set the current content
    pub fn set_content(&mut self, new_content: Map<String, Value>) {
        self.content = new_content;
    }

    /// set the from neighbour of a message
    pub fn set_from_node(&mut self, from_node: String) -> Result<bool> {
        self.from = from_node;
        Ok(true)
    }

    /// get the from neighbour of a message
    pub fn get_from_node(&self) -> Result<String> {
        Ok(self.from.clone())
    }

    /// get mutable from neighbour of a message
    pub fn get_mut_from_node(&mut self) -> Result<&mut String> {
        Ok(&mut self.from)
    }

    /// set the structure of the message
    pub fn set_structure(
        &mut self,
        new_structure: loki_spec::loki_spec::message::Message,
    ) -> Result<bool> {
        self.structure = new_structure;
        Ok(true)
    }

    /// get the structure of the message
    pub fn get_structure(&self) -> loki_spec::loki_spec::message::Message {
        self.structure.clone()
    }

    /// get the mutable structure of the message
    pub fn get_mut_structure(&mut self) -> &mut loki_spec::loki_spec::message::Message {
        &mut self.structure
    }

    /// mutate the current message
    pub fn mutate(&mut self) {
        let current_structure = self.get_structure();
        for structure_attr in current_structure.get_attrs() {
            let attr_type = structure_attr.get_attr_type();
            let attr_mutation = structure_attr.get_attr_mutator();
            match &attr_type[..] {
                "Number" => {
                    let cur_val = self
                        .get_mut_content()
                        .get(&structure_attr.get_attr_name())
                        .unwrap();
                    if cur_val.is_u64() {
                        let cur_u64_val = cur_val.as_u64().unwrap();
                        let mut mutated_val: u128 = 0;
                        match &attr_mutation[..] {
                            "random_Number" | "edge_value" => {
                                mutated_val =
                                    generate_random_unsigned_number(64, get_current_language());
                            }
                            "Decreasing" => {
                                mutated_val = strictly_decreasing_mutate_for_unsigned_number(
                                    cur_u64_val as u128,
                                    64,
                                    get_current_language(),
                                );
                            }
                            "Increaseing" => {
                                mutated_val = strictly_increasing_mutate_for_unsigned_number(
                                    cur_u64_val as u128,
                                    64,
                                    get_current_language(),
                                );
                            }
                            _ => {}
                        }
                        self.get_mut_content()[&structure_attr.get_attr_name()] =
                            Value::Number(Number::from(mutated_val as u64));
                    } else if cur_val.is_i64() {
                        let cur_i64_val = cur_val.as_i64().unwrap();
                        let mut mutated_val: i128 = 0;
                        match &attr_mutation[..] {
                            "random_Number" | "edge_value" => {
                                mutated_val =
                                    generate_random_signed_number(64, get_current_language());
                            }
                            "Decreasing" => {
                                mutated_val = strictly_decreasing_mutate_for_signed_number(
                                    cur_i64_val as i128,
                                    64,
                                    get_current_language(),
                                );
                            }
                            "Increaseing" => {
                                mutated_val = strictly_increasing_mutate_for_signed_number(
                                    cur_i64_val as i128,
                                    64,
                                    get_current_language(),
                                );
                            }
                            _ => {}
                        }
                        self.get_mut_content()[&structure_attr.get_attr_name()] =
                            Value::Number(Number::from(mutated_val as i64));
                    }
                }
                "String" => {
                    let cur_val = self
                        .get_mut_content()
                        .get(&structure_attr.get_attr_name())
                        .unwrap();
                    let cur_string_val = cur_val.as_str().unwrap();
                    let mutated_val: String = random_mutate_string(cur_string_val.to_string());
                    self.get_mut_content()[&structure_attr.get_attr_name()] =
                        Value::String(String::from(mutated_val));
                }
                "Bool" => {
                    let mutated_val = generate_random_bool();
                    self.get_mut_content()[&structure_attr.get_attr_name()] =
                        Value::Bool(mutated_val);
                }
                "Byte" => {
                    let cur_val = self
                        .get_mut_content()
                        .get(&structure_attr.get_attr_name())
                        .unwrap();
                    let cur_bytes_val = cur_val.as_array().unwrap().clone();
                    let cur_vecu8_val: Vec<u8> = cur_bytes_val
                        .iter()
                        .map(|v| (v.as_u64().unwrap() as u8))
                        .collect();
                    let mutated_val = random_mutate_byte(cur_vecu8_val);
                    self.get_mut_content()[&structure_attr.get_attr_name()] = Value::Array(
                        mutated_val
                            .iter()
                            .map(|v| Value::Number(Number::from(*v as u64)))
                            .collect::<Vec<_>>(),
                    );
                }
                "Timestamp" => {
                    let cur_val = self
                        .get_mut_content()
                        .get(&structure_attr.get_attr_name())
                        .unwrap();
                    let cur_timestamp_val = cur_val.as_str().unwrap();
                    let mut mutated_val: String = cur_timestamp_val.to_string();
                    match &attr_mutation[..] {
                        "random_Timestamp" => {
                            mutated_val = random_mutate_long_number(cur_timestamp_val.to_string());
                        }
                        "Decreasing" => {
                            mutated_val = fine_tuning_mutate_for_long_number(
                                cur_timestamp_val.to_string(),
                                100,
                                '-'.to_string(),
                            );
                        }
                        "Increasing" => {
                            mutated_val = fine_tuning_mutate_for_long_number(
                                cur_timestamp_val.to_string(),
                                100,
                                '+'.to_string(),
                            );
                        }
                        _ => {}
                    };
                    self.get_mut_content()[&structure_attr.get_attr_name()] =
                        Value::String(String::from(mutated_val));
                }
                "Hash" => {
                    todo!()
                }
                "BigNumber" => {
                    let cur_val = self
                        .get_mut_content()
                        .get(&structure_attr.get_attr_name())
                        .unwrap();
                    let cur_bignumber_val = cur_val.as_str().unwrap();
                    let mut mutated_val: String = cur_bignumber_val.to_string();
                    match &attr_mutation[..] {
                        "random_Timestamp" => {
                            mutated_val = random_mutate_long_number(cur_bignumber_val.to_string());
                        }
                        "Decreasing" => {
                            mutated_val = fine_tuning_mutate_for_long_number(
                                cur_bignumber_val.to_string(),
                                100,
                                '-'.to_string(),
                            );
                        }
                        "Increasing" => {
                            mutated_val = fine_tuning_mutate_for_long_number(
                                cur_bignumber_val.to_string(),
                                100,
                                '+'.to_string(),
                            );
                        }
                        _ => {}
                    };
                    self.get_mut_content()[&structure_attr.get_attr_name()] =
                        Value::String(String::from(mutated_val));
                }
                "Array" => {
                    let cur_val = self
                        .get_mut_content()
                        .get(&structure_attr.get_attr_name())
                        .unwrap();
                    let cur_array_val = cur_val.as_array().unwrap();
                    let array_ref = structure_attr.get_attr_reff();
                    let array_ele = current_structure.get_attr_by_name(array_ref).unwrap();
                    let array_type = array_ele.get_attr_type();
                    let mut current_array = match &array_type.to_ascii_lowercase()[..] {
                        "number" => {
                            let ele_len = match array_ele.get_attr_size().parse::<u32>() {
                                Ok(v) => v,
                                Err(_) => 64,
                            };
                            let mut arr = Array::new(BasicType::NUMBER, ele_len, false);
                            arr.set_content(cur_array_val.iter().map(|v| v.to_string()).collect());
                            arr
                        }
                        "string" => {
                            let ele_len = match array_ele.get_attr_size().parse::<u32>() {
                                Ok(v) => v,
                                Err(_) => 64,
                            };
                            let mut arr = Array::new(BasicType::STRING, ele_len, false);
                            arr.set_content(cur_array_val.iter().map(|v| v.to_string()).collect());
                            arr
                        }
                        "bool" => {
                            let mut arr = Array::new(BasicType::BOOL, 8, false);
                            arr.set_content(cur_array_val.iter().map(|v| v.to_string()).collect());
                            arr
                        }
                        "byte" => {
                            let ele_len = match array_ele.get_attr_size().parse::<u32>() {
                                Ok(v) => v,
                                Err(_) => 64,
                            };
                            let mut arr = Array::new(BasicType::BYTE, ele_len, false);
                            arr.set_content(cur_array_val.iter().map(|v| v.to_string()).collect());
                            arr
                        }
                        "timestamp" => unsafe {
                            let mut arr =
                                Array::new(BasicType::TIMESTAMP, TIMESTAMP_LENGTH as u32, false);
                            arr.set_content(cur_array_val.iter().map(|v| v.to_string()).collect());
                            arr
                        },
                        "bignumber" => {
                            let ele_len = match array_ele.get_attr_size().parse::<u32>() {
                                Ok(v) => v,
                                Err(_) => 64,
                            };
                            let mut arr = Array::new(BasicType::BIGNUMBER, ele_len, false);
                            arr.set_content(cur_array_val.iter().map(|v| v.to_string()).collect());
                            arr
                        }
                        _ => Array::new(BasicType::NUMBER, 64, false),
                    };
                    random_mutate_array(&mut current_array);
                    let mutated_arr = match &array_type.to_ascii_lowercase()[..] {
                        "number" => Value::Array(
                            current_array
                                .get_content()
                                .iter()
                                .map(|v| Value::Number(Number::from(v.parse::<u64>().unwrap())))
                                .collect(),
                        ),
                        "string" => Value::Array(
                            current_array
                                .get_content()
                                .iter()
                                .map(|v| Value::String(String::from(v)))
                                .collect(),
                        ),
                        "bool" => Value::Array(
                            current_array
                                .get_content()
                                .iter()
                                .map(|v| Value::Bool(v.parse::<bool>().unwrap()))
                                .collect(),
                        ),
                        "byte" => Value::Array(
                            current_array
                                .get_content()
                                .iter()
                                .map(|v| {
                                    Value::Number(Number::from(v.parse::<u8>().unwrap() as u64))
                                })
                                .collect(),
                        ),
                        "timestamp" | "bignumber" => Value::Array(
                            current_array
                                .get_content()
                                .iter()
                                .map(|v| Value::String(String::from(v)))
                                .collect(),
                        ),
                        _ => Value::Array(vec![]),
                    };
                    self.get_mut_content()[&structure_attr.get_attr_name()] = mutated_arr;
                }
                "Signature" => {
                    todo!()
                }
                _ => {}
            }
        }
    }

    /// generate a new message of certain type
    pub fn generate(_msg_type: String) -> Self {
        todo!()
    }
}
