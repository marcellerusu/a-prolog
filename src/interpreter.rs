use std::collections::HashMap;

use crate::parser::Value;

#[derive(Debug)]
pub struct DB {
    facts: Vec<Value>,
}

fn search(a: &Value, b: &Value, map: &mut HashMap<String, Value>) -> bool {
    match (a, b) {
        (Value::Fact(name_a, args_a), Value::Fact(name_b, args_b)) => {
            if name_a != name_b || args_a.len() != args_b.len() {
                return false;
            }

            for (lhs, rhs) in args_a.iter().zip(args_b) {
                if !search(lhs, rhs, map) {
                    return false;
                }
            }
            true
        }
        (Value::Fact(_, _), _) => todo!(),
        (Value::Str(_), Value::Fact(_, _)) => todo!(),
        (Value::Str(a), Value::Str(b)) => a == b,
        (Value::Str(_), Value::Int(_)) => false,
        (Value::Int(_), Value::Fact(_, _)) => todo!(),
        (Value::Int(_), Value::Str(_)) => false,
        (Value::Int(a), Value::Int(b)) => a == b,
        (Value::Str(_), Value::Variable(_)) => todo!(),
        (Value::Int(_), Value::Variable(_)) => todo!(),
        (Value::Variable(name), rhs) => {
            map.insert(name.to_owned(), rhs.to_owned());
            true
        }
    }
}

impl DB {
    pub fn new(facts: Vec<Value>) -> DB {
        DB { facts }
    }

    pub fn query(&self, query_node: &Value) -> Vec<HashMap<String, Value>> {
        match query_node {
            Value::Fact(_, _) => {
                let mut results: Vec<HashMap<String, Value>> = vec![];

                for fact in self.facts.iter() {
                    let mut map: HashMap<String, Value> = HashMap::new();
                    if search(query_node, fact, &mut map) {
                        results.push(map);
                    }
                }

                results
            }
            _ => panic!(),
        }
    }
}
