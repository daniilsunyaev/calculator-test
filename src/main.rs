use std::env;
use std::error::Error;

mod lexer;
mod parser;

fn main() {
    match run(env::args()) {
        Ok(message) => println!("{}", message),
        Err(error) => eprintln!("{}", error),
    }
}

pub fn run<I>(mut cli_args: I) -> Result<String, Box<dyn Error>>
where
    I: Iterator<Item = String>,
{
    cli_args.next(); // skip exec filename

    let tokenized_result: Result<Vec<lexer::Token>, lexer::LexerError> =
        cli_args
        .map(|arg| lexer::to_token(&arg))
        .collect();

    let calculation = parser::parse(&tokenized_result?);

    Ok(calculation?.execute().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn build_cli_args(args_str: &str) -> impl Iterator<Item = String> + '_ {
        let mut base_args = vec!["exec/path".to_string()];
        let provided_args: Vec<String> = args_str.split(' ').map(|s| s.to_string()).collect();
        base_args.extend(provided_args);
        base_args.into_iter()
    }

    #[test]
    fn valid_input() {
        assert_eq!(run(build_cli_args("1 - 2")).unwrap(), "-1");
        assert_eq!(run(build_cli_args("III - 2")).unwrap(), "1");
        assert_eq!(run(build_cli_args("X + 2")).unwrap(), "12");
        assert_eq!(run(build_cli_args("X / II")).unwrap(), "5");
        assert_eq!(run(build_cli_args("10 * IX")).unwrap(), "90");
    }

    #[test]
    fn invalid_input() {
        assert_eq!(
            run(build_cli_args("0 - 2")).unwrap_err().to_string(),
            "number '0' is not allowed, one should use number between 1 and 10"
        );

        assert_eq!(
            run(build_cli_args("XI -  2")).unwrap_err().to_string(),
            "unknown token 'XI'"
        );

        assert_eq!(
            run(build_cli_args("X + -2")).unwrap_err().to_string(),
            "number '-2' is not allowed, one should use number between 1 and 10"
        );

        assert_eq!(
            run(build_cli_args("X - 2 + 3")).unwrap_err().to_string(),
            "exactly 3 arguments expected, 5 provided instead"
        );

        assert_eq!(
            run(build_cli_args("1 2 3")).unwrap_err().to_string(),
            "invalid argument '2', operator (+ - * /) expected"
        );

        assert_eq!(
            run(build_cli_args("2 + -")).unwrap_err().to_string(),
            "invalid argument '-', arabic or roman number from 1 to 10 expected"
        );
    }
}
