mod cpu;
use cpu::*;

fn main() {
    let mut cpu = CPU::new(256);
    {
        let ram = cpu.ram();
        println!("{:?}", ram.inspect(0, 16));

        ram.write(0, 0b0110111);
        ram.write(1, 100);
        ram.write(2, 576);
        ram.write(3, 1982);

        println!("{:?}", ram.inspect(0, 16));
    }
    println!("{}", cpu.pc());
    cpu.tick();
    println!("{}", cpu.pc());
    println!("{:?}", cpu.registers());
}
