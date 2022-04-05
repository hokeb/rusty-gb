use crate::cpu::Cpu;
use crate::register::Flag::{C, H, N, Z};

pub fn nop(_: &mut Cpu) {}

pub fn ld_bc_nn(cpu: &mut Cpu) {
    let w = cpu.get_word();
    cpu.reg.set_bc(w);
}

pub fn dec_b(cpu: &mut Cpu) {
    let v = cpu.reg.b.wrapping_sub(1);
    cpu.reg.flag(Z, v == 0);
    cpu.reg.flag(H, (cpu.reg.b & 0x0F) == 0);
    cpu.reg.flag(N, true);
    cpu.reg.b = v;
}

pub fn ld_b_n(cpu: &mut Cpu) {
    cpu.reg.b = cpu.get_byte();
}

pub fn ld_c_n(cpu: &mut Cpu) {
    cpu.reg.c = cpu.get_byte();
}

pub fn jr_nz_n(cpu: &mut Cpu) {
    if !cpu.reg.flag_is_set(Z) {
        cpu.cpu_jr();
    } else {
        cpu.reg.pc += 1;
    }
}

pub fn ld_hl_nn(cpu: &mut Cpu) {
    let w = cpu.get_word();
    cpu.reg.set_hl(w);
}

pub fn ld_sp_nn(cpu: &mut Cpu) {
    let w = cpu.get_word();
    cpu.reg.sp = w;
}

pub fn ld_hld_a(cpu: &mut Cpu) {
    cpu.mmu.write(cpu.reg.get_hl(), cpu.reg.a);
}

pub fn ld_a_n(cpu: &mut Cpu) {
    let b = cpu.get_byte();
    cpu.reg.a = b;
}

pub fn ld_d_b(cpu: &mut Cpu) {
    cpu.reg.b = cpu.reg.d;
}

pub fn xor_a(cpu: &mut Cpu) {
    let a = cpu.reg.a;
    let r: u8 = a ^ cpu.reg.a;
    cpu.reg.flag(Z, r == 0);
    cpu.reg.flag(C, false);
    cpu.reg.flag(H, false);
    cpu.reg.flag(N, false);
    cpu.reg.a = r;
}

pub fn jp_nn(cpu: &mut Cpu) {
    let w = cpu.get_word();
    cpu.reg.pc = w;
}

pub fn ret(_: &mut Cpu) {
    todo!()
}

pub fn call_nn(_: &mut Cpu) {
    todo!();
    // let w = cpu.get_word();
    //cpu.write_word(w);
    // cpu.reg.pc = w;
}

pub fn ld_nn_a(_: &mut Cpu) {
    todo!();
    // let w = cpu.get_word();
    // println!("Address for LD NN, A: {:X?}", w);
    //TODO: need to work on MBC for this to be complete
}