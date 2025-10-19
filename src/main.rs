mod cartridge;
mod cpu;
mod bus;


use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let res: cartridge::Header;
    let mut file = File::open(&args[1]).expect("File not found");
    let mut data = Vec::new();
    if args.len() < 2 {
        println!("Please supply a file name");
        return;
    }

    file.read_to_end(&mut data).expect("Failed to read file");
    res = cartridge::strip_header(&data);
    println!("{}", res);
}
