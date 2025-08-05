// pack_types.rs - Exact translation of pack_types.go

use std::fmt;
use std::error::Error;

// Constants - exact same as Go
pub const MAX_VALUE_SIZE: u32 = 384 * 1024 * 1024; // 384MB
pub const MAX_VALUE_NUM: u32 = 65536;
pub const MAX_ELEMENT_NUM: u32 = 65536;
pub const MAX_PACK_SIZE: u32 = 512 * 1024 * 1024; // 512MB

// ValueType - exact same values as Go
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ValueType {
    Int = 0,
    Data = 1,
    Str = 2,
    UniStr = 3,
    Int64 = 4,
}

impl ValueType {
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(ValueType::Int),
            1 => Some(ValueType::Data),
            2 => Some(ValueType::Str),
            3 => Some(ValueType::UniStr),
            4 => Some(ValueType::Int64),
            _ => None,
        }
    }
}

// PackError - exact same error types as Go
#[derive(Debug, Clone)]
pub enum PackError {
    NumberExceeds,
    SizeOver,
    InvalidType,
    SameNameExists,
    ZeroNumValue,
    IoError,
}

impl fmt::Display for PackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PackError::NumberExceeds => write!(f, "Number exceeds"),
            PackError::SizeOver => write!(f, "Size over"),
            PackError::InvalidType => write!(f, "Invalid type"),
            PackError::SameNameExists => write!(f, "Same name exists"),
            PackError::ZeroNumValue => write!(f, "Zero num value"),
            PackError::IoError => write!(f, "IO error"),
        }
    }
}

impl Error for PackError {}

// Value - exact same as Go (enum, not struct!)
#[derive(Debug, Clone)]
pub enum Value {
    Int(u32),
    Data(Vec<u8>),
    Str(String),
    UniStr(String),
    Int64(u64),
}

// Element - exact same structure as Go
#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub type_: ValueType,    // Go field: Type
    pub values: Vec<Value>,
    
    // JSON hints - exact same as Go
    pub json_hint_is_array: bool,
    pub json_hint_is_bool: bool,
    pub json_hint_is_date_time: bool,
    pub json_hint_is_ip: bool,
    pub json_hint_group_name: String,
}

impl Element {
    pub fn new(name: String, type_: ValueType) -> Self {
        Self {
            name,
            type_,
            values: Vec::new(),
            json_hint_is_array: false,
            json_hint_is_bool: false,
            json_hint_is_date_time: false,
            json_hint_is_ip: false,
            json_hint_group_name: String::new(),
        }
    }

    // NumValue - exact same as Go
    pub fn num_value(&self) -> usize {
        self.values.len()
    }

    // GetIntValue - exact same as Go
    pub fn get_int_value(&self, index: u32) -> u32 {
        if let Some(Value::Int(val)) = self.values.get(index as usize) {
            *val
        } else {
            0
        }
    }

    // GetStrValue - exact same as Go
    pub fn get_str_value(&self, index: u32) -> String {
        if let Some(Value::Str(val)) = self.values.get(index as usize) {
            val.clone()
        } else {
            String::new()
        }
    }

    // GetDataValue - exact same as Go
    pub fn get_data_value(&self, index: u32) -> Vec<u8> {
        if let Some(Value::Data(val)) = self.values.get(index as usize) {
            val.clone()
        } else {
            Vec::new()
        }
    }
}

// Pack - exact same structure as Go
#[derive(Debug, Clone)]
pub struct Pack {
    pub elements: Vec<Element>,
}

impl Pack {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    // AddElement - exact same logic as Go
    pub fn add_element(&mut self, e: Element) -> Result<(), PackError> {
        if e.values.is_empty() {
            return Err(PackError::ZeroNumValue);
        }

        // Check for duplicate names - exact same as Go
        for existing in &self.elements {
            if existing.name == e.name {
                return Err(PackError::SameNameExists);
            }
        }

        self.elements.push(e);
        Ok(())
    }

    // GetNum - exact same as Go
    pub fn get_num(&self) -> usize {
        self.elements.len()
    }

    // GetElement - exact same logic as Go (case-insensitive)
    pub fn get_element(&self, name: &str, t: Option<ValueType>) -> Option<&Element> {
        let n = name.to_uppercase();
        for e in &self.elements {
            if n == e.name.to_uppercase() && (t.is_none() || t == Some(e.type_)) {
                return Some(e);
            }
        }
        None
    }

    // GetInt - exact same as Go
    pub fn get_int(&self, name: &str) -> u32 {
        self.get_int_ex(name, 0)
    }

    // GetIntEx - exact same as Go
    pub fn get_int_ex(&self, name: &str, index: u32) -> u32 {
        if let Some(e) = self.get_element(name, Some(ValueType::Int)) {
            e.get_int_value(index)
        } else {
            0
        }
    }

    // GetStr - exact same as Go
    pub fn get_str(&self, name: &str) -> String {
        self.get_str_ex(name, 0)
    }

    // GetStrEx - exact same as Go
    pub fn get_str_ex(&self, name: &str, index: u32) -> String {
        if let Some(e) = self.get_element(name, Some(ValueType::Str)) {
            e.get_str_value(index)
        } else {
            String::new()
        }
    }

    // GetData - exact same as Go
    pub fn get_data(&self, name: &str) -> Vec<u8> {
        self.get_data_ex(name, 0)
    }

    // GetDataEx - exact same as Go
    pub fn get_data_ex(&self, name: &str, index: u32) -> Vec<u8> {
        if let Some(e) = self.get_element(name, Some(ValueType::Data)) {
            e.get_data_value(index)
        } else {
            Vec::new()
        }
    }

    // GetBool - exact same as Go
    pub fn get_bool(&self, name: &str) -> bool {
        self.get_bool_ex(name, 0)
    }

    // GetBoolEx - exact same as Go
    pub fn get_bool_ex(&self, name: &str, index: u32) -> bool {
        self.get_int_ex(name, index) != 0
    }

    // GetArray - returns all values in an element as a vector
    pub fn get_array(&self, name: &str) -> Option<Vec<Value>> {
        if let Some(element) = self.get_element(name, None) {
            Some(element.values.clone())
        } else {
            None
        }
    }

    // GetDataSize - exact same as Go
    pub fn get_data_size(&self, name: &str) -> u32 {
        self.get_data_size_ex(name, 0)
    }

    // GetDataSizeEx - exact same as Go
    pub fn get_data_size_ex(&self, name: &str, index: u32) -> u32 {
        self.get_data_ex(name, index).len() as u32
    }

    // GetIp - get IPv4 address from int value (matches SoftEther stable PackGetIp)
    pub fn get_ip(&self, name: &str) -> String {
        self.get_ip_ex(name, 0)
    }

    // GetIpEx - get IPv4 address from int value at specific index
    pub fn get_ip_ex(&self, name: &str, index: u32) -> String {
        let ip_int = self.get_int_ex(name, index);
        if ip_int == 0 {
            return String::new();
        }
        
        // Convert from network byte order (big-endian) to IPv4 string
        // SoftEther stores IPs as big-endian 32-bit integers
        format!("{}.{}.{}.{}", 
            (ip_int >> 24) & 0xFF,
            (ip_int >> 16) & 0xFF, 
            (ip_int >> 8) & 0xFF,
            ip_int & 0xFF
        )
    }

    // GetIpAsBytes - get IPv4 address as 4-byte array
    pub fn get_ip_as_bytes(&self, name: &str) -> [u8; 4] {
        let ip_int = self.get_int(name);
        if ip_int == 0 {
            return [0, 0, 0, 0];
        }
        
        // Convert from network byte order to byte array
        [
            ((ip_int >> 24) & 0xFF) as u8,
            ((ip_int >> 16) & 0xFF) as u8,
            ((ip_int >> 8) & 0xFF) as u8,
            (ip_int & 0xFF) as u8,
        ]
    }
}
