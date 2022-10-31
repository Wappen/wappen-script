mod operator;
mod value;

use crate::node::Node;
use crate::runner::operator::get_operator;
use crate::runner::value::Value;
use crate::Token;
use crate::Token::Operator;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;

type Expression = Rc<RefCell<Node<Token>>>;

pub struct Runner {}

#[derive(Debug)]
pub enum RuntimeError {
    FunctionNotFound(String),
    VariableNotFound(String),
    OperatorExpected(String),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RuntimeError {}

pub struct Scope {
    variables: HashMap<Value, Value>,
    functions: HashMap<Value, Expression>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
}

impl Runner {
    pub fn run(e: Expression) -> Option<Value> {
        let mut stack = vec![Scope::new()];
        Runner::execute(e, &mut stack)
    }

    fn execute(expression: Expression, stack: &mut Vec<Scope>) -> Option<Value> {
        if let Some(token) = expression.borrow().value() {
            match token {
                Operator(str) => get_operator(str)
                    .unwrap()
                    .evaluate(&expression, stack)
                    .unwrap(),
                Token::LiteralStr(str) => Some(Value::String(str[1..str.len() - 1].to_string())),
                Token::LiteralNum(str) => Some(Value::Number(f64::from_str(str).unwrap())),
                Token::StructStart(_) => todo!(),
                Token::Identifier(str) => Some(Value::String(str.to_string())),
                _ => None,
            }
        } else {
            let mut result = None;

            for branch in expression.borrow().branches() {
                stack.push(Scope::new());
                result = Runner::execute(branch.clone(), stack);
            }

            result
        }
    }
}
