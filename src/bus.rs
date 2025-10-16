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
}

