use crate::runner::operator::Operator;
use crate::runner::value::Value;
use crate::runner::{Expression, RuntimeError, Scope};
use crate::Runner;

pub struct Deref {}

impl Deref {
    pub const NAME: &'static str = "!";
}

impl Operator for Deref {
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
