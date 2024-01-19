use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    CompoundTerm(String, Vec<Value>),
    Predicate(String, Vec<Value>, Box<Value>),
    List(Vec<Value>),
    Str(String),
    Int(usize),
    Variable(String),
    Eq(Box<Value>, Box<Value>),
    And(Box<Value>, Box<Value>),
    GreaterThan(Box<Value>, Box<Value>),
    LessThan(Box<Value>, Box<Value>),
    GreaterThanEqual(Box<Value>, Box<Value>),
    LessThanEqual(Box<Value>, Box<Value>),
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
                panic!("couldn't find token {:?}", self.tokens.get(self.idx..))
            }
        }
    }

    pub fn parse(&mut self) -> Vec<Value> {
        let mut ast = vec![];

        while self.idx < self.tokens.len() {
            ast.push(self.parse_expr());
            self.consume(|t| t.as_dot());
        }

        ast
    }

    fn parse_single_expr(&mut self) -> Value {
        if self.scan(|t| t.as_id()) && self.scan_ahead(1, |t| t.as_open_paren()) {
            self.parse_fact_or_predicate()
        } else if self.scan(|t| t.as_str()) {
            self.parse_str()
        } else if self.scan(|t| t.as_int()) {
            self.parse_int()
        } else if self.scan(|t| t.as_variable()) {
            self.parse_variable()
        } else if self.scan(|t| t.as_open_square_brace()) {
            self.parse_list()
        } else if self.scan(|t| t.as_underscore()) {
            self.parse_underscore()
        } else {
            panic!("no expr found {:?}", self.tokens.get(self.idx..))
        }
    }

    fn parse_expr(&mut self) -> Value {
        let expr = self.parse_single_expr();

        if self.scan(|t| t.as_eq()) {
            self.parse_eq(expr)
        } else if self.scan(|t| t.as_comma()) {
            self.parse_and(expr)
        } else if self.scan(|t| t.as_greater_than()) {
            self.parse_greater_than(expr)
        } else if self.scan(|t| t.as_less_than()) {
            self.parse_less_than(expr)
        } else if self.scan(|t| t.as_greater_than_equal()) {
            self.parse_greater_than_equal(expr)
        } else if self.scan(|t| t.as_less_than_equal()) {
            self.parse_less_than_equal(expr)
        } else {
            expr
        }
    }

    fn parse_less_than_equal(&mut self, left: Value) -> Value {
        self.consume(|t| t.as_less_than_equal());
        let right = self.parse_single_expr();
        Value::LessThanEqual(Box::new(left), Box::new(right))
    }

    fn parse_greater_than_equal(&mut self, left: Value) -> Value {
        self.consume(|t| t.as_greater_than_equal());
        let right = self.parse_single_expr();
        Value::GreaterThanEqual(Box::new(left), Box::new(right))
    }

    fn parse_less_than(&mut self, left: Value) -> Value {
        self.consume(|t| t.as_less_than());
        let right = self.parse_single_expr();
        Value::LessThan(Box::new(left), Box::new(right))
    }

    fn parse_greater_than(&mut self, left: Value) -> Value {
        self.consume(|t| t.as_greater_than());
        let right = self.parse_single_expr();
        Value::GreaterThan(Box::new(left), Box::new(right))
    }

    fn parse_and(&mut self, left: Value) -> Value {
        self.consume(|t| t.as_comma());
        let right = self.parse_single_expr();
        Value::And(Box::new(left), Box::new(right))
    }

    fn parse_eq(&mut self, left: Value) -> Value {
        self.consume(|t| t.as_eq());
        let right = self.parse_single_expr();
        Value::Eq(Box::new(left), Box::new(right))
    }

    fn parse_underscore(&mut self) -> Value {
        self.consume(|t| t.as_underscore());
        panic!("");
    }

    fn parse_list(&mut self) -> Value {
        self.consume(|t| t.as_open_square_brace());
        let mut values: Vec<Value> = vec![];
        while !self.scan(|t| t.as_close_square_brace()) {
            values.push(self.parse_expr());

            if !self.scan(|t| t.as_close_square_brace()) {
                self.consume(|t| t.as_comma());
            }
        }
        self.consume(|t| t.as_close_square_brace());
        Value::List(values)
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

    fn parse_fact_or_predicate(&mut self) -> Value {
        let name = self.consume(|t| t.as_id());
        self.consume(|t| t.as_open_paren());
        let mut args: Vec<Value> = vec![];

        while !self.scan(|t| t.as_close_paren()) {
            args.push(self.parse_single_expr());
            if !self.scan(|t| t.as_close_paren()) {
                self.consume(|t| t.as_comma());
            }
        }

        self.consume(|t| t.as_close_paren());
        if self.scan(|t| t.as_back_arrow()) {
            self.consume(|t| t.as_back_arrow());
            let body = self.parse_expr();
            Value::Predicate(name, args, Box::new(body))
        } else {
            Value::CompoundTerm(name, args)
        }
    }
}
