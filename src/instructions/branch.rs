pub enum BranchType {
    BEQ  = 0b000000001100011,
    BNE  = 0b001000001100011,
    BLT  = 0b100000001100011,
    BGE  = 0b101000001100011,
    BLTU = 0b110000001100011,
    BGEU = 0b111000001100011,
    MASK = 0b111000001111111,
}

impl BranchType {
    pub fn check(instruction: u32, branch_type: BranchType) -> bool {
        ((instruction & BranchType::MASK as u32) ^ branch_type as u32) == 0
    }
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
