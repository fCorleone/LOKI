#![allow(missing_docs)]
//! the types supported by LOKI
use anyhow::Result;
use std::str::FromStr;

#[derive(Clone)]
pub enum BasicType {
    BOOL,
    NUMBER,
    BYTE,
    STRING,
    BIGNUMBER,
    TIMESTAMP,
}

/// construct basic types according to the string
impl FromStr for BasicType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = match &s.to_ascii_lowercase()[..] {
            "bool" => Self::BOOL,
            "number" => Self::NUMBER,
            "byte" => Self::BYTE,
            "string" => Self::STRING,
            "bignumber" => Self::BIGNUMBER,
            "timestamp" => Self::TIMESTAMP,
            _ => return Err("unsupported basic type".to_string()),
        };
        Ok(t)
    }
}

/// the array type in LOKI
pub struct Array {
    ele_type: BasicType,
    length: u32,
    fixed: bool,
    content: Vec<String>,
}

impl Array {
    /// construct a new array
    pub fn new(ele_type: BasicType, length: u32, fixed: bool) -> Self {
        let content = vec!["".to_string()];
        Array {
            ele_type,
            length,
            fixed,
            content,
        }
    }

    /// whether the array's length is fixed
    pub fn is_fixed(&self) -> bool {
        self.fixed
    }

    /// whether the array is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// modify the content
    pub fn update_content(&mut self, new_content: Vec<String>) {
        self.content = new_content;
    }

    /// get the content
    pub fn get_content(&self) -> &Vec<String> {
        &self.content
    }

    /// get the mutate content
    pub fn get_mut_content(&mut self) -> &mut Vec<String> {
        &mut self.content
    }

    /// get the element type
    pub fn get_ele_ty(&self) -> BasicType {
        self.ele_type.clone()
    }

    /// set the fixed
    pub fn set_fixed(&mut self, fixed: bool) {
        self.fixed = fixed;
    }

    /// get the length
    pub fn get_length(&self) -> u32 {
        self.length
    }

    /// set the length
    pub fn set_length(&mut self, new_len: u32) {
        self.length = new_len;
    }
}
