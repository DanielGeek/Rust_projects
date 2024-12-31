
use app::dft::{ntt::Table, DFT};

fn is_power_of_two(x: usize) -> bool {
    x != 0 && (x & (x - 1)) == 0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --release -- <number>");
        std::process::exit(1);
    }
    let number: usize = args[1].parse().expect("Invalid number");

    if !is_power_of_two(number) {
        eprintln!("Error: The input number must be a power of 2.");
        std::process::exit(1);
    }

    println!("You entered: {}", number);

    // Create NTT table
    let ntt_table = Table::new(number);

    // Create a sample vector
    let mut data: Vec<u64> = (0..number as u64).collect();
    println!("Input data: {:?}", data);

    // Perform forward NTT
    ntt_table.forward_inplace(&mut data);
    println!("After forward NTT: {:?}", data);

    // Perform backward NTT
    ntt_table.backward_inplace(&mut data);
    println!("After backward NTT: {:?}", data);
}
