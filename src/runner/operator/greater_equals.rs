use crate::runner::operator::{cascade_cmp, Operator};
use crate::runner::value::Value;
use crate::runner::{Context, Expression, RuntimeError};

pub struct GreaterEquals {}

impl GreaterEquals {
    pub const NAME: &'static str = ">=";
}

impl Operator for GreaterEquals {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        Ok(Some(cascade_cmp(expression, context, |a, b| a >= b)))
    }
}
