//! Test the enhanced pack_types functionality

use mayaqua::{Pack, Element, Value, ValueType};

fn main() {
    println!("🚀 Enhanced Pack Types Test");
    println!("===========================");
    
    // Create a mock VPN server response
    let server_response = create_mock_server_response();
    
    println!("\n📦 Available fields: {:?}", server_response.get_names());
    
    // Test the new network configuration extraction
    let network_config = server_response.get_network_config();
    println!("\n🌐 Network Configuration:");
    println!("  Client IP: {} ({:?})", network_config.client_ip, network_config.client_ip_bytes);
    println!("  Subnet Mask: {} ({:?})", network_config.subnet_mask, network_config.subnet_mask_bytes);
    println!("  Gateway: {} ({:?})", network_config.gateway_ip, network_config.gateway_ip_bytes);
    println!("  DNS Servers: {}, {}", network_config.dns_server1, network_config.dns_server2);
    println!("  MTU: {}", network_config.mtu);
    println!("  Use DHCP: {}", network_config.use_dhcp);
    
    // Test TUN/TAP configuration extraction
    if server_response.has_valid_client_ip() {
        println!("\n✅ Valid IP assignment found!");
        
        let tuntap_config = server_response.get_tuntap_config("vpn0");
        println!("\n🔧 TUN/TAP Configuration:");
        println!("  Interface: {}", tuntap_config.interface_name);
        println!("  IP Address: {}", tuntap_config.ip_address);
        println!("  Subnet Mask: {}", tuntap_config.subnet_mask);
        println!("  Gateway: {}", tuntap_config.gateway);
        println!("  DNS Servers: {:?}", tuntap_config.dns_servers);
        println!("  MTU: {}", tuntap_config.mtu);
    }
    
    // Test DHCP options extraction
    let dhcp_options = server_response.extract_dhcp_options();
    println!("\n🏠 DHCP Options:");
    println!("  DNS Servers: {:?}", dhcp_options.dns_servers);
    
    println!("\n✅ Enhanced pack_types functionality working!");
}

fn create_mock_server_response() -> Pack {
    let mut pack = Pack::new();
    
    // Client IP assignment (192.168.10.100)
    let mut client_ip_element = Element::new("client_ip".to_string(), ValueType::Int);
    let ip_int = ((192u32) << 24) | ((168u32) << 16) | ((10u32) << 8) | 100u32;
    client_ip_element.values.push(Value::Int(ip_int));
    pack.add_element(client_ip_element).unwrap();
    
    // Subnet mask (255.255.255.0)
    let mut subnet_element = Element::new("subnet_mask".to_string(), ValueType::Int);
    let mask_int = ((255u32) << 24) | ((255u32) << 16) | ((255u32) << 8) | 0u32;
    subnet_element.values.push(Value::Int(mask_int));
    pack.add_element(subnet_element).unwrap();
    
    // Gateway IP (192.168.10.1)
    let mut gateway_element = Element::new("gateway_ip".to_string(), ValueType::Int);
    let gateway_int = ((192u32) << 24) | ((168u32) << 16) | ((10u32) << 8) | 1u32;
    gateway_element.values.push(Value::Int(gateway_int));
    pack.add_element(gateway_element).unwrap();
    
    // DNS servers
    let mut dns1_element = Element::new("dns_server1".to_string(), ValueType::Int);
    let dns1_int = ((8u32) << 24) | ((8u32) << 16) | ((8u32) << 8) | 8u32; // 8.8.8.8
    dns1_element.values.push(Value::Int(dns1_int));
    pack.add_element(dns1_element).unwrap();
    
    let mut dns2_element = Element::new("dns_server2".to_string(), ValueType::Int);
    let dns2_int = ((8u32) << 24) | ((8u32) << 16) | ((4u32) << 8) | 4u32; // 8.8.4.4
    dns2_element.values.push(Value::Int(dns2_int));
    pack.add_element(dns2_element).unwrap();
    
    // MTU
    let mut mtu_element = Element::new("mtu".to_string(), ValueType::Int);
    mtu_element.values.push(Value::Int(1500));
    pack.add_element(mtu_element).unwrap();
    
    pack
}
