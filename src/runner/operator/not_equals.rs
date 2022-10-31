use crate::runner::operator::Operator;
use crate::runner::value::Value;
use crate::runner::{Expression, RuntimeError, Scope};
use crate::Runner;
use std::collections::HashSet;

pub struct NotEquals {}

impl NotEquals {
    pub const NAME: &'static str = "!=";
}

impl Operator for NotEquals {
    fn evaluate(
        &self,
        expression: &Expression,
        stack: &mut Vec<Scope>,
    ) -> Result<Option<Value>, RuntimeError> {
        let mut set = HashSet::new();
        for branch in expression.borrow().branches() {
            set.insert(Runner::execute(branch.clone(), stack));
        }
        Ok(Some(Value::from(
            set.len() == expression.borrow().branches().len(),
        )))
    }
}