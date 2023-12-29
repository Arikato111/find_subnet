/**
 * func: convert binary number to base 10 number
 */
pub fn bin_to_int(bin: &String) -> String {
    let sbin: Vec<&str> = bin.split(".").collect();
    let mut c_sbin: String = "".to_string();
    for i in sbin.iter() {
        let j = isize::from_str_radix(&i, 2).expect("not binary");
        if c_sbin.len() == 0 {
            c_sbin = format!("{}", j.to_string());
        } else {
            c_sbin = format!("{}.{}", c_sbin, j.to_string());
        }
    }
    c_sbin
}

/**
 * func: add dot to subnet string after function `get_subnet`
 */
pub fn subnet_format(subnet: String) -> String {
    subnet
        .as_bytes()
        .rchunks(8)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(".")
}

/**
 * func: convert input to subnet, but it has no dot yet.
 */
pub fn get_subnet(mask: u8) -> String {
    let mut count_limit = 32;
    let mut subnet: String = "".to_string();
    let mut count = mask;
    while count > 0 {
        subnet += "1";
        count -= 1;
        count_limit -= 1;
    }
    while count_limit > 0 {
        subnet += "0";
        count_limit -= 1;
    }
    subnet
}

/**
 * func: convert 0 to 1 and 1 to 0. for convert subnet mask to whildcard
 */
pub fn flip_bit(bit: String) -> String {
    let mut bit_source: Vec<&str> = bit.split("").collect();
    let limit_loop: u8 = bit_source.len() as u8;

    for i in 0..limit_loop {
        match bit_source.get(i as usize) {
            Some(v) => {
                if *v == "0" {
                    bit_source[i as usize] = "1";
                } else if *v == "1" {
                    bit_source[i as usize] = "0";
                }
            }
            None => todo!(),
        }
    }
    bit_source.join("")
}
