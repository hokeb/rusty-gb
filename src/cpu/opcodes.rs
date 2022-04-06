use crate::cpu::Cpu;
use crate::register::Flag::*;

pub fn cb_opcode(cpu: &mut Cpu) -> u32 { cpu.execute_cb_opcode() }

pub fn nop(_: &mut Cpu) -> u32 { 1 }
pub fn stop(_: &mut Cpu) -> u32 { 1 } // TODO: fix this in MMU
pub fn halt(cpu: &mut Cpu) -> u32 { cpu.halted = true; 1 }

// LOAD / STORE
pub fn ld_a_n(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.get_byte(); 2 }
pub fn ld_c_n(cpu: &mut Cpu) -> u32 { cpu.reg.c = cpu.get_byte(); 2 }
pub fn ld_d_n(cpu: &mut Cpu) -> u32 { cpu.reg.d = cpu.get_byte(); 2 }
pub fn ld_e_n(cpu: &mut Cpu) -> u32 { cpu.reg.e = cpu.get_byte(); 2 }
pub fn ld_h_n(cpu: &mut Cpu) -> u32 { cpu.reg.h = cpu.get_byte(); 2 }
pub fn ld_l_n(cpu: &mut Cpu) -> u32 { cpu.reg.l = cpu.get_byte(); 2 }

pub fn ld_a_nn(cpu: &mut Cpu) -> u32 { let a = cpu.get_word(); cpu.reg.a = cpu.mmu.read(a); 4 }

pub fn ld_hl_n(cpu: &mut Cpu) -> u32 { let v = cpu.get_byte(); cpu.mmu.write(cpu.reg.get_hl(), v); 3 }

pub fn ld_n_b(cpu: &mut Cpu) -> u32 { cpu.reg.b = cpu.get_byte(); 2 }

pub fn ld_nn_a(cpu: &mut Cpu) -> u32 { let a = cpu.get_word(); cpu.mmu.write(a, cpu.reg.a); 4  }
pub fn ld_nn_sp(cpu: &mut Cpu) -> u32 { let a = cpu.get_word(); cpu.mmu.write_word(a, cpu.reg.sp); 5 }
pub fn ld_hl_cp_n(cpu: &mut Cpu) -> u32 { let r = cpu.alu_add16imm(cpu.reg.sp); cpu.reg.set_hl(r); 3 }

pub fn ld_a_bc_n(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.mmu.read(cpu.reg.get_bc()); 2 }
pub fn ld_a_de_n(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.mmu.read(cpu.reg.get_de()); 2 }
pub fn ld_a_hli_n(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.mmu.read(cpu.reg.hl_inc()); 2 }
pub fn ld_a_hld_n(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.mmu.read(cpu.reg.hl_dec()); 2 }

pub fn ld_a_a(_: &mut Cpu) -> u32 { 1 }
pub fn ld_a_b(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.reg.b; 1 }
pub fn ld_a_c(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.reg.c; 1 }
pub fn ld_a_d(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.reg.d; 1 }
pub fn ld_a_e(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.reg.e; 1 }
pub fn ld_a_h(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.reg.h; 1 }
pub fn ld_a_l(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.reg.l; 1 }
pub fn ld_a_hl_n(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.mmu.read(cpu.reg.get_hl()); 2  }

pub fn ld_b_b(_: &mut Cpu) -> u32 { 1 }
pub fn ld_b_a(cpu: &mut Cpu) -> u32 { cpu.reg.b = cpu.reg.a; 1 }
pub fn ld_b_c(cpu: &mut Cpu) -> u32 { cpu.reg.b = cpu.reg.c; 1 }
pub fn ld_b_d(cpu: &mut Cpu) -> u32 { cpu.reg.b = cpu.reg.d; 1 }
pub fn ld_b_e(cpu: &mut Cpu) -> u32 { cpu.reg.b = cpu.reg.e; 1 }
pub fn ld_b_h(cpu: &mut Cpu) -> u32 { cpu.reg.b = cpu.reg.h; 1 }
pub fn ld_b_l(cpu: &mut Cpu) -> u32 { cpu.reg.b = cpu.reg.l; 1 }
pub fn ld_b_hl_n(cpu: &mut Cpu) -> u32 { cpu.reg.b = cpu.mmu.read(cpu.reg.get_hl()); 2  }

pub fn ld_c_c(_: &mut Cpu) -> u32 { 1 }
pub fn ld_c_a(cpu: &mut Cpu) -> u32 { cpu.reg.c = cpu.reg.a; 1 }
pub fn ld_c_b(cpu: &mut Cpu) -> u32 { cpu.reg.c = cpu.reg.b; 1 }
pub fn ld_c_d(cpu: &mut Cpu) -> u32 { cpu.reg.c = cpu.reg.d; 1 }
pub fn ld_c_e(cpu: &mut Cpu) -> u32 { cpu.reg.c = cpu.reg.e; 1 }
pub fn ld_c_h(cpu: &mut Cpu) -> u32 { cpu.reg.c = cpu.reg.h; 1 }
pub fn ld_c_l(cpu: &mut Cpu) -> u32 { cpu.reg.c = cpu.reg.l; 1 }
pub fn ld_c_hl_n(cpu: &mut Cpu) -> u32 { cpu.reg.c = cpu.mmu.read(cpu.reg.get_hl()); 2 }

pub fn ld_d_d(_: &mut Cpu) -> u32 { 1 }
pub fn ld_d_a(cpu: &mut Cpu) -> u32 { cpu.reg.d = cpu.reg.a; 1 }
pub fn ld_d_b(cpu: &mut Cpu) -> u32 { cpu.reg.d = cpu.reg.b; 1 }
pub fn ld_d_c(cpu: &mut Cpu) -> u32 { cpu.reg.d = cpu.reg.c; 1 }
pub fn ld_d_e(cpu: &mut Cpu) -> u32 { cpu.reg.d = cpu.reg.e; 1 }
pub fn ld_d_h(cpu: &mut Cpu) -> u32 { cpu.reg.d = cpu.reg.h; 1 }
pub fn ld_d_l(cpu: &mut Cpu) -> u32 { cpu.reg.d = cpu.reg.l; 1 }
pub fn ld_d_hl_n(cpu: &mut Cpu) -> u32 { cpu.reg.d = cpu.mmu.read(cpu.reg.get_hl()); 2 }

pub fn ld_e_e(_: &mut Cpu) -> u32 { 1 }
pub fn ld_e_a(cpu: &mut Cpu) -> u32 { cpu.reg.e = cpu.reg.a; 1 }
pub fn ld_e_b(cpu: &mut Cpu) -> u32 { cpu.reg.e = cpu.reg.b; 1 }
pub fn ld_e_c(cpu: &mut Cpu) -> u32 { cpu.reg.e = cpu.reg.c; 1 }
pub fn ld_e_d(cpu: &mut Cpu) -> u32 { cpu.reg.e = cpu.reg.d; 1 }
pub fn ld_e_h(cpu: &mut Cpu) -> u32 { cpu.reg.e = cpu.reg.h; 1 }
pub fn ld_e_l(cpu: &mut Cpu) -> u32 { cpu.reg.e = cpu.reg.l; 1 }
pub fn ld_e_hl_n(cpu: &mut Cpu) -> u32 { cpu.reg.e = cpu.mmu.read(cpu.reg.get_hl()); 2 }

pub fn ld_h_h(_: &mut Cpu) -> u32 { 1 }
pub fn ld_h_a(cpu: &mut Cpu) -> u32 { cpu.reg.h = cpu.reg.a; 1 }
pub fn ld_h_b(cpu: &mut Cpu) -> u32 { cpu.reg.h = cpu.reg.b; 1 }
pub fn ld_h_c(cpu: &mut Cpu) -> u32 { cpu.reg.h = cpu.reg.c; 1 }
pub fn ld_h_d(cpu: &mut Cpu) -> u32 { cpu.reg.h = cpu.reg.d; 1 }
pub fn ld_h_e(cpu: &mut Cpu) -> u32 { cpu.reg.h = cpu.reg.e; 1 }
pub fn ld_h_l(cpu: &mut Cpu) -> u32 { cpu.reg.h = cpu.reg.l; 1 }
pub fn ld_h_hl_n(cpu: &mut Cpu) -> u32 { cpu.reg.h = cpu.mmu.read(cpu.reg.get_hl()); 2 }

pub fn ld_l_l(_: &mut Cpu) -> u32 { 1 }
pub fn ld_l_a(cpu: &mut Cpu) -> u32 { cpu.reg.l = cpu.reg.a; 1 }
pub fn ld_l_b(cpu: &mut Cpu) -> u32 { cpu.reg.l = cpu.reg.b; 1 }
pub fn ld_l_c(cpu: &mut Cpu) -> u32 { cpu.reg.l = cpu.reg.c; 1 }
pub fn ld_l_d(cpu: &mut Cpu) -> u32 { cpu.reg.l = cpu.reg.d; 1 }
pub fn ld_l_e(cpu: &mut Cpu) -> u32 { cpu.reg.l = cpu.reg.e; 1 }
pub fn ld_l_h(cpu: &mut Cpu) -> u32 { cpu.reg.l = cpu.reg.h; 1 }
pub fn ld_l_hl_n(cpu: &mut Cpu) -> u32 { cpu.reg.l = cpu.mmu.read(cpu.reg.get_hl()); 2 }

pub fn ld_hl_n_a(cpu: &mut Cpu) -> u32 { cpu.mmu.write(cpu.reg.get_hl(), cpu.reg.a); 2 }
pub fn ld_hl_n_b(cpu: &mut Cpu) -> u32 { cpu.mmu.write(cpu.reg.get_hl(), cpu.reg.b); 2 }
pub fn ld_hl_n_c(cpu: &mut Cpu) -> u32 { cpu.mmu.write(cpu.reg.get_hl(), cpu.reg.c); 2 }
pub fn ld_hl_n_d(cpu: &mut Cpu) -> u32 { cpu.mmu.write(cpu.reg.get_hl(), cpu.reg.d); 2 }
pub fn ld_hl_n_e(cpu: &mut Cpu) -> u32 { cpu.mmu.write(cpu.reg.get_hl(), cpu.reg.e); 2 }
pub fn ld_hl_n_h(cpu: &mut Cpu) -> u32 { cpu.mmu.write(cpu.reg.get_hl(), cpu.reg.h); 2 }
pub fn ld_hl_n_l(cpu: &mut Cpu) -> u32 { cpu.mmu.write(cpu.reg.get_hl(), cpu.reg.l); 2 }

pub fn ld_bc_m_a(cpu: &mut Cpu) -> u32 { cpu.mmu.write(cpu.reg.get_bc(), cpu.reg.a); 2 }
pub fn ld_de_m_a(cpu: &mut Cpu) -> u32 { cpu.mmu.write(cpu.reg.get_de(), cpu.reg.a); 2 }
pub fn ld_hli_a(cpu: &mut Cpu) -> u32 { cpu.mmu.write(cpu.reg.hl_inc(), cpu.reg.a); 2 }
pub fn ld_hld_a(cpu: &mut Cpu) -> u32 { cpu.mmu.write(cpu.reg.hl_dec(), cpu.reg.a); 2 }

pub fn ld_a_io_n(cpu: &mut Cpu) -> u32 { let a = 0xFF00 | cpu.get_byte() as u16; cpu.reg.a = cpu.mmu.read(a); 3 }
pub fn ld_a_io_c(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.mmu.read(0xFF00 | cpu.reg.c as u16); 2 }
pub fn ld_io_n_a(cpu: &mut Cpu) -> u32 { let a = 0xFF00 | cpu.get_byte() as u16; cpu.mmu.write(a, cpu.reg.a); 3 }
pub fn ld_io_c_a(cpu: &mut Cpu) -> u32 { cpu.mmu.write(0xFF00 | cpu.reg.c as u16, cpu.reg.a); 2 }

pub fn ld_bc_nn(cpu: &mut Cpu) -> u32 { let v = cpu.get_word(); cpu.reg.set_bc(v); 3 }
pub fn ld_de_nn(cpu: &mut Cpu) -> u32 { let v = cpu.get_word(); cpu.reg.set_de(v); 3 }
pub fn ld_hl_nn(cpu: &mut Cpu) -> u32 { let v = cpu.get_word(); cpu.reg.set_hl(v); 3 }
pub fn ld_sp_nn(cpu: &mut Cpu) -> u32 { cpu.reg.sp = cpu.get_word(); 3 }
pub fn ld_sp_hl(cpu: &mut Cpu) -> u32 { cpu.reg.sp = cpu.reg.get_hl(); 2 }

// DATA PROCESSING
pub fn add_a_n(cpu: &mut Cpu) -> u32 { let v = cpu.get_byte(); cpu.alu_add(v, false); 2 }
pub fn add_a_a(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.a, false); 1}
pub fn add_a_b(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.b, false); 1 }
pub fn add_a_c(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.c, false); 1 }
pub fn add_a_d(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.d, false); 1 }
pub fn add_a_e(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.e, false); 1 }
pub fn add_a_h(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.h, false); 1 }
pub fn add_a_l(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.l, false); 1 }
pub fn add_a_hl_n(cpu: &mut Cpu) -> u32 { let v = cpu.mmu.read(cpu.reg.get_hl()); cpu.alu_add(v, false); 2 }
pub fn add_sp_n(cpu: &mut Cpu) -> u32 { cpu.reg.sp = cpu.alu_add16imm(cpu.reg.sp); 4 }

pub fn add_hl_bc(cpu: &mut Cpu) -> u32 { cpu.alu_add16(cpu.reg.get_bc()); 2 }
pub fn add_hl_de(cpu: &mut Cpu) -> u32 { cpu.alu_add16(cpu.reg.get_de()); 2 }
pub fn add_hl_hl(cpu: &mut Cpu) -> u32 { cpu.alu_add16(cpu.reg.get_hl()); 2 }
pub fn add_hl_sp(cpu: &mut Cpu) -> u32 { cpu.alu_add16(cpu.reg.sp); 2 }

pub fn adc_a_n(cpu: &mut Cpu) -> u32 { let v = cpu.get_byte(); cpu.alu_add(v, true); 2 }
pub fn adc_a_a(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.a, true); 1}
pub fn adc_a_b(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.b, true); 1 }
pub fn adc_a_c(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.c, true); 1 }
pub fn adc_a_d(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.d, true); 1 }
pub fn adc_a_e(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.e, true); 1 }
pub fn adc_a_h(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.h, true); 1 }
pub fn adc_a_l(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.l, true); 1 }
pub fn adc_a_hl_n(cpu: &mut Cpu) -> u32 { let v = cpu.mmu.read(cpu.reg.get_hl()); cpu.alu_add(v, true); 2 }

pub fn sub_a_n(cpu: &mut Cpu) -> u32 { let v = cpu.get_byte(); cpu.alu_sub(v, false); 2 }
pub fn sub_a_a(cpu: &mut Cpu) -> u32 { cpu.alu_sub(cpu.reg.a, false); 1 }
pub fn sub_a_b(cpu: &mut Cpu) -> u32 { cpu.alu_sub(cpu.reg.b, false); 1 }
pub fn sub_a_c(cpu: &mut Cpu) -> u32 { cpu.alu_sub(cpu.reg.c, false); 1 }
pub fn sub_a_d(cpu: &mut Cpu) -> u32 { cpu.alu_sub(cpu.reg.d, false); 1 }
pub fn sub_a_e(cpu: &mut Cpu) -> u32 { cpu.alu_sub(cpu.reg.e, false); 1 }
pub fn sub_a_h(cpu: &mut Cpu) -> u32 { cpu.alu_sub(cpu.reg.h, false); 1 }
pub fn sub_a_l(cpu: &mut Cpu) -> u32 { cpu.alu_sub(cpu.reg.l, false); 1 }
pub fn sub_a_hl_n(cpu: &mut Cpu) -> u32 { let v = cpu.mmu.read(cpu.reg.get_hl()); cpu.alu_sub(v, false); 2 }

pub fn sbc_a_n(cpu: &mut Cpu) -> u32 { let v = cpu.get_byte(); cpu.alu_sub(v, true); 2 }
pub fn sbc_a_a(cpu: &mut Cpu) -> u32 { cpu.alu_sub(cpu.reg.a, true); 1 }
pub fn sbc_a_b(cpu: &mut Cpu) -> u32 { cpu.alu_sub(cpu.reg.b, true); 1 }
pub fn sbc_a_c(cpu: &mut Cpu) -> u32 { cpu.alu_sub(cpu.reg.c, true); 1 }
pub fn sbc_a_d(cpu: &mut Cpu) -> u32 { cpu.alu_sub(cpu.reg.d, true); 1 }
pub fn sbc_a_e(cpu: &mut Cpu) -> u32 { cpu.alu_sub(cpu.reg.e, true); 1 }
pub fn sbc_a_h(cpu: &mut Cpu) -> u32 { cpu.alu_sub(cpu.reg.h, true); 1 }
pub fn sbc_a_l(cpu: &mut Cpu) -> u32 { cpu.alu_sub(cpu.reg.l, true); 1 }
pub fn sbc_a_hl_n(cpu: &mut Cpu) -> u32 { let v = cpu.mmu.read(cpu.reg.get_hl()); cpu.alu_sub(v, true); 2 }

pub fn inc_b(cpu: &mut Cpu) -> u32 { cpu.reg.b = cpu.alu_inc(cpu.reg.b); 1 }
pub fn inc_c(cpu: &mut Cpu) -> u32 { cpu.reg.c = cpu.alu_inc(cpu.reg.c); 1 }
pub fn inc_d(cpu: &mut Cpu) -> u32 { cpu.reg.d = cpu.alu_inc(cpu.reg.d); 1 }
pub fn inc_e(cpu: &mut Cpu) -> u32 { cpu.reg.e = cpu.alu_inc(cpu.reg.e); 1 }
pub fn inc_h(cpu: &mut Cpu) -> u32 { cpu.reg.h = cpu.alu_inc(cpu.reg.h); 1 }
pub fn inc_l(cpu: &mut Cpu) -> u32 { cpu.reg.l = cpu.alu_inc(cpu.reg.l); 1 }
pub fn inc_a(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.alu_inc(cpu.reg.a); 1 }

pub fn dec_b(cpu: &mut Cpu) -> u32 { cpu.reg.b = cpu.alu_dec(cpu.reg.b); 1 }
pub fn dec_c(cpu: &mut Cpu) -> u32 { cpu.reg.c = cpu.alu_dec(cpu.reg.c); 1 }
pub fn dec_d(cpu: &mut Cpu) -> u32 { cpu.reg.d = cpu.alu_dec(cpu.reg.d); 1 }
pub fn dec_e(cpu: &mut Cpu) -> u32 { cpu.reg.e = cpu.alu_dec(cpu.reg.e); 1 }
pub fn dec_h(cpu: &mut Cpu) -> u32 { cpu.reg.h = cpu.alu_dec(cpu.reg.h); 1 }
pub fn dec_l(cpu: &mut Cpu) -> u32 { cpu.reg.l = cpu.alu_dec(cpu.reg.l); 1 }
pub fn dec_a(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.alu_dec(cpu.reg.a); 1 }

pub fn dec_bc(cpu: &mut Cpu) -> u32 { cpu.reg.set_bc(cpu.reg.get_bc().wrapping_sub(1)); 2 }
pub fn dec_de(cpu: &mut Cpu) -> u32 { cpu.reg.set_de(cpu.reg.get_de().wrapping_sub(1)); 2 }
pub fn dec_hl(cpu: &mut Cpu) -> u32 { cpu.reg.set_hl(cpu.reg.get_hl().wrapping_sub(1)); 2 }
pub fn dec_sp(cpu: &mut Cpu) -> u32 { cpu.reg.sp = cpu.reg.sp.wrapping_sub(1); 2 }

pub fn inc_bc(cpu: &mut Cpu) -> u32 { cpu.reg.set_bc(cpu.reg.get_bc().wrapping_add(1)); 2 }
pub fn inc_de(cpu: &mut Cpu) -> u32 { cpu.reg.set_de(cpu.reg.get_de().wrapping_add(1)); 2 }
pub fn inc_hl(cpu: &mut Cpu) -> u32 { cpu.reg.set_hl(cpu.reg.get_hl().wrapping_add(1)); 2 }
pub fn inc_sp(cpu: &mut Cpu) -> u32 { cpu.reg.sp = cpu.reg.sp.wrapping_add(1); 2 }

pub fn inc_hl_m(cpu: &mut Cpu) -> u32 { let a = cpu.reg.get_hl(); let v = cpu.mmu.read(a); let v2 = cpu.alu_inc(v); cpu.mmu.write(a, v2); 3 }
pub fn dec_hl_m(cpu: &mut Cpu) -> u32 { let a = cpu.reg.get_hl(); let v = cpu.mmu.read(a); let v2 = cpu.alu_dec(v); cpu.mmu.write(a, v2); 3 }

pub fn and_a_n(cpu: &mut Cpu) -> u32 { let v = cpu.get_byte(); cpu.alu_and(v); 2 }
pub fn and_a_a(cpu: &mut Cpu) -> u32 { cpu.alu_and(cpu.reg.a); 1 }
pub fn and_a_b(cpu: &mut Cpu) -> u32 { cpu.alu_and(cpu.reg.b); 1 }
pub fn and_a_c(cpu: &mut Cpu) -> u32 { cpu.alu_and(cpu.reg.c); 1 }
pub fn and_a_d(cpu: &mut Cpu) -> u32 { cpu.alu_and(cpu.reg.d); 1 }
pub fn and_a_e(cpu: &mut Cpu) -> u32 { cpu.alu_and(cpu.reg.e); 1 }
pub fn and_a_h(cpu: &mut Cpu) -> u32 { cpu.alu_and(cpu.reg.h); 1 }
pub fn and_a_l(cpu: &mut Cpu) -> u32 { cpu.alu_and(cpu.reg.l); 1 }
pub fn and_a_hl_n(cpu: &mut Cpu) -> u32 { let v = cpu.mmu.read(cpu.reg.get_hl()); cpu.alu_and(v); 2 }

pub fn xor_a_n(cpu: &mut Cpu) -> u32 { let v = cpu.get_byte(); cpu.alu_xor(v); 2 }
pub fn xor_a_a(cpu: &mut Cpu) -> u32 { cpu.alu_xor(cpu.reg.a); 1 }
pub fn xor_a_b(cpu: &mut Cpu) -> u32 { cpu.alu_xor(cpu.reg.b); 1 }
pub fn xor_a_c(cpu: &mut Cpu) -> u32 { cpu.alu_xor(cpu.reg.c); 1 }
pub fn xor_a_d(cpu: &mut Cpu) -> u32 { cpu.alu_xor(cpu.reg.d); 1 }
pub fn xor_a_e(cpu: &mut Cpu) -> u32 { cpu.alu_xor(cpu.reg.e); 1 }
pub fn xor_a_h(cpu: &mut Cpu) -> u32 { cpu.alu_xor(cpu.reg.h); 1 }
pub fn xor_a_l(cpu: &mut Cpu) -> u32 { cpu.alu_xor(cpu.reg.l); 1 }
pub fn xor_a_hl_n(cpu: &mut Cpu) -> u32 { let v = cpu.mmu.read(cpu.reg.get_hl()); cpu.alu_xor(v); 2 }

pub fn or_a_n(cpu: &mut Cpu) -> u32 { let v = cpu.get_byte(); cpu.alu_or(v); 2 }
pub fn or_a_a(cpu: &mut Cpu) -> u32 { cpu.alu_or(cpu.reg.a); 1 }
pub fn or_a_b(cpu: &mut Cpu) -> u32 { cpu.alu_or(cpu.reg.b); 1 }
pub fn or_a_c(cpu: &mut Cpu) -> u32 { cpu.alu_or(cpu.reg.c); 1 }
pub fn or_a_d(cpu: &mut Cpu) -> u32 { cpu.alu_or(cpu.reg.d); 1 }
pub fn or_a_e(cpu: &mut Cpu) -> u32 { cpu.alu_or(cpu.reg.e); 1 }
pub fn or_a_h(cpu: &mut Cpu) -> u32 { cpu.alu_or(cpu.reg.h); 1 }
pub fn or_a_l(cpu: &mut Cpu) -> u32 { cpu.alu_or(cpu.reg.l); 1 }
pub fn or_a_hl_n(cpu: &mut Cpu) -> u32 { let v = cpu.mmu.read(cpu.reg.get_hl()); cpu.alu_or(v); 2 }

pub fn cp_a_n(cpu: &mut Cpu) -> u32 { let v = cpu.get_byte(); cpu.alu_cp(v); 2 }
pub fn cp_a_a(cpu: &mut Cpu) -> u32 { cpu.alu_cp(cpu.reg.a); 1 }
pub fn cp_a_b(cpu: &mut Cpu) -> u32 { cpu.alu_cp(cpu.reg.b); 1 }
pub fn cp_a_c(cpu: &mut Cpu) -> u32 { cpu.alu_cp(cpu.reg.c); 1 }
pub fn cp_a_d(cpu: &mut Cpu) -> u32 { cpu.alu_cp(cpu.reg.d); 1 }
pub fn cp_a_e(cpu: &mut Cpu) -> u32 { cpu.alu_cp(cpu.reg.e); 1 }
pub fn cp_a_h(cpu: &mut Cpu) -> u32 { cpu.alu_cp(cpu.reg.h); 1 }
pub fn cp_a_l(cpu: &mut Cpu) -> u32 { cpu.alu_cp(cpu.reg.l); 1 }
pub fn cp_a_hl_n(cpu: &mut Cpu) -> u32 { let v = cpu.mmu.read(cpu.reg.get_hl()); cpu.alu_cp(v); 2 }

// BIT MANIPULATIONS
pub fn rl_a(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.alu_rl(cpu.reg.a); cpu.reg.flag(Z, false); 1 }
pub fn rl_ca(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.alu_rlc(cpu.reg.a); cpu.reg.flag(Z, false); 1 }
pub fn rr_a(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.alu_rr(cpu.reg.a); cpu.reg.flag(Z, false); 1 }
pub fn rr_ca(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.alu_rrc(cpu.reg.a); cpu.reg.flag(Z, false); 1 }

pub fn daa(cpu: &mut Cpu) -> u32 { cpu.alu_daa(); 1 }

pub fn scf(cpu: &mut Cpu) -> u32 { cpu.reg.flag(C, true); cpu.reg.flag(H, false); cpu.reg.flag(N, false); 1 }

pub fn cpl(cpu: &mut Cpu) -> u32 { cpu.reg.a = !cpu.reg.a; cpu.reg.flag(H, true); cpu.reg.flag(N, true); 1 }
pub fn ccf(cpu: &mut Cpu) -> u32 { let v = !cpu.reg.get_flag(C); cpu.reg.flag(C, v); cpu.reg.flag(H, false); cpu.reg.flag(N, false); 1 }

// STACK
pub fn pop_bc(cpu: &mut Cpu) -> u32 { let v = cpu.pop_stack(); cpu.reg.set_bc(v); 3 }
pub fn pop_de(cpu: &mut Cpu) -> u32 { let v = cpu.pop_stack(); cpu.reg.set_de(v); 3 }
pub fn pop_hl(cpu: &mut Cpu) -> u32 { let v = cpu.pop_stack(); cpu.reg.set_hl(v); 3 }
pub fn pop_af(cpu: &mut Cpu) -> u32 { let v = cpu.pop_stack() & 0xFFF0; cpu.reg.set_af(v); 3  }

pub fn push_bc(cpu: &mut Cpu) -> u32 { cpu.push_stack(cpu.reg.get_bc()); 4 }
pub fn push_de(cpu: &mut Cpu) -> u32 { cpu.push_stack(cpu.reg.get_de()); 4 }
pub fn push_hl(cpu: &mut Cpu) -> u32 { cpu.push_stack(cpu.reg.get_hl()); 4 }
pub fn push_af(cpu: &mut Cpu) -> u32 { cpu.push_stack(cpu.reg.get_af()); 4 }

// JUMP
pub fn jr_n(cpu: &mut Cpu) -> u32 { cpu.cpu_jr(); 3 }
pub fn jr_c_n(cpu: &mut Cpu) -> u32 { if cpu.reg.get_flag(C) { cpu.cpu_jr(); 3 } else { cpu.reg.pc += 1; 2 } }
pub fn jr_nc_n(cpu: &mut Cpu) -> u32 { if !cpu.reg.get_flag(C) { cpu.cpu_jr(); 3 } else { cpu.reg.pc += 1; 2 } }
pub fn jr_z_n(cpu: &mut Cpu) -> u32 { if cpu.reg.get_flag(Z) { cpu.cpu_jr(); 3 } else { cpu.reg.pc += 1; 2 } }
pub fn jr_nz_n(cpu: &mut Cpu) -> u32 { if !cpu.reg.get_flag(Z) { cpu.cpu_jr(); 3 } else { cpu.reg.pc += 1; 2 } }

pub fn jp_nn(cpu: &mut Cpu) -> u32 { cpu.reg.pc = cpu.get_word(); 4 }
pub fn jp_hl(cpu: &mut Cpu) -> u32 { cpu.reg.pc = cpu.reg.get_hl(); 1 }
pub fn jp_c_nn(cpu: &mut Cpu) -> u32 { if cpu.reg.get_flag(C) { cpu.reg.pc = cpu.get_word(); 4 } else { cpu.reg.pc += 2; 3 } }
pub fn jp_nc_nn(cpu: &mut Cpu) -> u32 { if !cpu.reg.get_flag(C) { cpu.reg.pc = cpu.get_word(); 4 } else { cpu.reg.pc += 2; 3 } }
pub fn jp_z_nn(cpu: &mut Cpu) -> u32 { if cpu.reg.get_flag(Z) { cpu.reg.pc = cpu.get_word(); 4 } else { cpu.reg.pc += 2; 3 } }
pub fn jp_nz_nn(cpu: &mut Cpu) -> u32 { if !cpu.reg.get_flag(Z) { cpu.reg.pc = cpu.get_word(); 4 } else { cpu.reg.pc += 2; 3 } }

pub fn call_nn(cpu: &mut Cpu) -> u32 { cpu.push_stack(cpu.reg.pc + 2); cpu.reg.pc = cpu.get_word(); 6 }
pub fn call_c_nn(cpu: &mut Cpu) -> u32 { if cpu.reg.get_flag(C) { cpu.push_stack(cpu.reg.pc + 2); cpu.reg.pc = cpu.get_word(); 6 } else { cpu.reg.pc += 2; 3 } }
pub fn call_nc_nn(cpu: &mut Cpu) -> u32 { if !cpu.reg.get_flag(C) { cpu.push_stack(cpu.reg.pc + 2); cpu.reg.pc = cpu.get_word(); 6 } else { cpu.reg.pc += 2; 3 } }
pub fn call_z_nn(cpu: &mut Cpu) -> u32 { if cpu.reg.get_flag(Z) { cpu.push_stack(cpu.reg.pc + 2); cpu.reg.pc = cpu.get_word(); 6 } else { cpu.reg.pc += 2; 3 } }
pub fn call_nz_nn(cpu: &mut Cpu) -> u32 { if !cpu.reg.get_flag(Z) { cpu.push_stack(cpu.reg.pc + 2); cpu.reg.pc = cpu.get_word(); 6 } else { cpu.reg.pc += 2; 3 } }

pub fn di(cpu: &mut Cpu) -> u32 { cpu.interrupt_enabled = false; 1 }
pub fn ei(cpu: &mut Cpu) -> u32 { cpu.interrupt_enabled = true; 1 }

pub fn ret(cpu: &mut Cpu) -> u32 { cpu.reg.pc = cpu.pop_stack(); 4 }
pub fn ret_i(cpu: &mut Cpu) -> u32 { cpu.reg.pc = cpu.pop_stack(); cpu.interrupt_enabled = true; 4 }
pub fn ret_c(cpu: &mut Cpu) -> u32 { if cpu.reg.get_flag(C) { cpu.reg.pc = cpu.pop_stack(); 5 } else { 2 } }
pub fn ret_nc(cpu: &mut Cpu) -> u32 { if !cpu.reg.get_flag(C) { cpu.reg.pc = cpu.pop_stack(); 5 } else { 2 } }
pub fn ret_z(cpu: &mut Cpu) -> u32 { if cpu.reg.get_flag(Z) { cpu.reg.pc = cpu.pop_stack(); 5 } else { 2 } }
pub fn ret_nz(cpu: &mut Cpu) -> u32 { if !cpu.reg.get_flag(Z) { cpu.reg.pc = cpu.pop_stack(); 5 } else { 2 } }

pub fn rst_00(cpu: &mut Cpu) -> u32 { cpu.push_stack(cpu.reg.pc); cpu.reg.pc = 0x00; 4 }
pub fn rst_08(cpu: &mut Cpu) -> u32 { cpu.push_stack(cpu.reg.pc); cpu.reg.pc = 0x08; 4 }
pub fn rst_10(cpu: &mut Cpu) -> u32 { cpu.push_stack(cpu.reg.pc); cpu.reg.pc = 0x10; 4 }
pub fn rst_18(cpu: &mut Cpu) -> u32 { cpu.push_stack(cpu.reg.pc); cpu.reg.pc = 0x18; 4 }
pub fn rst_20(cpu: &mut Cpu) -> u32 { cpu.push_stack(cpu.reg.pc); cpu.reg.pc = 0x20; 4 }
pub fn rst_28(cpu: &mut Cpu) -> u32 { cpu.push_stack(cpu.reg.pc); cpu.reg.pc = 0x28; 4 }
pub fn rst_30(cpu: &mut Cpu) -> u32 { cpu.push_stack(cpu.reg.pc); cpu.reg.pc = 0x30; 4 }
pub fn rst_38(cpu: &mut Cpu) -> u32 { cpu.push_stack(cpu.reg.pc); cpu.reg.pc = 0x38; 4 }