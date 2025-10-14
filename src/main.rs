mod cartridge;
mod cpu;

// TODO: the gb cpu is little endian, so when we are reading any 16bit value, we must remember that the least significant bit is stored first.
// for the most part, this won't change a lot of how the code works, but for any instructions that use any 16bit values, we will need to account for
// the endianess.

fn main() {
    println!("Hello, world!");
}
