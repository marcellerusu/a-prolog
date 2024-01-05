use std::collections::HashMap;

use crate::parser::Value;

#[derive(Debug)]
pub struct DB {
    facts: Vec<Value>,
}

impl DB {
    pub fn new(facts: Vec<Value>) -> DB {
        DB { facts }
    }

    pub fn query(
        &self,
        query_node: &Value,
        env: &mut HashMap<String, Value>,
    ) -> Vec<HashMap<String, Value>> {
        match query_node {
            Value::Fact(_, _) => {
                let mut results: Vec<HashMap<String, Value>> = vec![];

                for fact in self.facts.iter() {
                    let mut map: HashMap<String, Value> = env.clone();
                    if self.unify(query_node, fact, &mut map) {
                        for (key, val) in map.iter() {
                            env.insert(key.clone(), val.clone());
                        }
                        results.push(map);
                    }
                }

                results
            }
            _ => panic!(),
        }
    }

    fn unify(&self, query: &Value, b: &Value, map: &mut HashMap<String, Value>) -> bool {
        match (query, b) {
            (Value::Fact(name_a, args_a), Value::Fact(name_b, args_b)) => {
                if name_a != name_b || args_a.len() != args_b.len() {
                    return false;
                }

                for (lhs, rhs) in args_a.iter().zip(args_b) {
                    if !self.unify(lhs, rhs, map) {
                        return false;
                    }
                }
                true
            }
            (Value::Str(_), Value::Fact(_, _)) => todo!(),
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Str(_), Value::Int(_)) => false,
            (Value::Int(_), Value::Fact(_, _)) => todo!(),
            (Value::Int(_), Value::Str(_)) => false,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Variable(a), Value::Variable(b)) => a == b,
            (Value::Variable(name), rhs) => {
                if let Some(val) = map.get(name) {
                    rhs == val
                } else {
                    map.insert(name.to_owned(), rhs.to_owned());
                    true
                }
            }
            (_, Value::Variable(name)) => {
                if let Some(val) = map.get(name) {
                    query == val
                } else {
                    map.insert(name.to_owned(), query.to_owned());
                    true
                }
            }
            (Value::Predicate(..), _) => todo!(),
            (
                Value::Fact(query_name, query_args),
                Value::Predicate(pred_name, pred_args, pred_body),
            ) => {
                if query_name != pred_name {
                    return false;
                }
                if query_args.len() != pred_args.len() {
                    return false;
                }

                for (query_arg, pred_arg) in query_args.iter().zip(pred_args) {
                    if !self.unify(query_arg, pred_arg, map) {
                        return false;
                    }
                }

                let mut scope = map.clone();
                if self.query(pred_body, &mut scope).len() > 0 {
                    for name in query_args.iter().filter_map(|n| match n {
                        Value::Variable(name) => Some(name),
                        _ => None,
                    }) {
                        map.insert(name.clone(), scope.get(name).unwrap().clone());
                    }
                    true
                } else {
                    false
                }
            }

            (_, Value::Predicate(..)) => false,
            (Value::Fact(_, _), Value::Str(_)) => todo!(),
            (Value::Fact(_, _), Value::Int(_)) => todo!(),
        }
    }
}
