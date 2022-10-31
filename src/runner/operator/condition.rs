use crate::runner::operator::Operator;
use crate::runner::value::Value;
use crate::runner::{Expression, RuntimeError, Scope};
use crate::Runner;

pub struct Condition {}

impl Condition {
    pub const NAME: &'static str = "?";
}

impl Operator for Condition {
    fn evaluate(
        &self,
        expression: &Expression,
        stack: &mut Vec<Scope>,
    ) -> Result<Option<Value>, RuntimeError> {
        let condition = Runner::execute(
            expression.borrow().branches().get(0).unwrap().clone(),
            stack,
        )
        .expect("Got no condition!");

        return if bool::from(condition) {
            Ok(Runner::execute(
                expression.borrow().branches().get(1).unwrap().clone(),
                stack,
            ))
        } else {
            Ok(Runner::execute(
                expression.borrow().branches().get(2).unwrap().clone(),
                stack,
            ))
        };
    }
}
