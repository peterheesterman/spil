
#[derive(PartialEq, Debug)]
enum Token {
    Id(usize),
    Number(usize),
    Literal(usize),
    OpenBraket,
    CloseBraket,
    WhiteSpace,
}

#[derive(PartialEq, Debug)]
pub struct Lexeme {
    value: String
}

#[derive(PartialEq, Debug)]
pub struct TokenStream {
    symbols: Vec<Lexeme>,
    tokens: Vec<Token>,
}

pub fn tokenise(input: String) -> TokenStream {
    let mut tokens: Vec<Token> = vec![];
    let mut symbols: Vec<Lexeme> = vec![];

    let mut accumulate = false;
    let mut accumulator = String::new();

    for character in input.chars() {
        let lexeme = String::new();
        match character {
            '(' => tokens.push(Token::OpenBraket),
            ')' => tokens.push(Token::CloseBraket),
            literal @ _ if literal.is_ascii_digit() || character == '-' => {
                accumulator.push(character);
                if !accumulate {
                    accumulate = true;
                    tokens.push(Token::Number(symbols.len()));
                } 
            },
            'A'..='z' => {
                accumulator.push(character);
                if !accumulate {
                    accumulate = true;
                    tokens.push(Token::Id(symbols.len()));
                } 
            },
            '"' | '\'' => {
                // TODO: include "strings"
            },
            ' ' | '\n' | '\t' => {
                if accumulate {
                    symbols.push(Lexeme { value: accumulator });
                    accumulate = false;
                    accumulator = String::new();
                }
                tokens.push(Token::WhiteSpace);
            },
            _ => tokens.push(Token::CloseBraket)
        }
        println!("{}", character);
    }

    if accumulator.len() > 0 {
        symbols.push(Lexeme { value: accumulator });
    }

    TokenStream { symbols, tokens }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = TokenStream {
            symbols: vec![ 
                Lexeme { value: String::from("list") }, 
                Lexeme { value: String::from("1") }, 
                Lexeme { value: String::from("2") }, 
                Lexeme { value: String::from("3") },
                Lexeme { value: String::from("-44") },
            ],
            tokens: vec![
                Token::OpenBraket, 
                Token::Id(0), 
                Token::WhiteSpace, 
                Token::Number(1), 
                Token::WhiteSpace, 
                Token::Number(2), 
                Token::WhiteSpace, 
                Token::Number(3), 
                Token::WhiteSpace, 
                Token::Number(4), 
                Token::CloseBraket,
            ]
        };
            
        assert_eq!(tokenise(String::from("(list 1 2 3 -44)")), result);
    }
}
