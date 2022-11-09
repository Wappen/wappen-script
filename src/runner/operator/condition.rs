use crate::runner::operator::Operator;
use crate::runner::value::Value;
use crate::runner::{Context, Expression, RuntimeError};
use crate::Runner;

pub struct Condition {}

impl Condition {
    pub const NAME: &'static str = "?";
}

impl Operator for Condition {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        let condition =
            Runner::execute(&expression.get_branch(0), context).ok_or(RuntimeError::NoValue)?;

        if bool::from(condition) {
            Ok(Runner::execute(&expression.get_branch(1), context))
        } else if expression.get_branches().len() == 3 {
            Ok(Runner::execute(&expression.get_branch(2), context))
        } else {
            Ok(None)
        }
    }
}
