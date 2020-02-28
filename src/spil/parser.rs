
use super::Node;
use super::Token;
use super::TokenStream;
use super::Lexeme;
use super::AST;

pub(crate) fn parse(token_stream: TokenStream) -> AST {
    AST { root: Node { right: None, left: None, value: Token::Number(0) }}
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
                Token::OpenBraket, 
                Token::Operator(0), 
                Token::Number(1), 
                Token::Number(2), 
                Token::CloseBraket,
            ]
        };
        let result = AST {
            root: Node { right: None, left: None, value: Token::Number(0) }
        };
        assert_eq!(parse(token_stream), result);
    }
}
