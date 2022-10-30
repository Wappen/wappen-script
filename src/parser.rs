use crate::node::Node;
use crate::Token;
use std::cell::RefCell;
use std::rc::Rc;

pub fn parse(tokens: Vec<Token>) -> Node<Token> {
    let mut working_stack = vec![];
    working_stack.push(Rc::new(RefCell::new(Node::new(None))));

    for token in tokens {
        match token {
            Token::Operator(_) => {
                let mut last = working_stack.last_mut().unwrap().borrow_mut();
                last.set_value(Some(token));
            }
            Token::LiteralStr(_) | Token::LiteralNum(_) | Token::Identifier(_) => {
                let branch = Rc::new(RefCell::new(Node::new(Some(token))));
                let mut last = working_stack.last_mut().unwrap().borrow_mut();
                last.add_branch(branch);
            }
            Token::ScopeIn(_) => {
                let branch = Rc::new(RefCell::new(Node::new(None)));
                {
                    let mut last = working_stack.last_mut().unwrap().borrow_mut();
                    last.add_branch(branch.clone());
                }
                working_stack.push(branch);
            }
            Token::ScopeOut(_) | Token::StructEnd(_) => {
                working_stack.remove(working_stack.len() - 1);
            }
            Token::StructStart(_) => {
                let branch = Rc::new(RefCell::new(Node::new(Some(token))));
                {
                    let mut last = working_stack.last_mut().unwrap().borrow_mut();
                    last.add_branch(branch.clone());
                }
                working_stack.push(branch);
            }
        }
    }

    working_stack.remove(0).take()
}
