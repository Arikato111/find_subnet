use find_subnet::subnet::subnet_main::subnet_process;
use find_subnet::{help, show_version};
use std::env::args;

fn main() {
    let argv: Vec<String> = args().collect();
    let (command, option) = command_option_fliter(&argv);

    if option.contains(&"-h".to_string()) || option.contains(&"--help".to_string()) {
        help();
        return;
    } else if option.contains(&"-v".to_string()) || option.contains(&"--version".to_string()) {
        show_version();
        return;
    }

    match command.get(1) {
        Some(_) => subnet_process(&command, &option),
        None => help(),
    }
}

fn command_option_fliter(argv: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut command: Vec<String> = vec![];
    let mut option: Vec<String> = vec![];

    for arg in argv.iter() {
        let arg_split: Vec<&str> = arg.split("").collect();
        if arg_split[1] == "-" {
            option.push(arg.clone());
        } else {
            command.push(arg.clone());
        }
    }
    (command, option)
}
