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
    
    // GetNames - get all element names (for debugging)
    pub fn get_names(&self) -> Vec<String> {
        self.elements.iter().map(|e| e.name.clone()).collect()
    }

    // === TUN/TAP Integration Methods ===
    
    // GetNetworkConfig - extract complete network configuration for TUN/TAP setup
    pub fn get_network_config(&self) -> NetworkConfig {
        NetworkConfig {
            client_ip: self.get_ip("client_ip"),
            client_ip_bytes: self.get_ip_as_bytes("client_ip"),
            subnet_mask: self.get_ip("subnet_mask"),
            subnet_mask_bytes: self.get_ip_as_bytes("subnet_mask"),
            gateway_ip: self.get_ip("gateway_ip"),
            gateway_ip_bytes: self.get_ip_as_bytes("gateway_ip"),
            dns_server1: self.get_ip("dns_server1"),
            dns_server2: self.get_ip("dns_server2"),
            dhcp_server: self.get_ip("dhcp_server"),
            domain_name: self.get_str("domain_name"),
            mtu: self.get_int("mtu"),
            use_dhcp: self.get_bool("use_dhcp"),
            lease_time: self.get_int("lease_time"),
        }
    }

    // GetTunTapConfig - extract configuration specifically for TUN/TAP device setup
    pub fn get_tuntap_config(&self, interface_name: &str) -> TunTapConfig {
        let net_config = self.get_network_config();
        
        TunTapConfig {
            interface_name: interface_name.to_string(),
            ip_address: net_config.client_ip,
            subnet_mask: net_config.subnet_mask,
            mtu: if net_config.mtu > 0 { net_config.mtu } else { 1500 },
            gateway: net_config.gateway_ip,
            dns_servers: vec![
                net_config.dns_server1,
                net_config.dns_server2,
            ].into_iter().filter(|dns| !dns.is_empty() && dns != "0.0.0.0").collect(),
            use_dhcp: net_config.use_dhcp,
        }
    }

    // HasValidClientIP - check if packet contains a valid client IP assignment
    pub fn has_valid_client_ip(&self) -> bool {
        let ip = self.get_ip("client_ip");
        !ip.is_empty() && ip != "0.0.0.0" && ip != "255.255.255.255"
    }

    // ExtractDhcpOptions - extract DHCP options from server response
    pub fn extract_dhcp_options(&self) -> DhcpOptions {
        DhcpOptions {
            subnet_mask: self.get_ip("dhcp_subnet_mask"),
            router: self.get_ip("dhcp_router"),
            dns_servers: self.extract_dns_servers(),
            domain_name: self.get_str("dhcp_domain_name"),
            lease_time: self.get_int("dhcp_lease_time"),
            renewal_time: self.get_int("dhcp_renewal_time"),
            rebinding_time: self.get_int("dhcp_rebinding_time"),
            broadcast_address: self.get_ip("dhcp_broadcast"),
        }
    }

    // Helper method to extract multiple DNS servers
    fn extract_dns_servers(&self) -> Vec<String> {
        let mut dns_servers = Vec::new();
        
        // Try standard DNS fields
        let dns1 = self.get_ip("dns_server1");
        if !dns1.is_empty() && dns1 != "0.0.0.0" {
            dns_servers.push(dns1);
        }
        
        let dns2 = self.get_ip("dns_server2");
        if !dns2.is_empty() && dns2 != "0.0.0.0" {
            dns_servers.push(dns2);
        }
        
        // Try DHCP DNS fields
        let dhcp_dns1 = self.get_ip("dhcp_dns1");
        if !dhcp_dns1.is_empty() && dhcp_dns1 != "0.0.0.0" {
            dns_servers.push(dhcp_dns1);
        }
        
        let dhcp_dns2 = self.get_ip("dhcp_dns2");
        if !dhcp_dns2.is_empty() && dhcp_dns2 != "0.0.0.0" {
            dns_servers.push(dhcp_dns2);
        }
        
        dns_servers.dedup();
        dns_servers
    }
}

// Network configuration structure for TUN/TAP setup
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub client_ip: String,
    pub client_ip_bytes: [u8; 4],
    pub subnet_mask: String,
    pub subnet_mask_bytes: [u8; 4],
    pub gateway_ip: String,
    pub gateway_ip_bytes: [u8; 4],
    pub dns_server1: String,
    pub dns_server2: String,
    pub dhcp_server: String,
    pub domain_name: String,
    pub mtu: u32,
    pub use_dhcp: bool,
    pub lease_time: u32,
}

// TUN/TAP specific configuration
#[derive(Debug, Clone)]
pub struct TunTapConfig {
    pub interface_name: String,
    pub ip_address: String,
    pub subnet_mask: String,
    pub mtu: u32,
    pub gateway: String,
    pub dns_servers: Vec<String>,
    pub use_dhcp: bool,
}

// DHCP options extracted from server response
#[derive(Debug, Clone)]
pub struct DhcpOptions {
    pub subnet_mask: String,
    pub router: String,
    pub dns_servers: Vec<String>,
    pub domain_name: String,
    pub lease_time: u32,
    pub renewal_time: u32,
    pub rebinding_time: u32,
    pub broadcast_address: String,
}
