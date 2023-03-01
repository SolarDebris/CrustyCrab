/* Check Virtual Machine Module
 * Author: Alex Schmith
 * Description: This module will check to see if its running inside a virtual machine
 * Supported Architectures: ALL
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



pub fn run() -> String {
	//helper_func();

	// Return value must be a String, not &str. Conversion can be
	return String::from("Hello World!");
}



// Any auxiliary functions you may want or need should NOT be
// declared as public.
fn helper_func() {
	// this function just counts to 2^16
	let mut i: u16 = 0;
	while i < u16::MAX {
		i += 1;
	}
}
