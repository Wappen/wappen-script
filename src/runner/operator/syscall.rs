use crate::runner::{Context, Expression, RuntimeError, Scope};
use crate::runner::operator::Operator;
use crate::runner::value::Value;

pub struct SysCall {}

impl SysCall {
    pub const NAME: &'static str = "~";
}

impl Operator for SysCall {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        todo!()
    }
}
