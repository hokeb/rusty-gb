use crate::cartridge::Cartridge;

pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_on: bool,
    ram_mode: bool,
    rom_bank: usize,
    ram_bank: usize
}

impl Mbc1 {
    pub fn new(data: Vec<u8>) -> Mbc1 {

        Mbc1 {
            rom: data,
            ram: vec![],
            ram_on: false,
            ram_mode: false,
            rom_bank: 1,
            ram_bank: 0
        }
    }
}

impl Cartridge for Mbc1 {
    fn read(&self, _: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, _: u16, _: u8) {
        todo!()
    }
}