use crate::runner::operator::{cascade_cmp, Operator};
use crate::runner::value::Value;
use crate::runner::{Expression, RuntimeError, Scope};

pub struct GreaterEquals {}

impl GreaterEquals {
    pub const NAME: &'static str = ">=";
}

impl Operator for GreaterEquals {
    fn evaluate(
        &self,
        expression: &Expression,
        stack: &mut Vec<Scope>,
    ) -> Result<Option<Value>, RuntimeError> {
        Ok(Some(cascade_cmp(expression, stack, |a, b| a >= b)))
    }
}
