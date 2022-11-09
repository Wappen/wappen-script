use syscalls::{syscall, Sysno};

use crate::runner::operator::Operator;
use crate::runner::value::Value;
use crate::runner::{Context, Expression, RuntimeError};
use crate::Runner;

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
        let sysno = Sysno::from(f64::from(
            Runner::execute(&expression.get_branch(0), context).ok_or(RuntimeError::NoValue)?,
        ) as i32);

        match unsafe { syscall!(sysno) } {
            Ok(ok) => Ok(Some(Value::Number(ok as f64))),
            Err(errno) => Err(RuntimeError::SysCallError(sysno, errno)),
        }
    }
}
