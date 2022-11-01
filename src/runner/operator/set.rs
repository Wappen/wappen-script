use crate::runner::{Context, Expression, RuntimeError, Scope};
use crate::Runner;
use crate::runner::operator::Operator;
use crate::runner::value::Value;

pub struct Set {}

impl Set {
    pub const NAME: &'static str = "=";
}

impl Operator for Set {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        let key = Runner::execute(
            expression.borrow().branches().get(0).unwrap().clone(),
            context,
        )
            .expect("Got no key!");

        let value = Runner::execute(
            expression.borrow().branches().get(1).unwrap().clone(),
            context,
        )
            .expect("Got no value!");

        let vars = &mut context.stack.last_mut().unwrap().variables;
        vars.insert(key, value.clone());

        Ok(Some(value))
    }
}
