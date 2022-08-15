use std::env;
use std::process::exit;
use std::time::{Duration, Instant};
use std::net::{
    Shutdown, TcpStream, IpAddr, Ipv4Addr, SocketAddr
};


fn main() {
    let start = Instant::now();

    let cli_args: Vec<String> = env::args().collect();

    if cli_args.len() != 4 {
        println!("Not enough arguments!");
        help(&cli_args[0]);
        exit(1);
    }

    let args = parse_args(cli_args);
    
    for port in args.1[0]..args.1[1] {
        print!("                                           \r");
        print!("{}/{}\r", port, args.1[1]);
        let val = connect_to_port(args.0, &port, &args.2);
        if val {
            println!("Port {} is open", port);
        }
    }

    println!("Elapsed time is {} seconds", start.elapsed().as_secs());
}


fn parse_args(args: Vec<String>) -> ([u8; 4], [u16; 2], u64) {
    let first_arg = args[1].split("."); 
    let ip: [u8; 4] = match first_arg.map(|num| num.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into() {
            Ok(val) => val,
            Err(_) => {
                println!("Not an ip address!");
                exit(1);
            },
        };

    let second_arg = args[2].split(",");
    let ports: [u16; 2] = match second_arg.map(|num| num.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid port range");
                exit(1);
            },
        };

    let time = args[3].parse::<u64>().unwrap();
    let ret_vec = (ip, ports, time);

    return ret_vec;
}


fn help(executable: &String) -> () {
    println!("Usage: ./{} ip port_start,port_end scan_timeout", executable);
}


fn connect_to_port(ip: [u8; 4], port: &u16, time: &u64) -> bool {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3])), *port);

    let stream = match TcpStream::connect_timeout(&socket, Duration::new(*time, 0)) {
        Ok(tcpstream) => tcpstream,
        Err(_) => return false,
    };

    match stream.shutdown(Shutdown::Both) {
        Ok(_) => true,
        Err(_) => false,
    }
}
