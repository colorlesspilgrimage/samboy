#[derive(PartialEq)]
pub enum CartridgeType {
    ROM,
    MBC1,
    MBC1_RAM,
    MBC1_RAM_BATT,
    MBC2,
    MBC2_BATT,
    ROM_RAM,
    ROM_RAM_BATT,
    MMM01,
    MMM01_RAM,
    MMM01_RAM_BATT,
    MBC3_TIMER_BATT,
    MBC3_TIMER_RAM_BATT,
    MBC3,
    MBC3_RAM,
    MBC3_RAM_BATT,
    MBC5,
    MBC5_RAM,
    MBC5_RAM_BATT,
    MBC5_RUMB,
    MBC5_RUMB_RAM,
    MBC5_RUMB_RAM_BATT,
    MBC6,
    MBC7_SENSOR_RUMB_RAM_BATT,
    POCKET_CAM,
    BANDAI_TAMA5,
    HUC3,
    HUC1_RAM_BATT,
}

const HAS_RAM: [CartridgeType; 15] = [
    CartridgeType::MBC1_RAM,
    CartridgeType::MBC1_RAM_BATT,
    CartridgeType::ROM_RAM,
    CartridgeType::ROM_RAM_BATT,
    CartridgeType::MMM01_RAM,
    CartridgeType::MMM01_RAM_BATT,
    CartridgeType::MBC3_TIMER_RAM_BATT,
    CartridgeType::MBC3_RAM,
    CartridgeType::MBC3_RAM_BATT,
    CartridgeType::MBC5_RAM,
    CartridgeType::MBC5_RAM_BATT,
    CartridgeType::MBC5_RUMB_RAM,
    CartridgeType::MBC5_RUMB_RAM_BATT,
    CartridgeType::MBC7_SENSOR_RUMB_RAM_BATT,
    CartridgeType::HUC1_RAM_BATT,
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
            cartridge_type: CartridgeType::ROM,
            rom_size: 0,
            ram_size: 0,
        }
    }
}

/// Strip the header data from a cartridge and return a filled out [Header] struct.
pub fn strip_header(bytestream: &[u8]) -> Header {
    let mut header = Header::new();
    header.title = str::from_utf8(&bytestream[308..323]).unwrap().to_string();

    header.cgb = bytestream[323];
    header.sgb = bytestream[326];
    header.cartridge_type = match bytestream[327] {
        0x00 => CartridgeType::ROM,
        0x01 => CartridgeType::MBC1,
        0x02 => CartridgeType::MBC1_RAM,
        0x03 => CartridgeType::MBC1_RAM_BATT,
        0x05 => CartridgeType::MBC2,
        0x06 => CartridgeType::MBC2_BATT,
        0x08 => CartridgeType::ROM_RAM,
        0x09 => CartridgeType::ROM_RAM_BATT,
        0x0B => CartridgeType::MMM01,
        0x0C => CartridgeType::MMM01_RAM,
        0x0D => CartridgeType::MMM01_RAM_BATT,
        0x0F => CartridgeType::MBC3_TIMER_BATT,
        0x10 => CartridgeType::MBC3_TIMER_RAM_BATT,
        0x11 => CartridgeType::MBC3,
        0x12 => CartridgeType::MBC3_RAM,
        0x13 => CartridgeType::MBC3_RAM_BATT,
        0x19 => CartridgeType::MBC5,
        0x1A => CartridgeType::MBC5_RAM,
        0x1B => CartridgeType::MBC5_RAM_BATT,
        0x1C => CartridgeType::MBC5_RUMB,
        0x1D => CartridgeType::MBC5_RUMB_RAM,
        0x1E => CartridgeType::MBC5_RUMB_RAM_BATT,
        0x20 => CartridgeType::MBC6,
        0x22 => CartridgeType::MBC7_SENSOR_RUMB_RAM_BATT,
        0xFC => CartridgeType::POCKET_CAM,
        0xFD => CartridgeType::BANDAI_TAMA5,
        0xFE => CartridgeType::HUC3,
        0xFF => CartridgeType::HUC1_RAM_BATT,
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
