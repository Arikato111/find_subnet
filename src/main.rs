use find_subnet::{help, show_version};
use find_subnet::subnet::subnet_main::subnet_process;
use std::env::args;

fn main() {
    let argv: Vec<String> = args().collect();

    match argv.get(1) {
        Some(v) => match v.as_str() {
            "--version" | "-v" => show_version(),
            "-h" => help(),
            _ => subnet_process(&argv),
        },
        None => help(),
    }
}
