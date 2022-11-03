use crate::runner::operator::{cascade_eval, Operator};
use crate::runner::value::Value;
use crate::runner::{Context, Expression, RuntimeError};

pub struct Not {}

impl Not {
    pub const NAME: &'static str = "!";
}

impl Operator for Not {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        let value = cascade_eval(expression, context, |a: bool, b: bool| a & b);
        Ok(Some(Value::from(!bool::from(value)))) // Invert using De Morgan's Law
    }
}
