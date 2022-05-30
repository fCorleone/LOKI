// use anyhow::{anyhow, Result};
use anyhow::{ Result};


#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ProtoEnum {
    name: String,
    attrs: Vec<ProtoAttribute>
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ProtoMessage {
    name: String,
    attrs: Vec<ProtoAttribute>,
    path: String,
    same_name: bool,
}


//==========================      PtotoEnum      ==========================/
#[allow(dead_code)]
impl ProtoEnum {
    /// construct a new ProtoMessage with content
    pub fn new(name: String, attrs: Vec<ProtoAttribute>) -> Self {
        Self {
            name,
            attrs,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) -> bool {
        self.name = name;
        true
    }

    pub fn get_attrs(&self) -> &Vec<ProtoAttribute> {
        &self.attrs
    }

    pub fn get_mut_attrs(&mut self) -> &mut Vec<ProtoAttribute> {
        &mut self.attrs
    }

    pub fn get_attrs_len(&self) -> usize {
        self.attrs.len()
    }

    pub fn set_attrs(&mut self, attrs: Vec<ProtoAttribute>) -> Result<bool> {
        self.attrs = attrs;
        Ok(true)
    }

    pub fn push_attr(&mut self, attr: ProtoAttribute) -> Result<bool> {
        self.attrs.push(attr);
        Ok(true)
    }
}

#[allow(dead_code)]
pub fn copy_protoenum(msg: &ProtoEnum) -> ProtoEnum {
    let mut ret = ProtoEnum::default();
    ret.name = msg.name.clone();

    for i in 0..msg.attrs.len() {
        let attr = if let Some(attr) = msg.attrs.get(i) {
            attr
        } else {
            todo!() //TODO
        };
        ret.attrs.push(copy_proto_attribute(attr));
    }
    return ret;
}

pub fn construct_protoenum() -> ProtoEnum {
    let msg = ProtoEnum::default();

    return msg;
}






//==========================      ProtoMessage ==========================/
#[allow(dead_code)]
impl ProtoMessage {
    /// construct a new PtotoMessage with content
    pub fn new(name: String, attrs: Vec<ProtoAttribute>, path: String, same_name: bool) -> Self {
        Self {
            name,
            attrs,
            path,
            same_name,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) -> bool {
        self.name = name;
        true
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn set_path(&mut self, path: String) -> bool {
        self.path = path;
        true
    }

    pub fn is_same_name(&self) -> bool {
        self.same_name
    }

    pub fn set_same_name(&mut self, same_name: bool) -> bool {
        self.same_name = same_name;
        true
    }

    pub fn get_attrs(&self) -> &Vec<ProtoAttribute> {
        &self.attrs
    }

    pub fn get_mut_attrs(&mut self) -> &mut Vec<ProtoAttribute> {
        &mut self.attrs
    }

    pub fn get_attrs_len(&self) -> usize {
        self.attrs.len()
    }

    pub fn set_attrs(&mut self, attrs: Vec<ProtoAttribute>) -> Result<bool> {
        self.attrs = attrs;
        Ok(true)
    }

    pub fn push_attr(&mut self, attr: ProtoAttribute) -> Result<bool> {
        self.attrs.push(attr);
        Ok(true)
    }
}

#[allow(dead_code)]
pub fn copy_protomessage(msg: &ProtoMessage) -> ProtoMessage {
    let mut ret = ProtoMessage::default();
    ret.name = msg.name.clone();

    for i in 0..msg.attrs.len() {
        let attr = if let Some(attr) = msg.attrs.get(i) {
            attr
        } else {
            todo!() //TODO
        };
        ret.attrs.push(copy_proto_attribute(attr));
    }
    return ret;
}

pub fn construct_protomessage() -> ProtoMessage {
    let msg = ProtoMessage::default();

    return msg;
}







//==========================      ProtoAttribute    ==========================/
/**
 * Type: (   'double'
        |   'float'
        |   'int32'
        |   'int64'
        |   'uint32'
        |   'uint64'
        |   'sint32'
        |   'sint64'
        |   'fixed32'
        |   'fixed64'
        |   'sfixed32'
        |   'sfixed64'
        |   'bool'
        |   'string'
        |   'bytes'
        |   'message' => Struct.Struct
        |   'enum' => Struct.Struct
        |   'repeated' => array
        |   'oneof' : list options, <option:type name>, <option>, <>, ... <> =>Oneof
        |   ‘map’:  => Map
        )
 */
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ProtoAttribute {
    attr_type: String,
    attr_name: String,
    attr_ele_type: String,
    attr_reff: String,
    attr_size: String,
    attr_value: String,
    options: String,
    key_type: String,
    value_type: String,
}

#[allow(dead_code)]
impl ProtoAttribute {
    /// construct a new Attribute with content
    pub fn new(
        attr_name: String,
        attr_type: String,
        attr_ele_type: String,
        attr_reff: String,
        attr_size: String,
        attr_value: String,
        options: String,
        key_type: String,
        value_type: String,
    ) -> Self {
        Self {
            attr_name,
            attr_type,
            attr_ele_type,
            attr_reff,
            attr_size,
            attr_value,
            options,
            key_type,
            value_type,
        }
    }

    pub fn get_attr_name(&self) -> String {
        self.attr_name.clone()
    }

    pub fn set_attr_name(&mut self, attr_name: String) -> bool {
        self.attr_name = attr_name;
        true
    }

    pub fn get_attr_type(&self) -> String {
        self.attr_type.clone()
    }

    pub fn set_attr_type(&mut self, attr_type: String) -> bool {
        self.attr_type = attr_type;
        true
    }

    pub fn get_attr_ele_type(&self) -> String{
        self.attr_ele_type.clone()
    }

    pub fn set_attr_ele_type(&mut self, attr_ele_type: String) -> bool {
        self.attr_ele_type = attr_ele_type;
        true
    }

    pub fn get_attr_reff(&self) -> String {
        self.attr_reff.clone()
    }

    pub fn set_attr_reff(&mut self, attr_reff: String) -> bool {
        self.attr_reff = attr_reff;
        true
    }

    pub fn get_attr_size(&self) -> String {
        self.attr_size.clone()
    }

    pub fn set_attr_size(&mut self, attr_size: String) -> bool {
        self.attr_size = attr_size;
        true
    }

    pub fn get_attr_value(&self) -> String {
        self.attr_value.clone()
    }

    pub fn set_attr_value(&mut self, attr_value: String) -> bool {
        self.attr_value = attr_value;
        true
    }

    pub fn get_options(&self) -> String {
        self.options.clone()
    }

    pub fn set_options(&mut self, options: String) -> bool {
        self.options = options;
        true
    }

    pub fn get_key_type(&self) -> String {
        self.key_type.clone()
    }

    pub fn set_key_type(&mut self, key_type: String) -> bool {
        self.key_type = key_type;
        true
    }

    pub fn get_value_type(&self) -> String {
        self.key_type.clone()
    }

    pub fn set_value_type(&mut self, value_type: String) -> bool {
        self.value_type = value_type;
        true
    }
}


#[allow(dead_code)]
pub fn copy_proto_attribute(attr: &ProtoAttribute) -> ProtoAttribute {
    let mut ret = construct_proto_attribute();
    ret.attr_name = attr.attr_name.clone();
    ret.attr_type = attr.attr_type.clone();
    ret.attr_ele_type = attr.attr_ele_type.clone();
    ret.attr_reff = attr.attr_reff.clone();
    ret.attr_size = attr.attr_size.clone();
    ret.attr_value = attr.attr_value.clone();
    ret.options = attr.options.clone();
    ret.key_type = attr.key_type.clone();
    ret.value_type = attr.value_type.clone();

    return ret;
}

pub fn construct_proto_attribute() -> ProtoAttribute {
    let attr = ProtoAttribute::default();

    return attr;
}





// //==========================      OneOf ==========================/
// #[derive(Default, Debug, Clone, PartialEq, Eq)]
// pub struct OneOf {
//     name: String,
//     attrs: Vec<ProtoAttribute>
// }


// #[allow(dead_code)]
// impl OneOf {
//     /// construct a new PtotoMessage with content
//     pub fn new(name: String, attrs: Vec<ProtoAttribute>) -> Self {
//         Self {
//             name,
//             attrs,
//         }
//     }

//     pub fn get_name(&self) -> String {
//         self.name.clone()
//     }

//     pub fn set_name(&mut self, name: String) -> bool {
//         self.name = name;
//         true
//     }

//     pub fn get_attrs(&self) -> &Vec<ProtoAttribute> {
//         &self.attrs
//     }

//     pub fn get_mut_attrs(&mut self) -> &mut Vec<ProtoAttribute> {
//         &mut self.attrs
//     }

//     pub fn get_attrs_len(&self) -> usize {
//         self.attrs.len()
//     }

//     pub fn set_attrs(&mut self, attrs: Vec<ProtoAttribute>) -> Result<bool> {
//         self.attrs = attrs;
//         Ok(true)
//     }

//     pub fn push_attr(&mut self, attr: ProtoAttribute) -> Result<bool> {
//         self.attrs.push(attr);
//         Ok(true)
//     }
// }


// pub fn copy_oneof(msg: &OneOf) -> OneOf {
//     let mut ret = OneOf::default();
//     ret.name = msg.name.clone();

//     for i in 0..msg.attrs.len() {
//         let attr = if let Some(attr) = msg.attrs.get(i) {
//             attr
//         } else {
//             todo!() //TODO
//         };
//         ret.attrs.push(copy_proto_attribute(attr));
//     }
//     return ret;
// }

// pub fn construct_oneof() -> OneOf {
//     let msg = OneOf::default();

//     return msg;
// }


