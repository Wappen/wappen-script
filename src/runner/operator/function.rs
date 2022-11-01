use crate::runner::{Context, Expression, RuntimeError, Scope};
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
        let key = Runner::execute(
            expression.borrow().branches().get(0).unwrap().clone(),
            context,
        )
            .expect("Got no key!");

        let body = expression.borrow().branches().get(1).unwrap().clone();

        let functions = &mut context.stack.last_mut().unwrap().functions;
        functions.insert(key.clone(), body);

        Ok(Some(key))
    }
}
