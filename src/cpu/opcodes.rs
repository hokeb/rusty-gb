use crate::cpu::Cpu;
use crate::register::Flag::*;

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

pub fn ld_hl_n(cpu: &mut Cpu) -> u32 { let v = cpu.get_byte(); cpu.mmu.write(cpu.reg.get_hl(), v); 3 }

pub fn ld_n_b(cpu: &mut Cpu) -> u32 { cpu.reg.b = cpu.get_byte(); 2 }

pub fn ld_nn_sp(cpu: &mut Cpu) -> u32 { let a = cpu.get_word(); cpu.mmu.write_word(a, cpu.reg.sp); 5 }

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

pub fn ld_bc_nn(cpu: &mut Cpu) -> u32 { let v = cpu.get_word(); cpu.reg.set_bc(v); 3 }
pub fn ld_de_nn(cpu: &mut Cpu) -> u32 { let v = cpu.get_word(); cpu.reg.set_de(v); 3 }
pub fn ld_hl_nn(cpu: &mut Cpu) -> u32 { let v = cpu.get_word(); cpu.reg.set_hl(v); 3 }
pub fn ld_sp_nn(cpu: &mut Cpu) -> u32 { cpu.reg.sp = cpu.get_word(); 3 }

// DATA PROCESSING
pub fn add_a_a(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.a, false); 1}
pub fn add_a_b(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.b, false); 1 }
pub fn add_a_c(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.c, false); 1 }
pub fn add_a_d(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.d, false); 1 }
pub fn add_a_e(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.e, false); 1 }
pub fn add_a_h(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.h, false); 1 }
pub fn add_a_l(cpu: &mut Cpu) -> u32 { cpu.alu_add(cpu.reg.l, false); 1 }
pub fn add_a_hl_n(cpu: &mut Cpu) -> u32 { let v = cpu.mmu.read(cpu.reg.get_hl()); cpu.alu_add(v, false); 2 }

pub fn add_hl_bc(cpu: &mut Cpu) -> u32 { cpu.alu_add16(cpu.reg.get_bc()); 2 }
pub fn add_hl_de(cpu: &mut Cpu) -> u32 { cpu.alu_add16(cpu.reg.get_de()); 2 }
pub fn add_hl_hl(cpu: &mut Cpu) -> u32 { cpu.alu_add16(cpu.reg.get_hl()); 2 }
pub fn add_hl_sp(cpu: &mut Cpu) -> u32 { cpu.alu_add16(cpu.reg.sp); 2 }

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

// BIT MANIPULATIONS
pub fn rl_a(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.alu_rl(cpu.reg.a); cpu.reg.flag(Z, false); 1 }
pub fn rl_ca(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.alu_rlc(cpu.reg.a); cpu.reg.flag(Z, false); 1 }
pub fn rr_a(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.alu_rr(cpu.reg.a); cpu.reg.flag(Z, false); 1 }
pub fn rr_ca(cpu: &mut Cpu) -> u32 { cpu.reg.a = cpu.alu_rrc(cpu.reg.a); cpu.reg.flag(Z, false); 1 }

pub fn daa(cpu: &mut Cpu) -> u32 { cpu.alu_daa(); 1 }

pub fn scf(cpu: &mut Cpu) -> u32 { cpu.reg.flag(C, true); cpu.reg.flag(H, false); cpu.reg.flag(N, false); 1 }

pub fn cpl(cpu: &mut Cpu) -> u32 { cpu.reg.a = !cpu.reg.a; cpu.reg.flag(H, true); cpu.reg.flag(N, true); 1 }
pub fn ccf(cpu: &mut Cpu) -> u32 { let v = !cpu.reg.get_flag(C); cpu.reg.flag(C, v); cpu.reg.flag(H, false); cpu.reg.flag(N, false); 1 }

// JUMP
pub fn jr_n(cpu: &mut Cpu) -> u32 { cpu.cpu_jr(); 3 }

pub fn jr_c_n(cpu: &mut Cpu) -> u32 { if cpu.reg.get_flag(C) { cpu.cpu_jr(); 3 } else { cpu.reg.pc += 1; 2 } }
pub fn jr_nc_n(cpu: &mut Cpu) -> u32 { if !cpu.reg.get_flag(C) { cpu.cpu_jr(); 3 } else { cpu.reg.pc += 1; 2 } }

pub fn jr_z_n(cpu: &mut Cpu) -> u32 { if cpu.reg.get_flag(Z) { cpu.cpu_jr(); 3 } else { cpu.reg.pc += 1; 2 } }
pub fn jr_nz_n(cpu: &mut Cpu) -> u32 { if !cpu.reg.get_flag(Z) { cpu.cpu_jr(); 3 } else { cpu.reg.pc += 1; 2 } }