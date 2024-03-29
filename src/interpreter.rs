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
    ) -> Option<HashMap<String, Value>> {
        assert!(matches!(query_node, Value::CompoundTerm(_, _)));

        for fact in self.facts.iter() {
            let mut map: HashMap<String, Value> = env.clone();
            if self.unify(query_node, fact, &mut map) {
                let mut vars: HashMap<String, Value> = HashMap::new();
                DB::get_vars(query_node, &map, &mut vars);
                for (key, val) in vars.clone().iter() {
                    let actual_val = DB::instantiate(val, &map);
                    vars.insert(key.clone(), actual_val.clone());
                    env.insert(key.clone(), actual_val.clone());
                }

                return Some(vars);
            }
        }

        None
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
                    // println!("{:?} => {:?}", name, right);
                    out.insert(name.clone(), right.as_ref().to_owned());
                }
                _ => panic!(),
            },
            Value::And(left, right) => {
                DB::get_vars(left, env, out);
                DB::get_vars(right, env, out);
            }
            Value::LessThan(left, right) => {
                DB::get_vars(left, env, out);
                DB::get_vars(right, env, out);
            }
            Value::LessThanEqual(left, right) => {
                DB::get_vars(left, env, out);
                DB::get_vars(right, env, out);
            }
            Value::GreaterThan(left, right) => {
                DB::get_vars(left, env, out);
                DB::get_vars(right, env, out);
            }
            Value::GreaterThanEqual(left, right) => {
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
            Value::LessThan(left, right) => Value::LessThan(
                Box::new(DB::instantiate(left, map)),
                Box::new(DB::instantiate(right, map)),
            ),
            Value::LessThanEqual(left, right) => Value::LessThanEqual(
                Box::new(DB::instantiate(left, map)),
                Box::new(DB::instantiate(right, map)),
            ),
            Value::GreaterThan(left, right) => Value::GreaterThan(
                Box::new(DB::instantiate(left, map)),
                Box::new(DB::instantiate(right, map)),
            ),
            Value::GreaterThanEqual(left, right) => Value::GreaterThanEqual(
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
                match pred_body.as_ref() {
                    Value::CompoundTerm(_, _) => {
                        if self.query(pred_body, &mut scope).is_some() {
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
                    Value::LessThan(left, right) => {
                        match (left.as_ref(), right.as_ref()) {
                            (Value::Variable(_), Value::Variable(_)) => {
                                panic!("can't compare variables")
                            }
                            (Value::Int(left), Value::Int(right)) => left < right,
                            (Value::Variable(name), Value::Int(right)) => {
                                if let Some(value) = scope.get(name) {
                                    if let Value::Int(left) = value {
                                        left < right
                                    } else {
                                        panic!("var not int");
                                    }
                                } else {
                                    // TODO: use random number generator
                                    map.insert(name.clone(), Value::Int(right - 1));
                                    true
                                }
                            }
                            (Value::Int(left), Value::Variable(name)) => {
                                if let Some(value) = scope.get(name) {
                                    if let Value::Int(right) = value {
                                        left < right
                                    } else {
                                        panic!("var not int");
                                    }
                                } else {
                                    // TODO: use random number generator
                                    scope.insert(name.clone(), Value::Int(left + 1));
                                    true
                                }
                            }
                            _ => panic!("unknown"),
                        }
                    }
                    Value::Predicate(_, _, _) => todo!(),
                    Value::List(_) => todo!(),
                    Value::Str(_) => todo!(),
                    Value::Int(_) => todo!(),
                    Value::Variable(_) => todo!(),
                    Value::Eq(_, _) => todo!(),
                    Value::And(_, _) => todo!(),
                    Value::GreaterThan(left, right) => {
                        match (left.as_ref(), right.as_ref()) {
                            (Value::Variable(_), Value::Variable(_)) => {
                                panic!("can't compare variables")
                            }
                            (Value::Int(left), Value::Int(right)) => left > right,
                            (Value::Variable(name), Value::Int(right)) => {
                                if let Some(value) = scope.get(name) {
                                    if let Value::Int(left) = value {
                                        left > right
                                    } else {
                                        panic!("var not int");
                                    }
                                } else {
                                    // TODO: use random number generator
                                    map.insert(name.clone(), Value::Int(right + 1));
                                    true
                                }
                            }
                            (Value::Int(left), Value::Variable(name)) => {
                                if let Some(value) = scope.get(name) {
                                    if let Value::Int(right) = value {
                                        left > right
                                    } else {
                                        panic!("var not int");
                                    }
                                } else {
                                    // TODO: use random number generator
                                    scope.insert(name.clone(), Value::Int(left - 1));
                                    true
                                }
                            }
                            _ => panic!("unknown"),
                        }
                    }
                    Value::GreaterThanEqual(left, right) => {
                        match (left.as_ref(), right.as_ref()) {
                            (Value::Variable(_), Value::Variable(_)) => {
                                panic!("can't compare variables")
                            }
                            (Value::Int(left), Value::Int(right)) => left >= right,
                            (Value::Variable(name), Value::Int(right)) => {
                                if let Some(value) = scope.get(name) {
                                    if let Value::Int(left) = value {
                                        left >= right
                                    } else {
                                        panic!("var not int");
                                    }
                                } else {
                                    // TODO: use random number generator
                                    map.insert(name.clone(), Value::Int(*right));
                                    true
                                }
                            }
                            (Value::Int(left), Value::Variable(name)) => {
                                if let Some(value) = scope.get(name) {
                                    if let Value::Int(right) = value {
                                        left >= right
                                    } else {
                                        panic!("var not int");
                                    }
                                } else {
                                    // TODO: use random number generator
                                    scope.insert(name.clone(), Value::Int(*left));
                                    true
                                }
                            }
                            _ => panic!("unknown"),
                        }
                    }
                    Value::LessThanEqual(left, right) => {
                        match (left.as_ref(), right.as_ref()) {
                            (Value::Variable(_), Value::Variable(_)) => {
                                panic!("can't compare variables")
                            }
                            (Value::Int(left), Value::Int(right)) => left <= right,
                            (Value::Variable(name), Value::Int(right)) => {
                                if let Some(value) = scope.get(name) {
                                    if let Value::Int(left) = value {
                                        left <= right
                                    } else {
                                        panic!("var not int");
                                    }
                                } else {
                                    // TODO: use random number generator
                                    map.insert(name.clone(), Value::Int(*right));
                                    true
                                }
                            }
                            (Value::Int(left), Value::Variable(name)) => {
                                if let Some(value) = scope.get(name) {
                                    if let Value::Int(right) = value {
                                        left <= right
                                    } else {
                                        panic!("var not int");
                                    }
                                } else {
                                    // TODO: use random number generator
                                    scope.insert(name.clone(), Value::Int(*left));
                                    true
                                }
                            }
                            _ => panic!("unknown"),
                        }
                    }
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
            (_query, Value::Eq(_left, _right)) => {
                todo!()
            }
            (Value::And(_, _), _) => todo!(),
            (query, Value::And(left, right)) => {
                self.unify(query, left, map) && self.unify(query, right, map)
            }
            (Value::List(_), _) => todo!(),
            (Value::Str(_), _) => todo!(),
            (Value::Int(_), _) => todo!(),
            (Value::GreaterThan(_, _), _) => todo!(),
            (Value::LessThan(_, _), _) => todo!(),
            (Value::GreaterThanEqual(_, _), _) => todo!(),
            (Value::LessThanEqual(_, _), _) => todo!(),
            (Value::CompoundTerm(_, _), _) => todo!(),
        }
    }
}
