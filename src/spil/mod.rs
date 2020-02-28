
#[derive(PartialEq, Debug)]
enum Token {
    Id(usize),
    Number(usize),
    Literal(usize),
    Operator(usize),
    OpenBraket,
    CloseBraket,
    WhiteSpace,
}

#[derive(PartialEq, Debug)]
struct Lexeme {
    value: String
}

#[derive(PartialEq, Debug)]
pub(crate) struct TokenStream {
    symbols: Vec<Lexeme>,
    tokens: Vec<Token>,
}

#[derive(PartialEq, Debug)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: Token
}

#[derive(PartialEq, Debug)]
pub(crate) struct AST {
    root: Node
}

mod tokeniser;
mod parser;

pub fn run(source_code: String) {
    let token_stream = tokeniser::tokenise(source_code, true);
    let ast = parser::parse(token_stream);

    println!("{:?}", ast)
}
