use std::{fs, process};
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket, TcpListener, TcpStream, Shutdown};
use std::process::{Command};
use rand::Rng;
use regex::Regex;
use log::{info, warn, error, debug};
use crabby_patty_formula::*;



fn main() {
    // clear console first
    Command::new("clear").status().unwrap();
    // print the super cool banner
    banner();

    //Vec<(ID, PORT)>
    let mut listen_tracker: Vec<(u64, u16, UdpSocket)> = Vec::new();
    let mut relay_port = 2000;

    let mut listen_port: u16 = 1337;

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
                open_crusty_crab(&mut listen_tracker, listen_port, relay_port);
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
                            println!("[+] Setting default listener port to {}", value);},
                            Err(e) => println!("Those are the wrong ingredients!"),
                        }
                    }
                    else if option.eq("protocol"){
                        println!("[+] Setting default listener protocol to {}", value);
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
                if option.eq("ls") {
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

                    println!("Closing the register")
                }
                else if curr.eq("ls"){
                    println!("Spongebob look at all me customers")
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
fn open_crusty_crab(tracker: &mut Vec<(u64, u16, UdpSocket)>, port: u16, relay_port: u16){
    println!("Opening crusty crab");
    let mut local: String = "127.0.0.1:".to_owned();
    local.push_str(&relay_port.to_string()[..]);
    
    let relay = UdpSocket::bind(local).unwrap();
    tracker.push(((tracker.len() as u64) + 1, relay_port, relay));


    let mut new_listen = new_lsn(tracker.len() as u64);
    let address = SocketAddr::from(([127, 0, 0, 1], port));

    //Hardcoded "UDP" until tracker variable is set up in main
    //lsn_run(&mut new_listen, "udp", address, port);

}
