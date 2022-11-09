use crate::runner::context::Context;
use crate::runner::expression::Expression;
use crate::runner::operator::get_operator;
use crate::runner::value::Value;
use crate::Token::Operator;
use crate::{parse, tokenize, Token};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;
use std::str::FromStr;
use syscalls::{Errno, Sysno};

pub mod context;
mod expression;
mod operator;
mod value;

pub struct Runner {}

#[derive(Debug)]
pub enum RuntimeError {
    FunctionNotFound(Value),
    VariableNotFound(Value),
    NoValue,
    OperatorExpected(String),
    SysCallError(Sysno, Errno),
    IllegalBinaryConversion,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::FunctionNotFound(value) => {
                write!(f, "Function for value {} not found!", value)
            }
            RuntimeError::VariableNotFound(value) => {
                write!(f, "Variable for value {} not found!", value)
            }
            RuntimeError::OperatorExpected(name) => {
                write!(f, "Expected an operator, got {} instead", name)
            }
            RuntimeError::SysCallError(sysno, errno) => {
                write!(f, "Syscall {} returned error {}", sysno, errno)
            }
            RuntimeError::IllegalBinaryConversion => {
                write!(
                    f,
                    "Illegal conversion to binary, use explicit binary operators instead"
                )
            }
            RuntimeError::NoValue => {
                write!(f, "Got no value")
            }
        }
    }
}

impl Error for RuntimeError {}

impl Runner {
    pub fn run_file(
        relative_path: &Path,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        let script_path = context.get_current_path().join(relative_path);
        let code = std::fs::read_to_string(&script_path).expect("Could not read file!");
        let directory = script_path
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
            Runner::execute(&Expression::new(ast), context).ok_or(RuntimeError::NoValue)?,
        ))
    }

    fn execute(expression: &Expression, context: &mut Context) -> Option<Value> {
        if let Some(token) = expression.get_token().borrow().value() {
            match token {
                Operator(str) => get_operator(str)
                    .unwrap()
                    .evaluate(expression, context)
                    .unwrap(),
                Token::LiteralStr(str) => Some(Value::String(str[1..str.len() - 1].to_string())),
                Token::LiteralNum(str) => Some(Value::Number(f64::from_str(str).unwrap())),
                Token::StructStart(_) => Some(Value::make_struct(expression, context)),
                Token::Identifier(str) => Some(Value::String(str.to_string())),
                _ => None,
            }
        } else {
            let mut result = None;

            for branch in expression.get_branches() {
                result = Runner::execute(&branch, context);
            }

            result
        }
    }
}
