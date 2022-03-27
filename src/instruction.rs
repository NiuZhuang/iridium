#[derive(Debug, PartialEq)]
pub enum Opcode {
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    HLT,  // Halt
    JMP,  // Jump
    JMPB, // Jump backwards
    JMPF, // Jump forwards
    IGL,  // Illegal
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
