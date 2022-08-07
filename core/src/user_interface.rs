//! The interface that user needs to extend and implement for LOKI adaption
use crate::loki_message::LokiMessage;
use serde_json::{Map, Value};

/// the adatption interface
/// encode the LOKI message into a stream
pub fn encode(msg_name: String, content: Map<String, Value>) -> Vec<u8> {
    let msg = crate::loki_message::get_structure_from_msg_type(msg_name).unwrap();
    return crate::encode_map_to_buffer::encode(msg, content);
}

/// decode a stream to a LOKI message
pub fn decode(msg_name: String, stream: &[u8]) -> Map<String, Value> {
    let msg = crate::loki_message::get_structure_from_msg_type(msg_name).unwrap();
    return crate::decode_map_to_buffer::decode(msg, stream);
}

/// the package sending interface of tested systems
pub fn send_packets(_target_id: String, _msg: LokiMessage) {
    // call the sned_packets in bcos-loki. The library is named liblokiSend.a
    todo!();
}

/// receive the packets
// pub fn recv_packets(from_id: &[u8], payload: &[u8], msg_type: u32) {
//     // todo: convert the from id into a string
//     let msg_type_string = get_fisco_bcos_msg_type(msg_type);
//     let msg_structure =
//         crate::loki_message::get_structure_from_msg_type(msg_type_string.clone()).unwrap();
//     let content = decode(msg_type_string, payload);
//     let mut recv_msg = LokiMessage::new(from_id, msg_structure, content);
//     crate::engine::LOKI_ENGINE
//         .lock()
//         .unwrap()
//         .passive_sending(recv_msg);
// }

pub fn get_fisco_bcos_msg_type(msg_type: u32) -> String {
    match msg_type {
        0 => String::from("PrePreparePacket"),
        1 => String::from("PreparePacket"),
        2 => String::from("CommitPacket"),
        3 => String::from("ViewChangePacket"),
        4 => String::from("NewViewPacket"),
        5 => String::from("CommittedProposalRequest"),
        6 => String::from("CommittedProposalResponse"),
        7 => String::from("PreparedProposalRequest"),
        8 => String::from("PreparedProposalResponse"),
        9 => String::from("CheckPoint"),
        10 => String::from("RecoverRequest"),
        11 => String::from("RecoverResponse"),
        _ => String::from(""),
    }
    // PrePreparePacket = 0x00,
    // PreparePacket = 0x01,
    // CommitPacket = 0x02,
    // ViewChangePacket = 0x03,
    // NewViewPacket = 0x04,
    // CommittedProposalRequest = 0x5,
    // CommittedProposalResponse = 0x6,
    // PreparedProposalRequest = 0x7,
    // PreparedProposalResponse = 0x8,
    // CheckPoint = 0x9,
    // RecoverRequest = 0xa,
    // RecoverResponse = 0xb,

    // match msg_type {
    //     1000 => String::from("PBFT"),
    //     1001 => String::from("Raft"),
    //     2000 => String::from("BlockSync"),
    //     2001 => String::from("TxsSync"),
    //     3000 => String::from("AMOP"),
    //     _ => String::from(""),
    // }
}

pub fn get_fisco_bcos_type_int(msg_type: String) -> u32 {
    match &msg_type[..] {
        "PrePreparePacket" => 0,
        "PreparePacket" => 1,
        "CommitPacket" => 2,
        "ViewChangePacket" => 3,
        "NewViewPacket" => 4,
        "CommittedProposalRequest" => 5,
        "CommittedProposalResponse" => 6,
        "PreparedProposalRequest" => 7,
        "PreparedProposalResponse" => 8,
        "CheckPoint" => 9,
        "RecoverRequest" => 10,
        "RecoverResponse" => 11,
        _ => 100,
    }
}

// registerHandlerByMsgType
