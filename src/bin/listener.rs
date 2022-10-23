use std::net::{SocketAddr, UdpSocket, TcpListener};
use crabby_patty_formula::*;

// Send packets over
//  - Bytes (UDP, TCP)
//  - HTTP
//  - DNS

// Defines essential functions

fn main(){
    let port: u16 = 420;
    let id: u64 = 69;
    let protocol: &str = "tcp";

    let mut lsn = new_lsn(id);

    let address = SocketAddr::from(([127, 0, 0, 1], port));
    crabby_patty_formula::lsn_run(&mut lsn, protocol, address);
}
