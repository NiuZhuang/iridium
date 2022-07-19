use crate::assembler::instruction_parsers::{instruction, AssemblerInstruction};

use nom::{
    multi::many1,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

pub fn program(input: &str) -> IResult<&str, Program> {
    let (input, instructions) = many1(instruction)(input)?;

    Ok((input, Program { instructions }))
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes());
        }

        program
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let result = program("load $0 #100");
        assert!(result.is_ok());
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(1, p.instructions.len());
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program("load $0 #100");
        assert!(result.is_ok());
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode);
    }
}
