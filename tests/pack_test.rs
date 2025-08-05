// tests/pack_test.rs - Standalone test for Pack functionality

use mayaqua::*;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Rust Pack Implementation ===");
    
    // Create a new pack
    let mut pack = Pack::new();
    
    // Test AddStr
    pack.add_str("test_string", "Hello World");
    pack.add_int("test_int", 12345);
    pack.add_bool("test_bool", true);
    pack.add_data("test_data", vec![0x01, 0x02, 0x03, 0x04]);
    
    println!("Pack has {} elements", pack.elements.len());
    
    // Test serialization
    let buf = pack.to_buf()?;
    println!("Serialized pack size: {} bytes", buf.len());
    println!("First 16 bytes: {:?}", &buf[..16]);
    
    // Test deserialization
    let mut cursor = Cursor::new(&buf);
    let pack2 = read_pack(&mut cursor)?;
    println!("Deserialized pack has {} elements", pack2.elements.len());
    
    // Verify data integrity
    for elem in &pack2.elements {
        println!("Element: {}, Type: {:?}, Values: {}", 
            elem.name, elem.type_ as u32, elem.values.len());
    }
    
    // Test getter methods
    println!("test_string: '{}'", pack2.get_str("test_string"));
    println!("test_int: {}", pack2.get_int("test_int"));
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_serialization() {
        let mut pack = Pack::new();
        pack.add_str("test", "value");
        pack.add_int("number", 123);
        
        let buf = pack.to_buf().unwrap();
        assert!(!buf.is_empty());
        
        let mut cursor = Cursor::new(&buf);
        let pack2 = read_pack(&mut cursor).unwrap();
        
        assert_eq!(pack2.elements.len(), 2);
        assert_eq!(pack2.get_str("test"), "value");
        assert_eq!(pack2.get_int("number"), 123);
    }
}
