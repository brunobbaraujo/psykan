use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
pub struct Node<F> {
    key: Vec<String>,
    parent: RefCell<Weak<Node<F>>>,
    children: RefCell<Vec<Rc<Node<F>>>>,
    data: Option<F>,
}

impl<F> PartialEq for Node<F> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<F> Node<F> {
    pub fn new(key: Vec<String>, data: Option<F>) -> Rc<Self> {
        Rc::new(Self {
            key,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
            data: data,
        })
    }

    pub fn add_child(parent: &Rc<Node<F>>, child: Rc<Node<F>>) {
        child.parent.replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(Rc::clone(&child));
    }

    pub fn remove_child(parent: &Rc<Node<F>>, child_key: &Vec<String>) -> Option<Rc<Node<F>>> {
        let mut children = parent.children.borrow_mut();
        if let Some(pos) = children.iter().position(|c| c.key() == child_key) {
            Some(children.remove(pos))
        } else {
            None
        }
    }

    pub fn key(&self) -> &Vec<String> {
        &self.key
    }

    pub fn get_parent(&self) -> Option<Rc<Node<F>>> {
        self.parent.borrow().upgrade()
    }

    pub fn get_children(&self) -> Vec<Rc<Node<F>>> {
        self.children.borrow_mut().sort_by_key(|c| c.key().clone());
        self.children.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node: Rc<Node<i32>> = Node::new(vec!["root".to_string()], None);
        assert_eq!(node.key(), &vec!["root".to_string()]);
        assert!(node.get_parent().is_none());
        assert!(node.get_children().is_empty());
    }

    #[test]
    fn test_add_child() {
        let parent: Rc<Node<i32>> = Node::new(vec!["parent".to_string()], None);
        let child: Rc<Node<i32>> = Node::new(vec!["child".to_string()], None);
        Node::add_child(&parent, child.clone());

        assert_eq!(parent.get_children().len(), 1);
        assert_eq!(parent.get_children()[0].key(), &vec!["child".to_string()]);
        assert_eq!(
            child.get_parent().unwrap().key(),
            &vec!["parent".to_string()]
        );
    }

    #[test]
    fn test_remove_child() {
        let parent: Rc<Node<i32>> = Node::new(vec!["parent".to_string()], None);
        let child1: Rc<Node<i32>> = Node::new(vec!["child1".to_string()], None);
        let child2: Rc<Node<i32>> = Node::new(vec!["child2".to_string()], None);

        Node::add_child(&parent, child1.clone());
        Node::add_child(&parent, child2.clone());

        assert_eq!(parent.get_children().len(), 2);

        let removed = Node::remove_child(&parent, &vec!["child1".to_string()]);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().key(), &vec!["child1".to_string()]);

        assert_eq!(parent.get_children().len(), 1);
        assert_eq!(parent.get_children()[0].key(), &vec!["child2".to_string()]);
    }
}
