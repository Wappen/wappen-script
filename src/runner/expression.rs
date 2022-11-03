use crate::node::Node;
use crate::Token;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Expression(Rc<RefCell<Node<Token>>>);

impl Expression {
    pub fn new(node: Node<Token>) -> Self {
        Self(Rc::new(RefCell::new(node)))
    }

    pub fn get_branches(&self) -> Vec<Expression> {
        self.0
            .borrow()
            .branches()
            .iter()
            .map(|b| Expression(b.clone()))
            .collect()
    }

    pub fn get_branch(&self, index: usize) -> Expression {
        Expression(self.0.borrow().branches().get(index).unwrap().clone())
    }

    pub fn get_token(&self) -> &RefCell<Node<Token>> {
        &self.0
    }
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        Expression(self.0.clone())
    }
}
