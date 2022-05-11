//! encoding and decoding methods for different rules
//! Currently, we support the protobuf and RLP

use crate::loki_message::*;
use integer_encoding::VarInt;

/// the encoding ways for protobuf
pub fn encode_protobuf(message: LokiMessage) -> String {
    let mut res: Vec<u8> = vec![];
    let cur_structure = message.get_structure();
    let mut field_num = 1;
    for attr in cur_structure.get_attrs() {
        let attr_ty = attr.get_attr_type();
        let attr_name = attr.get_attr_name();
        let msg_content = message.get_content();
        match &attr_ty[..] {
            "Number" => {
                let cur_val = msg_content.get(&attr_name).unwrap();
                if cur_val.is_u64() {
                    let key: u8 = (field_num << 3) | 0;
                    res.push(key);
                    let cur_u64_val = cur_val.as_u64().unwrap();
                    let mut stream = cur_u64_val.encode_var_vec();
                    res.append(&mut stream);
                } else {
                    let key: u8 = (field_num << 3) | 0;
                    res.push(key);
                    let cur_i64_val = cur_val.as_i64().unwrap();
                    let mut stream = cur_i64_val.encode_var_vec();
                    res.append(&mut stream);
                }
            }
            "Bool" => {
                let cur_val = msg_content.get(&attr_name).unwrap();
                let cur_bool_val = cur_val.as_bool().unwrap();
                let key: u8 = (field_num << 3) | 0;
                res.push(key);
                if cur_bool_val == true {
                    let mut stream = 1_u64.encode_var_vec();
                    res.append(&mut stream);
                } else {
                    let mut stream = 0_u64.encode_var_vec();
                    res.append(&mut stream);
                }
            }
            "String" => {
                let cur_val = msg_content.get(&attr_name).unwrap();
                let cur_string_val = cur_val.as_str().unwrap().to_string();
                let mut data = length_delimited(field_num, cur_string_val);
                res.append(&mut data);
            }
            _ => {
                todo!()
            }
        }
        field_num += 1;
    }
    "".to_string()
}

/// the decoding ways for protobuf
pub fn decode_protobuf(_stream: Vec<u8>) -> LokiMessage {
    todo!()
}

/// get the length delimited encoding
pub fn length_delimited(field_num: u8, data: String) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    let key: u8 = (field_num << 3) | 2;
    res.push(key);
    let mut field_data = data.as_bytes().to_vec();
    res.append(&mut field_data.len().encode_var_vec());
    res.append(&mut field_data);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    /*
     * Test encoding implementation
     */

    #[test]
    fn test_length_delimited() {
        let res = length_delimited(2, "testing".to_string());
        let expect_res = vec![0x12_u8, 0x07_u8, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];
        assert_eq!(res, expect_res);
    }
}
