use std::process::Command;


// Function to bind socket
// Function to execute command
//

pub fn shell(){
    if let Ok(command) = Command::new("/bin/sh").output(){
        println!("{}", String::from_utf8_lossy(&command.stdout));
    }

    Command::new("ls").spawn();

}



