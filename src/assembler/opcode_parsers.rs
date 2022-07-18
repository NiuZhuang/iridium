use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::{bytes::complete::tag_no_case, IResult};

pub fn opcode_load(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag_no_case("load")(input)?;

    Ok((input, (Token::Op { code: Opcode::LOAD })))
}

mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        let result = opcode_load("Load");
        assert!(result.is_ok());

        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, "");

        let result = opcode_load("aold");
        assert!(result.is_err());
    }
}
