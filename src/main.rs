use parser::Value;

mod interpreter;
mod lexer;
mod parser;

fn query() -> Value {
    let tokens = lexer::tokenize("user(?name, ?name).".to_string());

    parser::Parser::new(tokens).parse().first().unwrap().clone()
}

fn main() {
    let program = "
    user(\"marcelle\", 26).
    user(\"jack\", 30).
    user(30, 30).
    "
    .to_string();

    let tokens = lexer::tokenize(program);
    // println!("{:?}", tokens);

    let facts = parser::Parser::new(tokens).parse();

    let db = interpreter::DB::new(facts);

    println!("{:?}", db.query(&query()));
}
