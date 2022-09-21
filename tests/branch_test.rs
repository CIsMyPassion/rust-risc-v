use rust_risc_v::*;
use rust_risc_v::instructions::branch::*;

#[test]
fn beq_test() {
    let mut cpu = CPU::new(16);
    // setup register with value
    cpu.registers().write(1, 1);
    // setup ram with instructions
    cpu.ram().write_word(0, branch(BranchType::BEQ, 0, 0, 8));
    cpu.ram().write_word(8, branch(BranchType::BEQ, 0, 0, 16));
    cpu.ram().write_word(24, branch(BranchType::BEQ, 0, 1, 16));
    cpu.ram().write_word(28, branch(BranchType::BEQ, 0, 0, 0b1111111100100));
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 8);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 24);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 28);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 0);
}

#[test]
fn bne_test() {
    let mut cpu = CPU::new(16);
    // setup register with value
    cpu.registers().write(1, 1);
    // setup ram with instructions
    cpu.ram().write_word(0, branch(BranchType::BNE, 0, 1, 8));
    cpu.ram().write_word(8, branch(BranchType::BNE, 0, 1, 16));
    cpu.ram().write_word(24, branch(BranchType::BNE, 0, 0, 16));
    cpu.ram().write_word(28, branch(BranchType::BNE, 0, 1, 0b1111111100100));
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 8);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 24);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 28);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 0);
}

#[test]
fn blt_test() {
    let mut cpu = CPU::new(16);
    // setup register with value
    cpu.registers().write(1, 1);
    cpu.registers().write(2, (-255 as i32) as u32);
    // setup ram with instructions
    cpu.ram().write_word(0, branch(BranchType::BLT, 2, 1, 8));
    cpu.ram().write_word(8, branch(BranchType::BLT, 0, 1, 16));
    cpu.ram().write_word(24, branch(BranchType::BLT, 0, 0, 16));
    cpu.ram().write_word(28, branch(BranchType::BLT, 0, 1, 0b1111111100100));
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 8);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 24);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 28);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 0);
}

#[test]
fn bge_test() {
    let mut cpu = CPU::new(16);
    // setup register with value
    cpu.registers().write(1, 1);
    cpu.registers().write(2, (-255 as i32) as u32);
    // setup ram with instructions
    cpu.ram().write_word(0, branch(BranchType::BGE, 1, 1, 8));
    cpu.ram().write_word(8, branch(BranchType::BGE, 1, 0, 16));
    cpu.ram().write_word(24, branch(BranchType::BGE, 2, 0, 16));
    cpu.ram().write_word(28, branch(BranchType::BGE, 0, 0, 0b1111111100100));
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 8);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 24);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 28);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 0);
}

#[test]
fn bltu_test() {
    let mut cpu = CPU::new(16);
    // setup register with value
    cpu.registers().write(1, 1);
    cpu.registers().write(2, 255);
    // setup ram with instructions
    cpu.ram().write_word(0, branch(BranchType::BLTU, 1, 2, 8));
    cpu.ram().write_word(8, branch(BranchType::BLTU, 0, 1, 16));
    cpu.ram().write_word(24, branch(BranchType::BLTU, 0, 0, 16));
    cpu.ram().write_word(28, branch(BranchType::BLTU, 0, 1, 0b1111111100100));
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 8);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 24);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 28);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 0);
}

#[test]
fn bgeu_test() {
    let mut cpu = CPU::new(16);
    // setup register with value
    cpu.registers().write(1, 1);
    cpu.registers().write(2, 255);
    // setup ram with instructions
    cpu.ram().write_word(0, branch(BranchType::BGEU, 1, 1, 8));
    cpu.ram().write_word(8, branch(BranchType::BGEU, 1, 0, 16));
    cpu.ram().write_word(24, branch(BranchType::BGEU, 0, 2, 16));
    cpu.ram().write_word(28, branch(BranchType::BGEU, 0, 0, 0b1111111100100));
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 8);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 24);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 28);
    // execute instruction and verify program counter
    cpu.tick();
    assert_eq!(*cpu.pc(), 0);
}
