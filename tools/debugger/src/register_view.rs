```rust id="regview01"
#[derive(Debug, Clone)]
pub struct CpuRegisters {
    pub pc: u64,   // program counter
    pub sp: u64,   // stack pointer
    pub lr: u64,   // link register
    pub cpsr: u32, // status register
}

pub struct RegisterView {
    pub registers: CpuRegisters,
}

impl RegisterView {
    pub fn new() -> Self {
        Self {
            registers: CpuRegisters {
                pc: 0x1000,
                sp: 0x8000,
                lr: 0x0,
                cpsr: 0,
            },
        }
    }

    pub fn update_pc(&mut self, value: u64) {
        self.registers.pc = value;
    }

    pub fn update_sp(&mut self, value: u64) {
        self.registers.sp = value;
    }

    pub fn update_lr(&mut self, value: u64) {
        self.registers.lr = value;
    }

    pub fn print_registers(&self) {
        println!("CPU Registers");
        println!("-------------");
        println!("PC  = 0x{:X}", self.registers.pc);
        println!("SP  = 0x{:X}", self.registers.sp);
        println!("LR  = 0x{:X}", self.registers.lr);
        println!("CPSR= 0x{:X}", self.registers.cpsr);
    }

    pub fn dump_state(&self) -> CpuRegisters {
        self.registers.clone()
    }
}
```
