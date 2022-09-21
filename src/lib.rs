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
        if Self::check_instruction_group(instruction, MASK, LUI) {
            // lui
            self.lui(instruction);
        } else if Self::check_instruction_group(instruction, MASK, AUIPC) {
            // auipc
            self.auipc(instruction);
        } else if Self::check_instruction_group(instruction, MASK, JAL) {
            // jal
            todo!("jal not implemented");
        } else if Self::check_instruction_group(instruction, MASK, JALR) {
            // jalr
            todo!("jalr not implemented")
        } else if Self::check_instruction_group(instruction, MASK, BRANCH) {
            // branch
            self.branch(instruction);
        } else if Self::check_instruction_group(instruction, MASK, LOAD) {
            // load
            todo!("load group not implemented");
        } else if Self::check_instruction_group(instruction, MASK, STORE) {
            // store
            todo!("store group not implemented");
        } else if Self::check_instruction_group(instruction, MASK, MATHI) {
            // math intermediate
            todo!("math intermediate group not implemented");
        } else if Self::check_instruction_group(instruction, MASK, MATH) {
            // math
            todo!("math group not implemented");
        } else if Self::check_instruction_group(instruction, MASK, FENCE) {
            // fence
            todo!("fence group not implemented");
        } else if Self::check_instruction_group(instruction, MASK, CSR) {
            // csr
            todo!("csr group not implemented");
        }
    }

    pub fn registers(&mut self) -> &mut Registers {
        &mut self.registers
    }

    pub fn pc(&mut self) -> &u32 {
        &self.pc
    }

    pub fn ram(&mut self) -> &mut RAM {
        &mut self.ram
    }

    fn check_instruction_group(instruction: u32, mask: u32, group: u32) -> bool {
        let group_mask = instruction & mask;
        let group_compare = group_mask ^ group;
        let is_group = group_compare == 0;

        is_group
    }

    fn extract_rd_register(instruction: u32) -> u8 {
        ((instruction >> 7) & 0b11111) as u8
    }

    fn extract_rs1_register(instruction: u32) -> u8 {
        ((instruction >> 15) & 0b11111) as u8
    }

    fn extract_rs2_register(instruction: u32) -> u8 {
        ((instruction >> 20) & 0b11111) as u8
    }

    fn extract_immediate_31_12(instruction: u32) -> u32 {
        instruction & 0xFFFFF000
    }

    fn extract_immediate_12_1(instruction: u32) -> u16 {
        let imm_4_1 =  ((instruction >>  7) as u16) & 0b0000000011110;
        let imm_10_5 = ((instruction >> 20) as u16) & 0b0011111100000;
        let imm_11 =   ((instruction <<  4) as u16) & 0b0100000000000;
        let imm_12 =   ((instruction >> 19) as u16) & 0b1000000000000;

        imm_4_1 | imm_10_5 | imm_11 | imm_12
    }

    fn lui(&mut self, instruction: u32) {
        // extract destination register
        let rd = Self::extract_rd_register(instruction);
        // extract immediate value
        let immediate_value = Self::extract_immediate_31_12(instruction);
        // store immediate value in destination register
        self.registers.write(rd, immediate_value);
        // increment program counter
        self.pc += 4;
    }

    fn auipc(&mut self, instruction: u32) {
        // extract destination register
        let rd = Self::extract_rd_register(instruction);
        // extract immediate value
        let immediate_value = Self::extract_immediate_31_12(instruction);
        // add immediate value to program counter
        if immediate_value == 0 {
            self.pc += 4;
        } else {
            self.pc += immediate_value;
        }
        // write program counter to target register
        self.registers.write(rd, self.pc);
    }

    fn branch(&mut self, instruction: u32) {
        let rs1_index = Self::extract_rs1_register(instruction);
        let rs2_index = Self::extract_rs2_register(instruction);
        let rs1 = self.registers.read(rs1_index);
        let rs2 = self.registers.read(rs2_index);
        let immediate = Self::extract_immediate_12_1(instruction);
        let mut branch = false;

        if Self::check_instruction_group(instruction, BranchType::MASK as u32, BranchType::BEQ as u32) {
            // beq
            branch = rs1 == rs2;
        } else if Self::check_instruction_group(instruction, BranchType::MASK as u32, BranchType::BNE as u32) {
            // bne
            branch = rs1 != rs2;
        } else if Self::check_instruction_group(instruction, BranchType::MASK as u32, BranchType::BLT as u32) {
            // blt
            branch = (rs1 as i32) < (rs2 as i32);
        } else if Self::check_instruction_group(instruction, BranchType::MASK as u32, BranchType::BGE as u32) {
            // bge
            branch = (rs1 as i32) >= (rs2 as i32);
        } else if Self::check_instruction_group(instruction, BranchType::MASK as u32, BranchType::BLTU as u32) {
            // bltu
            branch = rs1 < rs2;
        } else if Self::check_instruction_group(instruction, BranchType::MASK as u32, BranchType::BGEU as u32) {
            // bgeu
            branch = rs1 >= rs2;
        }

        if branch {
            let sign = immediate & 0b1000000000000;
            let amount = immediate & 0b111111111111;

            if sign == 0 {
                self.pc += amount as u32;
            } else {
                let neg = (amount ^ 0b111111111111) as u32 + 1;
                self.pc -= neg;
            }
        } else {
            self.pc += 4;
        }
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
