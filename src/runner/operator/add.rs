use crate::runner::{Context, Expression, RuntimeError};
use crate::runner::operator::{cascade_eval, Operator};
use crate::runner::value::Value;

pub struct Add {}

impl Add {
    pub const NAME: &'static str = "+";
}

impl Operator for Add {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        Ok(Some(cascade_eval(expression, context, |a: f64, b: f64| {
            a + b
        })))
    }
}
