use std::cell::RefCell;
use std::rc::Rc;

pub struct Node<T> {
    value: Option<T>,
    branches: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Self {
            value: None,
            branches: vec![],
        }
    }
}

impl<T> Node<T> {
    pub fn new(value: Option<T>) -> Self {
        Self {
            value,
            branches: vec![],
        }
    }

    pub fn add_branch(&mut self, branch: Rc<RefCell<Node<T>>>) {
        self.branches.push(branch);
    }

    pub fn set_value(&mut self, value: Option<T>) {
        self.value = value;
    }

    pub fn branches(&self) -> &Vec<Rc<RefCell<Node<T>>>> {
        &self.branches
    }

    pub fn value(&self) -> &Option<T> {
        &self.value
    }
}
