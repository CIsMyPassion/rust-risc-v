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

        // check instruction group
        if Self::check_instruction_group(instruction, LUI) {
            // lui
            self.lui(instruction);
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

    fn extract_destination_register(instruction: u32) -> u8 {
        ((instruction >> 7) & 0b11111) as u8
    }

    fn extract_immediate_31_12(instruction: u32) -> u32 {
        instruction & 0xFFFFF000
    }

    fn lui(&mut self, instruction: u32) {
        // extract destination register
        let destination_register = Self::extract_destination_register(instruction);
        // extract immediate value
        let immediate_value = Self::extract_immediate_31_12(instruction);
        // store immediate value in destination register
        self.registers[destination_register as usize] = immediate_value;
    }
}

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
