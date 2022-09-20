mod cpu;
use cpu::*;

fn main() {
    let mut cpu = CPU::new(2048);
    {
        let ram = cpu.ram();
        ram.write_word(0, cpu::instructions::lui(1, 0x053AC000));
        ram.write_word(4, cpu::instructions::auipc(2, 0x00000000));
        ram.write_word(8, cpu::instructions::auipc(0, 0x00001000));
    }
    cpu.tick();
    cpu.tick();

    println!("pc: {}", cpu.pc());
    println!("registers: {:?}", cpu.registers());
    println!("ram: {:?}", cpu.ram().inspect(0, 16));
}
