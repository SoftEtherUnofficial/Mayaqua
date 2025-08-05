// pack_reader.rs - Exact translation of pack_reader.go

use crate::pack_types::*;
use crate::memory::read_buf_str;
use std::io::Read;

// ReadPack read pack from buf
pub fn read_pack<R: Read>(r: &mut R) -> Result<Pack, PackError> {
    let mut pack = Pack::new();
    
    let mut num_bytes = [0u8; 4];
    r.read_exact(&mut num_bytes).map_err(|_| PackError::IoError)?;
    let num = u32::from_be_bytes(num_bytes);
    
    println!("[DEBUG] Pack has {} elements (max allowed: {})", num, MAX_ELEMENT_NUM);
    
    if num > MAX_ELEMENT_NUM {
        return Err(PackError::NumberExceeds);
    }

    pack.elements.reserve(num as usize);

    for _ in 0..num {
        let e = read_element(r)?;
        pack.add_element(e)?;
    }

    Ok(pack)
}

// ReadElement read element from a reader
pub fn read_element<R: Read>(r: &mut R) -> Result<Element, PackError> {
    let name = read_buf_str(r).map_err(|_| PackError::IoError)?;

    let mut type_bytes = [0u8; 4];
    r.read_exact(&mut type_bytes).map_err(|_| PackError::IoError)?;
    let element_type = ValueType::from_u32(u32::from_be_bytes(type_bytes))
        .ok_or(PackError::InvalidType)?;

    let mut n_bytes = [0u8; 4];
    r.read_exact(&mut n_bytes).map_err(|_| PackError::IoError)?;
    let n = u32::from_be_bytes(n_bytes);
    
    println!("[DEBUG] Element '{}' has {} values (max allowed: {})", name, n, MAX_VALUE_NUM);
    
    if n > MAX_VALUE_NUM {
        return Err(PackError::NumberExceeds);
    }

    let mut values = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let v = read_value(r, element_type)?;
        values.push(v);
    }

    Ok(Element {
        name,
        type_: element_type,
        values,
        json_hint_is_array: false,
        json_hint_is_bool: false,
        json_hint_is_date_time: false,
        json_hint_is_ip: false,
        json_hint_group_name: String::new(),
    })
}

// ReadValue read value from a reader
pub fn read_value<R: Read>(r: &mut R, t: ValueType) -> Result<Value, PackError> {
    match t {
        ValueType::Int => {
            let mut bytes = [0u8; 4];
            r.read_exact(&mut bytes).map_err(|_| PackError::IoError)?;
            Ok(Value::Int(u32::from_be_bytes(bytes)))
        }
        ValueType::Int64 => {
            let mut bytes = [0u8; 8];
            r.read_exact(&mut bytes).map_err(|_| PackError::IoError)?;
            Ok(Value::Int64(u64::from_be_bytes(bytes)))
        }
        ValueType::Data => {
            let mut s_bytes = [0u8; 4];
            r.read_exact(&mut s_bytes).map_err(|_| PackError::IoError)?;
            let s = u32::from_be_bytes(s_bytes);
            
            if s > MAX_VALUE_SIZE {
                return Err(PackError::SizeOver);
            }
            
            let mut data = vec![0u8; s as usize];
            r.read_exact(&mut data).map_err(|_| PackError::IoError)?;
            Ok(Value::Data(data))
        }
        ValueType::Str => {
            let mut s_bytes = [0u8; 4];
            r.read_exact(&mut s_bytes).map_err(|_| PackError::IoError)?;
            let s = u32::from_be_bytes(s_bytes);
            
            if s > MAX_VALUE_SIZE - 1 {
                return Err(PackError::SizeOver);
            }
            
            let mut data = vec![0u8; s as usize];
            r.read_exact(&mut data).map_err(|_| PackError::IoError)?;
            let string = String::from_utf8(data).map_err(|_| PackError::IoError)?;
            Ok(Value::Str(string))
        }
        ValueType::UniStr => {
            panic!("unimplemented");
        }
    }
}
