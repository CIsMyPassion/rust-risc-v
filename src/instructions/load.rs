pub enum LoadType {
    LB   = 0b000000000000011,
    LH   = 0b001000000000011,
    LW   = 0b010000000000011,
    LBU  = 0b100000000000011,
    LHU  = 0b101000000000011,
    MASK = 0b111000001111111,
}

impl LoadType {
    pub fn check(instruction: u32, load_type: LoadType) -> bool {
        ((instruction & LoadType::MASK as u32) ^ load_type as u32) == 0
    }
}

pub fn load(load_type: LoadType, rd: u8, rs1: u8, value: u16) -> u32 {
    let shifted_rd  = (rd  as u32) <<  7;
    let shifted_rs1 = (rs1 as u32) << 15;
    let imm_11_0    = ((value as u32) & 0b111111111111) << 20;

    load_type as u32 | shifted_rd | shifted_rs1 | imm_11_0
}
