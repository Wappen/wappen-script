use crate::runner::{Context, Expression, RuntimeError};
use crate::runner::operator::{cascade_eval, Operator};
use crate::runner::value::Value;

pub struct Or {}

impl Or {
    pub const NAME: &'static str = "|";
}

impl Operator for Or {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        Ok(Some(cascade_eval(
            expression,
            context,
            |a: bool, b: bool| a | b,
        )))
    }
}
