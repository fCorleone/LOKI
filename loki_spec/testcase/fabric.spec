<LOKI specname="fabric:example">
	
	<Message name="PrePrepare">
		<Attribute name="View" type=Number size="64" mutation="random_number"/>
		<Attribute name="Seq" type=Number size="64" mutation="random_number" />
		<Attribute name="Proposal" type=Struct ref="Proposal"/>
		<Attribute name="PrevCommitSignatures" type=Array ref="Signature"/>
		<ExpectedMsg mag_name="Prepare" identify="View,Seq" msg_identify="Prepare.View,Prepare.Seq" />
	</Message>

	<Message name="Prepare">
		<Attribute name="View" type=Number size="64" mutation="random_number"/>
		<Attribute name="Seq" type=Number size="64" mutation="random_number" />
		<Attribute name="Digest" type=String size="no_limit" mutation="random_string" />
		<Attribute name="Assist" type=Bool mutation="random_bool" />
 		<ExpectedMsg mag_name="Commit" identify="View,Seq" msg_identify="Commit.View,Commit.Seq" />
	</Message>

	<Message name="Commit">
		<Attribute name="View" type=Number size="64" mutation="random_number"/>
		<Attribute name="Seq" type=Number size="64" mutation="random_number" />
		<Attribute name="Digest" type=String size="no_limit" mutation="random_string" />
		<Attribute name="Assist" type=Bool mutation="random_bool" />
		<Attribute name="Signature" type=Struct ref="Signature"/>
	</Message>

	<Struct>
		<Attribute name="Signer" type=Number size="64" mutation="fixed/random_number"/>
		<Attribute name="Value" type=String size="no_limit" mutation="random_string"/>
		<Attribute name="msg" type=String size="no_limit" mutation="random_string"/>
	</Struct>

</LOKI>