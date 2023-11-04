use crate::subnet::{bin_to_int, flip_bit, get_subnet, subnet_format};

pub fn subnet_process(command: &Vec<String>, option: &Vec<String>) {
    let mut i;
    let mut su: String;
    let wildcard: bool =
        option.contains(&"-w".to_string()) || option.contains(&"--wildcard".to_string());

    println!("");
    for comd in command.iter().skip(1) {
        i = match comd.parse::<u8>() {
            Ok(v) => {
                if v > 32 {
                    println!("{} is Error max 32", comd);
                    continue;
                }
                v
            }
            Err(_) => {
                println!("{} is Error input", comd);
                continue;
            }
        };

        if wildcard {
            su = subnet_format(flip_bit(get_subnet(i as u8)));
        } else {
            su = subnet_format(get_subnet(i as u8));
        }
        println!("/{} = {} | {}", i, su, bin_to_int(&su));
    }
}
