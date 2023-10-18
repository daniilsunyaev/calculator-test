use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    Arabic(i8),
    Roman(i8),
    Plus,
    Minus,
    Multiplication,
    Division,
    Unknown(String)
}

pub fn to_token(string: &str) -> Result<Token, LexerError> {
    let token = match string {
        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Multiplication,
        "/" => Token::Division,
        "I" => Token::Roman(1),
        "II" => Token::Roman(2),
        "III" => Token::Roman(3),
        "IV" => Token::Roman(4),
        "V" => Token::Roman(5),
        "VI" => Token::Roman(6),
        "VII" => Token::Roman(7),
        "VIII" => Token::Roman(8),
        "IX" => Token::Roman(9),
        "X" => Token::Roman(10),
        _ => {
            if let Ok(number) = string.parse::<i8>() {
                Token::Arabic(number)
            } else {
                Token::Unknown(string.to_string())
            }
        }
    };

    match token {
        Token::Unknown(string) => Err(LexerError::UnknownToken(string)),
        Token::Arabic(number) => {
            if 0 < number && number <= 10 {
                Ok(Token::Arabic(number))
            } else {
                Err(LexerError::NumberIsNotAllowed(number))
            }
        },
        _ => Ok(token)
    }
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    NumberIsNotAllowed(i8),
    UnknownToken(String),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::NumberIsNotAllowed(number) =>
                write!(f, "number '{}' is not allowed, one should use number between 1 and 10", number),
            LexerError::UnknownToken(string) => write!(f, "unknown token '{}'", string),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers() {
        assert_eq!(to_token("3").unwrap(), Token::Arabic(3));
        assert_eq!(to_token("10").unwrap(), Token::Arabic(10));
        assert_eq!(to_token("IV").unwrap(), Token::Roman(4));
        assert_eq!(to_token("X").unwrap(), Token::Roman(10));
        assert_eq!(to_token("12").unwrap_err(), LexerError::NumberIsNotAllowed(12));
        assert_eq!(to_token("0").unwrap_err(), LexerError::NumberIsNotAllowed(0));
        assert_eq!(to_token("-3").unwrap_err(), LexerError::NumberIsNotAllowed(-3));
    }

    #[test]
    fn parse_operators() {
        assert_eq!(to_token("-").unwrap(), Token::Minus);
        assert_eq!(to_token("/").unwrap(), Token::Division);
        assert_eq!(to_token("*").unwrap(), Token::Multiplication);
        assert_eq!(to_token("+").unwrap(), Token::Plus);
    }

    #[test]
    fn parse_junk() {
        assert_eq!(to_token("foo").unwrap_err(), LexerError::UnknownToken("foo".to_string()));
        assert_eq!(to_token("XX").unwrap_err(), LexerError::UnknownToken("XX".to_string()));
        assert_eq!(to_token(" ").unwrap_err(), LexerError::UnknownToken(" ".to_string()));
    }
}
