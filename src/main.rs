use std::collections::HashMap;

use parser::Value;

mod interpreter;
mod lexer;
mod parser;

fn query() -> Value {
    let tokens = lexer::tokenize("thirty_year_old_student(?name).".to_string());

    parser::Parser::new(tokens).parse().first().unwrap().clone()
}

fn main() {
    let program = "
    person(\"jack\", 30, \"student\").

    student(?name, ?age) :- person(?name, ?age, \"student\").
    
    thirty_year_old_student(?name) :- student(?name, 30).
    "
    .to_string();

    let tokens = lexer::tokenize(program);
    // println!("{:?}", tokens);

    let facts = parser::Parser::new(tokens).parse();
    // println!("{:?}", facts);

    let db = interpreter::DB::new(facts);

    println!("{:?}", db.query(&query(), &mut HashMap::new()));
}
