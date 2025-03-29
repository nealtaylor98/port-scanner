use std::net::{IpAddr, Ipv4Addr};

use port_scanner::scan_ports;

#[test]
#[should_panic(expected = "Invalid port range")]
fn test_scan_ports_invalid_range() {
    scan_ports(IpAddr::V4(Ipv4Addr::LOCALHOST), 5001, 5000);
}
