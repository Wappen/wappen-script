use crate::runner::{Context, Expression, RuntimeError};
use crate::runner::operator::{cascade_cmp, Operator};
use crate::runner::value::Value;

pub struct LessEquals {}

impl LessEquals {
    pub const NAME: &'static str = "<=";
}

impl Operator for LessEquals {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        Ok(Some(cascade_cmp(expression, context, |a, b| a <= b)))
    }
}
