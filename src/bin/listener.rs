use std::net::{SocketAddr, UdpSocket, TcpListener};
use std::{thread, time};
use std::sync::{Arc, Mutex};
use crabby_patty_formula::*;
use std::mem::drop;

// Send packets over
//  - Bytes (UDP, TCP)
//  - HTTP
//  - DNS

// Defines essential functions

fn main(){
    let port: u16 = 2120;
    let id: u64 = 69;
    let protocol: &str = "udp";

    let mut lsn = new_lsn(id);

    let address = SocketAddr::from(([127, 0, 0, 1], port));

    let mut sb: Arc<Mutex<SharedBuffer>> = Arc::new(Mutex::new(SharedBuffer {
        cc: 0,
        buff: [0; 2048].to_vec(),
    }));

    let mut sb_arc = Arc::clone(&sb);

    // spawn the listnener
    let thr = thread::spawn(move ||
        {
            crabby_patty_formula::lsn_run(&mut lsn, protocol, address, &mut sb);
        }
    );

    // send some control codes
    let mut code: u8 = 1;
    while code < 7 {
        if code == 2 {
            code += 1;
        }
        let mut buffer = sb_arc.lock().unwrap();
        buffer.cc = code;
        drop(buffer);
        thread::sleep(time::Duration::from_millis(10));
        buffer = sb_arc.lock().unwrap();
        println!("{} {}", code, String::from_utf8_lossy(&buffer.buff[..]));
        code += 1;
        thread::sleep(time::Duration::from_millis(10));
    }

    thr.join().unwrap();
}
