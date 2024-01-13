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
            Value::CompoundTerm(_, _) => {
                let mut results: Vec<HashMap<String, Value>> = vec![];

                for fact in self.facts.iter() {
                    let mut map: HashMap<String, Value> = env.clone();
                    if self.unify(query_node, fact, &mut map) {
                        let mut vars: HashMap<String, Value> = HashMap::new();
                        DB::get_vars(query_node, &map, &mut vars);
                        println!("{:?}", query_node);
                        for (key, val) in vars.clone().iter() {
                            let actual_val = DB::instantiate(val, &map);
                            vars.insert(key.clone(), actual_val.clone());
                            env.insert(key.clone(), actual_val.clone());
                        }

                        if !vars.is_empty() {
                            results.push(vars);
                        }
                    }
                }

                results
            }
            Value::And(left, right) => {
                let left_results = self.query(left, env);
                if left_results.is_empty() {
                    return vec![];
                }

                [left_results, self.query(right, env)].concat()
            }
            Value::Eq(left, right) => match left.as_ref() {
                Value::Variable(name) => {
                    let right = DB::instantiate(right, env);
                    env.insert(name.clone(), right.clone());
                    vec![HashMap::from([(name.clone(), right)])]
                }
                _ => {
                    assert!(DB::instantiate(left, env) == DB::instantiate(right, env));
                    panic!();
                }
            },
            _ => panic!("unexpected query_node {:?}", query_node),
        }
    }

    fn get_vars(query: &Value, env: &HashMap<String, Value>, out: &mut HashMap<String, Value>) {
        match query {
            Value::CompoundTerm(_, values) => {
                values.iter().for_each(|val| DB::get_vars(val, env, out))
            }
            Value::Predicate(_, _, _) => todo!(),
            Value::List(values) => values.iter().for_each(|val| DB::get_vars(val, env, out)),
            Value::Str(_) => (),
            Value::Int(_) => (),
            Value::Variable(name) => {
                out.insert(name.clone(), env.get(name).unwrap().clone());
            }
            Value::Eq(left, right) => match left.as_ref() {
                Value::Variable(name) => {
                    println!("{:?} => {:?}", name, right);
                    out.insert(name.clone(), right.as_ref().to_owned());
                }
                _ => panic!(),
            },
            Value::And(left, right) => {
                DB::get_vars(left, env, out);
                DB::get_vars(right, env, out);
            }
        }
    }

    fn instantiate(value: &Value, map: &HashMap<String, Value>) -> Value {
        match value {
            Value::CompoundTerm(_, _) => todo!(),
            Value::Predicate(_, _, _) => todo!(),
            Value::List(values) => Value::List(
                values
                    .iter()
                    .map(|val| DB::instantiate(val, map))
                    .collect::<Vec<_>>(),
            ),
            Value::Str(_) => value.to_owned(),
            Value::Int(_) => value.to_owned(),
            Value::Variable(name) => map.get(name).unwrap().to_owned(),
            Value::Eq(left, right) => Value::Eq(
                Box::new(DB::instantiate(left, map)),
                Box::new(DB::instantiate(right, map)),
            ),
            Value::And(left, right) => Value::And(
                Box::new(DB::instantiate(left, map)),
                Box::new(DB::instantiate(right, map)),
            ),
        }
    }

    fn unify(&self, query: &Value, fact: &Value, map: &mut HashMap<String, Value>) -> bool {
        match (query, fact) {
            (Value::CompoundTerm(name_a, args_a), Value::CompoundTerm(name_b, args_b)) => {
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
            (Value::Str(_), Value::CompoundTerm(_, _)) => todo!(),
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Str(_), Value::Int(_)) => false,
            (Value::Int(_), Value::CompoundTerm(_, _)) => todo!(),
            (Value::Int(_), Value::Str(_)) => false,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Variable(a), Value::Variable(b)) => match (map.get(a), map.get(b)) {
                (None, None) => {
                    map.insert(a.clone(), Value::Variable(b.clone()));
                    true
                } // this will fail at some point
                (None, Some(value)) => {
                    map.insert(a.clone(), value.clone());
                    true
                }
                (Some(value), None) => {
                    map.insert(b.clone(), value.clone());
                    true
                }
                (Some(a), Some(b)) => a == b,
            },
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
                Value::CompoundTerm(query_name, query_args),
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
                if !self.query(pred_body, &mut scope).is_empty() {
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
            (Value::CompoundTerm(_, _), Value::Str(_)) => false,
            (Value::CompoundTerm(_, _), Value::Int(_)) => false,
            (Value::CompoundTerm(_, _), Value::List(_)) => false,
            (Value::List(_), Value::CompoundTerm(_, _)) => false,
            (Value::List(a), Value::List(b)) => {
                a.len() == b.len()
                    && a.iter()
                        .zip(b)
                        .all(|(query, fact)| self.unify(query, fact, map))
            }
            (Value::List(_), Value::Str(_)) => todo!(),
            (Value::List(_), Value::Int(_)) => todo!(),
            (Value::Str(_), Value::List(_)) => todo!(),
            (Value::Int(_), Value::List(_)) => todo!(),
            (Value::Eq(_, _), _) => todo!(),
            (query, Value::Eq(left, right)) => {
                todo!()
            }
            (Value::And(_, _), _) => todo!(),
            (query, Value::And(left, right)) => {
                self.unify(query, left, map) && self.unify(query, right, map)
            }
        }
    }
}
