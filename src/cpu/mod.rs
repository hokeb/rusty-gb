mod opcodes;
use crate::cpu::opcodes::*;
use crate::mmu::Mmu;
use crate::register::Flag::*;
use crate::register::Registers;

pub struct Cpu {
    mmu: Mmu,
    reg: Registers,
    halted: bool,
    _interrupt_flag: u8,
    interrupt_enabled: bool,
}

impl Cpu {
    pub fn new(rom_path: std::path::PathBuf) -> Cpu {
        Cpu {
            mmu: Mmu::new(rom_path),
            reg: Registers::new(),
            halted: false,
            _interrupt_flag: 0, // TODO: implement this when interrupts are implemented
            interrupt_enabled: false
        }
    }

    pub fn tick(&mut self) -> u32 {
        if self.halted { return 1; }

        let pc = self.reg.pc;
        let opcode = self.get_byte();
        self.execute_opcode(opcode, pc)
    }

    fn execute_opcode(&mut self, op_code: u8, pc: u16) -> u32 {
        // TODO: make this prettier
        print!("Opcode: {:X?}, PC: {:X?}", op_code, pc);

        let (inst, mnemonic): (fn(&mut Cpu) -> u32, &'static str) = match op_code {
            0x00 => (nop,           "NOP"),
            0x01 => (ld_bc_nn,      "LD BC, NN"),
            0x02 => (ld_bc_m_a,     "LD (BC), A"),
            0x03 => (inc_bc,        "INC BC"),
            0x04 => (inc_b,         "INC B"),
            0x05 => (dec_b,         "DEC B"),
            0x06 => (ld_n_b,        "LD B, N"),
            0x07 => (rl_ca,         "RLCA"),
            0x08 => (ld_nn_sp,      "LD NN, SP"),
            0x09 => (add_hl_bc,     "ADD HL, BC"),
            0x0A => (ld_a_bc_n,     "LD A,(BC) N"),
            0x0B => (dec_bc,        "DEC BC"),
            0x0C => (inc_c,         "INC C"),
            0x0D => (dec_c,         "DEC C"),
            0x0E => (ld_c_n,        "LD C, N"),
            0x0F => (rr_ca,         "RRCA"),
            0x10 => (stop,          "STOP"), // TODO: Fix this in MMU
            0x11 => (ld_de_nn,      "LD DE, NN"),
            0x12 => (ld_de_m_a,     "LD (DE), A"),
            0x13 => (inc_de,        "INC DE"),
            0x14 => (inc_d,         "INC D"),
            0x15 => (dec_d,         "DEC D"),
            0x16 => (ld_d_n,        "LD D, N"),
            0x17 => (rl_a,          "RLA"),
            0x18 => (jr_n,          "JR N"),
            0x19 => (add_hl_de,     "ADD HL, DE"),
            0x1A => (ld_a_de_n,     "LD A,(DE) N"),
            0x1B => (dec_de,        "DEC DE"),
            0x1C => (inc_e,         "INC E"),
            0x1D => (dec_e,         "DEC E"),
            0x1E => (ld_e_n,        "LD E, N"),
            0x1F => (rr_a,          "RRA"),
            0x20 => (jr_nz_n,       "JR NZ, N"),
            0x21 => (ld_hl_nn,      "LD HL, NN"),
            0x22 => (ld_hli_a,      "LD (HL+), A"),
            0x23 => (inc_hl,        "INC HL"),
            0x24 => (inc_h,         "INC H"),
            0x25 => (dec_h,         "DEC H"),
            0x26 => (ld_h_n,        "LD H, N"),
            0x27 => (daa,           "DAA"),
            0x28 => (jr_z_n,        "JR Z, N"),
            0x29 => (add_hl_hl,     "ADD HL, HL"),
            0x2A => (ld_a_hli_n,    "LD A, (HL+) N"),
            0x2B => (dec_hl,        "DEC HL"),
            0x2C => (inc_l,         "INC L"),
            0x2D => (dec_l,         "DEC L"),
            0x2E => (ld_l_n,        "LD L, N"),
            0x2F => (cpl,           "CPL"),
            0x30 => (jr_nc_n,       "JR NC, N"),
            0x31 => (ld_sp_nn,      "LD SP, NN"),
            0x32 => (ld_hld_a,      "LD (HL-), A"),
            0x33 => (inc_sp,        "INC SP"),
            0x34 => (inc_hl_m,      "INC (HL)"),
            0x35 => (dec_hl_m,      "DEC (HL)"),
            0x36 => (ld_hl_n,       "LD (HL), N"),
            0x37 => (scf,           "SCF"),
            0x38 => (jr_c_n,        "JR C, N"),
            0x39 => (add_hl_sp,     "ADD HL, SP"),
            0x3A => (ld_a_hld_n,    "LD A, (HL-) N"),
            0x3B => (dec_sp,        "DEC SP"),
            0x3C => (inc_a,         "INC A"),
            0x3D => (dec_a,         "DEC A"),
            0x3E => (ld_a_n,        "LD A, N"),
            0x3F => (ccf,           "CCF"),
            0x40 => (ld_b_b,        "LD B, B"),
            0x41 => (ld_b_c,        "LD B, C"),
            0x42 => (ld_b_d,        "LD B, D"),
            0x43 => (ld_b_e,        "LD B, E"),
            0x44 => (ld_b_h,        "LD B, H"),
            0x45 => (ld_b_l,        "LD B, L"),
            0x46 => (ld_b_hl_n,     "LD B, (HL)"),
            0x47 => (ld_b_a,        "LD B, A"),
            0x48 => (ld_c_b,        "LD C, B"),
            0x49 => (ld_c_c,        "LD C, C"),
            0x4A => (ld_c_d,        "LD C, D"),
            0x4B => (ld_c_e,        "LD C, E"),
            0x4C => (ld_c_h,        "LD C, H"),
            0x4D => (ld_c_l,        "LD C, L"),
            0x4E => (ld_c_hl_n,     "LD C, (HL)"),
            0x4F => (ld_c_a,        "LD C, A"),
            0x50 => (ld_d_b,        "LD D, B"),
            0x51 => (ld_d_c,        "LD D, C"),
            0x52 => (ld_d_d,        "LD D, D"),
            0x53 => (ld_d_e,        "LD D, E"),
            0x54 => (ld_d_h,        "LD D, H"),
            0x55 => (ld_d_l,        "LD D, L"),
            0x56 => (ld_d_hl_n,     "LD D, (HL)"),
            0x57 => (ld_d_a,        "LD D, A"),
            0x58 => (ld_e_b,        "LD E, B"),
            0x59 => (ld_e_c,        "LD E, C"),
            0x5A => (ld_e_d,        "LD E, D"),
            0x5B => (ld_e_e,        "LD E, E"),
            0x5C => (ld_e_h,        "LD E, H"),
            0x5D => (ld_e_l,        "LD E, L"),
            0x5E => (ld_e_hl_n,     "LD E, (HL)"),
            0x5F => (ld_e_a,        "LD E, A"),
            0x60 => (ld_h_b,        "LD H, B"),
            0x61 => (ld_h_c,        "LD H, C"),
            0x62 => (ld_h_d,        "LD H, D"),
            0x63 => (ld_h_e,        "LD H, E"),
            0x64 => (ld_h_h,        "LD H, H"),
            0x65 => (ld_h_l,        "LD H, L"),
            0x66 => (ld_h_hl_n,     "LD H, (HL)"),
            0x67 => (ld_h_a,        "LD H, A"),
            0x68 => (ld_l_b,        "LD L, B"),
            0x69 => (ld_l_c,        "LD L, C"),
            0x6A => (ld_l_d,        "LD L, D"),
            0x6B => (ld_l_e,        "LD L, E"),
            0x6C => (ld_l_h,        "LD L, H"),
            0x6D => (ld_l_l,        "LD L, L"),
            0x6E => (ld_l_hl_n,     "LD L, (HL)"),
            0x6F => (ld_l_a,        "LD L, A"),
            _ => { panic!("opcode not implemented!") }
        };

        println!(" {}", mnemonic);
        let cycles = (inst)(self);
        cycles
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

    fn push_stack(&mut self, w: u16) {
        self.reg.sp -= 2;
        self.mmu.write_word(self.reg.sp, w);
    }

    fn pop_stack(&mut self) -> u16 {
        let res = self.mmu.read_word(self.reg.sp);
        self.reg.sp += 2;
        res
    }

    fn alu_add(&mut self, b: u8, usec: bool) {
        let c = if usec && self.reg.get_flag(C) { 1 } else { 0 };
        let a = self.reg.a;
        let r = a.wrapping_add(b).wrapping_add(c);
        self.reg.flag(Z, r == 0);
        self.reg.flag(H, (a & 0xF) + (b & 0xF) + c > 0xF);
        self.reg.flag(N, false);
        self.reg.flag(C, (a as u16) + (b as u16) + (c as u16) > 0xFF);
        self.reg.a = r;
    }

    fn alu_sub(&mut self, b: u8, usec: bool) {
        let c = if usec && self.reg.get_flag(C) { 1 } else { 0 };
        let a = self.reg.a;
        let r = a.wrapping_sub(b).wrapping_sub(c);
        self.reg.flag(Z, r == 0);
        self.reg.flag(H, (a & 0x0F) < (b & 0x0F) + c);
        self.reg.flag(N, true);
        self.reg.flag(C, (a as u16) < (b as u16) + (c as u16));
        self.reg.a = r;
    }

    fn alu_and(&mut self, b: u8) {
        let r = self.reg.a & b;
        self.reg.flag(Z, r == 0);
        self.reg.flag(H, true);
        self.reg.flag(C, false);
        self.reg.flag(N, false);
        self.reg.a = r;
    }

    fn alu_or(&mut self, b: u8) {
        let r = self.reg.a | b;
        self.reg.flag(Z, r == 0);
        self.reg.flag(C, false);
        self.reg.flag(H, false);
        self.reg.flag(N, false);
        self.reg.a = r;
    }

    fn alu_xor(&mut self, b: u8) {
        let r = self.reg.a ^ b;
        self.reg.flag(Z, r == 0);
        self.reg.flag(C, false);
        self.reg.flag(H, false);
        self.reg.flag(N, false);
        self.reg.a = r;
    }

    fn alu_cp(&mut self, b: u8) {
        let r = self.reg.a;
        self.alu_sub(b, false);
        self.reg.a = r;
    }

    fn alu_inc(&mut self, a: u8) -> u8 {
        let r = a.wrapping_add(1);
        self.reg.flag(Z, r == 0);
        self.reg.flag(H, (a & 0x0F) + 1 > 0x0F);
        self.reg.flag(N, false);
        return r
    }

    fn alu_dec(&mut self, a: u8) -> u8 {
        let r = a.wrapping_sub(1);
        self.reg.flag(Z, r == 0);
        self.reg.flag(H, (a & 0x0F) == 0);
        self.reg.flag(N, true);
        return r
    }

    fn alu_add16(&mut self, b: u16) {
        let a = self.reg.get_hl();
        let r = a.wrapping_add(b);
        self.reg.flag(H, (a & 0x07FF) + (b & 0x07FF) > 0x07FF);
        self.reg.flag(N, false);
        self.reg.flag(C, a > 0xFFFF - b);
        self.reg.set_hl(r);
    }

    fn alu_add16imm(&mut self, a: u16) -> u16 {
        let b = self.get_byte() as i8 as i16 as u16;
        self.reg.flag(N, false);
        self.reg.flag(Z, false);
        self.reg.flag(H, (a & 0x000F) + (b & 0x000F) > 0x000F);
        self.reg.flag(C, (a & 0x00FF) + (b & 0x00FF) > 0x00FF);
        return a.wrapping_add(b)
    }

    fn alu_swap(&mut self, a: u8) -> u8 {
        self.reg.flag(Z, a == 0);
        self.reg.flag(C, false);
        self.reg.flag(H, false);
        self.reg.flag(N, false);
        (a >> 4) | (a << 4)
    }

    fn alu_srflagupdate(&mut self, r: u8, c: bool) {
        self.reg.flag(H, false);
        self.reg.flag(N, false);
        self.reg.flag(Z, r == 0);
        self.reg.flag(C, c);
    }

    fn alu_rlc(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = (a << 1) | (if c { 1 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r
    }

    fn alu_rl(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = (a << 1) | (if self.reg.get_flag(C) { 1 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r
    }

    fn alu_rrc(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (if c { 0x80 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r
    }

    fn alu_rr(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (if self.reg.get_flag(C) { 0x80 } else { 0 });
        self.alu_srflagupdate(r, c);
        return r
    }

    fn alu_sla(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = a << 1;
        self.alu_srflagupdate(r, c);
        return r
    }

    fn alu_sra(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (a & 0x80);
        self.alu_srflagupdate(r, c);
        return r
    }

    fn alu_srl(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = a >> 1;
        self.alu_srflagupdate(r, c);
        return r
    }

    fn alu_bit(&mut self, a: u8, b: u8) {
        let r = a & (1 << (b as u32)) == 0;
        self.reg.flag(N, false);
        self.reg.flag(H, true);
        self.reg.flag(Z, r);
    }

    fn alu_daa(&mut self) {
        let mut a = self.reg.a;
        let mut adjust = if self.reg.get_flag(C) { 0x60 } else { 0x00 };
        if self.reg.get_flag(H) { adjust |= 0x06; };
        if !self.reg.get_flag(N) {
            if a & 0x0F > 0x09 { adjust |= 0x06; };
            if a > 0x99 { adjust |= 0x60; };
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }

        self.reg.flag(C, adjust >= 0x60);
        self.reg.flag(H, false);
        self.reg.flag(Z, a == 0);
        self.reg.a = a;
    }
}