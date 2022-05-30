use protobuf::Message;
use protobuf_json_mapping::*;
use serde_json::{Map, Value, json};
use crate::protos::token::transaction::*;
use crate::protos::token::prover::*;
use crate::protos::token::expectations::*;

pub fn encode(msg: loki_spec::loki_spec::message::Message, content:Map<String, Value>) -> Vec<u8>{
	let msg_name = msg.get_name();
	match msg_name.as_str() {
		"TokenToIssue" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<TokenToIssue>(&json_str).expect("TokenToIssue parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("TokenToIssue write to bytes error!"); 
			return bytes;
		}
		"RecipientTransferShare" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<RecipientTransferShare>(&json_str).expect("RecipientTransferShare parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("RecipientTransferShare write to bytes error!"); 
			return bytes;
		}
		"TokenOutput" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<TokenOutput>(&json_str).expect("TokenOutput parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("TokenOutput write to bytes error!"); 
			return bytes;
		}
		"UnspentTokens" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<UnspentTokens>(&json_str).expect("UnspentTokens parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("UnspentTokens write to bytes error!"); 
			return bytes;
		}
		"ListRequest" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<ListRequest>(&json_str).expect("ListRequest parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("ListRequest write to bytes error!"); 
			return bytes;
		}
		"ImportRequest" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<ImportRequest>(&json_str).expect("ImportRequest parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("ImportRequest write to bytes error!"); 
			return bytes;
		}
		"TransferRequest" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<TransferRequest>(&json_str).expect("TransferRequest parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("TransferRequest write to bytes error!"); 
			return bytes;
		}
		"RedeemRequest" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<RedeemRequest>(&json_str).expect("RedeemRequest parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("RedeemRequest write to bytes error!"); 
			return bytes;
		}
		"AllowanceRecipientShare" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<AllowanceRecipientShare>(&json_str).expect("AllowanceRecipientShare parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("AllowanceRecipientShare write to bytes error!"); 
			return bytes;
		}
		"ApproveRequest" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<ApproveRequest>(&json_str).expect("ApproveRequest parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("ApproveRequest write to bytes error!"); 
			return bytes;
		}
		"ExpectationRequest" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<ExpectationRequest>(&json_str).expect("ExpectationRequest parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("ExpectationRequest write to bytes error!"); 
			return bytes;
		}
		"Header" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<Header>(&json_str).expect("Header parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("Header write to bytes error!"); 
			return bytes;
		}
		"Command" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<Command>(&json_str).expect("Command parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("Command write to bytes error!"); 
			return bytes;
		}
		"SignedCommand" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<SignedCommand>(&json_str).expect("SignedCommand parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("SignedCommand write to bytes error!"); 
			return bytes;
		}
		"CommandResponseHeader" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<CommandResponseHeader>(&json_str).expect("CommandResponseHeader parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("CommandResponseHeader write to bytes error!"); 
			return bytes;
		}
		"Error" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<Error>(&json_str).expect("Error parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("Error write to bytes error!"); 
			return bytes;
		}
		"CommandResponse" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<CommandResponse>(&json_str).expect("CommandResponse parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("CommandResponse write to bytes error!"); 
			return bytes;
		}
		"SignedCommandResponse" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<SignedCommandResponse>(&json_str).expect("SignedCommandResponse parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("SignedCommandResponse write to bytes error!"); 
			return bytes;
		}
		"TokenTransaction" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<TokenTransaction>(&json_str).expect("TokenTransaction parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("TokenTransaction write to bytes error!"); 
			return bytes;
		}
		"PlainTokenAction" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<PlainTokenAction>(&json_str).expect("PlainTokenAction parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("PlainTokenAction write to bytes error!"); 
			return bytes;
		}
		"PlainImport" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<PlainImport>(&json_str).expect("PlainImport parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("PlainImport write to bytes error!"); 
			return bytes;
		}
		"PlainTransfer" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<PlainTransfer>(&json_str).expect("PlainTransfer parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("PlainTransfer write to bytes error!"); 
			return bytes;
		}
		"PlainApprove" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<PlainApprove>(&json_str).expect("PlainApprove parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("PlainApprove write to bytes error!"); 
			return bytes;
		}
		"PlainTransferFrom" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<PlainTransferFrom>(&json_str).expect("PlainTransferFrom parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("PlainTransferFrom write to bytes error!"); 
			return bytes;
		}
		"PlainOutput" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<PlainOutput>(&json_str).expect("PlainOutput parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("PlainOutput write to bytes error!"); 
			return bytes;
		}
		"InputId" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<InputId>(&json_str).expect("InputId parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("InputId write to bytes error!"); 
			return bytes;
		}
		"PlainDelegatedOutput" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<PlainDelegatedOutput>(&json_str).expect("PlainDelegatedOutput parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("PlainDelegatedOutput write to bytes error!"); 
			return bytes;
		}
		"TokenExpectation" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<TokenExpectation>(&json_str).expect("TokenExpectation parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("TokenExpectation write to bytes error!"); 
			return bytes;
		}
		"PlainExpectation" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<PlainExpectation>(&json_str).expect("PlainExpectation parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("PlainExpectation write to bytes error!"); 
			return bytes;
		}
		"PlainTokenExpectation" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<PlainTokenExpectation>(&json_str).expect("PlainTokenExpectation parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("PlainTokenExpectation write to bytes error!"); 
			return bytes;
		}
		_ => println!()
	}
	return [].to_vec()
}
