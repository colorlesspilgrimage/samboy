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
                // all writes to working ram must be mirror to echo ram
                self.wram[addr] = data;
                self.eram[addr] = data;
            },
            0xFE00..=0xFE9F => self.oam[addr] = data,
            0xFF00..=0xFF7F => self.io_reg[addr] = data,
            0xFF80..=0xFFFe => self.hram[addr] = data,
            _ => panic!("Unrecognized write address: {}", addr),
        }
    }

    pub fn read(&mut self, addr: usize) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[addr],
            0xC000..=0xDFFF => self.wram[addr],
            0xFE00..=0xFE9F => self.oam[addr],
            0xFF00..=0xFF7F => self.io_reg[addr],
            0xFF80..=0xFFFe => self.hram[addr],
            _ => panic!("Unrecognized read address: {}", addr),
        }
    }
}

