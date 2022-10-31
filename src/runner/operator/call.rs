use crate::runner::operator::Operator;
use crate::runner::value::Value;
use crate::runner::{Expression, RuntimeError, Scope};
use crate::Runner;

pub struct Call {}

impl Call {
    pub const NAME: &'static str = "@";
}

impl Operator for Call {
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

        for scope in stack.iter().rev() {
            let functions = &scope.functions;
            if functions.contains_key(&key) {
                return Ok(Runner::execute(functions.get(&key).unwrap().clone(), stack));
            }
        }
        Err(RuntimeError::FunctionNotFound(format!(
            "Function with key {} not found",
            key
        )))
    }
}
