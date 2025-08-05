// pack_writer.rs - Exact translation of pack_writer.go

use crate::pack_types::*;
use crate::memory::write_buf_str;
use std::io::Write;

impl Pack {
    // AddStr add string value
    pub fn add_str(&mut self, name: &str, str_val: &str) -> Option<&Element> {
        let e = Element {
            name: name.to_string(),
            type_: ValueType::Str,
            values: vec![Value::Str(str_val.to_string())],
            json_hint_is_array: false,
            json_hint_is_bool: false,
            json_hint_is_date_time: false,
            json_hint_is_ip: false,
            json_hint_group_name: String::new(),
        };
        if self.add_element(e).is_err() {
            return None;
        }
        self.elements.last()
    }

    // AddBool add bool (as integer)
    pub fn add_bool(&mut self, name: &str, b: bool) -> Option<&Element> {
        let v = if b { 1u32 } else { 0u32 };
        self.add_int(name, v)
        // Note: JsonHint_IsBool not implemented yet
    }

    // AddInt add integer value
    pub fn add_int(&mut self, name: &str, i: u32) -> Option<&Element> {
        let e = Element {
            name: name.to_string(),
            type_: ValueType::Int,
            values: vec![Value::Int(i)],
            json_hint_is_array: false,
            json_hint_is_bool: false,
            json_hint_is_date_time: false,
            json_hint_is_ip: false,
            json_hint_group_name: String::new(),
        };
        if self.add_element(e).is_err() {
            return None;
        }
        self.elements.last()
    }

    // AddData add data value
    pub fn add_data(&mut self, name: &str, data: Vec<u8>) -> Option<&Element> {
        let e = Element {
            name: name.to_string(),
            type_: ValueType::Data,
            values: vec![Value::Data(data)],
            json_hint_is_array: false,
            json_hint_is_bool: false,
            json_hint_is_date_time: false,
            json_hint_is_ip: false,
            json_hint_group_name: String::new(),
        };
        if self.add_element(e).is_err() {
            return None;
        }
        self.elements.last()
    }

    // AddIp32 add ipv4 (matches SoftEther stable PackAddIp)
    pub fn add_ip32(&mut self, name: &str, ip: u32) -> Option<&Element> {
        // Note: JsonHint_IsIP not implemented yet
        self.add_bool(&format!("{}@ipv6_bool", name), false);
        self.add_data(&format!("{}@ipv6_array", name), vec![0u8; 16]);
        self.add_int(&format!("{}@ipv6_scope_id", name), 0);

        // Store IP in network byte order (big-endian) to match SoftEther stable
        self.add_int(name, ip)
    }

    // AddIp add IPv4 from string (helper method)
    pub fn add_ip(&mut self, name: &str, ip_str: &str) -> Option<&Element> {
        if let Ok(addr) = ip_str.parse::<std::net::Ipv4Addr>() {
            let octets = addr.octets();
            // Convert to network byte order (big-endian) to match SoftEther stable
            let ip_int = ((octets[0] as u32) << 24) |
                        ((octets[1] as u32) << 16) |
                        ((octets[2] as u32) << 8) |
                        (octets[3] as u32);
            self.add_ip32(name, ip_int)
        } else {
            self.add_ip32(name, 0) // Invalid IP becomes 0.0.0.0
        }
    }

    // ToBuf To buffer
    pub fn to_buf(&self) -> Result<Vec<u8>, PackError> {
        let mut buf = Vec::new();
        
        buf.extend_from_slice(&(self.elements.len() as u32).to_be_bytes());
        
        for e in &self.elements {
            e.write(&mut buf)?;
        }
        
        Ok(buf)
    }
}

impl Element {
    pub fn write<W: Write>(&self, w: &mut W) -> Result<(), PackError> {
        write_buf_str(w, &self.name).map_err(|_| PackError::IoError)?;
        
        w.write_all(&(self.type_ as u32).to_be_bytes()).map_err(|_| PackError::IoError)?;
        
        w.write_all(&(self.num_value() as u32).to_be_bytes()).map_err(|_| PackError::IoError)?;
        
        for v in &self.values {
            v.write(w, self.type_)?;
        }
        
        Ok(())
    }
}

impl Value {
    pub fn write<W: Write>(&self, w: &mut W, t: ValueType) -> Result<(), PackError> {
        match (self, t) {
            (Value::Int(val), ValueType::Int) => {
                w.write_all(&val.to_be_bytes()).map_err(|_| PackError::IoError)?;
            }
            (Value::Int64(val), ValueType::Int64) => {
                w.write_all(&val.to_be_bytes()).map_err(|_| PackError::IoError)?;
            }
            (Value::Data(data), ValueType::Data) => {
                let s = data.len() as i32;
                w.write_all(&s.to_be_bytes()).map_err(|_| PackError::IoError)?;
                w.write_all(data).map_err(|_| PackError::IoError)?;
            }
            (Value::Str(string), ValueType::Str) => {
                let b = string.as_bytes();
                let s = b.len() as u32;
                w.write_all(&s.to_be_bytes()).map_err(|_| PackError::IoError)?;
                w.write_all(b).map_err(|_| PackError::IoError)?;
            }
            (Value::UniStr(_), ValueType::UniStr) => {
                panic!("unimplemented");
            }
            _ => {
                return Err(PackError::InvalidType);
            }
        }
        Ok(())
    }
}
