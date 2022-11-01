use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::str::FromStr;

use crate::node::Node;
use crate::runner::operator::get_operator;
use crate::runner::value::Value;
use crate::Token::Operator;
use crate::{parse, tokenize, Token};

mod operator;
mod value;

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

#[derive(Default)]
pub struct Scope {
    variables: HashMap<Value, Value>,
    functions: HashMap<Value, Expression>,
}

pub struct Context {
    pub stack: Vec<Scope>,
    pub open_paths: Vec<PathBuf>,
}

impl Context {
    pub fn new(stack: Vec<Scope>, open_paths: Vec<PathBuf>) -> Self {
        Self { stack, open_paths }
    }

    pub fn get_current_path(&self) -> PathBuf {
        let mut current_path = PathBuf::default();

        for open_path in self.open_paths.iter() {
            current_path.push(open_path);
        }

        current_path
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            stack: vec![Scope::default()],
            open_paths: vec![PathBuf::from(".")],
        }
    }
}

impl Runner {
    pub fn run_file(path: &Path, context: &mut Context) -> Result<Option<Value>, RuntimeError> {
        let code = std::fs::read_to_string(context.get_current_path().join(path))
            .expect("Could not include file!");
        let directory = path
            .parent()
            .expect("Could not determine working directory!");

        context.open_paths.push(directory.to_path_buf());
        let result = Runner::run_code(code, context);
        context.open_paths.pop();
        result
    }

    pub fn run_code(code: String, context: &mut Context) -> Result<Option<Value>, RuntimeError> {
        let tokens = tokenize(&code);
        let ast = parse(tokens);
        Ok(Some(
            Runner::execute(Rc::new(RefCell::new(ast)), context).expect("Nothing returned :("),
        ))
    }

    fn execute(expression: Expression, context: &mut Context) -> Option<Value> {
        if let Some(token) = expression.borrow().value() {
            match token {
                Operator(str) => get_operator(str)
                    .unwrap()
                    .evaluate(&expression, context)
                    .unwrap(),
                Token::LiteralStr(str) => Some(Value::String(str[1..str.len() - 1].to_string())),
                Token::LiteralNum(str) => Some(Value::Number(f64::from_str(str).unwrap())),
                Token::StructStart(_) => Some(Value::make_struct(expression.clone(), context)),
                Token::Identifier(str) => Some(Value::String(str.to_string())),
                _ => None,
            }
        } else {
            let mut result = None;

            for branch in expression.borrow().branches() {
                result = Runner::execute(branch.clone(), context);
            }

            result
        }
    }
}
