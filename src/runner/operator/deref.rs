use crate::runner::{Context, Expression, RuntimeError, Scope};
use crate::Runner;
use crate::runner::operator::Operator;
use crate::runner::value::Value;

pub struct Deref {}

impl Deref {
    pub const NAME: &'static str = "!";
}

impl Operator for Deref {
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
            let vars = &scope.variables;
            if vars.contains_key(&key) {
                return Ok(Some(vars.get(&key).unwrap().clone()));
            }
        }
        Err(RuntimeError::VariableNotFound(format!(
            "Variable with key {} not found",
            key
        )))
    }
}
