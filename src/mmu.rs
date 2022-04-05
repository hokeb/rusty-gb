use crate::cartridge::{Cartridge, new_cartridge};

pub struct Mmu
{
    cartridge: Box<dyn Cartridge>,
    v_ram: Vec<u8>,
    e_ram: Vec<u8>,
    w_ram: Vec<u8>,
    oam: Vec<u8>,
    h_ram: Vec<u8>
}

impl Mmu {
    pub fn new(rom_path: std::path::PathBuf) -> Mmu {
        Mmu {
            cartridge: new_cartridge(rom_path),
            v_ram: vec![],
            e_ram: vec![],
            w_ram: vec![],
            oam: vec![],
            h_ram: vec![]
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0    ..= 0x7FFF => self.cartridge.read(addr), // ROM BANK 01 ~ NN (swappable ROM BANK)
            0x8000 ..= 0x9FFF => self.v_ram[addr as usize], // VRAM
            0xA000 ..= 0xBFFF => todo!(),                   // EXTERNAL RAM BANK (swappable RAM BANK)
            0xC000 ..= 0xCFFF => self.w_ram[addr as usize], // WRAM (Worker RAM)
            0xD000 ..= 0xDFFF => todo!(),                   // CGB Swappable WRAM (BANK 1 ~ 7)
            0xFE00 ..= 0xFE9F => self.oam[addr as usize],   // OAM - Sprite Attribute Table
            0xFF00 ..= 0xFF7F => todo!(),                   // I/O Registers
            0xFF80 ..= 0xFFEE => self.h_ram[addr as usize], // HRAM (High RAM)
            0xFFFF ..= 0xFFFF => todo!(),                   // Interrupt Enable Register (IE)
            _ => panic!("Address is prohibited or non existent on the GB")
        }
    }
}