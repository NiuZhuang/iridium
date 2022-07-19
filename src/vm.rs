use crate::instruction::Opcode;

pub struct VM {
    /// Array that simulates having hardware registers
    pub registers: [i32; 32],
    /// Program counter that tracks which byte is being executed
    pc: usize,
    /// The bytecode of the program being run
    pub program: Vec<u8>,

    /// Used for heap memory
    heap: Vec<u8>,

    /// Contains the remainder of modulo division ops
    remainder: u32,
    /// Contains the result of the last comparison operation
    equal_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            heap: vec![],
            remainder: 0,
            equal_flag: false,
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

    /// Adds an arbitrary byte to the VM's program
    pub fn add_byte(&mut self, b: u8) {
        self.program.push(b);
    }

    /// Adds arbitrary bytes list to the VM's program
    pub fn add_bytes(&mut self, mut b: Vec<u8>) {
        self.program.append(&mut b);
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
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 == register2;
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 != register2;
                self.next_8_bits();
            }
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 > register2;
                self.next_8_bits();
            }
            Opcode::GTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 >= register2;

                self.next_8_bits();
            }
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 < register2;
                self.next_8_bits();
            }
            Opcode::LTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 <= register2;
                self.next_8_bits();
            }
            Opcode::JMPE => {
                if self.equal_flag {
                    let register = self.next_8_bits() as usize;
                    let target = self.registers[register];
                    self.pc = target as usize;
                } else {
                    // TODO: Fix the bits
                }
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return false;
            },
            Opcode::NOP => {
                self.next_8_bits();
                self.next_8_bits();
                self.next_8_bits();
            },
            Opcode::ALOC => {
                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0)
            },
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
        let test_bytes = vec![Opcode::HLT as u8, 0, 0, 0];
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
        test_vm.program = vec![Opcode::LOAD as u8, 0, 1, 244]; // 用两个 u8 类型的数据，组成小端格式，以表达 500

        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 3; // JMP target
        test_vm.program = vec![Opcode::JMP as u8, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 3);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2; // JMPF value, foward 2
        test_vm.program = vec![Opcode::JMPF as u8, 0, 0, 0, Opcode::HLT as u8, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;

        test_vm.program = vec![Opcode::EQ as u8, 0, 1, 0, Opcode::EQ as u8, 0, 1, 0];
        test_vm.run_once();
        assert!(test_vm.equal_flag);

        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 20;
        test_vm.program = vec![Opcode::NEQ as u8, 0, 1, 0, Opcode::NEQ as u8, 0, 1, 0];
        test_vm.run_once();
        assert!(test_vm.equal_flag);

        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
    }

    #[test]
    fn test_gte_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![
            Opcode::GTE as u8,
            0,
            1,
            0,
            Opcode::GTE as u8,
            0,
            1,
            0,
            Opcode::GTE as u8,
            0,
            1,
            0,
        ];

        test_vm.run_once();
        assert!(test_vm.equal_flag);

        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert!(test_vm.equal_flag);

        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
    }

    #[test]
    fn test_lte_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![
            Opcode::LTE as u8,
            0,
            1,
            0,
            Opcode::LTE as u8,
            0,
            1,
            0,
            Opcode::LTE as u8,
            0,
            1,
            0,
        ];
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert!(test_vm.equal_flag);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert!(test_vm.equal_flag);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![
            Opcode::LT as u8,
            0,
            1,
            0,
            Opcode::LT as u8,
            0,
            1,
            0,
            Opcode::LT as u8,
            0,
            1,
            0,
        ];
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert!(test_vm.equal_flag);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![
            Opcode::GT as u8,
            0,
            1,
            0,
            Opcode::GT as u8,
            0,
            1,
            0,
            Opcode::GT as u8,
            0,
            1,
            0,
        ];
        test_vm.run_once();
        assert!(test_vm.equal_flag);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
    }

    #[test]
    fn test_jmpe_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![
            Opcode::JMPE as u8,
            0,
            0,
            0,
            Opcode::JMPE as u8,
            0,
            0,
            0,
            Opcode::JMPE as u8,
            0,
            0,
            0,
        ];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }

    #[test]
    fn test_aloc_opcode(){
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 1024;
        test_vm.program = vec![17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.heap.len(), 1024);
    }
}
