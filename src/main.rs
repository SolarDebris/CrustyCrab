use std::{process, fs};
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::{Command};

fn main() {
    // clear console first
    Command::new("clear"). status(). unwrap();
    // print the super cool banner
    banner();

    // main program loop
    loop {
        // print the prompt and read in a command

        print!("CrustyCrab $ ");
        io::stdout().flush().unwrap();
        let mut usr_cmd = String::new();
        io::stdin().read_line(&mut usr_cmd);
        // parses away arguments


        // allows user to execute multiple commands in a single line seperated by ;
        let mut cmds = usr_cmd.split(";");

        // loop through each command given
        let mut head = cmds.next();
        while head != None {
            let current_cmd = head.unwrap().trim();
            // switch statement for each possible command
            if current_cmd.eq("exit") { // quit the program
                process::exit(0);
            }
            else if current_cmd.contains("help") { // print the help menu
                help();
            }
            else if current_cmd.eq("banner") { // print the banner
                banner();
            }
            else if current_cmd.eq("selhost") { // only in here to test the function works as intended
                select_host_ip();
            }

            head = cmds.next();
        }
    }
}

// print ascii art
fn banner(){
    let contents = fs::read_to_string("static/banner.txt");
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
}

// open listener
fn open_crusty_crab(){
    println!("Opening crusty crab");
}

// select server ip
fn select_host_ip(){
    print!("Enter host ip address: ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf);
    let ip: SocketAddr = buf.trim().parse().expect("Unable to parse socket address");
    println!("{:?}", ip);
}
