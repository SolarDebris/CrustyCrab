//
//
// Create listener that does udp and tcp
//
// Send packets over
//  - Bytes
//  - HTTP
//  - DNS
//
//  - SSH (sftp)
use std::net::{SocketAddr, UdpSocket, TcpStream, TcpListener, IpAddr, Ipv4Addr};

// Defines essential functions
pub trait Listen {
    fn run();
    fn new_connection(addr: SocketAddr);
}

// Defines a UDP listener
<<<<<<< HEAD
pub struct UdpListener {
    pub socket: UdpSocket,
    // some vector of sorts to store already made connections
}
//impl Listen for UdpListener {
=======
pub struct UdpListener() {
    pub socket: UdpSocket
    // some vector of sorts to store already made connections
}

impl Listen for UdpListener {
>>>>>>> refs/remotes/origin/main
    // TODO
//}

fn main() {

}


fn main(){
    println!("hello world");
}
