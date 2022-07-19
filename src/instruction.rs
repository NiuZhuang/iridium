/// Opcodes for the [instruction set](https://gitlab.com/subnetzero/iridium/blob/master/docs/manual.adoc).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Opcode {
    /// LOAD $0 $1 $2
    ///
    /// Combines the second and third operand fields into a u16 which is then loaded into the register.
    LOAD = 0,
    /// ADD $0 $1 $2
    ///
    /// Adds the contents of registers specified in operand 1 and 2 and places the result in register 3.
    ADD = 1,
    /// SUB $0 $1 $2
    ///
    /// Subtracts register 2 from register 1 and places the result in register 3
    SUB = 2,
    /// MUL $0 $1 $2
    ///
    /// Multiplies the contents of registers specified in operand 1 and 2 and places the result in register 3.
    MUL = 3,
    /// DIV $0 $1 $2
    ///
    /// Divides the contents of registers in operand 1 and 2; results go in register 3. The remainder goes in the remainder field of the VM.
    DIV = 4,
    /// Halts execution of the program.
    HLT = 5,
    /// JMP $0
    ///
    /// Jumps directly to the address in the specified in the register.
    JMP = 6,
    /// JMPB $0
    ///
    /// Relative jump backward by the number in the register.
    JMPB = 7,
    /// JMPB $0
    ///
    /// Relative jump forward by the number in the register.
    JMPF = 8,
    /// EQ $0 $1
    ///
    /// Checks the values in registers 1 and 2 and sets the VM equal flag to true if they are, false if not
    EQ = 9,
    /// NEQ $0 $1
    ///
    /// Checks the values in registers 1 and 2 and sets the VM equal flag to false if they are, true if not
    NEQ = 10,
    /// GTE $0 $1
    ///
    /// Checks if register 1 is >= register 2
    GTE = 11,
    /// LET $0 $1
    ///
    /// Checks if register 1 is <= register 2
    LTE = 12,
    /// LT $0 $1
    ///
    /// Checks if register 1 is < register 2
    LT = 13,
    /// GT $0 $1
    ///
    /// Checks if register 1 is > register 2
    GT = 14,
    /// JMPE $0 $1 $2
    ///
    /// Direct jump to the value in the register if the VM’s equal_flag is true
    JMPE = 15,
    /// NOP
    ///
    /// Does nothing; is a no-op.
    NOP = 16,
    /// ALOC $1
    ///
    /// Increases the heap by the amount specified in the first register
    ALOC = 17,
    /// Used if an illegal opcode got in to the bytecode.
    IGL = 100,
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
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GTE,
            12 => Opcode::LTE,
            13 => Opcode::LT,
            14 => Opcode::GT,
            15 => Opcode::JMPE,
            16 => Opcode::NOP,
            17 => Opcode::ALOC,
            _ => Opcode::IGL,
        }
    }
}

impl From<&str> for Opcode {
    fn from(v: &str) -> Self {
        match v {
            "load" => Opcode::LOAD,
            "add" => Opcode::ADD,
            "sub" => Opcode::SUB,
            "mul" => Opcode::MUL,
            "div" => Opcode::DIV,
            "hlt" => Opcode::HLT,
            "jmp" => Opcode::JMP,
            "jmpf" => Opcode::JMPF,
            "jmpb" => Opcode::JMPB,
            "eq" => Opcode::EQ,
            "neq" => Opcode::NEQ,
            "gte" => Opcode::GTE,
            "gt" => Opcode::GT,
            "lte" => Opcode::LTE,
            "lt" => Opcode::LT,
            "jmpe" => Opcode::JMPE,
            "nop" => Opcode::NOP,
            "aloc" => Opcode::ALOC,
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

    #[test]
    fn test_str_to_opcode() {
        let opcode = Opcode::from("load");
        assert_eq!(opcode, Opcode::LOAD);
        let opcode = Opcode::from("illegal");
        assert_eq!(opcode, Opcode::IGL);
    }
}
