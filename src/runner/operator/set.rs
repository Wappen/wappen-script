use crate::runner::operator::Operator;
use crate::runner::value::Value;
use crate::runner::{Expression, RuntimeError, Scope};
use crate::Runner;

pub struct Set {}

impl Set {
    pub const NAME: &'static str = "=";
}

impl Operator for Set {
    fn evaluate(
        &self,
        expression: &Expression,
        stack: &mut Vec<Scope>,
    ) -> Result<Option<Value>, RuntimeError> {
        let key = Runner::execute(
            expression.borrow().branches().get(0).unwrap().clone(),
            stack,
        )
        .expect("Got no key!");

        let value = Runner::execute(
            expression.borrow().branches().get(1).unwrap().clone(),
            stack,
        )
        .expect("Got no value!");

        let vars = &mut stack.last_mut().unwrap().variables;
        vars.insert(key, value.clone());

        Ok(Some(value))
    }
}
