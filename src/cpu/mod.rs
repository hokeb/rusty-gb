mod opcodes;
use crate::cpu::opcodes::*;
use crate::mmu::Mmu;
use crate::register::Flag::*;
use crate::register::Registers;

pub struct Cpu {
    mmu: Mmu,
    pub reg: Registers,
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
            0x70 => (ld_hl_n_b,     "LD (HL), B"),
            0x71 => (ld_hl_n_c,     "LD (HL), C"),
            0x72 => (ld_hl_n_d,     "LD (HL), D"),
            0x73 => (ld_hl_n_e,     "LD (HL), E"),
            0x74 => (ld_hl_n_h,     "LD (HL), H"),
            0x75 => (ld_hl_n_l,     "LD (HL), L"),
            0x76 => (halt,          "HALT"),
            0x77 => (ld_hl_n_a,     "LD (HL), A"),
            0x78 => (ld_a_b,        "LD A, B"),
            0x79 => (ld_a_c,        "LD A, C"),
            0x7A => (ld_a_d,        "LD A, D"),
            0x7B => (ld_a_e,        "LD A, E"),
            0x7C => (ld_a_h,        "LD A, H"),
            0x7D => (ld_a_l,        "LD A, L"),
            0x7E => (ld_a_hl_n,     "LD A, (HL)"),
            0x7F => (ld_a_a,        "LD A, A"),
            0x80 => (add_a_b,       "ADD A, B"),
            0x81 => (add_a_c,       "ADD A, C"),
            0x82 => (add_a_d,       "ADD A, D"),
            0x83 => (add_a_e,       "ADD A, E"),
            0x84 => (add_a_h,       "ADD A, H"),
            0x85 => (add_a_l,       "ADD A, L"),
            0x86 => (add_a_hl_n,    "ADD A, (HL)"),
            0x87 => (add_a_a,       "ADD A, A"),
            0x88 => (adc_a_b,       "ADC A, B"),
            0x89 => (adc_a_c,       "ADC A, C"),
            0x8A => (adc_a_d,       "ADC A, D"),
            0x8B => (adc_a_e,       "ADC A, E"),
            0x8C => (adc_a_h,       "ADC A, H"),
            0x8D => (adc_a_l,       "ADC A, L"),
            0x8E => (adc_a_hl_n,    "ADC A, (HL)"),
            0x8F => (adc_a_a,       "ADC A, A"),
            0x90 => (sub_a_b,       "SUB A, B"),
            0x91 => (sub_a_c,       "SUB A, C"),
            0x92 => (sub_a_d,       "SUB A, D"),
            0x93 => (sub_a_e,       "SUB A, E"),
            0x94 => (sub_a_h,       "SUB A, H"),
            0x95 => (sub_a_l,       "SUB A, L"),
            0x96 => (sub_a_hl_n,    "SUB A, (HL)"),
            0x97 => (sub_a_a,       "SUB A, A"),
            0x98 => (sbc_a_b,       "SBC A, B"),
            0x99 => (sbc_a_c,       "SBC A, C"),
            0x9A => (sbc_a_d,       "SBC A, D"),
            0x9B => (sbc_a_e,       "SBC A, E"),
            0x9C => (sbc_a_h,       "SBC A, H"),
            0x9D => (sbc_a_l,       "SBC A, L"),
            0x9E => (sbc_a_hl_n,    "SBC A, (HL)"),
            0x9F => (sbc_a_a,       "SBC A, A"),
            0xA0 => (and_a_b,       "AND A, B"),
            0xA1 => (and_a_c,       "AND A, C"),
            0xA2 => (and_a_d,       "AND A, D"),
            0xA3 => (and_a_e,       "AND A, E"),
            0xA4 => (and_a_h,       "AND A, H"),
            0xA5 => (and_a_l,       "AND A, L"),
            0xA6 => (and_a_hl_n,    "AND A, (HL)"),
            0xA7 => (and_a_a,       "AND A, A"),
            0xA8 => (xor_a_b,       "XOR A, B"),
            0xA9 => (xor_a_c,       "XOR A, C"),
            0xAA => (xor_a_d,       "XOR A, D"),
            0xAB => (xor_a_e,       "XOR A, E"),
            0xAC => (xor_a_h,       "XOR A, H"),
            0xAD => (xor_a_l,       "XOR A, L"),
            0xAE => (xor_a_hl_n,    "XOR A, (HL)"),
            0xAF => (xor_a_a,       "XOR A, A"),
            0xB0 => (or_a_b,        "OR A, B"),
            0xB1 => (or_a_c,        "OR A, C"),
            0xB2 => (or_a_d,        "OR A, D"),
            0xB3 => (or_a_e,        "OR A, E"),
            0xB4 => (or_a_h,        "OR A, H"),
            0xB5 => (or_a_l,        "OR A, L"),
            0xB6 => (or_a_hl_n,     "OR A, (HL)"),
            0xB7 => (or_a_a,        "OR A, A"),
            0xB8 => (cp_a_b,        "CP A, B"),
            0xB9 => (cp_a_c,        "CP A, C"),
            0xBA => (cp_a_d,        "CP A, D"),
            0xBB => (cp_a_e,        "CP A, E"),
            0xBC => (cp_a_h,        "CP A, H"),
            0xBD => (cp_a_l,        "CP A, L"),
            0xBE => (cp_a_hl_n,     "CP A, (HL)"),
            0xBF => (cp_a_a,        "CP A, A"),
            0xC0 => (ret_nz,        "RET NZ"),
            0xC1 => (pop_bc,        "POP BC"),
            0xC2 => (jp_nz_nn,      "JP NZ, NN"),
            0xC3 => (jp_nn,         "JP NN"),
            0xC4 => (call_nz_nn,    "CALL NZ, NN"),
            0xC5 => (push_bc,       "PUSH BC"),
            0xC6 => (add_a_n,       "ADD A, N"),
            0xC7 => (rst_00,        "RST 00h"),
            0xC8 => (ret_z,         "RET Z"),
            0xC9 => (ret,           "RET"),
            0xCA => (jp_z_nn,       "JP Z, NN"),
            0xCB => (cb_opcode,     "CB"),
            0xCC => (call_z_nn,     "CALL Z, NN"),
            0xCD => (call_nn,       "CALL NN"),
            0xCE => (adc_a_n,       "ADC A, N"),
            0xCF => (rst_08,        "RST 08h"),
            0xD0 => (ret_nc,        "RET NC"),
            0xD1 => (pop_de,        "POP DE"),
            0xD2 => (jp_nc_nn,      "JP NC, NN"),
            0xD4 => (call_nc_nn,    "CALL NC, NN"),
            0xD5 => (push_de,       "PUSH DE"),
            0xD6 => (sub_a_n,       "SUB A, N"),
            0xD7 => (rst_10,        "RST 10h"),
            0xD8 => (ret_c,         "RET C"),
            0xD9 => (ret_i,         "RETI"),
            0xDA => (jp_c_nn,       "JP C, NN"),
            0xDC => (call_c_nn,     "CALL C, NN"),
            0xDE => (sbc_a_n,       "SBC A, N"),
            0xDF => (rst_18,        "RST 18h"),
            0xE0 => (ld_io_n_a,     "LD (FF00+u8), A"),
            0xE1 => (pop_hl,        "POP HL"),
            0xE2 => (ld_io_c_a,     "LD (FF00+C), A"),
            0xE5 => (push_hl,       "PUSH HL"),
            0xE6 => (and_a_n,       "AND A, N"),
            0xE7 => (rst_20,        "RST 20h"),
            0xE8 => (add_sp_n,      "ADD SP, N"),
            0xE9 => (jp_hl,         "JP HL"),
            0xEA => (ld_nn_a,       "LD NN, A"),
            0xEE => (xor_a_n,       "XOR A, N"),
            0xEF => (rst_28,        "RST 28h"),
            0xF0 => (ld_a_io_n,     "LD A, (FFOO+u8)"),
            0xF1 => (pop_af,        "POP AF"),
            0xF2 => (ld_a_io_c,     "LD A, (FF00+C)"),
            0xF3 => (di,            "DI"),
            0xF5 => (push_af,       "PUSH AF"),
            0xF6 => (or_a_n,        "OR A, N"),
            0xF7 => (rst_30,        "RST 30h"),
            0xF8 => (ld_hl_cp_n,    "LD HL, SP+i8"),
            0xF9 => (ld_sp_hl,      "LD SP, HL"),
            0xFA => (ld_a_nn,       "LD A, NN"),
            0xFB => (ei,            "EI"),
            0xFE => (cp_a_n,        "CP A, N"),
            0xFF => (rst_38,        "RST 38"),
            _ => { panic!("opcode not implemented!") }
        };

        println!(" {}", mnemonic);
        let cycles = (inst)(self);
        cycles
    }

    fn execute_cb_opcode(&mut self) -> u32 {
        let op_code = self.get_byte();
        match op_code {
            _ => panic!()
        }
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