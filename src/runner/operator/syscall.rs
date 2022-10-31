use crate::runner::operator::Operator;
use crate::runner::value::Value;
use crate::runner::{Expression, RuntimeError, Scope};

pub struct SysCall {}

impl SysCall {
    pub const NAME: &'static str = "~";
}

impl Operator for SysCall {
    fn evaluate(
        &self,
        expression: &Expression,
        stack: &mut Vec<Scope>,
    ) -> Result<Option<Value>, RuntimeError> {
        todo!()
    }
}
