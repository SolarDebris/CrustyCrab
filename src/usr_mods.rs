// Custom Module System
// Description: A system that allows users to write their own custom
//              post-exploit modules.


/*** DEV TODO ***/
// Add your module here in the format
// mod module_name;
mod example;

// takes in a &str (the name of the module), runs that module, and then
// returns the output of the module as a vector of bytes.
pub fn dispatch(s: &str) -> Vec<u8> {
    let result = match s {

        /*** DEV TODO ***/
        // add a match statement for your module below in the format
        // "module_name" => module_name::run(),
        "example" => example::run(),
        &_ => "MODULE NOT FOUND".to_string(),
    };

    return result.into_bytes();
}


// prints to the screen a list of all installed modules
pub fn list_mods() {
    println!("Installed Modules:");

    /*** DEV TODO ***/
    // add a print statement below for your module in the following format:
    // println!("module_name - description of what the module does");
    println!("example - a template for developers to write their own modules");
}
