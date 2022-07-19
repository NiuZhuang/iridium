use crate::assembler::opcode_parsers::*;
use crate::assembler::operand_parsers::integer_operand;
use crate::assembler::register_parsers::register;
use crate::assembler::Token;

use nom::{
    character::complete::{multispace1, space1},
    combinator::opt,
    sequence::tuple,
    branch::alt,
    IResult,
};
#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

pub fn instruction(input: &str) -> IResult<&str, AssemblerInstruction> {
    alt((instruction_one, instruction_two))(input)
}

fn instruction_one(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, (o, _, r, _, i)) =
        tuple((opcode, space1, register, space1, integer_operand))(input)?;
    Ok((
        input,
        AssemblerInstruction {
            opcode: o,
            operand1: Some(r),
            operand2: Some(i),
            operand3: None,
        },
    ))
}

fn instruction_two(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, (o, opts)) = tuple((opcode, opt(multispace1)))(input)?;

    Ok((
        input,
        AssemblerInstruction {
            opcode: o,
            operand1: None,
            operand2: None,
            operand3: None,
        },
    ))
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];

        match &self.opcode {
            Token::Op { code } => match code {
                _ => {
                    results.push(*code as u8);
                }
            },
            _ => {
                print!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(t) => AssemblerInstruction::extract_operand(t, &mut results),
                None => {}
            }
        }

        results
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_one("load $0 #100");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_two() {
        let result = instruction_two("hlt");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::HLT },
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }
}
