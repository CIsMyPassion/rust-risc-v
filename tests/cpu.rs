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
    assert_eq!(cpu.registers()[0], 0);
    assert_eq!(cpu.registers()[1], 0xFFFFF000);
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
    assert_eq!(cpu.registers()[0], 0);
    assert_eq!(cpu.registers()[1], 8);
    assert_eq!(cpu.registers()[2], 4104);
    assert_eq!(*cpu.pc(), 4104);
}
