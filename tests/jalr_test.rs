use rust_risc_v::*;
use rust_risc_v::instructions::*;

#[test]
fn jalr_test() {
    let mut cpu = CPU::new(1024);

    cpu.registers().write(1, 512);
    cpu.registers().write(2, 64);
    cpu.ram().write_word(0, jalr(3, 1, 0));
    cpu.ram().write_word(4, jalr(4, 2, 0b111111100000));
    cpu.ram().write_word(512, jalr(5, 0, 4));

    cpu.tick();
    assert_eq!(*cpu.pc(), 512);
    assert_eq!(cpu.registers().read(3), 4);

    cpu.tick();
    assert_eq!(*cpu.pc(), 4);
    assert_eq!(cpu.registers().read(5), 516);

    cpu.tick();
    assert_eq!(*cpu.pc(), 32);
    assert_eq!(cpu.registers().read(4), 8);
}
