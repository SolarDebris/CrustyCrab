/* System Info Module
 * Author: Alex Schmith
 * Description: This module prints out system information
 * Supported Architectures: amd64
 * Supported Operating Systems: linux
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


	// Return value must be a String, not &str. Conversion can be
	return get_distro();
}


fn get_distro() -> String {
    let mut distro_file = File::open("/etc/os-release");

    let mut distro_cont = String::new();
    distro_file.expect("REASON").read_to_string(&mut distro_cont);

    return distro_cont
}

fn get_kernel_version() -> String {

    return String::from("Hello World");
}

fn get_user_accounts() -> String {

    return String::from("Hello World");
}
