use crate::runner::{Context, Expression, RuntimeError};
use crate::Runner;
use crate::runner::operator::Operator;
use crate::runner::value::Value;

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
        let condition = Runner::execute(
            expression.borrow().branches().get(0).unwrap().clone(),
            context,
        )
            .expect("Got no condition!");

        return if bool::from(condition) {
            Ok(Runner::execute(
                expression.borrow().branches().get(1).unwrap().clone(),
                context,
            ))
        } else if expression.borrow().branches().len() == 3 {
            Ok(Runner::execute(
                expression.borrow().branches().get(2).unwrap().clone(),
                context,
            ))
        } else {
            Ok(None)
        };
    }
}
