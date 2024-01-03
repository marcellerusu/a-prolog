use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Fact(String, Vec<Value>),
    Str(String),
    Int(usize),
    Variable(String),
}

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, idx: 0 }
    }

    fn scan<T>(&self, cond: fn(&Token) -> Option<T>) -> bool {
        self.tokens.get(self.idx).and_then(cond).is_some()
    }

    fn scan_ahead<T>(&self, offset: usize, cond: fn(&Token) -> Option<T>) -> bool {
        self.tokens.get(self.idx + offset).and_then(cond).is_some()
    }

    fn consume<T>(&mut self, cond: fn(&Token) -> Option<T>) -> T {
        match self.tokens.get(self.idx).and_then(cond) {
            Some(val) => {
                self.idx += 1;
                val
            }
            None => {
                panic!("couldn't find token")
            }
        }
    }

    pub fn parse(&mut self) -> Vec<Value> {
        let mut ast = vec![];

        while self.idx < self.tokens.len() {
            ast.push(self.parse_expr());
        }

        ast
    }

    fn parse_expr(&mut self) -> Value {
        if self.scan(|t| t.as_id()) && self.scan_ahead(1, |t| t.as_open_paren()) {
            self.parse_fact()
        } else if self.scan(|t| t.as_str()) {
            self.parse_str()
        } else if self.scan(|t| t.as_int()) {
            self.parse_int()
        } else if self.scan(|t| t.as_variable()) {
            self.parse_variable()
        } else {
            panic!("no expr found")
        }
    }

    fn parse_variable(&mut self) -> Value {
        let name = self.consume(|t| t.as_variable());
        Value::Variable(name)
    }

    fn parse_str(&mut self) -> Value {
        let val = self.consume(|t| t.as_str());
        Value::Str(val)
    }

    fn parse_int(&mut self) -> Value {
        let val = self.consume(|t| t.as_int());
        Value::Int(val)
    }

    fn parse_fact(&mut self) -> Value {
        let fact_name = self.consume(|t| t.as_id());
        self.consume(|t| t.as_open_paren());
        let mut args: Vec<Value> = vec![];
        while !self.scan(|t| t.as_close_paren()) {
            args.push(self.parse_expr());
            if !self.scan(|t| t.as_close_paren()) {
                self.consume(|t| t.as_comma());
            }
        }
        self.consume(|t| t.as_close_paren());
        self.consume(|t| t.as_dot());
        Value::Fact(fact_name, args)
    }
}
