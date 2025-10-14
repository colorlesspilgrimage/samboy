pub struct Header {
    title: String,
    mfg: u8,
    cgb: u8,
    sgb: u8,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    destination: u8,
    version_num: u8,
    header_check: u8,
    global_check: u16,
}

impl Header {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            mfg: 0,
            cgb: 0,
            sgb: 0,
            cartridge_type: 0,
            rom_size: 0,
            ram_size: 0,
            destination: 0,
            version_num: 0,
            header_check: 0,
            global_check: 0,
        }
    }
}

pub fn strip_header(bytestream: &[u8]) -> Header {
    return Header::new();
}
