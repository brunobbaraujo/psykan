use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::traits::NodeContent;

#[derive(Clone, Debug)]
pub struct Node<T: NodeContent> {
    key: String,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
    content: Option<T>,
}

impl<T: NodeContent> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T: NodeContent> Node<T> {
    pub fn new(key: String, content: Option<T>) -> Rc<Self> {
        Rc::new(Self {
            key,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
            content: content,
        })
    }

    pub fn add_child(parent: &Rc<Node<T>>, child: Rc<Node<T>>) {
        child.parent.replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(Rc::clone(&child));
    }

    pub fn remove_child(parent: &Rc<Node<T>>, child_key: String) -> Option<Rc<Node<T>>> {
        let mut children = parent.children.borrow_mut();
        if let Some(pos) = children.iter().position(|c| c.key() == child_key) {
            Some(children.remove(pos))
        } else {
            None
        }
    }

    pub fn key(&self) -> &str {
        &self.key[..]
    }

    pub fn get_parent(&self) -> Option<Rc<Node<T>>> {
        self.parent.borrow().upgrade()
    }

    pub fn get_children(&self) -> Vec<Rc<Node<T>>> {
        self.children
            .borrow_mut()
            .sort_by_key(|c| c.key().to_string());
        self.children.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestNodeContent {
        pub value: i32,
    }

    impl NodeContent for TestNodeContent {
        type Output = i32;

        fn id(&self) -> String {
            format!("TestNodeContent:{}", self.value)
        }

        fn dependencies(&self) -> Vec<String> {
            vec![]
        }

        fn execute(&self) -> Self::Output {
            self.value
        }
    }

    #[test]
    fn test_node_creation() {
        let node: Rc<Node<TestNodeContent>> = Node::new("root".to_string(), None);
        assert_eq!(node.key(), "root");
        assert!(node.get_parent().is_none());
        assert!(node.get_children().is_empty());
    }

    #[test]
    fn test_add_child() {
        let parent: Rc<Node<TestNodeContent>> = Node::new("parent".to_string(), None);
        let child: Rc<Node<TestNodeContent>> = Node::new("child".to_string(), None);
        Node::add_child(&parent, child.clone());

        assert_eq!(parent.get_children().len(), 1);
        assert_eq!(parent.get_children()[0].key(), "child");
        assert_eq!(child.get_parent().unwrap().key(), "parent");
    }

    #[test]
    fn test_remove_child() {
        let parent: Rc<Node<TestNodeContent>> = Node::new("parent".to_string(), None);
        let child1: Rc<Node<TestNodeContent>> = Node::new("child1".to_string(), None);
        let child2: Rc<Node<TestNodeContent>> = Node::new("child2".to_string(), None);

        Node::add_child(&parent, child1.clone());
        Node::add_child(&parent, child2.clone());

        assert_eq!(parent.get_children().len(), 2);

        let removed = Node::remove_child(&parent, "child1".to_string());
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().key(), "child1".to_string());

        assert_eq!(parent.get_children().len(), 1);
        assert_eq!(parent.get_children()[0].key(), "child2".to_string());
    }
}
