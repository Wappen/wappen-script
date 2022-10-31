use crate::runner::operator::Operator;
use crate::runner::value::Value;
use crate::runner::{Expression, RuntimeError, Scope};
use crate::{parse, tokenize, Runner};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

pub struct Include {}

impl Include {
    pub const NAME: &'static str = "#";
}

impl Operator for Include {
    fn evaluate(
        &self,
        expression: &Expression,
        stack: &mut Vec<Scope>,
    ) -> Result<Option<Value>, RuntimeError> {
        let mut result = None;

        for branch in expression.borrow().branches() {
            let arg = Runner::execute(branch.clone(), stack).expect("Got nothing to include!");

            if let Value::String(arg) = arg {
                if arg.contains('\n') {
                    let tokens = tokenize(&arg);
                    let ast = parse(tokens);
                    result = Runner::run(Rc::new(RefCell::new(ast))).or(result);
                } else {
                    let code =
                        std::fs::read_to_string(Path::new(&arg)).expect("Could not include file!");
                    let tokens = tokenize(&code);
                    let ast = parse(tokens);
                    result = Runner::run(Rc::new(RefCell::new(ast))).or(result);
                }
            }
        }

        Ok(result)
    }
}
