<LOKI specname="fisco_bcos_example"> 
    <Message name="BaseMessage">
        <Attribute name="version" type=Number mutator=random_Bool/>
        <Attribute name="index" type=String/>
        <Attribute name="hash" type=Byte/>
        <Attribute name="view" type=Number mutator=random_Number/>
        <Attribute name="timestamp" type=Timestamp mutator=random_Number/>
        <Attribute name="generatedFrom" type=String />
        <Attribute name="signatureData" type=Byte/>
        <Attribute name="signatureHash" type=Byte/>
    </Message>

    <Message name = "RawMessage">
        <Attribute name="version" type=Number mutator=random_Bool/>
        <Attribute name="type" type=Number mutator=random_Number/>
        <Attribute name="signatureData" type=Byte/>
        <Attribute name="payLoad" type=Byte/>
    </Message>

    <Message name = "ProposalRequest">
        <Attribute name="message" type=Struct ref="BaseMessage" />
        <Attribute name="size" type=Number mutator=random_Number/>
    </Message>

    <Message name = "RawNewViewMessage">
        <Attribute name="message" type=Struct ref="BaseMessage" />
        <Attribute name="viewChangeMsgList" type=Array ele_type="RawViewChangeMessage" />
        <Attribute name="prePrepareList" type=Array ele_type="PBFTRawMessage" />
    </Message>

    <Message name = "RawViewChangeMessage">
        <Attribute name="message" type=Struct ref="BaseMessage" />
        <Attribute name="committedProposal" type=Struct ref="PBFTRawProposal" />
        <Attribute name="preparedProposals" type=Array ele_type="PBFTRawMessage" />
    </Message>

    <Message name = "PBFTRawMessage">
        <Attribute name="hashFieldsData" type=Byte/>
        <Attribute name="consensusProposal" type=Struct ref="PBFTRawProposal" />
        <Attribute name="proposals" type=Array ele_type="PBFTRawProposal" />
        <Attribute name="signatureData" type=Byte />
    </Message>

    <Message name = "PBFTRawProposal">
        <Attribute name="proposal" type=Struct ref="RawProposal" />
        <Attribute name="nodeList" type=Array ele_type="String" />
        <Attribute name="signatureList" type=Array ele_type="Byte"  />
    </Message>

    <Message name = "RawProposal">
        <Attribute name="index" type=Number mutator=random_Number />
        <Attribute name="hash" type=Byte  />
        <Attribute name="data" type=Byte />
        <Attribute name="signature" type=Byte />
        <Attribute name="extraData" type=Byte />
        <Attribute name="sealerId" type=Number mutator=random_Number />
        <Attribute name="systemProposal" type=Bool mutator=random_Bool />
    </Message>

    <Message name="PrePreparePacket">
        <Attribute name="message" type=Struct ref="PBFTRawMessage"/>
        <ExpectedMsg name="PreparePacket" identify="message" excptedIdentify="message"/> 
        <ExpectedMsg name="ViewChangePacket" identify="message" excptedIdentify="message"/> 
    </Message>
    
    <Message name="PreparePacket">
        <Attribute name="message" type=Struct ref="PBFTRawMessage"/>
        <ExpectedMsg name="CommitPacket" identify="message" excptedIdentify="message"/> 
        <ExpectedMsg name="ViewChangePacket" identify="message" excptedIdentify="message"/> 
    </Message>

    <Message name="CommitPacket">
        <Attribute name="message" type=Struct ref="PBFTRawMessage"/>
        <ExpectedMsg name="PrePreparePacket" identify="message" excptedIdentify="message"/> 
        <ExpectedMsg name="ViewChangePacket" identify="message" excptedIdentify="message"/> 
    </Message>

    <Message name="CommittedProposalResponse">
        <Attribute name="message" type=Struct ref="PBFTRawMessage"/>
        <ExpectedMsg name="ViewChangePacket" identify="message" excptedIdentify="message"/> 
    </Message>
    
    <Message name="CheckPoint">
        <Attribute name="message" type=Struct ref="PBFTRawMessage"/>
        <ExpectedMsg name="ViewChangePacket" identify="message" excptedIdentify="message"/> 
    </Message>

    <Message name="RecoverRequest">
        <Attribute name="message" type=Struct ref="PBFTRawMessage"/>
        <ExpectedMsg name="ViewChangePacket" identify="message" excptedIdentify="message"/> 
    </Message>

    <Message name="RecoverResponse">
       <Attribute name="message" type=Struct ref="PBFTRawMessage"/>
       <ExpectedMsg name="ViewChangePacket" identify="message" excptedIdentify="message"/> 
    </Message>

    <Message name="ViewChangePacket">
       <Attribute name="message" type=Struct ref="RawViewChangeMessage"/>
       <ExpectedMsg name="ViewChangePacket" identify="message" excptedIdentify="message"/> 
    </Message>

    <Message name="PreparedProposalResponse">
       <Attribute name="message" type=Struct ref="RawViewChangeMessage"/>
       <ExpectedMsg name="ViewChangePacket" identify="message" excptedIdentify="message"/> 
    </Message>
    
    <Message name="NewViewPacket">
       <Attribute name="message" type=Struct ref="RawNewViewMessage"/>
       <ExpectedMsg name="ViewChangePacket" identify="message" excptedIdentify="message"/> 
    </Message>

    <Message name="CommittedProposalRequest">
       <Attribute name="message" type=Struct ref="ProposalRequest"/>
       <ExpectedMsg name="ViewChangePacket" identify="message" excptedIdentify="message"/> 
    </Message>

    <Message name="PreparedProposalRequest">
       <Attribute name="message" type=Struct ref="ProposalRequest"/>
       <ExpectedMsg name="ViewChangePacket" identify="message" excptedIdentify="message"/> 
    </Message>
</LOKI>