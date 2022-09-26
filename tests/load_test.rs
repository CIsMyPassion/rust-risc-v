use rust_risc_v::*;
use rust_risc_v::instructions::load::*;

#[test]
fn load_byte() {
    let mut cpu = CPU::new(256);

    cpu.ram().write_word(0, load(LoadType::LB, 2, 1, 0));
    cpu.ram().write_word(4, load(LoadType::LB, 3, 1, 1));
    cpu.ram().write(9, 0);
    cpu.ram().write(10, 129);
    cpu.registers().write(1, 9);

    cpu.tick();
    cpu.tick();

    assert_eq!(cpu.registers().read(2) as i32, 0);
    assert_eq!(cpu.registers().read(3) as i32, -127);
}

#[test]
fn load_half() {
    let mut cpu = CPU::new(256);

    cpu.ram().write_word(0, load(LoadType::LH, 2, 1, 0));
    cpu.ram().write_word(4, load(LoadType::LH, 3, 1, 2));
    cpu.ram().write_word(8, load(LoadType::LH, 4, 1, 4));
    cpu.ram().write_half(12, 0);
    cpu.ram().write_half(14, 128);
    cpu.ram().write_half(16, 65535);
    cpu.registers().write(1, 12);

    cpu.tick();
    cpu.tick();
    cpu.tick();

    assert_eq!(cpu.registers().read(2) as i32, 0);
    assert_eq!(cpu.registers().read(3) as i32, 128);
    assert_eq!(cpu.registers().read(4) as i32, -1);
}

#[test]
fn load_word() {
    let mut cpu = CPU::new(256);

    cpu.ram().write_word(0, load(LoadType::LW, 2, 1, 0));
    cpu.ram().write_word(4, load(LoadType::LW, 3, 1, 4));
    cpu.ram().write_word(8, load(LoadType::LW, 4, 1, 8));
    cpu.ram().write_word(12, 0);
    cpu.ram().write_word(16, 2048);
    cpu.ram().write_word(20, 4294967295);
    cpu.registers().write(1, 12);

    cpu.tick();
    cpu.tick();
    cpu.tick();

    assert_eq!(cpu.registers().read(2) as i32, 0);
    assert_eq!(cpu.registers().read(3) as i32, 2048);
    assert_eq!(cpu.registers().read(4) as i32, -1);
}
