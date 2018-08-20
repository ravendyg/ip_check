const IP32_CHUNKS_COUNT: usize = 4;
const IP32_CHUNK_LENGTH: usize = 8;
const IP32_LENGTH: u32 = 32;

fn ip_to_binary(ip: &str) -> Result<u32, &'static str> {
    let mut ip_binary = 0;
    for (i, chunk) in ip.split(".").enumerate() {
        match chunk.parse::<u32>() {
            Ok(a) => {
                if a > 255 {
                    return Err("malformed ip");
                }
                ip_binary |= a << ((IP32_CHUNKS_COUNT - i - 1) * IP32_CHUNK_LENGTH);
            },
            _ => return Err("malformed ip"),
        };
    }
    Ok(ip_binary)
}

fn parse_subnet(subnet: &str) -> Result<(u32, u32), &'static str> {
    let subnet_chunks = subnet.split("/").collect::<Vec<&str>>();
    if subnet_chunks.len() != 2 {
        return Err("malformed subnet");
    }
    let mut subnet_min_ip = match ip_to_binary(subnet_chunks[0]) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };
    let mask = match subnet_chunks[1].parse::<u32>() {
        // TODO: use and_then
        Ok(a) => {
            match mask_to_binary(a) {
                Ok(b) => b,
                Err(e) => return Err(e),
            }
        },
        _ => return Err("malformed mask"),
    };
    // just in case
    subnet_min_ip = subnet_min_ip & mask;

    Ok((subnet_min_ip, mask))
}

fn is_in_subnet_hard_way(ip: &str, subnet: &str) -> Result<bool, &'static str> {
    let ip_binary = match ip_to_binary(ip) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };
    match parse_subnet(subnet) {
        Ok((subnet_min_ip, mask)) => {
            let subnet_max_ip = !mask | subnet_min_ip;
            Ok(subnet_min_ip <= ip_binary && ip_binary <= subnet_max_ip)
        },
        Err(e) => Err(e),
    }
}

fn is_in_subnet_easy_way(ip: &str, subnet: &str) -> Result<bool, &'static str> {
    let ip_binary = match ip_to_binary(ip) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };
    match parse_subnet(subnet) {
        Ok((subnet_min_ip, mask)) => {
            Ok(subnet_min_ip == ip_binary & mask)
        },
        Err(e) => Err(e),
    }
}

fn mask_to_binary(mask: u32) -> Result<u32, &'static str> {
    let start = 0b1 << (IP32_LENGTH - 1);
    let mut binary_mask = 0;
    if mask > (IP32_LENGTH - 1) || mask < 1 {
        Err("invalid mask")
    } else {
        for _ in 0..mask {
            binary_mask = (binary_mask >> 1)  | start;
        }
        Ok(binary_mask)
    }
}

fn main() {
    let ip = "192.168.1.121";
    let subnet1 = "192.168.1.0/24";
    let subnet2 = "193.168.1.0/24";

    match is_in_subnet_hard_way(ip, subnet1) {
        Ok(res) => {
            let nt = if res { " " } else { " not " };
            println!("{} is{}in {}", ip, nt, subnet1);
        },
        Err(e) => println!("{}", e),
    }

    match is_in_subnet_hard_way(ip, subnet2) {
        Ok(res) => {
            let nt = if res { " " } else { " not " };
            println!("{} is{}in {}", ip, nt, subnet1);
        },
        Err(e) => println!("{}", e),
    }

    match is_in_subnet_easy_way(ip, subnet1) {
        Ok(res) => {
            let nt = if res { " " } else { " not " };
            println!("{} is{}in {}", ip, nt, subnet1);
        },
        Err(e) => println!("{}", e),
    }

    match is_in_subnet_easy_way(ip, subnet2) {
        Ok(res) => {
            let nt = if res { " " } else { " not " };
            println!("{} is{}in {}", ip, nt, subnet1);
        },
        Err(e) => println!("{}", e),
    }
}
