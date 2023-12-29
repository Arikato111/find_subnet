use find_subnet::subnet::subnet_main::subnet_process;
use find_subnet::{help, show_version};
use std::env::args;

fn main() {
    // keep argv
    let argv: Vec<String> = args().collect();
    let (command, option) = command_option_fliter(&argv);

    // check -h or --help option to show usage
    if option.contains(&"-h".to_string()) || option.contains(&"--help".to_string()) {
        help();
        return;
    // check -v or --version to show version of this program
    } else if option.contains(&"-v".to_string()) || option.contains(&"--version".to_string()) {
        show_version();
        return;
    }

    // check first position command 
    match command.get(1) {
        // if it has command input, it will call subnet process.
        Some(_) => subnet_process(&command, &option),
        // if user doesn't input any command it will show usage.
        None => help(),
    }
}

// this function for filter command and option
fn command_option_fliter(argv: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut command: Vec<String> = vec![];
    let mut option: Vec<String> = vec![];

    for arg in argv.iter() {
        let arg_split: Vec<&str> = arg.split("").collect();
        // check first char of string is - , that means it is option.
        if arg_split[1] == "-" {
            option.push(arg.clone());
        } else {
            command.push(arg.clone());
        }
    }
    (command, option)
}
