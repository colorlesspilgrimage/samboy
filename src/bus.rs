pub struct MemoryBus {
    vram: [u8; 8192],
    wram: [u8; 8192],
    eram: [u8; 8192],
    oam: [u8; 160],
    io_reg: [u8; 128],
    hram: [u8; 127],
    interrupt_enable: u8,
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            vram: [0; 8192],
            wram: [0; 8192],
            eram: [0; 8192],
            oam: [0; 160],
            io_reg: [0; 128],
            hram: [0; 127],
            interrupt_enable: 0,
        }
    }

    pub fn write(&mut self, addr: usize, data: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[addr] = data,
            0xC000..=0xDFFF => {
                // all writes to working ram must be mirrored to echo ram
                self.wram[addr] = data;
                self.eram[addr] = data;
            },
            0xFE00..=0xFE9F => self.oam[addr] = data,
            0xFF00..=0xFF7F => self.io_reg[addr] = data,
            0xFF80..=0xFFFE => self.hram[addr] = data,
            _ => panic!("Unrecognized write address: {}", addr),
        }
    }

    pub fn read(&mut self, addr: usize) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[addr],
            0xC000..=0xDFFF => self.wram[addr],
            0xFE00..=0xFE9F => self.oam[addr],
            0xFF00..=0xFF7F => self.io_reg[addr],
            0xFF80..=0xFFFE => self.hram[addr],
            _ => panic!("Unrecognized read address: {}", addr),
        }
    }

    pub fn read_range(&mut self, start_addr: usize, end_addr: usize) -> &[u8] {
        match start_addr {
            0x8000..=0x9FFF if end_addr <= 0x9FFF => &self.vram[start_addr..end_addr],
            0xC000..=0xDFFF if end_addr <= 0xDFFF => &self.wram[start_addr..end_addr],
            0xFE00..=0xFE9F if end_addr <= 0xFE9F => &self.oam[start_addr..end_addr],
            0xFF00..=0xFF7F if end_addr <= 0xFF7F => &self.io_reg[start_addr..end_addr],
            0xFF80..=0xFFFE if end_addr <= 0xFFFE => &self.hram[start_addr..end_addr],
            _ => panic!("Invalid read range: {} to {}!", start_addr, end_addr),
        }
    }

    pub fn write_range(&mut self, start_addr: usize, end_addr: usize, data: &[u8]) {

        fn write_data(dest: &mut [u8], src: &[u8]) {
            for i in 0..src.len() {
                dest[i] = src[i];
            }
        }

        match start_addr {
            0x8000..=0x9FFF if end_addr <= 0x9FFF => write_data(&mut self.vram[start_addr..], data),
            0xC000..=0xDFFF if end_addr <= 0xDFFF => write_data(&mut self.wram[start_addr..], data),
            0xFE00..=0xFE9F if end_addr <= 0xFE9F => write_data(&mut self.oam[start_addr..], data),
            0xFF00..=0xFF7F if end_addr <= 0xFF7F => write_data(&mut self.io_reg[start_addr..], data),
            0xFF80..=0xFFFE if end_addr <= 0xFFFE => write_data(&mut self.hram[start_addr..], data),
            _ => panic!("Invalid write range: {} to {}!", start_addr, end_addr),
        }
    }
}

