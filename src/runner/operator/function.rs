use crate::runner::operator::Operator;
use crate::runner::value::Value;
use crate::runner::{Expression, RuntimeError, Scope};
use crate::Runner;

pub struct Function {}

impl Function {
    pub const NAME: &'static str = "^";
}

impl Operator for Function {
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

        let body = expression.borrow().branches().get(1).unwrap().clone();

        let functions = &mut stack.last_mut().unwrap().functions;
        functions.insert(key.clone(), body);

        Ok(Some(key))
    }
}
