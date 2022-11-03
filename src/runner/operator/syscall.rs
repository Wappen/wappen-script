use syscalls::{syscall, Sysno};

use crate::runner::{Context, Expression, RuntimeError};
use crate::Runner;
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
        let sysno =
            Runner::execute(&expression.get_branch(0), context).expect("Got no syscall number!");

        match unsafe { syscall!(Sysno::from(f64::from(sysno) as i32)) } {
            Ok(ok) => Ok(Some(Value::Number(ok as f64))),
            Err(err) => Err(RuntimeError::SysCallError(format!(
                "SysCall returned error {}",
                err
            ))),
        }
    }
}
