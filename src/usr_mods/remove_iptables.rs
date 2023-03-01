/* Remove Ip Table Rules Module
 * Author: Alex Schmith
 * Description: This module will remove all ip table rules given it has admin access
 * Supported Architectures: x86
 * Supported Operating Systems: ALL
 */


// All modules must have a run function that returns a String.
// The returned String will be what is printed to the terminal
// when run. If your module does not perform any collection or
// has no need for printing anything, you can just return some
// String indicating termination, say "Done" for example.

// Run acts as a sort of 'main' for your module. It must always
// take in no parameters, it must be public, and it must return
// a String.

use crabby_patty_formula::*;

pub fn run() -> String {

	// Return value must be a String, not &str. Conversion can be
	return String::from("Hello World!");
}
