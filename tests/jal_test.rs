use rust_risc_v::*;
use rust_risc_v::instructions::*;

#[test]
fn jal_test() {
    let mut cpu = CPU::new(255);
    // setup ram with instructions
    cpu.ram().write_word(0, jal(0, 32));
    cpu.ram().write_word(32, jal(1, 128));
    cpu.ram().write_word(160, jal(2, 0b111111111111101100000));

    cpu.tick();
    assert_eq!(*cpu.pc(), 32);

    cpu.tick();
    assert_eq!(*cpu.pc(), 160);

    cpu.tick();
    assert_eq!(*cpu.pc(), 0);

    assert_eq!(cpu.registers().read(0), 0);
    assert_eq!(cpu.registers().read(1), 36);
    assert_eq!(cpu.registers().read(2), 164);
}

#[test]
fn jal_far_test() {
    let mut cpu = CPU::new(262144);
    // setup ram with instructions
    cpu.ram().write_word(0, jal(1, 0b011111111111111111100));
    cpu.ram().write_word(0b011111111111111111100, jal(2, 0b100000000000000000100));

    cpu.tick();
    assert_eq!(*cpu.pc(), 1048572);

    cpu.tick();
    assert_eq!(*cpu.pc(), 0);

    assert_eq!(cpu.registers().read(0), 0);
    assert_eq!(cpu.registers().read(1), 4);
    assert_eq!(cpu.registers().read(2), 1048576);
}
