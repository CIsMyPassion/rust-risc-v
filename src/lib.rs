pub mod instructions;

use instructions::*;
use instructions::branch::*;
use instructions::load::LoadType;

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
        if InstructionGroup::check(instruction, InstructionGroup::LUI) {
            // lui
            self.lui(instruction);
        } else if InstructionGroup::check(instruction, InstructionGroup::AUIPC) {
            // auipc
            self.auipc(instruction);
        } else if InstructionGroup::check(instruction, InstructionGroup::JAL) {
            // jal
            self.jal(instruction);
        } else if InstructionGroup::check(instruction, InstructionGroup::JALR) {
            // jalr
            self.jalr(instruction);
        } else if InstructionGroup::check(instruction, InstructionGroup::BRANCH) {
            // branch
            self.branch(instruction);
        } else if InstructionGroup::check(instruction, InstructionGroup::LOAD) {
            // load
            self.load(instruction);
        } else if InstructionGroup::check(instruction, InstructionGroup::STORE) {
            // store
            todo!("store group not implemented");
        } else if InstructionGroup::check(instruction, InstructionGroup::MATHI) {
            // math intermediate
            todo!("math intermediate group not implemented");
        } else if InstructionGroup::check(instruction, InstructionGroup::MATH) {
            // math
            todo!("math group not implemented");
        } else if InstructionGroup::check(instruction, InstructionGroup::FENCE) {
            // fence
            todo!("fence group not implemented");
        } else if InstructionGroup::check(instruction, InstructionGroup::CSR) {
            // csr
            todo!("csr group not implemented");
        }
    }

    pub fn registers(&mut self) -> &mut Registers {
        &mut self.registers
    }

    pub fn pc(&mut self) -> &mut u32 {
        &mut self.pc
    }

    pub fn ram(&mut self) -> &mut RAM {
        &mut self.ram
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

    fn extract_immediate_20_1(instruction: u32) -> u32 {
        let imm_10_1  = (instruction >> 20) & 0b000000000011111111110;
        let imm_11    = (instruction >>  9) & 0b000000000100000000000;
        let imm_19_12 = (instruction >>  0) & 0b011111111000000000000;
        let imm_20    = (instruction >> 11) & 0b100000000000000000000;

        imm_10_1 | imm_11 | imm_19_12 | imm_20
    }

    fn extract_immediate_12_1(instruction: u32) -> u16 {
        let imm_4_1 =  ((instruction >>  7) as u16) & 0b0000000011110;
        let imm_10_5 = ((instruction >> 20) as u16) & 0b0011111100000;
        let imm_11 =   ((instruction <<  4) as u16) & 0b0100000000000;
        let imm_12 =   ((instruction >> 19) as u16) & 0b1000000000000;

        imm_4_1 | imm_10_5 | imm_11 | imm_12
    }

    fn extract_immediate_11_0(instruction: u32) -> u16 {
        ((instruction >> 20) as u16) & 0b111111111111
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

    fn jal(&mut self, instruction: u32) {
        let rd_index = Self::extract_rd_register(instruction);
        let immediate = Self::extract_immediate_20_1(instruction);

        self.registers.write(rd_index, self.pc + 4);

        let sign   = immediate & 0b100000000000000000000;
        let amount = immediate & 0b011111111111111111111;

        if sign == 0 {
            self.pc += amount;
        } else {
            let neg = (amount ^ 0b11111111111111111111) + 1;
            self.pc -= neg;
        }
    }

    fn jalr(&mut self, instruction: u32) {
        let rd_index = Self::extract_rd_register(instruction);
        let rs1_index = Self::extract_rs1_register(instruction);
        let immediate = Self::extract_immediate_11_0(instruction);
        let mut total_address = self.registers.read(rs1_index);

        self.registers.write(rd_index, self.pc + 4);

        let sign   = immediate & 0b100000000000;
        let amount = immediate & 0b011111111111;

        if sign == 0 {
            total_address += amount as u32;
        } else {
            let neg = (amount ^ 0b11111111111) + 1;
            total_address -= neg as u32;
        }

        total_address &= 0xFFFFFFFE;

        self.pc = total_address;
    }

    fn branch(&mut self, instruction: u32) {
        let rs1_index = Self::extract_rs1_register(instruction);
        let rs2_index = Self::extract_rs2_register(instruction);
        let rs1 = self.registers.read(rs1_index);
        let rs2 = self.registers.read(rs2_index);
        let immediate = Self::extract_immediate_12_1(instruction);
        let mut branch = false;

        if BranchType::check(instruction, BranchType::BEQ) {
            // beq
            branch = rs1 == rs2;
        } else if BranchType::check(instruction, BranchType::BNE) {
            // bne
            branch = rs1 != rs2;
        } else if BranchType::check(instruction, BranchType::BLT) {
            // blt
            branch = (rs1 as i32) < (rs2 as i32);
        } else if BranchType::check(instruction, BranchType::BGE) {
            // bge
            branch = (rs1 as i32) >= (rs2 as i32);
        } else if BranchType::check(instruction, BranchType::BLTU) {
            // bltu
            branch = rs1 < rs2;
        } else if BranchType::check(instruction, BranchType::BGEU) {
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

    fn load(&mut self, instruction: u32) {
        let rd_index = Self::extract_rd_register(instruction);
        let rs1_index = Self::extract_rs1_register(instruction);
        let immediate = Self::extract_immediate_11_0(instruction);
        let mut total_address = self.registers.read(rs1_index);

        let sign   = immediate & 0b100000000000;
        let amount = immediate & 0b011111111111;
        let neg = (amount ^ 0b11111111111) + 1;

        if sign == 0 {
            total_address += amount as u32;
        } else {
            total_address -= neg as u32;
        }

        if LoadType::check(instruction, LoadType::LB) {
            // lb
            let load_u8 = self.ram.read_byte(total_address);
            let load_i8 = load_u8 as i8;
            let load_i32 = load_i8 as i32;
            self.registers.write(rd_index, load_i32 as u32);

        } else if LoadType::check(instruction, LoadType::LH) {
            // lh
            let load_u16 = self.ram.read_half(total_address) as i16 as i32;
            let load_i16 = load_u16 as i16;
            let load_i32 = load_i16 as i32;
            self.registers.write(rd_index, load_i32 as u32);

        } else if LoadType::check(instruction, LoadType::LW) {
            // lw
        } else if LoadType::check(instruction, LoadType::LBU) {
            // lbu
        } else if LoadType::check(instruction, LoadType::LHU) {
            // lhu
        }

        self.pc += 4;
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

impl RAM {
    pub fn new(size: u32) -> Self {
        RAM { data: vec![0; (size * 4) as usize] }
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        self.data[address as usize]
    }

    pub fn read_half(&self, address:u32) -> u16 {
        let b0 = (self.data[(address + 0) as usize] as u16) << 8;
        let b1 = (self.data[(address + 1) as usize] as u16) << 0;

        b0 | b1
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

    pub fn write_half(&mut self, address: u32, value: u16) {
        self.data[(address + 0) as usize] = (value >> 8) as u8;
        self.data[(address + 1) as usize] = (value >> 0) as u8;
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
