use rust_risc_v::*;
use rust_risc_v::instructions::load::*;

#[test]
fn load_test() {
    let mut cpu = CPU::new(256);

    cpu.ram().write_word(0, load(LoadType::LB, 2, 1, 0));
    cpu.ram().write_word(0, load(LoadType::LB, 3, 1, 1));
    cpu.ram().write(9, 0);
    cpu.ram().write(10, 128);
    cpu.registers().write(1, 9);

    println!("data1: {}", cpu.ram().read_byte(9));
    println!("data2: {}", cpu.ram().read_byte(10));

    cpu.tick();
    cpu.tick();

    assert_eq!(cpu.registers().read(2), 0);
    assert_eq!(cpu.registers().read(3), 128);
}
