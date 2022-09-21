pub mod branch;

pub enum InstructionGroup {
    LUI    = 0b0110111,
    AUIPC  = 0b0010111,
    JAL    = 0b1101111,
    JALR   = 0b1100111,
    BRANCH = 0b1100011,
    LOAD   = 0b0000011,
    STORE  = 0b0100011,
    MATHI  = 0b0010011,
    MATH   = 0b0110011,
    FENCE  = 0b0001111,
    CSR    = 0b1110011,
    MASK   = 0b1111111,
}

impl InstructionGroup {
    pub fn check(instruction: u32, group: InstructionGroup) -> bool {
        ((instruction & InstructionGroup::MASK as u32) ^ group as u32) == 0
    }
}

pub fn lui(destination: u8, value: u32) -> u32 {
    let truncated_value = value & 0xFFFFF000;
    let shifted_destination = (destination as u32) << 7;

    truncated_value | shifted_destination | InstructionGroup::LUI as u32
}

pub fn auipc(destination: u8, value: u32) -> u32 {
    let truncated_value = value & 0xFFFFF000;
    let shifted_destination = (destination as u32) << 7;

    truncated_value | shifted_destination | InstructionGroup::AUIPC as u32
}

pub fn jal(destination: u8, value: u32) -> u32 {
    let shifted_destination = (destination as u32) << 7;
    let imm_10_1  = (value & 0b000000000011111111110) << 20;
    let imm_11    = (value & 0b000000000100000000000) <<  9;
    let imm_19_12 = (value & 0b011111111000000000000) <<  0;
    let imm_20    = (value & 0b100000000000000000000) << 11;

    imm_10_1 | imm_11 | imm_19_12 | imm_20 | shifted_destination | InstructionGroup::JAL as u32
}
