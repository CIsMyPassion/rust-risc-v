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
