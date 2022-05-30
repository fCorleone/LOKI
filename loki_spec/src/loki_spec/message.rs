use anyhow::{anyhow, Result};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Message {
    name: String,
    attrs: Vec<Attribute>,
    expect_msgs: Vec<Expectedmsg>,
}

#[allow(dead_code)]
impl Message {
    /// construct a new message with content
    pub fn new(name: String, attrs: Vec<Attribute>, expect_msgs: Vec<Expectedmsg>) -> Self {
        Self {
            name,
            attrs,
            expect_msgs,
        }
    }

    /// get the attribute by its name
    pub fn get_attr_by_name(&self, name: String) -> Result<&Attribute> {
        for attr in self.get_attrs() {
            if attr.get_attr_name() == name {
                return Ok(attr);
            }
        }
        return Err(anyhow!("cannot find the attr with name {}", name));
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) -> bool {
        self.name = name;
        true
    }

    pub fn get_attrs(&self) -> &Vec<Attribute> {
        &self.attrs
    }

    pub fn get_mut_attrs(&mut self) -> &mut Vec<Attribute> {
        &mut self.attrs
    }

    pub fn get_attrs_len(&self) -> usize {
        self.attrs.len()
    }

    pub fn set_attrs(&mut self, attrs: Vec<Attribute>) -> Result<bool> {
        self.attrs = attrs;
        Ok(true)
    }

    pub fn push_attr(&mut self, attr: Attribute) -> Result<bool> {
        self.attrs.push(attr);
        Ok(true)
    }

    pub fn get_expect_msgs(&self) -> &Vec<Expectedmsg> {
        &self.expect_msgs
    }

    pub fn get_mut_expect_msgs(&mut self) -> &mut Vec<Expectedmsg> {
        &mut self.expect_msgs
    }

    pub fn get_expect_msgs_len(&self) -> usize {
        self.expect_msgs.len()
    }

    pub fn set_expect_msgs(&mut self, expect_msgs: Vec<Expectedmsg>) -> Result<bool> {
        self.expect_msgs = expect_msgs;
        Ok(true)
    }

    pub fn push_expect_msg(&mut self, expect_msg: Expectedmsg) -> Result<bool> {
        self.expect_msgs.push(expect_msg);
        Ok(true)
    }
}

pub fn copy_message(msg: &Message) -> Message {
    let mut ret = Message::default();
    ret.name = msg.name.clone();

    for i in 0..msg.attrs.len() {
        let attr = if let Some(attr) = msg.attrs.get(i) {
            attr
        } else {
            todo!() //TODO
        };
        ret.attrs.push(copy_attribute(attr));
    }

    for i in 0..msg.expect_msgs.len() {
        let expect_msg = if let Some(expect_msg) = msg.expect_msgs.get(i) {
            expect_msg
        } else {
            todo!() //TODO
        };
        ret.expect_msgs.push(copy_expected_msg(expect_msg));
    }

    return ret;
}

pub fn construct_message() -> Message {
    let msg = Message::default();

    return msg;
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    attr_name: String,
    attr_type: String,
    attr_ele_type: String,
    attr_reff: String,
    attr_size: String,
    attr_option: String,
    attr_key_type: String,
    attr_value_type: String,
    attr_mutator: String,
    attr_param: String,
    attr_algo: String,
}

#[allow(dead_code)]
impl Attribute {
    /// construct a new Attribute with content
    pub fn new(
        attr_name: String,
        attr_type: String,
        attr_ele_type: String,
        attr_reff: String,
        attr_size: String,
        attr_option: String,
        attr_key_type: String,
        attr_value_type: String,
        attr_mutator: String,
        attr_param: String,
        attr_algo: String,
    ) -> Self {
        Self {
            attr_name,
            attr_type,
            attr_ele_type,
            attr_reff,
            attr_size,
            attr_option,
            attr_key_type,
            attr_value_type,
            attr_mutator,
            attr_param,
            attr_algo,
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

    pub fn get_attr_ele_type(&self) -> String {
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

    pub fn get_attr_option(&self) -> String {
        self.attr_option.clone()
    }

    pub fn set_attr_option(&mut self, attr_option: String) -> bool {
        self.attr_option = attr_option;
        true
    }

    pub fn get_attr_key_type(&self) -> String {
        self.attr_key_type.clone()
    }

    pub fn set_attr_key_type(&mut self, attr_key_type: String) -> bool {
        self.attr_key_type = attr_key_type;
        true
    }

    pub fn get_attr_value_type(&self) -> String {
        self.attr_value_type.clone()
    }

    pub fn set_attr_value_type(&mut self, attr_value_type: String) -> bool {
        self.attr_value_type = attr_value_type;
        true
    }

    pub fn get_attr_mutator(&self) -> String {
        self.attr_mutator.clone()
    }

    pub fn set_attr_mutator(&mut self, attr_mutator: String) -> bool {
        self.attr_mutator = attr_mutator;
        true
    }

    pub fn get_attr_param(&self) -> String {
        self.attr_param.clone()
    }

    pub fn set_attr_param(&mut self, attr_param: String) -> bool {
        self.attr_param = attr_param;
        true
    }

    pub fn get_attr_algo(&self) -> String {
        self.attr_algo.clone()
    }

    pub fn set_attr_algo(&mut self, attr_algo: String) -> bool {
        self.attr_algo = attr_algo;
        true
    }
}

pub fn copy_attribute(attr: &Attribute) -> Attribute {
    let mut ret = construct_attribute();
    ret.attr_name = attr.attr_name.clone();
    ret.attr_type = attr.attr_type.clone();
    ret.attr_ele_type = attr.attr_ele_type.clone();
    ret.attr_reff = attr.attr_reff.clone();
    ret.attr_size = attr.attr_size.clone();
    ret.attr_option = attr.attr_option.clone();
    ret.attr_key_type = attr.attr_key_type.clone();
    ret.attr_value_type = attr.attr_value_type.clone();
    ret.attr_mutator = attr.attr_mutator.clone();
    ret.attr_param = attr.attr_param.clone();
    ret.attr_algo = attr.attr_algo.clone();

    return ret;
}

pub fn construct_attribute() -> Attribute {
    let attr = Attribute::default();

    return attr;
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Expectedmsg {
    expect_name: String,
    identify: String,
    expect_identify: String,
}

#[allow(dead_code)]
impl Expectedmsg {
    /// construct a new Expectedmsg with content
    pub fn new(expect_name: String, identify: String, expect_identify: String) -> Self {
        Self {
            expect_name,
            identify,
            expect_identify,
        }
    }

    pub fn get_expect_name(&self) -> String {
        self.expect_name.clone()
    }

    pub fn set_expect_name(&mut self, expect_name: String) -> bool {
        self.expect_name = expect_name;
        true
    }

    pub fn get_identify(&self) -> String {
        self.identify.clone()
    }

    pub fn set_identify(&mut self, identify: String) -> bool {
        self.identify = identify;
        true
    }

    pub fn get_expect_identify(&self) -> String {
        self.expect_identify.clone()
    }

    pub fn set_expect_identify(&mut self, expect_identify: String) -> bool {
        self.expect_identify = expect_identify;
        true
    }
}

pub fn copy_expected_msg(expect_msg: &Expectedmsg) -> Expectedmsg {
    let mut ret = construct_expected_msg();
    ret.expect_name = expect_msg.expect_name.clone();
    ret.identify = expect_msg.identify.clone();
    ret.expect_identify = expect_msg.expect_identify.clone();

    return ret;
}

pub fn construct_expected_msg() -> Expectedmsg {
    let expect_msg = Expectedmsg::default();

    return expect_msg;
}
