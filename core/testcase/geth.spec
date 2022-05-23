<LOKI specname="go-ethereum:example">

	<Message name="NewBlockMsg">
		<Attribute name="Block" type="Struct" ref="Block"/>
		<Attribute name="TD" type="Big_Number" mutation="random_BigNumber" />
	</Message>
	
	<Message name="NewBlockHashesMsg">
		<Attribute name="Hash" type="hash" size="32" mutation="func_hash" param="Msg::NewBlockMsg"/>
		<Attribute name="Number" type="number" size="64" mutation="random_number" />
	</Message>

	<Message name="GetBlockHeadersMsg">
		<Attribute name="RequestId" type="number" size="64" mutation="random_number" />
		<Attribute name="Origin" type="string" size="20" mutation="random_string" param="Array::neighbourhood_Address"/>
		<Attribute name="Amount" type="number" size="64" mutation="random_number" />
		<Attribute name="Skip" type="number" size="64" mutation="random_number" />
		<Attribute name="Reverse" type="bool" mutation="random_bool" />
		<ExceptedMsg mag_name="BlockHeadersMsg" identify="RequestId" msg_identify="BlockHeadersMsg.RequestId" />
	</Message>

	<Message name="BlockHeadersMsg">
		<Attribute name="RequestId" type="number" size="64" mutation="random_number" />
		<Attribute name="BlockHeaders" type="array" ref="Header"/>
	</Message>

	<Struct name="Header">
		<Attribute name="ParentHash" type="hash" size="32" mutation="fixed" />
		<Attribute name="UncleHash" type="hash" size="32" mutation="fixed" />
		<Attribute name="Coinbase" type="string" size="20" mutation="random_string" param="Array::neighbourhood_Address"/>
		<Attribute name="Root" type="hash" size="32" mutation="fixed" />
		<Attribute name="TxHash" type="hash" size="32" mutation="func_hash" param="Block::transactions" />
		<Attribute name="ReceiptHash" type="hash" size="32" mutation="fixed" />
		<Attribute name="Difficulty" type="Big_Number" mutation="random_BigNumber" />
		<Attribute name="Number" type="Big_Number" mutation="random_BigNumber" />
		<Attribute name="GasLimit" type="number" size="64" mutation="random_number" />
		<Attribute name="GasUsed" type="number" size="64" mutation="random_number" />
		<Attribute name="Time" type="time" mutation="random_time" />
		<Attribute name="Extra" type="string" size="no_limit" mutation="random_string" />
		<Attribute name="MixDigest" type="hash" size="32" mutation="fixed" />
	</Struct>

	<Struct name="TxData">
		<Attribute name="txtype" type="byte" size="4" mutation="random_byte"/>
		<Attribute name="txdata" type="string" size="no_limit" mutation="random_string"/>
		<Attribute name="gas" type="number" size="64" mutation="random_number"/>
		<Attribute name="nonce" type="number" size="64" mutation="random_number"/>
		<Attribute name="gasPrice" type="Big_Number" mutation="random_BigNumber"/>
	</Struct>

	<Struct name="Transaction">
		<Attribute name="inner" type="Struct" ref="TxData"/>
		<Attribute name="time" type="time" mutation="random_time" />
		<Attribute name="hash" type="hash" size="32" mutation="func_hash" param="inner"/>
		<Attribute name="size" type="number" mutation="func_sizeof" param="inner" />
		<Attribute name="from" type="string" size="20" mutation="random_string" param="Array::neighbourhood_Address"/>
		<!-- <Attribute name="from" type="string" size="20" mutation="fixed"/> -->
	</Struct>

	<Struct name="Block">
		<Attribute name="header" type="Struct" ref="Header"/>
		<Attribute name="uncles" type="array" ref="Header"/>
		<Attribute name="transactions" type="array" ref="Transaction"/>
		<Attribute name="Hash" type="hash" size="32" mutation="func_hash" param="transactions"/>
		<Attribute name="size" type="number" mutation="func_sizeof" param="transactions" />
		<Attribute name="TD" type="Big_Number" mutation="random_BigNumber" />
		<Attribute name="ReceivedAt" type="time" mutation="random_time" />
		<Attribute name="ReceivedFrom" type="string" size="20" mutation="random_string" param="Array::neighbourhood_Address"/>
		<!-- <Attribute name="ReceivedFrom" type="string" size="20" mutation="fixed"/> -->
	</Struct>

</LOKI>