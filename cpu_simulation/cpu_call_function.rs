pub struct CPU {
    pub registers: [u8; 16], // 16 个寄存器
    pub postion_in_memory: usize, // 程序计数器
    pub memory: [u8; 4096], // 4096 内存
    pub stack: [u16; 16], // 栈内存， 从分配的内存就能看出来是存储操作地址的
    pub stack_pointer: usize // 栈指针
}

impl CPU {
    /// 读取操作指令
    fn read_code(&self) -> u16 {
        let p = self.postion_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        op_byte1 << 8 | op_byte2
    }
    pub fn run(&mut self) {
        loop {
            let opcode = self.read_code();
            self.postion_in_memory += 2;
            
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = (opcode & 0x000F) as u8;
    
            let nnn = opcode & 0x0FFF;
            
            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (0, 0, 0xE, 0xE)  => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("open other code")
            }
        }
    }
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;
        
        if sp > stack.len() {
            panic!("Stack overflow!")
        }
        
        stack[sp] = self.postion_in_memory as u16;  // 将当前 cp 指针地址的值存入栈中
        self.stack_pointer += 1;
        self.postion_in_memory = addr as usize; // 将程序计数器指向函数地址
    }
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }
        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.postion_in_memory = addr as usize; // 将栈中保存的计数器地址赋值个程序计数器
    }
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow_detected) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = dbg!(val);
        if overflow_detected {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}