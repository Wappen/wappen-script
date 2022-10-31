use crate::runner::operator::{cascade_eval, Operator};
use crate::runner::value::Value;
use crate::runner::{Expression, RuntimeError, Scope};

pub struct And {}

impl And {
    pub const NAME: &'static str = "&";
}

impl Operator for And {
    fn evaluate(
        &self,
        expression: &Expression,
        stack: &mut Vec<Scope>,
    ) -> Result<Option<Value>, RuntimeError> {
        Ok(Some(cascade_eval(expression, stack, |a: bool, b: bool| {
            a & b
        })))
    }
}
