grammar Loki_spec;

document: loki_begin spec_content* loki_end;

loki_begin: Lsign LOKI  SPECNAME Euqal STRING Rsign ;
loki_end: Lsign '/' LOKI Rsign;
spec_content: message+;

message: msg_begin msg_content* msg_end;
msg_begin: Lsign  Message Name Euqal STRING Rsign;
msg_end: Lsign '/' Message Rsign;
msg_content: attribute+ expectedMsg*;

attribute: attr_begin  attr_end;
attr_begin: Lsign  Attribute Name Euqal STRING Type Euqal attr_type ele_type* reff* size* option* key_type* value_type* mutator* param* algo*;
attr_end: '/' Rsign;

attr_type: 'Number' | 'String'
           | 'Bool' | 'Byte'
           | 'Timestamp'    | 'Hash'
           | 'BigNumber'    | 'Array'
           | 'Signature'    | 'Struct'
           | 'Oneof'        | 'Map';

ele_type: 'ele_type' '=' STRING;
reff: 'ref' '=' STRING;
size: 'size' '=' INT;
option: 'option' '=' STRING;
key_type: 'key_type' '=' STRING;
value_type: 'value_type' '=' STRING;
mutator: 'mutator' '=' mutation;
mutation: 'random_Number' | 'random_String'
        | 'random_Bool' | 'random_Byte'
        | 'random_Timestamp'    | 'func_hash'
        | 'random_BigNumber'    | 'Decreasing'
        | 'Increasing'          | 'edge_value' ;
param: 'param' '=' STRING;
algo: 'algo' '=' STRING;

expectedMsg: exc_begin exc_end;
exc_begin: Lsign ExpectedMsg Name Euqal STRING 'identify' Euqal STRING 'excptedIdentify' Euqal STRING;
exc_end: '/' Rsign;


//Loki 特殊关键字
SPECNAME: 'specname';
LOKI: 'LOKI';
Message:'Message';
Attribute:'Attribute';
Name:'name';
Type:'type';
Unlimited: 'unlimited';
ExpectedMsg: 'ExpectedMsg';
Lsign: '<';
Rsign: '>';
Euqal: '=';



//匹配注释
LINE_COMMENT : '//' .*? '\r'? '\n' -> skip ;    // 匹配"//" stuff '\n'
COMMENT      : '/*' .*? '*/'       -> skip ;    // 匹配"/*" stuff "*/"
WS : [ \t\r\n]+ -> skip ;    // 匹配一个或多个空格但丢弃

ID: [a-zA-Z]+ ; // 匹配一个或多个大小写字母
INT : [0-9]+ ;    // 匹配一个或多个数字

FLOAT: DIGIT+ '.' DIGIT*    // 匹配1. 39. 3.14159等等
     | '.' DIGIT+           // 匹配.1 .14159
     ;
fragment
DIGIT: [0-9] ;              // 匹配单个数字

STRING : '"' .*? '"' ;    // 匹配在双引号中的任意字符
//STRING: '"' (ESC|.)*? '"' ; // 匹配任意在双引号中的任意字符
//fragment
//ESC : '\\"' | '\\\\' ;    // 匹配字符\"和\\



SHEBANG : '#' '!' ~('\n'|'\r')* -> channel(HIDDEN);