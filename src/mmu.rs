use crate::cartridge::{Cartridge, new_cartridge};

pub struct Mmu
{
    cartridge: Box<dyn Cartridge>,
    v_ram: Vec<u8>,
    _e_ram: Vec<u8>,
    w_ram: [u8; 0x8000],
    w_rambank: usize,
    oam: Vec<u8>,
    h_ram: Vec<u8>,
    int_f: u8,
    int_e: u8,
}

impl Mmu {
    pub fn new(rom_path: std::path::PathBuf) -> Mmu {
        Mmu {
            cartridge: new_cartridge(rom_path),
            v_ram: vec![],
            _e_ram: vec![],
            w_ram: [0; 0x8000],
            w_rambank: 1,
            oam: vec![],
            h_ram: vec![0; 0x7F],
            int_f: 0,
            int_e: 0,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000 ..= 0x7FFF => self.cartridge.read(addr), // ROM BANK 01 ~ NN (swappable ROM BANK)
            0x8000 ..= 0x9FFF => self.v_ram[addr as usize], // VRAM
            0xA000 ..= 0xBFFF => todo!(),                   // EXTERNAL RAM BANK (swappable RAM BANK)
            0xC000 ..= 0xCFFF => self.w_ram[addr as usize], // WRAM (Worker RAM)
            0xD000 ..= 0xDFFF => todo!(),                   // CGB Swappable WRAM (BANK 1 ~ 7)
            0xFE00 ..= 0xFE9F => self.oam[addr as usize],   // OAM - Sprite Attribute Table
            0xFF00 ..= 0xFF7F => 0,                         // I/O Registers TODO: do not send 0s back
            0xFF80 ..= 0xFFEE => self.h_ram[addr as usize], // HRAM (High RAM)
            0xFFFF            => todo!(),                   // Interrupt Enable Register (IE)
            _ => panic!("Address is prohibited or non existent on the GB")
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        (self.read(addr) as u16) | ((self.read(addr + 1) as u16) << 8)
    }

    pub fn write(&mut self, addr: u16, b: u8) {
        println!("Writing to byte: {:X?}", addr);
        match addr {
            0xD000 ..= 0xDFFF => self.w_ram[(self.w_rambank * 0x1000) | (addr as usize & 0x0FFF)] = b,
            0xFF01 ..= 0xFF02 => println!("Serial Transfer, probably won't implement"),
            0xFF10 ..= 0xFF3F => println!("Sound command - NYI"), // TODO: sound at some point
            0xFF40 ..= 0xFF4B => println!("LCD stuff, probably won't implement"),
            0xFF0F            => self.int_f = b,
            0xFF80 ..= 0xFFFE => self.h_ram[addr as usize & 0x007F] = b,
            0xFFFF            => self.int_e = b,
            _ => panic!("Writing to {:X?} not yet implemented", addr)
        }
    }

    pub fn write_word(&mut self, addr: u16, w: u16) {
        self.write(addr, (w & 0xFF) as u8);
        self.write(addr + 1, (w >> 8) as u8);
    }
}