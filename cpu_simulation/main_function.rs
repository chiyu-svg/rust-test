mod  cpu_call;

use cpu_call::CPU;
fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        postion_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    
    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00; // 调用函数
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    
    mem[0x100] = 0x80; mem[0x101] = 0x14; // 寄存器2加到寄存器1上
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    mem[0x104] = 0x00; mem[0x105] = 0xEE; // 退出指令

    cpu.run();
    
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}