use crate::cartridge::Cartridge;
use crate::cartridge::cartridge_info::CartridgeInfo;

pub struct Mbc0 {
    _cart_info: CartridgeInfo,
    rom: Vec<u8>,
    _ram: Vec<u8>,
    _ram_on: bool,
    _ram_mode: bool,
    _rom_bank: u16,
    _ram_bank: u16
}

impl Mbc0 {
    pub fn new(data: Vec<u8>, cart_info: CartridgeInfo) -> Mbc0 {

        let cart_ram_size: usize = cart_info.ram_size.to_usize();

        Mbc0 {
            _cart_info: cart_info,
            rom: data,
            _ram: vec![0; cart_ram_size],
            _ram_on: false,
            _ram_mode: false,
            _rom_bank: 1,
            _ram_bank: 0
        }
    }
}

impl Cartridge for Mbc0 {
    fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn write(&mut self, _: u16, _: u8) {
        eprintln!("Attempting to write to cartridge ROM without an MBC")
    }
}