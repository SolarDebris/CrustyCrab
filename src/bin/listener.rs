use std::net::{SocketAddr, UdpSocket};

// Send packets over
//  - Bytes (UDP, TCP)
//  - HTTP
//  - DNS

// Defines essential functions
pub trait Listen {
    fn run();
    fn new_connection(addr: SocketAddr);
}

// Defines a UDP listener
pub struct UdpListener {
    pub socket: UdpSocket,
    // some vector of sorts to store already made connections
}

//impl Listen for UdpListener {
    // TODO
//}

fn main(){
    let _port = 420;
    let _protocol: &str = "http";

}
