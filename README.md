Installation:
* You'll need cargo installed (comes with Rust installation https://www.rust-lang.org/tools/install)
* Clone repo
* Navigate to folder and run cargo install --path ./

Usage: port_scanner <ADDR> <PORT_LOW> [PORT_HIGH]
* Addr is the IP address you want to scan (recommended use local ip such as 127.0.0.1)
* Then you define the port range
* Port_High is optional

Arguments:
  <ADDR>       
  <PORT_LOW>   
  [PORT_HIGH]  [default: 0]

Options:
  -h, --help  Print help

Example Usage:
port_scanner 127.0.0.1 0 25565

![image](https://github.com/user-attachments/assets/e4a126c1-945c-4e37-b9dd-03f6302cd3ea)
