use std::{process, fs};
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::{Command};
use rand::Rng;

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
            else if current_cmd.eq("pwd")
                || current_cmd.eq("whoami")
                || current_cmd.eq("clear")
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
                    println!("Sending out patty")
                }
                else if curr.eq("anchovy"){
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
                    println!("Spongebob look at all the customers me boi (anchovy ls)");
                }
                else if curr.contains("select"){
                    println!("One krabby patty coming up (anchovy select)")
                }
                else if curr.eq("spawn"){
                    create_anchovy();
                }
                else if curr.contains("kill"){
                    // kill anchovy based on its number
                    println!("sPongBOB what are you doin to me customers (anchovy kill)");
                }
            }
            else if current_cmd.contains("listen") {
                let mut command = current_cmd.split(' ');

                command.next();

                let mut curr_head = command.next();
                let mut curr = curr_head.unwrap().trim();
                println!("{}", curr);
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


// print ascii art
fn banner(){
    // !TODO make it so that it chooses a random ascii art

    let mut rng = rand::thread_rng();

    let mut file = format!("static/art/banner{}.txt",rng.gen_range(0..4));
    let contents = fs::read_to_string(file);
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
// !TODO This is not necessary we just need to grab public ip of server and use that
fn select_host_ip(){
    print!("Enter host ip address: ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    let _output = io::stdin().read_line(&mut buf);
    let ip: SocketAddr = buf.trim().parse().expect("Unable to parse socket address");
    println!("{:?}", ip);
}
