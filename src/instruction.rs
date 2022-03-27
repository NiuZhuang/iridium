/// Opcodes for the [instruction set](https://gitlab.com/subnetzero/iridium/blob/master/docs/manual.adoc).
#[derive(Debug, PartialEq)]
pub enum Opcode {
    /// LOAD $0 $1 $2
    /// 
    /// Combines the second and third operand fields into a u16 which is then loaded into the register.
    LOAD,
    /// ADD $0 $1 $2
    /// 
    /// Adds the contents of registers specified in operand 1 and 2 and places the result in register 3.
    ADD,
    /// SUB $0 $1 $2
    /// 
    /// Subtracts register 2 from register 1 and places the result in register 3
    SUB,
    /// MUL $0 $1 $2
    /// 
    /// Multiplies the contents of registers specified in operand 1 and 2 and places the result in register 3.
    MUL,
    /// DIV $0 $1 $2 
    /// 
    /// Divides the contents of registers in operand 1 and 2; results go in register 3. The remainder goes in the remainder field of the VM.
    DIV,
    /// Halts execution of the program.
    HLT,
    /// JMP $0 
    /// 
    /// Jumps directly to the address in the specified in the register.
    JMP,
    /// JMPB $0 
    /// 
    /// Relative jump backward by the number in the register.
    JMPB, 
    /// JMPB $0 
    /// 
    /// Relative jump forward by the number in the register.
    JMPF,
    /// Used if an illegal opcode got in to the bytecode. 
    IGL, 
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::LOAD,
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            5 => Opcode::HLT,
            6 => Opcode::JMP,
            7 => Opcode::JMPB,
            8 => Opcode::JMPF,
            _ => Opcode::IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
