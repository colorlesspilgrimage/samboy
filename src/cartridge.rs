use std::fmt;


#[derive(PartialEq, Debug)]
pub enum CartridgeType {
    Rom,
    Mbc1,
    Mbc1Ram,
    Mbc1RamBatt,
    Mbc2,
    Mbc2Batt,
    RomRam,
    RomRamBatt,
    Mmm01,
    Mmm01Ram,
    Mmm01RamBatt,
    Mbc3TimerBatt,
    Mbc3TimerRamBatt,
    Mbc3,
    Mbc3Ram,
    Mbc3RamBatt,
    Mbc5,
    Mbc5Ram,
    Mbc5RamBatt,
    Mbc5Rumb,
    Mbc5RumbRam,
    Mbc5RumbRamBatt,
    Mbc6,
    Mbc7SensorRumbRamBatt,
    PocketCam,
    BandaiTama5,
    Huc3,
    Huc1RamBatt,
}

const HAS_RAM: [CartridgeType; 15] = [
    CartridgeType::Mbc1Ram,
    CartridgeType::Mbc1RamBatt,
    CartridgeType::RomRam,
    CartridgeType::RomRamBatt,
    CartridgeType::Mmm01Ram,
    CartridgeType::Mmm01RamBatt,
    CartridgeType::Mbc3TimerRamBatt,
    CartridgeType::Mbc3Ram,
    CartridgeType::Mbc3RamBatt,
    CartridgeType::Mbc5Ram,
    CartridgeType::Mbc5RamBatt,
    CartridgeType::Mbc5RumbRam,
    CartridgeType::Mbc5RumbRamBatt,
    CartridgeType::Mbc7SensorRumbRamBatt,
    CartridgeType::Huc1RamBatt,
];

pub struct Header {
    title: String,
    cgb: u8,
    sgb: u8,
    cartridge_type: CartridgeType,
    rom_size: u8,
    ram_size: u8,
}

impl Header {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            cgb: 0,
            sgb: 0,
            cartridge_type: CartridgeType::Rom,
            rom_size: 0,
            ram_size: 0,
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Title: {}\nCGB: {}\nSGB: {}\nCartridge Type: {:?}\nROM Size: {}\nRAM Size: {}\n",
            self.title, self.cgb,
            self.sgb, self.cartridge_type,
            self.rom_size, self.ram_size)
    }
}

/// Strip the header data from a cartridge and return a filled out [Header] struct.
pub fn strip_header(bytestream: &[u8]) -> Header {
    let mut header = Header::new();
    header.title = str::from_utf8(&bytestream[308..323]).unwrap().to_string();

    header.cgb = bytestream[323];
    header.sgb = bytestream[326];
    header.cartridge_type = match bytestream[327] {
        0x00 => CartridgeType::Rom,
        0x01 => CartridgeType::Mbc1,
        0x02 => CartridgeType::Mbc1Ram,
        0x03 => CartridgeType::Mbc1RamBatt,
        0x05 => CartridgeType::Mbc2,
        0x06 => CartridgeType::Mbc2Batt,
        0x08 => CartridgeType::RomRam,
        0x09 => CartridgeType::RomRamBatt,
        0x0B => CartridgeType::Mmm01,
        0x0C => CartridgeType::Mmm01Ram,
        0x0D => CartridgeType::Mmm01RamBatt,
        0x0F => CartridgeType::Mbc3TimerBatt,
        0x10 => CartridgeType::Mbc3TimerRamBatt,
        0x11 => CartridgeType::Mbc3,
        0x12 => CartridgeType::Mbc3Ram,
        0x13 => CartridgeType::Mbc3RamBatt,
        0x19 => CartridgeType::Mbc5,
        0x1A => CartridgeType::Mbc5Ram,
        0x1B => CartridgeType::Mbc5RamBatt,
        0x1C => CartridgeType::Mbc5Rumb,
        0x1D => CartridgeType::Mbc5RumbRam,
        0x1E => CartridgeType::Mbc5RumbRamBatt,
        0x20 => CartridgeType::Mbc6,
        0x22 => CartridgeType::Mbc7SensorRumbRamBatt,
        0xFC => CartridgeType::PocketCam,
        0xFD => CartridgeType::BandaiTama5,
        0xFE => CartridgeType::Huc3,
        0xFF => CartridgeType::Huc1RamBatt,
        _ => panic!("Unknown cartridge type!"),
    };

    // gbdev.io documentation references this math to determine the ROM size.
    // to keep things fitting in a u8, we multiple by 32 instead of 32000.
    // all the ROM and RAM size values need to be multiplied by 1000
    // to get the "true" size in KiB..
    header.rom_size = 32 * (1 << bytestream[328]);

    if HAS_RAM.contains(&header.cartridge_type) {
        header.ram_size = match bytestream[329] {
            0x02 => 8,
            0x03 => 32,
            0x04 => 128,
            0x05 => 64,
            _ => panic!("unknown ram size conversion!"),
        };
    } else {
        header.ram_size = 0;
    }

    header
}

