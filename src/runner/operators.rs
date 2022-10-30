use crate::runner::{Expression, RuntimeError, Scope, Value};
use crate::{parse, tokenize, Runner, Token};
use phf::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::path::Path;
use std::rc::Rc;

pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Set,
    Deref,
    Conditional,
    Or,
    And,
    Eq,
    NotEq,
    Less,
    Greater,
    LessOrEq,
    GreaterOrEq,
    Function,
    Call,
    Include,
    SysCall,
}

static OPERATOR_TYPES: Map<&str, Operator> = phf_map! {
    "+" => Operator::Add,
    "-" => Operator::Subtract,
    "*" => Operator::Multiply,
    "/" => Operator::Divide,
    "=" => Operator::Set,
    "!" => Operator::Deref,
    "?" => Operator::Conditional,
    "|" => Operator::Or,
    "&" => Operator::And,
    "==" => Operator::Eq,
    "!=" => Operator::NotEq,
    "<" => Operator::Less,
    ">" => Operator::Greater,
    "<=" => Operator::LessOrEq,
    ">=" => Operator::GreaterOrEq,
    "^" => Operator::Function,
    "@" => Operator::Call,
    "#" => Operator::Include,
    "~" => Operator::SysCall,
};

impl Operator {
    pub fn evaluate(
        expression: &Expression,
        stack: &mut Vec<Scope>,
    ) -> Result<Option<Value>, RuntimeError> {
        return match expression.borrow().value().as_ref().unwrap() {
            Token::Operator(str) => {
                if let Some(op_type) = OPERATOR_TYPES.get(str.as_str()) {
                    match op_type {
                        Operator::Add => Ok(Some(Operator::cascade_eval(
                            expression,
                            stack,
                            |a: f64, b: f64| a + b,
                        ))),
                        Operator::Subtract => Ok(Some(Operator::cascade_eval(
                            expression,
                            stack,
                            |a: f64, b: f64| a - b,
                        ))),
                        Operator::Multiply => Ok(Some(Operator::cascade_eval(
                            expression,
                            stack,
                            |a: f64, b: f64| a * b,
                        ))),
                        Operator::Divide => Ok(Some(Operator::cascade_eval(
                            expression,
                            stack,
                            |a: f64, b: f64| a / b,
                        ))),
                        Operator::Set => {
                            let key = Runner::execute(
                                expression.borrow().branches().get(0).unwrap().clone(),
                                stack,
                            )
                            .expect("Got no key!");

                            let value = Runner::execute(
                                expression.borrow().branches().get(1).unwrap().clone(),
                                stack,
                            )
                            .expect("Got no value!");

                            let vars = &mut stack.last_mut().unwrap().variables;
                            vars.insert(key, value.clone());

                            Ok(Some(value))
                        }
                        Operator::Deref => {
                            let key = Runner::execute(
                                expression.borrow().branches().get(0).unwrap().clone(),
                                stack,
                            )
                            .expect("Got no key!");

                            for scope in stack.iter().rev() {
                                let vars = &scope.variables;
                                if vars.contains_key(&key) {
                                    return Ok(Some(vars.get(&key).unwrap().clone()));
                                }
                            }
                            Err(RuntimeError::VariableNotFound(format!(
                                "Variable with key {} not found",
                                key
                            )))
                        }
                        Operator::Conditional => {
                            let condition = Runner::execute(
                                expression.borrow().branches().get(0).unwrap().clone(),
                                stack,
                            )
                            .expect("Got no condition!");

                            return if bool::from(condition) {
                                Ok(Runner::execute(
                                    expression.borrow().branches().get(1).unwrap().clone(),
                                    stack,
                                ))
                            } else {
                                Ok(Runner::execute(
                                    expression.borrow().branches().get(2).unwrap().clone(),
                                    stack,
                                ))
                            };
                        }
                        Operator::Or => Ok(Some(Operator::cascade_eval(
                            expression,
                            stack,
                            |a: bool, b: bool| a | b,
                        ))),
                        Operator::And => Ok(Some(Operator::cascade_eval(
                            expression,
                            stack,
                            |a: bool, b: bool| a & b,
                        ))),
                        Operator::Eq => {
                            let mut set = HashSet::new();
                            for branch in expression.borrow().branches() {
                                set.insert(Runner::execute(branch.clone(), stack));
                            }
                            Ok(Some(Value::from(set.len() == 1)))
                        }
                        Operator::NotEq => {
                            let mut set = HashSet::new();
                            for branch in expression.borrow().branches() {
                                set.insert(Runner::execute(branch.clone(), stack));
                            }
                            Ok(Some(Value::from(
                                set.len() == expression.borrow().branches().len(),
                            )))
                        }
                        Operator::Less => {
                            Ok(Some(Operator::cascade_cmp(expression, stack, |a, b| a < b)))
                        }
                        Operator::Greater => {
                            Ok(Some(Operator::cascade_cmp(expression, stack, |a, b| a > b)))
                        }
                        Operator::LessOrEq => {
                            Ok(Some(Operator::cascade_cmp(expression, stack, |a, b| {
                                a <= b
                            })))
                        }
                        Operator::GreaterOrEq => {
                            Ok(Some(Operator::cascade_cmp(expression, stack, |a, b| {
                                a >= b
                            })))
                        }
                        Operator::Function => {
                            let key = Runner::execute(
                                expression.borrow().branches().get(0).unwrap().clone(),
                                stack,
                            )
                            .expect("Got no key!");

                            let body = expression.borrow().branches().get(1).unwrap().clone();

                            let functions = &mut stack.last_mut().unwrap().functions;
                            functions.insert(key.clone(), body);

                            Ok(Some(key))
                        }
                        Operator::Call => {
                            let key = Runner::execute(
                                expression.borrow().branches().get(0).unwrap().clone(),
                                stack,
                            )
                            .expect("Got no key!");

                            for scope in stack.iter().rev() {
                                let functions = &scope.functions;
                                if functions.contains_key(&key) {
                                    return Ok(Runner::execute(
                                        functions.get(&key).unwrap().clone(),
                                        stack,
                                    ));
                                }
                            }
                            Err(RuntimeError::FunctionNotFound(format!(
                                "Function with key {} not found",
                                key
                            )))
                        }
                        Operator::Include => {
                            let mut result = None;

                            for branch in expression.borrow().branches() {
                                let arg = Runner::execute(branch.clone(), stack)
                                    .expect("Got nothing to include!");

                                if let Value::String(arg) = arg {
                                    if arg.contains('\n') {
                                        let tokens = tokenize(&arg);
                                        let ast = parse(tokens);
                                        result = Runner::run(Rc::new(RefCell::new(ast))).or(result);
                                    } else {
                                        let code = std::fs::read_to_string(Path::new(&arg))
                                            .expect("Could not include file!");
                                        let tokens = tokenize(&code);
                                        let ast = parse(tokens);
                                        result = Runner::run(Rc::new(RefCell::new(ast))).or(result);
                                    }
                                }
                            }

                            Ok(result)
                        }
                        Operator::SysCall => {
                            todo!()
                        }
                    }
                } else {
                    Err(RuntimeError::OperatorExpected(format!(
                        "Expected an operator but got {}",
                        str
                    )))
                }
            }
            invalid_token => Err(RuntimeError::OperatorExpected(format!(
                "Expected an operator but got {}",
                invalid_token
            ))),
        };
    }

    fn cascade_eval<T>(expression: &Expression, stack: &mut Vec<Scope>, f: fn(T, T) -> T) -> Value
    where
        T: Into<Value>,
        T: From<Value>,
    {
        let mut result = Runner::execute(
            expression.borrow().branches().get(0).unwrap().clone(),
            stack,
        )
        .expect("Got no result!")
        .into();

        for i in 1..expression.borrow().branches().len() {
            let tmp = Runner::execute(
                expression.borrow().branches().get(i).unwrap().clone(),
                stack,
            )
            .expect("Got no result!")
            .into();
            result = f(result, tmp);
        }

        result.into()
    }

    fn cascade_cmp(
        expression: &Expression,
        stack: &mut Vec<Scope>,
        f: fn(Value, Value) -> bool,
    ) -> Value {
        let mut a = Runner::execute(
            expression.borrow().branches().get(0).unwrap().clone(),
            stack,
        )
        .expect("Got no result!");

        for i in 1..expression.borrow().branches().len() {
            let tmp = Runner::execute(
                expression.borrow().branches().get(i).unwrap().clone(),
                stack,
            )
            .expect("Got no result!");

            if !f(a, tmp.clone()) {
                return Value::Bool(false);
            }

            a = tmp;
        }

        Value::Bool(true)
    }
}
