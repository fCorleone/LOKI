<LOKI specname="fabric_example"> 
    <Message name="p2p_packet"> 
        <Attribute name="message type" type=String /> 
    </Message> 
    
    <Message name="viewchange">  
        <Attribute name="message_type" type=String /> 
        <Attribute name="view" type=Number ref = "testref"/> 
        <Attribute name="PBFTMessage" type=Struct />
        <Attribute name="parents" type=Array ele_type="Header" />
        <Attribute name="testOneof" type=Oneof option="String test1, Int test2, p2p_packet test3"/>
        
        <ExpectedMsg name="A" identify="1,2" excptedIdentify="A.1,A.2" /> 
    </Message> 

</LOKI>