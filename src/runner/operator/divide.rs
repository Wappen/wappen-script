use crate::runner::operator::{cascade_eval, Operator};
use crate::runner::value::Value;
use crate::runner::{Expression, RuntimeError, Scope};

pub struct Divide {}

impl Divide {
    pub const NAME: &'static str = "/";
}

impl Operator for Divide {
    fn evaluate(
        &self,
        expression: &Expression,
        stack: &mut Vec<Scope>,
    ) -> Result<Option<Value>, RuntimeError> {
        Ok(Some(cascade_eval(expression, stack, |a: f64, b: f64| {
            a / b
        })))
    }
}
