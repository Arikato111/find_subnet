use crate::VERSION;

pub fn help() {
    println!(r#"
subnet version {VERSION}

Note: 
 github: https://github.com/Arikato111/find_subnet

Usage:
    subnet [OPTION] INPUT 

INPUT 
    <0-32>                      to show subnet mask with single input
    <0-32> <0-32> <0-32> ...    to show subnet mask with multi input

OPTION 
    -w, --wildcard              to show wildcard
    -h, --help                  to show this message
"#);
    // println!("subnet version {}", VERSION);
    // println!("");
    // println!("");
    // println!("");
}

pub fn show_version() {
    println!("v{}", VERSION);
}