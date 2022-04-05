pub mod cartridge_info;
mod mbc1;
mod mbc0;

use std::fs::File;
use std::io::Read;
use crate::cartridge::cartridge_info::{CartridgeInfo, CartridgeType};
use crate::cartridge::mbc0::Mbc0;
use crate::cartridge::mbc1::Mbc1;

pub trait Cartridge {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
}

pub fn new_cartridge(rom_path: std::path::PathBuf) -> Box<dyn Cartridge> {
    let mut data = vec![];
    File::open(&rom_path)
        .and_then(|mut f| f.read_to_end(&mut data))
        .unwrap();

    let cart_info = CartridgeInfo::read(&data);

    match cart_info.c_type {
        CartridgeType::RomOnly => Box::new(Mbc0::new(data, cart_info)),
        CartridgeType::Mbc1    => Box::new(Mbc1::new(data)),
        _ => panic!("No MBC type specified") // change this to Err
    }
}



