use crate::runner::operator::Operator;
use crate::runner::value::Value;
use crate::runner::{Context, Expression, RuntimeError};
use crate::Runner;

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
        let key = Runner::execute(&expression.get_branch(0), context).expect("Got no key!");

        let value = Runner::execute(&expression.get_branch(1), context).expect("Got no value!");

        let vars = &mut context.stack.last_mut().unwrap().variables;
        vars.insert(key, value.clone());

        Ok(Some(value))
    }
}
