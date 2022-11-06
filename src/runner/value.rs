use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem::size_of;
use std::str::FromStr;

use crate::runner::context::{Context, Scope};
use crate::runner::{Expression, RuntimeError};
use crate::Runner;

trait Serialize {
    fn serialize(&self) -> Result<Vec<u8>, RuntimeError>;
}

#[derive(Debug, Clone, PartialOrd)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Binary(Vec<u8>),
    Struct(Vec<Value>),
}

impl Value {
    pub fn make_struct(expression: &Expression, context: &mut Context) -> Value {
        context.stack.push(Scope::default());
        let mut values: Vec<Value> = vec![];
        for branch in expression.get_branches() {
            let value = Runner::execute(&branch, context);

            if let Some(value) = value {
                values.push(value);
            }
        }
        context.stack.pop();
        Value::Struct(values)
    }

    fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), RuntimeError> {
        let offset = buf.len();
        let align = size_of::<isize>(); // TODO: Currently not valid for any type https://en.wikipedia.org/wiki/Data_structure_alignment#Computing_padding
        let padding = (align - (offset % align)) % align;

        if padding > 0 {
            buf.append(&mut Vec::with_capacity(padding));
        }

        match self {
            Value::Binary(vec) => buf.extend_from_slice(vec),
            Value::Struct(vec) => {
                for value in vec {
                    value.serialize(buf)?;
                }
            }
            _ => return Err(RuntimeError::IllegalBinaryConversion),
        }

        Ok(())
    }
}

impl Serialize for Value {
    fn serialize(&self) -> Result<Vec<u8>, RuntimeError> {
        let mut buf = Vec::new();
        self.serialize(&mut buf)?;
        Ok(buf)
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
            Value::Number(n) => n.to_bits().hash(state),
            Value::Bool(b) => b.hash(state),
            Value::Binary(v) => v.hash(state),
            Value::Struct(v) => v.hash(state),
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
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
            Value::Binary(v) => match v.try_into() {
                Ok(arr) => f64::from_ne_bytes(arr),
                Err(v) => v.len() as f64,
            },
            Value::Struct(v) => v.len() as f64,
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
            Value::Binary(v) => !v.is_empty(),
            Value::Struct(v) => !v.is_empty(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
