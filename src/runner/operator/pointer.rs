use crate::runner::operator::Operator;
use crate::runner::value::Value;
use crate::runner::{Context, Expression, RuntimeError};
use crate::Runner;

pub struct Pointer {}

impl Pointer {
    pub const NAME: &'static str = "->";
}

impl Operator for Pointer {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        let value =
            Runner::execute(&expression.get_branch(0), context).ok_or(RuntimeError::NoValue)?;

        match value {
            Value::String(str) => Ok(Some(Value::Binary(str.into_bytes()))),
            Value::Number(n) => Ok(Some(Value::Binary(Vec::from(n.to_ne_bytes())))),
            Value::Bool(b) => Ok(Some(Value::Binary(Vec::from([u8::from(b)])))),
            Value::Binary(mut vec) => {
                vec.shrink_to_fit();
                let ptr = vec.as_ptr();
                let ptr_bytes = (ptr as usize).to_ne_bytes();
                Ok(Some(Value::Binary(Vec::from(ptr_bytes))))
            }
            Value::Struct(_vec) => {
                todo!("Implement padding packing and c struct stuff")
            }
        }
    }
}
