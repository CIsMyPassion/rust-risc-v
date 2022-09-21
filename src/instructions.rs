pub const LUI: u32 = 0b0110111;
pub const AUIPC: u32 = 0b0010111;
pub const JAL: u32 = 0b1101111;
pub const JALR: u32 = 0b1100111;
pub const BRANCH: u32 = 0b1100011;
pub const LOAD: u32 = 0b0000011;
pub const STORE: u32 = 0b0100011;
pub const MATHI: u32 = 0b0010011;
pub const MATH: u32 = 0b0110011;
pub const FENCE: u32 = 0b0001111;
pub const CSR: u32 = 0b1110011;
pub const MASK: u32 = 0b1111111;

pub enum BranchType {
    BEQ =  0b000000001100011,
    BNE =  0b001000001100011,
    BLT =  0b100000001100011,
    BGE =  0b101000001100011,
    BLTU = 0b110000001100011,
    BGEU = 0b111000001100011,
    MASK = 0b111000001111111,
}

pub fn lui(destination: u8, value: u32) -> u32 {
    let truncated_value = value & 0xFFFFF000;
    let shifted_destination = (destination as u32) << 7;

    truncated_value | shifted_destination | LUI
}

pub fn auipc(destination: u8, value: u32) -> u32 {
    let truncated_value = value & 0xFFFFF000;
    let shifted_destination = (destination as u32) << 7;

    truncated_value | shifted_destination | AUIPC
}

pub fn branch(branch_type: BranchType, rs1: u8, rs2: u8, offset: u16) -> u32 {
    let shifted_rs1 = (rs1 as u32) << 15;
    let shifted_rs2 = (rs2 as u32) << 20;

    let imm_4_1 =  ((offset & 0b0000000011110) as u32) << 7;
    let imm_10_5 = ((offset & 0b0011111100000) as u32) << 20;
    let imm_11 =   ((offset & 0b0100000000000) as u32) >> 4;
    let imm_12 =   ((offset & 0b1000000000000) as u32) << 19;

    branch_type as u32 | shifted_rs1 | shifted_rs2 | imm_4_1 | imm_10_5 | imm_11 | imm_12
}
