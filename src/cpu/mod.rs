mod opcodes;

use crate::cpu::opcodes::*;
use crate::mmu::Mmu;
use crate::register::Registers;
use crate::util::Cycles;

pub struct Cpu {
    mmu: Mmu,
    reg: Registers,
    halted: bool,
    _interrupt_flag: u8,
    _interrupt_enabled: bool,
}

impl Cpu {
    pub fn new(rom_path: std::path::PathBuf) -> Cpu {
        Cpu {
            mmu: Mmu::new(rom_path),
            reg: Registers::new(),
            halted: false,
            _interrupt_flag: 0, // TODO: implement this when interrupts are implemented
            _interrupt_enabled: false
        }
    }

    pub fn tick(&mut self) -> Cycles {
        if self.halted { return Cycles(1); }

        let pc = self.reg.pc;
        let opcode = self.get_byte();
        self.execute_opcode(opcode, pc)
    }

    fn get_byte(&mut self) -> u8 {
        let b = self.mmu.read(self.reg.pc);
        self.reg.increment_pc();
        b
    }

    fn get_word(&mut self) -> u16 {
        let lb = self.get_byte() as u16;
        let hb = self.get_byte() as u16;

        lb | (hb << 8)
    }

    fn cpu_jr(&mut self) {
        let n = self.get_byte() as i8;
        self.reg.pc = ((self.reg.pc as u32 as i32) + (n as i32)) as u16;
    }

    fn execute_opcode(&mut self, op_code: u8, pc: u16) -> Cycles {
        // TODO: make this prettier
        print!("Opcode: {:X?}, PC: {:X?}", op_code, pc);

        let (inst, cycles, mnemonic): (fn(&mut Cpu), u32, &'static str) = match op_code {
            0x00 => (nop,            0, "NOP"),
            0x01 => (ld_bc_nn,       0, "LD BC, NN"),
            0x05 => (dec_b,          1, "DEC B"),
            0x06 => (ld_b_n,         2, "LD B, N"),
            0x0E => (ld_c_n,         2, "LD C, N"),
            0x20 => (jr_nz_n,       2, "JR NZ, N"), // TODO: fix name
            0x21 => (ld_hl_nn,       3, "LD HL, NN"),
            0x31 => (ld_sp_nn,       0, "LD SP, NN"),
            0x32 => (ld_hld_a,       2, "LD (HL-), A"),
            0x3E => (ld_a_n,         0, "LD A, N"),
            0x50 => (ld_d_b,         0, "LD D, B"),
            0xAF => (xor_a,          1, "XOR A"),
            0xC3 => (jp_nn,          0, "JP NN"),
            0xC9 => (ret,            0, "RET"),
            0xCD => (call_nn,        0, "CALL NN"),
            0xEA => (ld_nn_a,        0, "LD NN, A"),
            _ => { panic!("opcode not implemented!") }
        };

        println!(" {}", mnemonic);
        (inst)(self);
        Cycles(cycles)
    }
}