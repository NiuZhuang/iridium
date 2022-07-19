use crate::assembler::Token;
use nom::{
    bytes::complete::{tag},
    character::complete::digit1,
    combinator::map_res,
    IResult,
};

pub fn register(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("$")(input)?;
    let (input, reg_num) = map_res(digit1, str::parse)(input)?;

    Ok((input, Token::Register { reg_num }))
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register("$0");
        assert!(result.is_ok());
        let result = register("0");
        assert!(result.is_err());
        let result = register("$a");
        assert!(result.is_err());
    }

}
