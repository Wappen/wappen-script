use crate::runner::{Context, Expression, RuntimeError};
use crate::Runner;
use crate::runner::operator::Operator;
use crate::runner::value::Value;

pub struct Function {}

impl Function {
    pub const NAME: &'static str = "^";
}

impl Operator for Function {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        let key = Runner::execute(&expression.get_branch(0), context).expect("Got no key!");

        let body = expression.get_branch(1);

        let functions = &mut context.stack.last_mut().unwrap().functions;
        functions.insert(key.clone(), body);

        Ok(Some(key))
    }
}
