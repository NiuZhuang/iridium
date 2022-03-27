use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            let is_continue = self.execute_instruction();
            is_done = !is_continue;
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        // 如果 pc(程序计数器) 超出 program 的长度，则结束
        if self.pc >= self.program.len() {
            return false;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 / register2;

                self.remainder = (register1 % register2) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = (target) as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc += value;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc += value;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return false;
            }
            _ => {
                println!("Unrecognized opcode found! Terminating!");
                return false;
            }
        };

        true
    }

    pub fn get_test_vm() -> VM {
        let test_vm = VM::new();
        test_vm
    }

    pub fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    pub fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    pub fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | (self.program[self.pc + 1] as u16);
        self.pc += 2;
        result
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_hlt_opcode() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_igl_opcode() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![0, 0, 1, 244]; // 用两个 u8 类型的数据，组成小端格式，以表达 500

        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 3; // JMP target
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 3);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2; // JMPF value, foward 2
        test_vm.program = vec![8, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 4);
    }
}
