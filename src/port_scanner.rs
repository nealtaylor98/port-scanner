use std::{
    net::{IpAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

pub fn scan_ports(addr: IpAddr, low: u16, high: u16) -> Vec<String> {
    if low > high {
        panic!(
            "Invalid port range: low ({}) cannot be greater than high ({})",
            low, high
        );
    }

    let mut res: Vec<String> = Vec::new();
    for i in low..=high {
        if check_port(addr, i) {
            res.push(i.to_string());
        }
    }

    res
}

pub fn check_port(addr: IpAddr, port: u16) -> bool {
    let target = format!("{}:{}", addr, port);
    let address = target.to_socket_addrs().unwrap().next().unwrap();

    match TcpStream::connect_timeout(&address, Duration::from_secs(1)) {
        Ok(_) => true,
        Err(_) => false,
    }
}
