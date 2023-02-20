///! 包含累加器的 cpu
pub struct CPU {
    pub registers: [u8; 16], // 16 个寄存器
    pub postion_in_memory: usize, // 程序计数器 
    pub memory: [u8; 0x1000]
}

impl CPU {
    /// 获取操作指令
    pub fn read_opcode(&self) -> u16 {
        let p = self.postion_in_memory;
        let op_1 = self.memory[p] as u16;
        let op_2 = self.memory[p+1] as u16;
        op_1 << 8 | op_2
    }
    /// 累加器执行，执行指令集
    pub fn run(&mut self) {
        loop {
            let op_code = self.read_opcode();
            self.postion_in_memory += 2; // 程序计数器自加
            
            let c = ((op_code & 0xF000) >> 12) as u8;
            let x = ((op_code & 0x0F00) >> 8) as u8;
            let y = ((op_code & 0x00F0) >> 4) as u8;
            let d = (op_code & 0x000F) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => { return },
                (0x8, _, _, 0x4) => {
                    self.add_xy(x, y)
                },
                _ => { todo!() }
            }
        }
    }
    /// 加法操作
    pub fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}