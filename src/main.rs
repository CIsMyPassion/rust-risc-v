mod cpu;
use cpu::*;

fn main() {
    let mut cpu = CPU::new(256);
    {
        let ram = cpu.ram();
        ram.write_word(0, cpu::instructions::lui(0, 0xFF0FF000));
        ram.write_word(4, cpu::instructions::lui(1, 0x053AC000));
    }
    cpu.tick();
    cpu.tick();

    println!("pc: {}", cpu.pc());
    println!("registers: {:?}", cpu.registers());
    println!("ram: {:?}", cpu.ram().inspect(0, 16));
}
