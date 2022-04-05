use crate::cpu::Cpu;

pub struct GameBoy {
    cpu: Cpu
}

impl GameBoy {
    pub fn new(rom_path: std::path::PathBuf) -> GameBoy {
        GameBoy {
            cpu: Cpu::new(rom_path)
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.tick();
        }
    }
}