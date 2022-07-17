use crate::assembler::Token;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::digit1,
    combinator::map_res,
    IResult,
};

fn register(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("$")(input)?;
    let (input, reg_num) = map_res(digit1, str::parse)(input)?;

    Ok((input, Token::Register { reg_num: reg_num }))
}

fn integer_operand(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("#")(input)?;
    let (input, reg_num) = map_res(digit1, str::parse)(input)?;

    Ok((input, Token::IntegerOperand { value: reg_num }))
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register("$0");
        assert_eq!(result.is_ok(), true);
        let result = register("0");
        assert_eq!(result.is_ok(), false);
        let result = register("$a");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn text_integer_operand() {
        let result = integer_operand("#10");
        assert!(result.is_ok());
        let (_, value) = result.unwrap();

        assert_eq!(value, Token::IntegerOperand { value: 10 });
    }
}
