use crate::runner::{Context, Expression, RuntimeError};
use crate::runner::operator::{cascade_eval, Operator};
use crate::runner::value::Value;

pub struct And {}

impl And {
    pub const NAME: &'static str = "&";
}

impl Operator for And {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        Ok(Some(cascade_eval(
            expression,
            context,
            |a: bool, b: bool| a & b,
        )))
    }
}
