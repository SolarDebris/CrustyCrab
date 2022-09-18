use std::process::Command;


// Function to execute command
//

pub fn shell(){
    if let Ok(command) = Command::new("/bin/sh").output(){
        println!("{}", String::from_utf8_lossy(&command.stdout));
    }

    
}



