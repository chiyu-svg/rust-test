///! CPU 最基本的基石
mod chip_cpu;

use chip_cpu::CPU;

fn main() {
    let mut cpu =CPU {
        operation_code: 0,
        registers: [0; 2],
    };
    cpu.operation_code = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    
    cpu.run();
    
    println!("5 + 10 = {}", cpu.registers[0]);
}