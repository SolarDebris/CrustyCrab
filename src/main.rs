use std::{process, fs};
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::{Command};

fn main() {
    // clear console first
    Command::new("clear").status().unwrap();
    // print the super cool banner
    banner();

    // main program loop
    loop {
        // print the prompt and read in a command

        print!("CrustyCrab $ ");
        io::stdout().flush().unwrap();
        let mut usr_cmd = String::new();
        let _output = io::stdin().read_line(&mut usr_cmd);
        // parses away arguments


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
                // subset of listener commands
                open_crusty_crab();
            }
            else if current_cmd.eq("spawn") {
                // creates an implant
                create_anchovy();
            }

            else if current_cmd.eq("clear") || current_cmd.eq("cls") {
                Command::new("clear").status().unwrap();
            }
            else if current_cmd.eq("whoami") {
                Command::new("whoami").status().unwrap();
            }
            else if current_cmd.eq("ls") {
                Command::new("ls -la").status().unwrap();
            }
            else if current_cmd.eq("pwd") 
                || current_cmd.contains("cat") 
                || current_cmd.contains("cd") 
                || current_cmd.contains("mkdir") 
                || current_cmd.contains("rm") 
                || current_cmd.contains("mv") 
                || current_cmd.contains("cp") 
                || current_cmd.contains("grep") 
                || current_cmd.contains("diff") 
                || current_cmd.contains("tar") 
                || current_cmd.contains("cut") 
                || current_cmd.contains("sort") 
                || current_cmd.contains("uniq") 
                || current_cmd.contains("awk") 
                || current_cmd.contains("sed") 
                || current_cmd.contains("ifconfig"){
                Command::new(current_cmd).status().unwrap();
            }
            else if current_cmd.contains("exec") {
                select_host_ip();
            }
            else if current_cmd.contains("set") {
                // look for all commands that contain set
                let mut command = current_cmd.split(' ');

                command.next();
                let curr_head = command.next();
                let curr = curr_head.unwrap().trim();
                println!("{}", curr);
                if curr.eq("listen") {
                    // list all anchovies and get all info
                    println!("Spongebob look at all the customers me boi"); 
                }
                else if curr.eq("payload"){
                 
                }
                else if curr.eq("kill"){
                    // kill anchovy based on its number
                    println!("sPongBOB what are you doin to me customers");
                }
            }
            else if current_cmd.contains("anchovy") {
                let mut command = current_cmd.split(' ');

                command.next();
                let curr_head = command.next();
                let curr = curr_head.unwrap().trim();
                println!("{}", curr);
                if curr.eq("ls") {
                    // list all anchovies and get all info
                    println!("Spongebob look at all the customers me boi"); 
                }
                else if curr.eq("select"){
                 
                }
                else if curr.eq("kill"){
                    // kill anchovy based on its number
                    println!("sPongBOB what are you doin to me customers");
                }
            }

            head = cmds.next();
        }
    }
}


// print ascii art
fn banner(){
    let contents = fs::read_to_string("static/art/spongerob.txt");
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


// ```rust
// select_host_ip()
// ```


// select server ip
fn select_host_ip(){
    print!("Enter host ip address: ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    let _output = io::stdin().read_line(&mut buf);
    let ip: SocketAddr = buf.trim().parse().expect("Unable to parse socket address");
    println!("{:?}", ip);
}
