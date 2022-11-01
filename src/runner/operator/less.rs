use crate::runner::operator::{cascade_cmp, Operator};
use crate::runner::value::Value;
use crate::runner::{Context, Expression, RuntimeError, Scope};

pub struct Less {}

impl Less {
    pub const NAME: &'static str = "<";
}

impl Operator for Less {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        Ok(Some(cascade_cmp(expression, context, |a, b| a < b)))
    }
}
