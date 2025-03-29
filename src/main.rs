mod port_scanner;

use clap::Parser;
use port_scanner::scan_ports;
use std::net::IpAddr;

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

    let open_ports: Vec<String> = scan_ports(args.addr, args.port_low, args.port_high);

    for i in open_ports.into_iter() {
        println!("{} is open", i);
    }
}
