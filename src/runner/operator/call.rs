use crate::runner::{Context, Expression, RuntimeError, Scope};
use crate::Runner;
use crate::runner::operator::Operator;
use crate::runner::value::Value;

pub struct Call {}

impl Call {
    pub const NAME: &'static str = "@";
}

impl Operator for Call {
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

        for scope in context.stack.iter().rev() {
            let functions = &scope.functions;
            if functions.contains_key(&key) {
                return Ok(Runner::execute(
                    functions.get(&key).unwrap().clone(),
                    context,
                ));
            }
        }
        Err(RuntimeError::FunctionNotFound(format!(
            "Function with key {} not found",
            key
        )))
    }
}
