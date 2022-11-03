use std::collections::HashSet;

use crate::runner::{Context, Expression, RuntimeError};
use crate::Runner;
use crate::runner::operator::Operator;
use crate::runner::value::Value;

pub struct NotEquals {}

impl NotEquals {
    pub const NAME: &'static str = "!=";
}

impl Operator for NotEquals {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        let mut set = HashSet::new();
        for branch in expression.get_branches() {
            set.insert(Runner::execute(&branch.clone(), context));
        }
        Ok(Some(Value::from(
            set.len() == expression.get_branches().len(),
        )))
    }
}
