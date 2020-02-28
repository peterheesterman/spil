
use super::Token;
use super::TokenStream;
use super::Lexeme;

pub(crate) fn tokenise(input: String, remove_whites_space: bool) -> TokenStream {
    let mut tokens: Vec<Token> = vec![];
    let mut symbols: Vec<Lexeme> = vec![];

    let mut accumulate = false;
    let mut accumulator = String::new();

    let mut literal_accumulate = false;

    for character in input.chars() {
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
            operator @ _ if "+-*/%".contains(operator) => {
                tokens.push(Token::Operator(symbols.len()));
                symbols.push(Lexeme { value: operator.to_string() });
            },
            'A'..='z' | '.' => {
                accumulator.push(character);
                if !accumulate && !literal_accumulate {
                    accumulate = true;
                    tokens.push(Token::Id(symbols.len()));
                } 
            },
            '\'' => {
                if !literal_accumulate {
                    literal_accumulate = true;
                    tokens.push(Token::Literal(symbols.len()));
                } else {
                    symbols.push(Lexeme { value: accumulator });
                    literal_accumulate = false;
                    accumulator = String::new();
                }
            },
            ' ' | '\n' | '\t' => {
                if literal_accumulate {
                    accumulator.push(character);
                } else {
                    if accumulate {
                        symbols.push(Lexeme { value: accumulator });
                        accumulate = false;
                        literal_accumulate = false;
                        accumulator = String::new();
                    }
                    tokens.push(Token::WhiteSpace);
                }
            },
            literal @ _ => {
                panic!("{} is an invalid character.", literal);
            }
        }
    }

    if accumulator.len() > 0 {
        symbols.push(Lexeme { value: accumulator });
    }

    TokenStream { 
        symbols, 
        tokens: if remove_whites_space { 
            tokens
                .into_iter()
                .filter(|x| x != &Token::WhiteSpace).collect() 
        } else {
            tokens
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_list_with_literal_in_it() {
        let result = TokenStream {
            symbols: vec![ 
                Lexeme { value: String::from("list") },
                Lexeme { value: String::from("I have spaces in me") }, 
                Lexeme { value: String::from("-2.2") }, 
            ],
            tokens: vec![ 
                Token::OpenBraket,
                Token::Id(0),
                Token::WhiteSpace,
                Token::Literal(1),
                Token::WhiteSpace,
                Token::Number(2),
                Token::CloseBraket,
            ] 
        };
            
        assert_eq!(tokenise(String::from("(list \'I have spaces in me\' -2.2)"), false), result);
    }

    #[test]
    fn parse_literal_with_spaces_in_it() {
        let result = TokenStream {
            symbols: vec![ 
                Lexeme { value: String::from("I have spaces in me") }, 
            ],
            tokens: vec![ Token::Literal(0) ]
        };
            
        assert_eq!(tokenise(String::from("\'I have spaces in me\'"), false), result);
    }

    #[test]
    fn parse_list_of_numbers() {
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
            
        assert_eq!(tokenise(String::from("(list 1 2 3 -44)"), false), result);
    }
}
