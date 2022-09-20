pub mod instructions;

use instructions::*;

pub struct CPU {
    registers: Registers,
    pc: u32,
    ram: RAM,
}

impl CPU {
    pub fn new(ram_size: u32) -> Self {
        CPU { registers: Registers::new(), pc: 0, ram: RAM::new(ram_size) }
    }

    pub fn tick(&mut self) {
        // fetch instruchtion
        let instruction = self.ram.read_word(self.pc);

        // check instruction group
        if Self::check_instruction_group(instruction, LUI) {
            // lui
            self.lui(instruction);
        } else if Self::check_instruction_group(instruction, AUIPC) {
            // auipc
            self.auipc(instruction);
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
    }

    pub fn registers(&mut self) -> &[u32] {
        self.registers.inspect()
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
        self.registers.write(destination_register, immediate_value);
        // increment program counter
        self.pc += 4;
    }

    fn auipc(&mut self, instruction: u32) {
        // extract destination register
        let destination_register = Self::extract_destination_register(instruction);
        // extract immediate value
        let immediate_value = Self::extract_immediate_31_12(instruction);
        // add immediate value to program counter
        if immediate_value == 0 {
            self.pc += 4;
        } else {
            self.pc += immediate_value;
        }
        // write program counter to target register
        self.registers.write(destination_register, self.pc);
    }
}

pub struct Registers {
    registers: [u32; 32],
}

impl Registers {
    pub fn new() -> Self {
        Registers { registers: [0; 32] }
    }

    pub fn write(&mut self, register: u8, value: u32) {
        if register != 0 {
            self.registers[register as usize] = value;
        }
    }

    pub fn read(&self, register: u8) -> u32 {
        self.registers[register as usize]
    }

    pub fn inspect(&self) -> &[u32] {
        &self.registers[0..]
    }
}

pub struct RAM {
    data: Vec<u8>,
}

// refactor into u8
impl RAM {
    pub fn new(size: u32) -> Self {
        RAM { data: vec![0; (size * 4) as usize] }
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        self.data[address as usize]
    }

    pub fn read_word(&self, address: u32) -> u32 {
        let b0 = (self.data[(address + 0) as usize] as u32) << 24;
        let b1 = (self.data[(address + 1) as usize] as u32) << 16;
        let b2 = (self.data[(address + 2) as usize] as u32) << 8;
        let b3 = (self.data[(address + 3) as usize] as u32) << 0;

        b0 | b1 | b2 | b3
    }

    pub fn write(&mut self, address: u32, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn write_word(&mut self, address: u32, value: u32) {
        self.data[(address + 0) as usize] = (value >> 24) as u8;
        self.data[(address + 1) as usize] = (value >> 16) as u8;
        self.data[(address + 2) as usize] = (value >>  8) as u8;
        self.data[(address + 3) as usize] = (value >>  0) as u8;
    }

    pub fn inspect(&self, start: u32, length: u32) -> &[u8] {
        &self.data[start as usize..(start+length) as usize]
    }

    pub fn inspect_word(&self, start: u32, length: u32) -> Vec<u32> {
        let mut words = Vec::new();
        for i in (start..start+(length*4)).step_by(4) {
            let b0 = (self.data[(i + 0) as usize] as u32) << 24;
            let b1 = (self.data[(i + 1) as usize] as u32) << 16;
            let b2 = (self.data[(i + 2) as usize] as u32) << 8;
            let b3 = (self.data[(i + 3) as usize] as u32) << 0;

            words.push(b0 | b1 | b2 | b3);
        }

        words
    }
}
