<LOKI specname="fabric_example"> 
    <Message name="p2ppacket"> 
        <Attribute name="message type" type=String /> 
    </Message> 
    
    <Message name="viewchange">  
        <Attribute name="message type" type=String /> 
        <Attribute name="view" type=Number /> 
        <Attribute name="PBFTMessage" type=Struct ref="p2ppacket"/>
        <Attribute name="parents" type=Array ref="String" />
        
        <ExpectedMsg name="A" identify="1,2" excptedIdentify="A.1,A.2" /> 
    </Message> 

</LOKI>