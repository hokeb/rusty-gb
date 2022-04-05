
#[derive(Debug)]
pub enum Flag {
    C = 0b0001_0000,
    H = 0b0010_0000,
    N = 0b0100_0000,
    Z = 0b1000_0000
}

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x0D,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x100,
            sp: 0xFFFE
        }
    }

    pub fn increment_pc(&mut self) {
        self.pc += 1;
    }

    pub fn decrement_pc(&mut self) {
        self.pc -= 1;
    }

    pub fn flag_is_set(&self, flag: Flag) -> bool {
        let bit_mask = flag as u8;
        (self.f & bit_mask) > 0
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.f = flags & 0xF0;
    }

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | ((self.f & 0xF0) as u16)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00FF) as u8;
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}

#[cfg(test)]
mod test {
    use crate::register::{Flag, Registers};

    #[test]
    fn test_register_af() {
        let mut r = Registers::new();

        assert_eq!(r.get_af(), 0x01B0);
        r.set_af(0x3C80);
        assert_eq!(r.get_af(), 0x3C80);

        assert_eq!(r.f, 0x80);
        assert_eq!(r.flag_is_set(Flag::Z), true);
        assert_eq!(r.flag_is_set(Flag::C), false);

        r.set_flags(0b0010_0000);
        assert_eq!(r.flag_is_set(Flag::Z), false);
        assert_eq!(r.flag_is_set(Flag::N), false);
        assert_eq!(r.flag_is_set(Flag::H), true);
        assert_eq!(r.flag_is_set(Flag::C), false);
    }

    #[test]
    fn test_register_bc() {
        let mut r = Registers::new();

        assert_eq!(r.get_bc(), 0x000D);
        r.set_bc(0x1135);
        assert_eq!(r.get_bc(), 0x1135);
    }

    #[test]
    fn test_register_de() {
        let mut r = Registers::new();

        assert_eq!(r.get_de(), 0x00D8);
        r.set_de(0xD800);
        assert_eq!(r.get_de(), 0xD800);
    }

    #[test]
    fn test_register_hl() {
        let mut r = Registers::new();

        assert_eq!(r.get_hl(), 0x014D);
        r.set_hl(0x32AA);
        assert_eq!(r.get_hl(), 0x32AA);
    }
}
