use crate::lexer::Token;

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Calculation {
    left: i8,
    right: i8,
    operator: fn(i8, i8) -> i8,
}

impl Calculation {
    pub fn execute(&self) -> i8 {
        (self.operator)(self.left, self.right)
    }
}

pub fn parse(tokens: &Vec<Token>) -> Result<Calculation, ParserError> {
    if tokens.len() != 3 {
        return Err(ParserError::InvalidNumberOfArgumetns(tokens.len()))
    }

    let left = match tokens[0] {
        Token::Arabic(number) => number,
        Token::Roman(number) => number,
        _ => return Err(ParserError::InvalidArgument(tokens[0].clone(), "arabic or roman number from 1 to 10".to_string()))
    };

    let operator = match tokens[1] {
        Token::Plus => |a, b| a + b,
        Token::Minus => |a, b| a - b,
        Token::Multiplication => |a, b| a * b,
        Token::Division => |a, b| a / b,
        _ => return Err(ParserError::InvalidArgument(tokens[1].clone(), "operator (+ - * /)".to_string())),
    };

    let right = match tokens[2] {
        Token::Arabic(number) => number,
        Token::Roman(number) => number,
        _ => return Err(ParserError::InvalidArgument(tokens[2].clone(), "arabic or roman number from 1 to 10".to_string()))
    };

    Ok(Calculation { left, right, operator })
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    InvalidNumberOfArgumetns(usize),
    InvalidArgument(Token, String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::InvalidNumberOfArgumetns(number) =>
                write!(f, "exactly 3 arguments expected, {} provided instead", number),
            ParserError::InvalidArgument(token, entity) => write!(f, "invalid argument '{}', {} expected", token, entity),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_input() {
        let valid_input = vec![Token::Arabic(2), Token::Minus, Token::Roman(5)];
        assert!(matches!(
                parse(&valid_input), Ok(Calculation { left: 2, right: 5, operator: _ })
        ));

        let valid_input = vec![Token::Roman(10), Token::Plus, Token::Roman(1)];
        assert!(matches!(
                parse(&valid_input), Ok(Calculation { left: 10, right: 1, operator: _ })
        ));
    }

    #[test]
    fn parse_invalid_input() {
        let invalid_input = vec![Token::Arabic(2), Token::Arabic(1), Token::Roman(5)];
        assert!(matches!(
                parse(&invalid_input), Err(ParserError::InvalidArgument(..))
        ));

        let invalid_input = vec![Token::Arabic(2)];
        assert!(matches!(
                parse(&invalid_input), Err(ParserError::InvalidNumberOfArgumetns(1))
        ));
    }
}
