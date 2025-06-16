use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    key: Vec<String>,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn new(key: Vec<String>) -> Rc<Self> {
        Rc::new(Self {
            key,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    pub fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
        child.parent.replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(child);
        parent
            .children
            .borrow_mut()
            .sort_by_key(|c| c.key().clone());
    }

    pub fn remove_child(parent: &Rc<Node>, child_key: &Vec<String>) -> Option<Rc<Node>> {
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

    pub fn get_parent(&self) -> Option<Rc<Node>> {
        self.parent.borrow().upgrade()
    }

    pub fn get_children(&self) -> Vec<Rc<Node>> {
        self.children.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new(vec!["root".to_string()]);
        assert_eq!(node.key(), &vec!["root".to_string()]);
        assert!(node.get_parent().is_none());
        assert!(node.get_children().is_empty());
    }

    #[test]
    fn test_add_child() {
        let parent = Node::new(vec!["parent".to_string()]);
        let child = Node::new(vec!["child".to_string()]);
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
        let parent = Node::new(vec!["parent".to_string()]);
        let child1 = Node::new(vec!["child1".to_string()]);
        let child2 = Node::new(vec!["child2".to_string()]);

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
