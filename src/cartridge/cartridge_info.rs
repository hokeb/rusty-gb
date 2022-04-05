
mod header {

    pub const ENTRY_POINT: u16 = 0x100;
    pub const LOGO: u16 = 0x104;
    pub const TITLE: u16 = 0x134;
    pub const MANUFACTURER_CODE: u16 = 0x13F;
    pub const CGB_FLAG: u16 = 0x143;
    pub const NEW_LICENSE_CODE: u16 = 0x144;
    pub const SGB_FLAG: u16 = 0x146;
    pub const CARTRIDGE_TYPE: u16 = 0x147;
    pub const ROM_SIZE: u16 = 0x148;
    pub const RAM_SIZE: u16 = 0x149;
    pub const DESTINATION_CODE: u16 = 0x14A;
    pub const OLD_LICENSE_CODE: u16 = 0x14B;
    pub const VERSION_NUMBER: u16 = 0x14C;
    pub const HEADER_CHECKSUM: u16 = 0x14D;
    pub const GLOBAL_CHECKSUM: u16 = 0x14E;
}

pub enum CartridgeType {
    RomOnly,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
    Unknown
}

impl From<u8> for CartridgeType {
    fn from(byte: u8) -> CartridgeType {
        match byte {
            0x00          => CartridgeType::RomOnly,
            0x01 ..= 0x03 => CartridgeType::Mbc1,
            0x05 ..= 0x06 => CartridgeType::Mbc2,
            0x0F ..= 0x13 => CartridgeType::Mbc3,
            0x19 ..= 0x1E => CartridgeType::Mbc5,
            _             => CartridgeType::Unknown
        }
    }
}

pub enum RomSize {
    KB32,
    KB64,
    KB128,
    KB256,
    KB512,
    MB1,
    MB2,
    MB4,
    MB1p1,
    MB1p2,
    MB1p5,
}

impl From<u8> for RomSize {
    fn from(byte: u8) -> RomSize {
        match byte {
            0x00 => RomSize::KB32,
            0x01 => RomSize::KB64,
            0x02 => RomSize::KB128,
            0x03 => RomSize::KB256,
            0x04 => RomSize::KB512,
            0x05 => RomSize::MB1,
            0x06 => RomSize::MB2,
            0x07 => RomSize::MB4,
            0x52 => RomSize::MB1p1,
            0x53 => RomSize::MB1p2,
            0x54 => RomSize::MB1p5,
               _ => {
                   println!("ROM Size {:X?} undetermined, defaulting to KB32", byte);
                   RomSize::KB32
               }
        }
    }
}

pub enum RamSize {
    None,
    KB2,
    KB8,
    KB32,
    KB128,
    KB64,
}

impl From<u8> for RamSize {
    fn from(byte: u8) -> RamSize {
        match byte {
            0x00 => RamSize::None,
            0x01 => RamSize::KB2,
            0x02 => RamSize::KB8,
            0x03 => RamSize::KB32,
            0x04 => RamSize::KB128,
            0x05 => RamSize::KB64,
               _ => {
                   println!("RAM Size {:X?} undetermined", byte);
                   RamSize::None
               }
        }
    }
}

impl RamSize {
    pub fn to_usize(&self) -> usize {
        match self {
            RamSize::None  => 0x0,
            RamSize::KB2   => 0x800,
            RamSize::KB8   => 0x2000,
            RamSize::KB32  => 0x8000,
            RamSize::KB128 => 0x20000,
            RamSize::KB64  => 0x10000,
        }
    }
}

pub struct CartridgeInfo {
    pub title: String,
    pub c_type: CartridgeType,
    pub rom_size: RomSize,
    pub ram_size: RamSize,
    pub version: u8,
}

impl CartridgeInfo {
    pub fn read(data: &[u8]) -> CartridgeInfo {
        let type_code = data[header::CARTRIDGE_TYPE as usize];
        let version_code = data[header::VERSION_NUMBER as usize];
        let rom_size_code = data[header::ROM_SIZE as usize];
        let ram_size_code = data[header::RAM_SIZE as usize];

        CartridgeInfo {
            c_type: CartridgeType::from(type_code),
            version: version_code,
            rom_size: RomSize::from(rom_size_code),
            ram_size: RamSize::from(ram_size_code),
            title: get_title(data),
        }
    }
}

fn get_title(data: &[u8]) -> String {
    String::from_utf8(Vec::from(&data[0x134..=0x143])).unwrap()
}

