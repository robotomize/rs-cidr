use std::env;
use std::net::Ipv4Addr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <network>/<mask>", args[0]);
        std::process::exit(1);
    }

    let parts: Vec<&str> = args[1].split('/').collect();
    if parts.len() != 2 {
        println!("Invalid network format. Please use <network>/<mask>");
        std::process::exit(1);
    }

    let network = parts[0];
    let mask: u32 = parts[1].parse().expect("Invalid mask");

    match generate_subnet_addresses(network, mask) {
        Ok(addresses) => {
            for address in addresses {
                println!("{}", address);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn generate_subnet_addresses(network: &str, mask: u32) -> Result<Vec<Ipv4Addr>, &'static str> {
    if mask > 32 {
        return Err("Mask must be between 0 and 32");
    }

    let network_addr: Ipv4Addr = network.parse().map_err(|_| "Invalid network address")?;
    let num_addresses = 2u32.pow(32 - mask);
    let network_int = u32::from(network_addr);

    let addresses = (0..num_addresses).map(|i| {
        let addr_int = network_int + i;
        Ipv4Addr::new(
            ((addr_int >> 24) & 0xff) as u8,
            ((addr_int >> 16) & 0xff) as u8,
            ((addr_int >> 8) & 0xff) as u8,
            (addr_int & 0xff) as u8,
        )
    }).collect();

    Ok(addresses)
}
