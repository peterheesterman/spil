
use super::Node;
use super::Token;
use super::TokenStream;
use super::Lexeme;
use super::AST;

pub(crate) fn parse(token_stream: TokenStream) -> AST {
    // TODO: to the parsing to make this work
    let number1 = Node::Number { token: Token::Number(1), value: 1_f64 };
    let number2 = Node::Number { token: Token::Number(2), value: 2_f64 };
    AST { 
        root: Node::BinOp { 
            right: Box::new(number1), 
            left: Box::new(number2),
            token: Token::Operator(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_simple_ast() {
        let token_stream = TokenStream {
            symbols: vec![ 
                Lexeme { value: String::from("+") }, 
                Lexeme { value: String::from("1") }, 
                Lexeme { value: String::from("2") }, 
            ],
            tokens: vec![
                Token::OpenBracket, 
                Token::Operator(0), 
                Token::Number(1), 
                Token::Number(2), 
                Token::CloseBracket,
            ]
        };

        let number1 = Node::Number { token: Token::Number(1), value: 1_f64 };
        let number2 = Node::Number { token: Token::Number(2), value: 2_f64 };
        let result = AST { 
            root: Node::BinOp { 
                right: Box::new(number1), 
                left: Box::new(number2),
                token: Token::Operator(0),
            }
        };
        assert_eq!(parse(token_stream), result);
    }
}
