use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use crate::{Context, Runner};
use crate::runner::{Expression, Scope};
use crate::runner::value::Value::Struct;

#[derive(Debug, Clone, PartialOrd)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Struct(Vec<Value>),
}

impl Value {
    pub fn make_struct(expression: Expression, context: &mut Context) -> Value {
        context.stack.push(Scope::default());
        let mut values: Vec<Value> = vec![];
        for branch in expression.borrow().branches() {
            let value = Runner::execute(branch.clone(), context);

            if let Some(value) = value {
                values.push(value);
            }
        }
        context.stack.pop();
        Struct(values)
    }
}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Value::String(_), Value::String(_))
                | (Value::Number(_), Value::Number(_))
                | (Value::Bool(_), Value::Bool(_))
                | (Value::Struct(_), Value::Struct(_))
        )
    }
}

impl Eq for Value {}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::String(s) => s.hash(state),
            Value::Number(n) => n.to_string().hash(state),
            Value::Bool(b) => b.hash(state),
            Value::Struct(v) => v.hash(state),
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<Value> for String {
    fn from(v: Value) -> Self {
        match v {
            Value::String(s) => s,
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Struct(_) => todo!(),
        }
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<Value> for f64 {
    fn from(v: Value) -> Self {
        match v {
            Value::String(s) => f64::from_str(s.as_str()).unwrap(),
            Value::Number(n) => n,
            Value::Bool(b) => match b {
                true => 1.0,
                false => 0.0,
            },
            Value::Struct(_v) => todo!(),
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<Value> for bool {
    fn from(v: Value) -> Self {
        match v {
            Value::String(s) => !s.is_empty(),
            Value::Number(n) => n != 0.0,
            Value::Bool(b) => b,
            Value::Struct(v) => !v.is_empty(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
