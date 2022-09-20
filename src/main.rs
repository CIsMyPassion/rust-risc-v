mod cpu;
use cpu::*;

fn main() {
    let mut cpu = CPU::new(256);
    {
        let ram = cpu.ram();
        ram.write(0, cpu::instructions::lui(0, 0xFF0FF000));
        ram.write(1, cpu::instructions::lui(1, 0x053AC000));
        ram.write(2, 576);
        ram.write(3, 1982);
    }
    cpu.tick();
    cpu.tick();

    println!("pc: {}", cpu.pc());
    println!("registers: {:?}", cpu.registers());
    println!("ram: {:?}", cpu.ram().inspect(0, 16));
}
