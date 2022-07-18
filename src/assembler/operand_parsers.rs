
use crate::assembler::Token;
use nom::{
    bytes::complete::{tag},
    character::complete::digit1,
    combinator::map_res,
    IResult,
};

pub fn integer_operand(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("#")(input)?;
    let (input, reg_num) = map_res(digit1, str::parse)(input)?;

    Ok((input, Token::IntegerOperand { value: reg_num }))
}


mod tests {
    use super::*;

    #[test]
    fn text_integer_operand() {
        let result = integer_operand("#10");
        assert!(result.is_ok());
        let (_, value) = result.unwrap();

        assert_eq!(value, Token::IntegerOperand { value: 10 });
    }
}
