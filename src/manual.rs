use crate::VERSION;

pub fn help() {
    println!();
    println!("subnet version {}", VERSION);
    println!("\t<0-32>\t\tto show subnet mask with single input");
    println!("\t<0-32> <0-32> <0-32> ... \tto show subnet mask with multi input");
    println!("");
    println!("\t-w, --wildcard \t to show wildcard");
}

pub fn show_version() {
    println!("v{}", VERSION);
}