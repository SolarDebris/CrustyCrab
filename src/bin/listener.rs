use std::net::{SocketAddr, UdpSocket, TcpListner};

// Send packets over
//  - Bytes (UDP, TCP)
//  - HTTP
//  - DNS

// Defines essential functions
pub trait Listen {
    fn run();
    fn run_udp();
    fn run_tcp();
    fn new_connection(addr: SocketAddr);
}

// Defines a UDP listener
pub struct Listener {
    pub udp_sock: UdpSocket,
    pub tcp_sock: TcpListener,
    pub id:u64,
    pub status:u8,
    // some vector of sorts to store already made connections
}

impl Listen for Listener {

    fn run(&self, protocol:u8, address:SocketAddr) {
        match protocol {
            0 => run_udp(self, address);
            1 => run_tcp(self, address);
        }
    }

    // listenes using raw UDP
    fn run_udp(&self, address:SocketAddr) {
        self.udp_sock = UdpSocket::bind(address).expect("couldn't bind to address");
        loop {
            
        }
    }

    // listens using raw TCP
    fn run_tcp(&self, address:SocketAddr) {

    }
}

fn main(){
    let _port = 420;
    let _protocol: &str = "http";

}
