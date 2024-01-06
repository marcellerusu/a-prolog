use std::collections::HashMap;

use parser::Value;

mod interpreter;
mod lexer;
mod parser;

fn query() -> Value {
    let tokens = lexer::tokenize("append([1], [?x], [1, 2]).".to_string());

    parser::Parser::new(tokens).parse().first().unwrap().clone()
}

fn main() {
    let program = "
    append([?a], [?b], [?a, ?b]).
    "
    .to_string();

    let tokens = lexer::tokenize(program);
    // println!("{:?}", tokens);

    let facts = parser::Parser::new(tokens).parse();
    // println!("{:?}", facts);

    let db = interpreter::DB::new(facts);

    println!("{:?}", db.query(&query(), &mut HashMap::new()));
}
