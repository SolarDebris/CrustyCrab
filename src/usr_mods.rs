mod example;

pub fn dispatch(s: &str) {
    match s {
        "example" => example::run(),
        &_ => todo!(),
    };
}

pub fn list_mods() -> String {
    "Not yet implemented.".to_string()
}
