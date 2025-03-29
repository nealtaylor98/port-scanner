use std::{
    net::{IpAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

use clap::Parser;

#[derive(Debug, Parser)]
// #[command(version, about, long_about = None)]
struct Args {
    addr: IpAddr,
    port_low: u16,
    #[arg(default_value_t = 0)]
    port_high: u16,
}

fn main() {
    let mut args = Args::parse();

    if args.port_high == 0 {
        args.port_high = args.port_low;
    }

    if args.port_low > args.port_high {
        panic!(
            "cannot scan from low of {} to high of {} because high is too low",
            args.port_low, args.port_high
        );
    }

    let open_ports: Vec<String> = scan_ports(args.addr, args.port_low, args.port_high);

    for i in open_ports.into_iter() {
        println!("{} is open", i);
    }
}

fn scan_ports(addr: IpAddr, low: u16, high: u16) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    for i in low..=high {
        if check_port(addr, i) {
            res.push(i.to_string());
        }
    }

    res
}

fn check_port(addr: IpAddr, port: u16) -> bool {
    let target = format!("{}:{}", addr, port);
    let address = target.to_socket_addrs().unwrap().next().unwrap();

    match TcpStream::connect_timeout(&address, Duration::from_secs(1)) {
        Ok(_) => true,
        Err(_) => false,
    }
}
