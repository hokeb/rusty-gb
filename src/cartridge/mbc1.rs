use crate::cartridge::Cartridge;

pub struct Mbc1 {
    _rom: Vec<u8>,
    _ram: Vec<u8>,
    _ram_on: bool,
    _ram_mode: bool,
    _rom_bank: usize,
    _ram_bank: usize
}

impl Mbc1 {
    pub fn new(data: Vec<u8>) -> Mbc1 {

        Mbc1 {
            _rom: data,
            _ram: vec![],
            _ram_on: false,
            _ram_mode: false,
            _rom_bank: 1,
            _ram_bank: 0
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