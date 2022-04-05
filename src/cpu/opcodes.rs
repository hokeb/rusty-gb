use crate::cpu::Cpu;

pub fn nop(_: &mut Cpu) {}

pub fn ld_bc_nn(cpu: &mut Cpu) {
    let w = cpu.get_word();
    cpu.reg.set_bc(w);
}

pub fn ld_sp_nn(cpu: &mut Cpu) {
    let w = cpu.get_word();
    cpu.reg.sp = w;
}

pub fn ld_a_n(cpu: &mut Cpu) {
    let b = cpu.get_byte();
    cpu.reg.a = b;
}

pub fn ld_d_b(cpu: &mut Cpu) {
    cpu.reg.b = cpu.reg.d;
}

pub fn jp_nn(cpu: &mut Cpu) {
    let w = cpu.get_word();
    cpu.reg.pc = w;
}

pub fn ret(_: &mut Cpu) {
    todo!()
}

pub fn call_nn(cpu: &mut Cpu) {
    todo!();
    let w = cpu.get_word();
    //cpu.write_word(w);
    // cpu.reg.pc = w;
}

pub fn ld_nn_a(_: &mut Cpu) {
    todo!();
    // let w = cpu.get_word();
    // println!("Address for LD NN, A: {:X?}", w);
    //TODO: need to work on MBC for this to be complete
}