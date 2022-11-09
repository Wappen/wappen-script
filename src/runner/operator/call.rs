use crate::runner::operator::Operator;
use crate::runner::value::Value;
use crate::runner::{Context, Expression, RuntimeError};
use crate::Runner;

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
        let key =
            Runner::execute(&expression.get_branch(0), context).ok_or(RuntimeError::NoValue)?;

        for scope in context.stack.iter().rev() {
            let functions = &scope.functions;
            if functions.contains_key(&key) {
                return Ok(Runner::execute(
                    &functions.get(&key).unwrap().clone(),
                    context,
                ));
            }
        }
        Err(RuntimeError::FunctionNotFound(key))
    }
}
