use crate::runner::expression::Expression;
use crate::runner::value::Value;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Default)]
pub struct Scope {
    pub variables: HashMap<Value, Value>,
    pub functions: HashMap<Value, Expression>,
}

pub struct Context {
    pub stack: Vec<Scope>,
    pub open_paths: Vec<PathBuf>,
}

impl Context {
    pub fn new(stack: Vec<Scope>, open_paths: Vec<PathBuf>) -> Self {
        Self { stack, open_paths }
    }

    pub fn get_current_path(&self) -> PathBuf {
        let mut current_path = PathBuf::default();

        for open_path in self.open_paths.iter() {
            current_path.push(open_path);
        }

        current_path
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            stack: vec![Scope::default()],
            open_paths: vec![PathBuf::from(".")],
        }
    }
}
