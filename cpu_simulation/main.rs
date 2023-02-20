mod accumulator;

use accumulator::CPU;

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],  // 初始化累加器
        memory: [0; 4096],
        postion_in_memory: 0
    };
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 5;
    cpu.registers[3] = 20;

    let memory = &mut cpu.memory;
    memory[0] = 0x80; memory[1] = 0x14; // 将寄存器1 加到 寄存器 0 上
    memory[2] = 0x80; memory[3] = 0x24; // 将寄存器2 加到 寄存器 0 上
    memory[4] = 0x80; memory[5] = 0x34; // 将寄存器3 加到 寄存器 0 上

    cpu.run();
    
    println!("5 + 10 + 5 + 20 = {}", cpu.registers[0]);
}