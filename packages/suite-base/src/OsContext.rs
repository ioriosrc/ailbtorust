```rust
use std::net::{IpAddr, SocketAddrV4, SocketAddrV6};
use std::str::FromStr;

pub struct NetworkInterface {
    pub name: String,
    pub family: &'static str,
    pub internal: bool,
    pub address: IpAddr,
    pub cidr: Option<String>,
    pub mac: String,
    pub netmask: String,
}

#[derive(Debug)]
pub struct OsContext {
    platform: String,
    pid: u32,
    get_env_var: fn(env_var: &str) -> Option<String>,
    get_hostname: fn() -> String,
    get_network_interfaces: fn() -> Vec<NetworkInterface>,
    get_app_version: fn() -> String,
}

fn main() {
    // Example usage of OsContext
    let os_context = OsContext {
        platform: "Linux".to_string(),
        pid: 12345,
        get_env_var: |env_var| std::env::var(env_var).ok(),
        get_hostname: || hostname!().unwrap_or("localhost"),
        get_network_interfaces: || {
            let netifaces = vec![
                NetworkInterface {
                    name: "eth0".to_string(),
                    family: "IPv4",
                    internal: false,
                    address: IpAddr::V4(SocketAddrV4::from_str("192.168.1.1").unwrap()),
                    cidr: Some("255.255.255.0".to_string()),
                    mac: "00:1A:2B:3C:4D:5E".to_string(),
                    netmask: "255.255.255.0".to_string(),
                },
            ];
            netifaces
        },
        get_app_version: || std::env!("CARGO_PKG_VERSION").to_string(),
    };

    // Print some properties of the OsContext
    println!("Platform: {}", os_context.platform);
    println!("PID: {}", os_context.pid);
    println!("Hostname: {}", os_context.get_hostname());
    println!("Network Interfaces:");
    for iface in &os_context.get_network_interfaces() {
        println!("  Name: {}, Family: {}, Internal: {}, Address: {:?}, CIDR: {}, MAC: {}, Netmask: {:?}", iface.name, iface.family, iface.internal, iface.address, iface.cidr, iface.mac, iface.netmask);
    }
    println!("App Version: {}", os_context.get_app_version());
}
```

Note that the above code is a simplified example and does not handle all edge cases or errors. In practice, you would need to add error handling, logging, and other functionalities depending on your specific requirements.