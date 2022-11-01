use crate::runner::operator::{cascade_eval, Operator};
use crate::runner::value::Value;
use crate::runner::{Context, Expression, RuntimeError, Scope};

pub struct Multiply {}

impl Multiply {
    pub const NAME: &'static str = "*";
}

impl Operator for Multiply {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        Ok(Some(cascade_eval(expression, context, |a: f64, b: f64| {
            a * b
        })))
    }
}
