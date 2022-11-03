use std::path::Path;

use crate::runner::{Context, Expression, RuntimeError};
use crate::Runner;
use crate::runner::operator::Operator;
use crate::runner::value::Value;

pub struct Include {}

impl Include {
    pub const NAME: &'static str = "#";
}

impl Operator for Include {
    fn evaluate(
        &self,
        expression: &Expression,
        context: &mut Context,
    ) -> Result<Option<Value>, RuntimeError> {
        let mut result = None;

        for branch in expression.get_branches() {
            let arg = Runner::execute(&branch.clone(), context).expect("Got nothing to include!");

            if let Value::String(arg) = arg {
                if arg.contains('\n') {
                    result = Runner::run_code(arg, context).unwrap().or(result);
                } else {
                    result = Runner::run_file(Path::new(&arg), context)
                        .unwrap()
                        .or(result);
                }
            }
        }

        Ok(result)
    }
}
