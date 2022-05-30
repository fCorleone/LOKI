use protobuf::Message;
use protobuf_json_mapping::*;
use serde_json::{Map, Value, json};
use crate::protos::token::transaction::*;
use crate::protos::token::prover::*;
use crate::protos::token::expectations::*;

pub fn decode(msg: loki_spec::loki_spec::message::Message, stream: &[u8]) -> Map<String, Value>{
	let msg_name = msg.get_name();
	match msg_name.as_str() {
		"TokenToIssue" => { 
			let tmp = TokenToIssue::parse_from_bytes(stream).expect("TokenToIssue parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("TokenToIssue parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"RecipientTransferShare" => { 
			let tmp = RecipientTransferShare::parse_from_bytes(stream).expect("RecipientTransferShare parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("RecipientTransferShare parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"TokenOutput" => { 
			let tmp = TokenOutput::parse_from_bytes(stream).expect("TokenOutput parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("TokenOutput parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"UnspentTokens" => { 
			let tmp = UnspentTokens::parse_from_bytes(stream).expect("UnspentTokens parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("UnspentTokens parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"ListRequest" => { 
			let tmp = ListRequest::parse_from_bytes(stream).expect("ListRequest parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("ListRequest parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"ImportRequest" => { 
			let tmp = ImportRequest::parse_from_bytes(stream).expect("ImportRequest parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("ImportRequest parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"TransferRequest" => { 
			let tmp = TransferRequest::parse_from_bytes(stream).expect("TransferRequest parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("TransferRequest parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"RedeemRequest" => { 
			let tmp = RedeemRequest::parse_from_bytes(stream).expect("RedeemRequest parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("RedeemRequest parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"AllowanceRecipientShare" => { 
			let tmp = AllowanceRecipientShare::parse_from_bytes(stream).expect("AllowanceRecipientShare parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("AllowanceRecipientShare parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"ApproveRequest" => { 
			let tmp = ApproveRequest::parse_from_bytes(stream).expect("ApproveRequest parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("ApproveRequest parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"ExpectationRequest" => { 
			let tmp = ExpectationRequest::parse_from_bytes(stream).expect("ExpectationRequest parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("ExpectationRequest parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"Header" => { 
			let tmp = Header::parse_from_bytes(stream).expect("Header parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("Header parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"Command" => { 
			let tmp = Command::parse_from_bytes(stream).expect("Command parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("Command parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"SignedCommand" => { 
			let tmp = SignedCommand::parse_from_bytes(stream).expect("SignedCommand parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("SignedCommand parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"CommandResponseHeader" => { 
			let tmp = CommandResponseHeader::parse_from_bytes(stream).expect("CommandResponseHeader parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("CommandResponseHeader parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"Error" => { 
			let tmp = Error::parse_from_bytes(stream).expect("Error parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("Error parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"CommandResponse" => { 
			let tmp = CommandResponse::parse_from_bytes(stream).expect("CommandResponse parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("CommandResponse parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"SignedCommandResponse" => { 
			let tmp = SignedCommandResponse::parse_from_bytes(stream).expect("SignedCommandResponse parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("SignedCommandResponse parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"TokenTransaction" => { 
			let tmp = TokenTransaction::parse_from_bytes(stream).expect("TokenTransaction parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("TokenTransaction parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"PlainTokenAction" => { 
			let tmp = PlainTokenAction::parse_from_bytes(stream).expect("PlainTokenAction parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("PlainTokenAction parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"PlainImport" => { 
			let tmp = PlainImport::parse_from_bytes(stream).expect("PlainImport parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("PlainImport parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"PlainTransfer" => { 
			let tmp = PlainTransfer::parse_from_bytes(stream).expect("PlainTransfer parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("PlainTransfer parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"PlainApprove" => { 
			let tmp = PlainApprove::parse_from_bytes(stream).expect("PlainApprove parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("PlainApprove parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"PlainTransferFrom" => { 
			let tmp = PlainTransferFrom::parse_from_bytes(stream).expect("PlainTransferFrom parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("PlainTransferFrom parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"PlainOutput" => { 
			let tmp = PlainOutput::parse_from_bytes(stream).expect("PlainOutput parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("PlainOutput parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"InputId" => { 
			let tmp = InputId::parse_from_bytes(stream).expect("InputId parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("InputId parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"PlainDelegatedOutput" => { 
			let tmp = PlainDelegatedOutput::parse_from_bytes(stream).expect("PlainDelegatedOutput parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("PlainDelegatedOutput parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"TokenExpectation" => { 
			let tmp = TokenExpectation::parse_from_bytes(stream).expect("TokenExpectation parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("TokenExpectation parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"PlainExpectation" => { 
			let tmp = PlainExpectation::parse_from_bytes(stream).expect("PlainExpectation parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("PlainExpectation parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		"PlainTokenExpectation" => { 
			let tmp = PlainTokenExpectation::parse_from_bytes(stream).expect("PlainTokenExpectation parse from bytes error!"); 
			//convert tmp => map
			let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("PlainTokenExpectation parse to map_str error!");
			let v: Map<String,Value> = serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			return v;
		}
		_ => println!()
	}
	return serde_json::Map::new();
}
