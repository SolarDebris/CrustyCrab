//
//
// Create listener that does udp and tcp
//
// Send packets over
//  - Bytes (UDP, TCP)
//  - HTTP
//  - DNS
use std::net::{SocketAddr, UdpSocket, TcpStream, TcpListener, IpAddr, Ipv4Addr};

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
    println!("hello world this is the listener");
}
