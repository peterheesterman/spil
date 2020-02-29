
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
    value: String,
}

#[derive(PartialEq, Debug)]
pub(crate) struct TokenStream {
    symbols: Vec<Lexeme>,
    tokens: Vec<Token>,
}

#[derive(PartialEq, Debug)]
enum Node {
    ListOp {
        children: Vec<Node>,
        token: Token,
    },
    BinOp {
        left: Box<Node>,
        right: Box<Node>,
        token: Token,
    },
    UnaryOp {
        child: Box<Node>,
        token: Token,
    },
    Number {
        token: Token,
        value: f64,
    },
}

#[derive(PartialEq, Debug)]
pub(crate) struct AST {
    root: Node,
}

mod tokeniser;
mod parser;

pub fn run(source_code: String) {
    let token_stream = tokeniser::tokenise(source_code, true);
    let ast = parser::parse(token_stream);

    println!("{:?}", ast)
}
