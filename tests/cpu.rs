use rust_risc_v::*;

#[test]
fn lui_test() {
    let mut cpu = CPU::new(8);
    // setup ram with instructions
    cpu.ram().write_word(0, rust_risc_v::instructions::lui(0, 0xFFFFF000));
    cpu.ram().write_word(4, rust_risc_v::instructions::lui(1, 0xFFFFF000));
    // execute instructions
    cpu.tick();
    cpu.tick();
    // verify values
    assert_eq!(cpu.registers().read(0), 0);
    assert_eq!(cpu.registers().read(1), 0xFFFFF000);
    assert_eq!(*cpu.pc(), 8);
}

#[test]
fn auipc_test() {
    let mut cpu = CPU::new(2048);
    // setup ram with instructions
    cpu.ram().write_word(0, rust_risc_v::instructions::auipc(0, 0x00000000));
    cpu.ram().write_word(4, rust_risc_v::instructions::auipc(1, 0x00000000));
    cpu.ram().write_word(8, rust_risc_v::instructions::auipc(2, 0x00001000));
    // execute instructions
    cpu.tick();
    cpu.tick();
    cpu.tick();
    // verify values
    assert_eq!(cpu.registers().read(0), 0);
    assert_eq!(cpu.registers().read(1), 8);
    assert_eq!(cpu.registers().read(2), 4104);
    assert_eq!(*cpu.pc(), 4104);
}

#[test]
fn beq_test() {
    let mut cpu = CPU::new(16);
    cpu.registers().write(1, 1);
    cpu.ram().write_word(0, rust_risc_v::instructions::branch(instructions::BranchType::BEQ, 0, 0, 8));
    cpu.ram().write_word(8, rust_risc_v::instructions::branch(instructions::BranchType::BEQ, 0, 0, 16));
    cpu.ram().write_word(24, rust_risc_v::instructions::branch(instructions::BranchType::BEQ, 0, 1, 16));
    cpu.ram().write_word(28, rust_risc_v::instructions::branch(instructions::BranchType::BEQ, 0, 0, 0b1111111100100));

    //println!("8 ins: {:#b}", rust_risc_v::instructions::branch(instructions::BranchType::BEQ, 0, 0, 16));

    cpu.tick();
    assert_eq!(*cpu.pc(), 8);

    cpu.tick();
    assert_eq!(*cpu.pc(), 24);

    cpu.tick();
    assert_eq!(*cpu.pc(), 28);

    cpu.tick();
    assert_eq!(*cpu.pc(), 0);
}

#[test]
fn bne_test() {
    let mut cpu = CPU::new(16);
    cpu.registers().write(1, 1);
    cpu.ram().write_word(0, rust_risc_v::instructions::branch(instructions::BranchType::BNE, 0, 1, 8));
    cpu.ram().write_word(8, rust_risc_v::instructions::branch(instructions::BranchType::BNE, 0, 1, 16));
    cpu.ram().write_word(24, rust_risc_v::instructions::branch(instructions::BranchType::BNE, 0, 0, 16));
    cpu.ram().write_word(28, rust_risc_v::instructions::branch(instructions::BranchType::BNE, 0, 1, 0b1111111100100));

    cpu.tick();
    assert_eq!(*cpu.pc(), 8);

    cpu.tick();
    assert_eq!(*cpu.pc(), 24);

    cpu.tick();
    assert_eq!(*cpu.pc(), 28);

    cpu.tick();
    assert_eq!(*cpu.pc(), 0);
}

#[test]
fn blt_test() {
    let mut cpu = CPU::new(16);
    cpu.registers().write(1, 1);
    cpu.registers().write(2, (-255 as i32) as u32);
    cpu.ram().write_word(0, rust_risc_v::instructions::branch(instructions::BranchType::BLT, 2, 1, 8));
    cpu.ram().write_word(8, rust_risc_v::instructions::branch(instructions::BranchType::BLT, 0, 1, 16));
    cpu.ram().write_word(24, rust_risc_v::instructions::branch(instructions::BranchType::BLT, 0, 0, 16));
    cpu.ram().write_word(28, rust_risc_v::instructions::branch(instructions::BranchType::BLT, 0, 1, 0b1111111100100));

    cpu.tick();
    assert_eq!(*cpu.pc(), 8);

    cpu.tick();
    assert_eq!(*cpu.pc(), 24);

    cpu.tick();
    assert_eq!(*cpu.pc(), 28);

    cpu.tick();
    assert_eq!(*cpu.pc(), 0);
}

#[test]
fn bge_test() {
    let mut cpu = CPU::new(16);
    cpu.registers().write(1, 1);
    cpu.registers().write(2, (-255 as i32) as u32);
    cpu.ram().write_word(0, rust_risc_v::instructions::branch(instructions::BranchType::BGE, 1, 1, 8));
    cpu.ram().write_word(8, rust_risc_v::instructions::branch(instructions::BranchType::BGE, 1, 0, 16));
    cpu.ram().write_word(24, rust_risc_v::instructions::branch(instructions::BranchType::BGE, 2, 0, 16));
    cpu.ram().write_word(28, rust_risc_v::instructions::branch(instructions::BranchType::BGE, 0, 0, 0b1111111100100));

    println!("register2: {}", cpu.registers().read(2));

    cpu.tick();
    assert_eq!(*cpu.pc(), 8);

    cpu.tick();
    assert_eq!(*cpu.pc(), 24);

    cpu.tick();
    assert_eq!(*cpu.pc(), 28);

    cpu.tick();
    assert_eq!(*cpu.pc(), 0);
}

#[test]
fn bltu_test() {
    let mut cpu = CPU::new(16);
    cpu.registers().write(1, 1);
    cpu.registers().write(2, 255);
    cpu.ram().write_word(0, rust_risc_v::instructions::branch(instructions::BranchType::BLTU, 1, 2, 8));
    cpu.ram().write_word(8, rust_risc_v::instructions::branch(instructions::BranchType::BLTU, 0, 1, 16));
    cpu.ram().write_word(24, rust_risc_v::instructions::branch(instructions::BranchType::BLTU, 0, 0, 16));
    cpu.ram().write_word(28, rust_risc_v::instructions::branch(instructions::BranchType::BLTU, 0, 1, 0b1111111100100));

    cpu.tick();
    assert_eq!(*cpu.pc(), 8);

    cpu.tick();
    assert_eq!(*cpu.pc(), 24);

    cpu.tick();
    assert_eq!(*cpu.pc(), 28);

    cpu.tick();
    assert_eq!(*cpu.pc(), 0);
}

#[test]
fn bgeu_test() {
    let mut cpu = CPU::new(16);
    cpu.registers().write(1, 1);
    cpu.registers().write(2, 255);
    cpu.ram().write_word(0, rust_risc_v::instructions::branch(instructions::BranchType::BGEU, 1, 1, 8));
    cpu.ram().write_word(8, rust_risc_v::instructions::branch(instructions::BranchType::BGEU, 1, 0, 16));
    cpu.ram().write_word(24, rust_risc_v::instructions::branch(instructions::BranchType::BGEU, 0, 2, 16));
    cpu.ram().write_word(28, rust_risc_v::instructions::branch(instructions::BranchType::BGEU, 0, 0, 0b1111111100100));

    println!("register2: {}", cpu.registers().read(2));

    cpu.tick();
    assert_eq!(*cpu.pc(), 8);

    cpu.tick();
    assert_eq!(*cpu.pc(), 24);

    cpu.tick();
    assert_eq!(*cpu.pc(), 28);

    cpu.tick();
    assert_eq!(*cpu.pc(), 0);
}
