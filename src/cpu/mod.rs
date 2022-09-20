pub mod instructions;

use instructions::*;

pub struct CPU {
    registers: [u32; 32],
    pc: u32,
    ram: RAM,
}

impl CPU {
    pub fn new(ram_size: u32) -> Self {
        CPU { registers: [0; 32], pc: 0, ram: RAM::new(ram_size) }
    }

    pub fn tick(&mut self) {
        // fetch instruchtion
        let instruction = self.ram.read(self.pc);

        if Self::check_instruction_group(instruction, LUI) {
            // lui
            todo!("lui not implemented");
        } else if Self::check_instruction_group(instruction, AUIPC) {
            // auipc
            todo!("auipc not implemented");
        } else if Self::check_instruction_group(instruction, JAL) {
            // jal
            todo!("jal not implemented");
        } else if Self::check_instruction_group(instruction, JALR) {
            // jalr
            todo!("jalr not implemented")
        } else if Self::check_instruction_group(instruction, BRANCH) {
            // branch
            todo!("branch group not implemented");
        } else if Self::check_instruction_group(instruction, LOAD) {
            // load
            todo!("load group not implemented");
        } else if Self::check_instruction_group(instruction, STORE) {
            // store
            todo!("store group not implemented");
        } else if Self::check_instruction_group(instruction, MATHI) {
            // math intermediate
            todo!("math intermediate group not implemented");
        } else if Self::check_instruction_group(instruction, MATH) {
            // math
            todo!("math group not implemented");
        } else if Self::check_instruction_group(instruction, FENCE) {
            // fence
            todo!("fence group not implemented");
        } else if Self::check_instruction_group(instruction, CSR) {
            // csr
            todo!("csr group not implemented");
        }

        // increase program counter
        self.pc += 1;
    }

    pub fn registers(&mut self) -> &mut [u32; 32] {
        &mut self.registers
    }

    pub fn pc(&mut self) -> &mut u32 {
        &mut self.pc
    }

    pub fn ram(&mut self) -> &mut RAM {
        &mut self.ram
    }

    fn check_instruction_group(instruction: u32, group: u32) -> bool {
        instruction & group == group
    }
}

/*
pub struct Registers {
    pub x0: u32,
    pub x1: u32,
    pub x2: u32,
    pub x3: u32,
    pub x4: u32,
    pub x5: u32,
    pub x6: u32,
    pub x7: u32,
    pub x8: u32,
    pub x9: u32,
    pub x10: u32,
    pub x11: u32,
    pub x12: u32,
    pub x13: u32,
    pub x14: u32,
    pub x15: u32,
    pub x16: u32,
    pub x17: u32,
    pub x18: u32,
    pub x19: u32,
    pub x20: u32,
    pub x21: u32,
    pub x22: u32,
    pub x23: u32,
    pub x24: u32,
    pub x25: u32,
    pub x26: u32,
    pub x27: u32,
    pub x28: u32,
    pub x29: u32,
    pub x30: u32,
    pub x31: u32,
}

impl Registers {
    pub fn new() -> Self {
        Registers { x0: 0, x1: 0, x2: 0, x3: 0,
                    x4: 0, x5: 0, x6: 0, x7: 0,
                    x8: 0, x9: 0, x10: 0, x11: 0,
                    x12: 0, x13: 0, x14: 0, x15: 0,
                    x16: 0, x17: 0, x18: 0, x19: 0,
                    x20: 0, x21: 0, x22: 0, x23: 0,
                    x24: 0, x25: 0, x26: 0, x27: 0,
                    x28: 0, x29: 0, x30: 0, x31: 0 }
    }
}
*/

pub struct RAM {
    data: Vec<u32>,
}

// refactor into u8
impl RAM {
    pub fn new(size: u32) -> Self {
        RAM { data: vec![0; size as usize] }
    }

    pub fn read(&self, address: u32) -> u32 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u32, value: u32) {
        self.data[address as usize] = value;
    }

    pub fn inspect(&self, start: u32, length: u32) -> &[u32] {
        &self.data[start as usize..(start+length) as usize]
    }
}
