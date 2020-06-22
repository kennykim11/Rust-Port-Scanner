// INPUT : IPv4 address, Port Range
// FUNCTION : Enter an IP address and a port range 
//            where the program will then attempt to
//            find open ports on the given computer 
//            by connecting to each of them. On any 
//            successful connection ports, mark the 
//            port as open.
// OUTPUT : Status of port (open/closed)
use std::net::{TcpStream, SocketAddr};
use std::io;
use std::env;
use std::time::Duration;



// Main fn to receive ip addr, start port, and end port
fn main() {
    let mut args: Vec<String> = env::args().collect();
    println!("{}", args.len());

    if args.len() != 4 {
        let mut input = String::new();
        println!("Enter IP(v4): ");
        //io::stdin().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        args.push(input.clone());
        input.clear();

        println!("Enter port start: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        args.push(input.clone());
        input.clear();

        println!("Enter port end: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        args.push(input.clone());
        input.clear();
    }

    println!("{:?}", args);


    let start_port: u16 = args[2].trim().parse::<u16>().unwrap();
    let end_port: u16 = args[3].trim().parse::<u16>().unwrap();

    scan_range(args[1].trim().to_string().clone(), start_port, end_port);
}

// Scan within user specified range
fn scan_range(addr: std::string::String, start_port: u16, end_port: u16 ) -> () {

    for curr_port in start_port..end_port {
        let server_string = addr.clone() + ":" + curr_port.to_string().as_str().trim();
        let server: SocketAddr = server_string.parse().expect("Unable to parse socket address");
        if let Ok(stream) = TcpStream::connect_timeout(&server, Duration::new(0, 500000)) {
            println!("Connected to {}", server_string);
        } else {
            println!("Couldn't connect to {}", server_string);
        }
    }
}

//172.217.12.228:443
