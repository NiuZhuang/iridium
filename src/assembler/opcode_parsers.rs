use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::{character::complete::alpha1, IResult};

pub fn opcode(input: &str) -> IResult<&str, Token> {
    let (input, opcode) = alpha1(input)?;

    Ok((
        input,
        (Token::Op {
            code: Opcode::from(opcode),
        }),
    ))
}

mod tests {
    use super::*;

    #[test]
    fn test_opcode() {
        let result = opcode("load");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, "");
        let result = opcode("aold");
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
    
    }
}
