## LOKI v1.0接口设计（手工定义state spec）

### 1.Engine:

**Public interface:**

1. passiveSending(); passiveSending(initial_msgs[]) -> nextMsg 

    被动发包，可接受初始种子,nextMsg是JSON/protobuf格式

2) activeSending(nodes_addrs[], callback()) -> void;

   主动发包，启动一个线程，参数是所有节点的address和一个回调函数，用于调用链自己的发包接口

3. setNodeAddr(new_addrs[]) -> bool;

   更新除了LOKI节点之外的node addresses 

4. addMessagePool(Message)



**Private/Protected interface**:





### 2.Message Spec（elemet：next_msg）

**Public interface:**

1. specParser(xml_file) -> bool

   解析spec 的文件

**Private/Protected interface**:

1. parseXml(xml_root_element)->(msg[], state_model: WeightedGraph)

2. parseNextStates(xml_element)-> state[];

​		根据当前element包含的nextMessage属性获取状态信息

3. parseMessage(xml_element)->msg[];
4. addMessageList(msg[]) -> msg[];/addMessage(msg) -> msg[];
5. updateStateModel(state_model, state[])->state_model;



### 3.Adapter Interface

1. wrap()/unwrap();s

   格式转换，把JSON/protobuf类型的Msg与被测链的Message类型转换

   1.1 encrypt()/decrypt();

   1.2 serialize()/deserialize();

2. callback(map<addr, msg>)；

   被测链的broadcast或者sendpacket接口，按照它规定的格式进行参数准备

3. sign_callback(field) -> string;

   签名函数回调，engine仅传入需要的签名的field，接口层根据keypair做签名





Spec Xml Design:

```xml
<LOKI xmlns:https://www.lokifuzzer.com/2022/v1>
    <Packet name="a_name" next="next_possible_states">
      	<String name="a_name" value="SomeString" length=1 lengthType="bytes"
                minoccurs=1 maxoccurs=1 mutable="random/slight/constant/custom"
                mutref="s1"/>
      	<Number name="a_name" value=10 length=1 lengthType="bytes" endian="big"
                signed="true"  minoccurs=1 maxoccurs=1 valueType="hex/string"
                mutable="random/slight/constant/custom" mutref="s1">
          	<Relation type="size" of="a_name"/>
          	<Relation type="count" of="a_name"/>
          	<Relation type="offset" of="a_name" />
      	</Number>
      	<Block name="a_name" minoccurs=1 maxoccurs=1 
               mutable="random/slight/constant/custom" mutref="s1" size=""/>
      	<Choice name="a_name" minoccurs=1 maxoccurs=1 occurs=1/>
      	<Flags name="a_name" size="16" mutable="random/slight/constant/custom" mutref="s1">
  					<Flag name="a_name" position="0" size="1" value=1 valueType="hex/string"
                  mutable="random/slight/constant/custom" mutref="s1"/>
				</Flags>
      	<Padding name="a_name" aligned="true" alignment="8" alignedTo="a_name" 
                mutable="random/slight/constant/custom" mutref="s1" />
      	<Signature name="a_name">
          	<Field name="f1" ref="a_name"/>
      	</Signature>
      	<Hash name="a_name" algo="SHA256">
          	<Field name="f1" ref="a_name"/>
      	</Hash>
      	<TimeStamp></TimeStamp>
      	<BigNumber></BigNumber>
    </Packet>
  	<States name="next_possible_states">
      	<State name="a_state" ref_model="a_name"/>
      	<State name="b_state" ref_model="b_name"/>
  	</States>
  	<CustomMutation name="s1" fieldType="number" minValue="0" maxValue="100"
                    basedOnCur="true" fluctuation="10" valueList="[a,b,c,d]"/>
</LOKI>

```





### The protobuf encoder and decoder
LOKI now supports the automatically generation of protobuf encoder and decoder.
But remeber to modify two code snippets in the dependency library: protobuf-codegen
The first is at line 119:
```rust
// Delete this line:
// let cargo_out_dir = env::var("OUT_DIR").expect("OUT_DIR env var not set");
// and change to:
let cargo_out_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env var not set");
        
```

In addition, delete the following code at line 223 in the function 'run'
```rust
// if self.create_out_dir {
//     if out_dir.exists() {
//         fs::remove_dir_all(&out_dir)?;
//     }
//     fs::create_dir(&out_dir)?;
// }
```

Then You can use the protobuf automatical generator. First move all your proto files into `core/src/protos`, then just run `cargo build` and the `build.rs` will generate two files `decode_map_to_buffer.rs` and `encode_map_to_buffer.rs` for you! Enjoy!!