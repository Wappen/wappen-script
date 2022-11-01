use std::collections::HashSet;

use crate::runner::{Context, Expression, RuntimeError, Scope};
use crate::Runner;
use crate::runner::operator::Operator;
use crate::runner::value::Value;

pub struct Equals {}

impl Equals {
    pub const NAME: &'static str = "==";
}

impl Operator for Equals {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        let mut set = HashSet::new();
        for branch in expression.borrow().branches() {
            set.insert(Runner::execute(branch.clone(), context));
        }
        Ok(Some(Value::from(set.len() == 1)))
    }
}
