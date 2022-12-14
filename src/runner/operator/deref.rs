use crate::runner::operator::Operator;
use crate::runner::value::Value;
use crate::runner::{Context, Expression, RuntimeError};
use crate::Runner;

pub struct Deref {}

impl Deref {
    pub const NAME: &'static str = "$";
}

impl Operator for Deref {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        let key =
            Runner::execute(&expression.get_branch(0), context).ok_or(RuntimeError::NoValue)?;

        for scope in context.stack.iter().rev() {
            let vars = &scope.variables;
            if vars.contains_key(&key) {
                return Ok(Some(vars.get(&key).unwrap().clone()));
            }
        }
        Err(RuntimeError::VariableNotFound(key))
    }
}
