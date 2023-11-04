use crate::subnet::{bin_to_int, get_subnet, subnet_format};

pub fn subnet_process(argv: &Vec<String>) {
    let mut i;
    println!("");
    for arg in argv.iter().skip(1) {
        i = match arg.parse::<u8>() {
            Ok(v) => {
                if v > 32 {
                    println!("{} is Error max 32", arg);
                    continue;
                }
                v
            }
            Err(_) => {
                println!("{} is Error input", arg);
                continue;
            }
        };
        let su = subnet_format(get_subnet(i as u8).to_string());
        println!("/{} = {} | {}", i, su, bin_to_int(&su));
    }
}
