use crate::graph::node::Node;
use crate::traits::NodeContent;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

pub struct Graph<T: NodeContent> {
    pub root_nodes: RefCell<Vec<Rc<Node<T>>>>,
    _nodes_by_key: RefCell<Option<HashMap<Vec<String>, Rc<Node<T>>>>>,
    _visitation_order: RefCell<Option<Vec<Rc<Node<T>>>>>,
}

impl<T: NodeContent> Graph<T> {
    pub fn new() -> Self {
        Graph {
            root_nodes: RefCell::new(vec![]),
            _nodes_by_key: RefCell::new(None),
            _visitation_order: RefCell::new(None),
        }
    }

    pub fn add_root_node(&self, node: Rc<Node<T>>) {
        self.root_nodes.borrow_mut().push(node);
        // Invalidate cached visitation order and nodes by key
        *self._visitation_order.borrow_mut() = None;
        *self._nodes_by_key.borrow_mut() = None;
    }

    // Performs a depth-first search (DFS) starting from the root nodes
    // Returns a vector of nodes in the order they were visited
    pub fn visitation_order(&self) -> Vec<Rc<Node<T>>> {
        if self._visitation_order.borrow().is_some() {
            // If visitation order is already cached, return it
            return self._visitation_order.borrow().as_ref().unwrap().clone();
        }

        // Use node pointers as keys in the HashSet to avoid key cloning
        let mut visited = HashSet::new();
        let mut stack: VecDeque<Rc<Node<T>>> = VecDeque::new();
        // Initialize the stack with root nodes
        // We use Rc<Node> to ensure we can clone nodes without deep copying
        for root in self.root_nodes.borrow().iter() {
            stack.push_back(root.clone());
        }
        let mut result = Vec::new();

        while let Some(node) = stack.pop_front() {
            // Use the Rc's pointer address as the key for the HashSet

            if !visited.contains(node.key()) {
                visited.insert(node.key().clone());
                result.push(node.clone());

                // Get children and add them to the stack
                // Process in reverse to maintain original traversal order
                let children = node.get_children();
                for child in children.into_iter() {
                    stack.push_back(child);
                }
            }
        }

        // Cache the result
        self._visitation_order.borrow_mut().replace(result.clone());
        result
    }

    // Helper method to get nodes indexed by key
    pub fn nodes_by_key(&self) -> HashMap<Vec<String>, Rc<Node<T>>> {
        if self._nodes_by_key.borrow().is_some() {
            // If nodes_by_key is already cached, return it
            return self._nodes_by_key.borrow().as_ref().unwrap().clone();
        }
        let mut nodes_by_key = HashMap::new();
        for node in self.visitation_order() {
            nodes_by_key.insert(node.key().clone(), node.clone());
        }
        // Cache the result
        *self._nodes_by_key.borrow_mut() = Some(nodes_by_key.clone());
        return nodes_by_key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::node::Node;

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

    fn create_simple_graph() -> Graph<TestNodeContent> {
        let graph = Graph::new();
        let root: Rc<Node<TestNodeContent>> = Node::new(vec!["root"], None);
        let child1: Rc<Node<TestNodeContent>> = Node::new(vec!["child1"], None);
        let child1: Rc<Node<TestNodeContent>> = Node::new(vec!["child1"], None);
        let child2: Rc<Node<TestNodeContent>> = Node::new(vec!["child2"], None);

        Node::add_child(&root, child1.clone());
        Node::add_child(&root, child2.clone());

        graph.add_root_node(root.clone());
        graph
    }

    fn create_complex_graph() -> Graph<TestNodeContent> {
        let graph = Graph::new();
        let root1: Rc<Node<TestNodeContent>> = Node::new(vec!["root1"], None);
        let root2: Rc<Node<TestNodeContent>> = Node::new(vec!["root2"], None);
        let child1: Rc<Node<TestNodeContent>> = Node::new(vec!["child1"], None);
        let child2: Rc<Node<TestNodeContent>> = Node::new(vec!["child2"], None);
        let grandchild: Rc<TestNodeContent> = Node::new(vec!["grandchild"], None);

        Node::add_child(&root1, child1.clone());
        Node::add_child(&child1, grandchild.clone());
        Node::add_child(&root2, child2.clone());

        graph.add_root_node(root1.clone());
        graph.add_root_node(root2.clone());
        graph
    }

    #[test]
    fn test_graph_creation() {
        let graph: Graph<TestNodeContent> = Graph::new();
        assert!(graph.root_nodes.borrow().is_empty());
    }

    #[test]
    fn test_add_root_node() {
        let graph: Graph<TestNodeContent> = Graph::new();
        let node = Node::new(vec!["root".to_string()], None);
        graph.add_root_node(node.clone());
        assert_eq!(graph.root_nodes.borrow().len(), 1);
        assert_eq!(
            graph.root_nodes.borrow()[0].key(),
            &vec!["root".to_string()]
        );
    }

    #[test]
    fn test_visitation_order() {
        let graph = create_simple_graph();

        let order = graph.visitation_order();
        assert_eq!(order.len(), 3);
        assert_eq!(order[0].key(), &vec!["root".to_string()]);
        assert_eq!(order[1].key(), &vec!["child1".to_string()]);
        assert_eq!(order[2].key(), &vec!["child2".to_string()]);
    }

    #[test]
    fn test_complex_visitation_order() {
        let graph = create_complex_graph();

        let order = graph.visitation_order();
        assert_eq!(order.len(), 5);
        assert_eq!(order[0].key(), &vec!["root1".to_string()]);
        assert_eq!(order[1].key(), &vec!["root2".to_string()]);
        assert_eq!(order[2].key(), &vec!["child1".to_string()]);
        assert_eq!(order[3].key(), &vec!["child2".to_string()]);
        assert_eq!(order[4].key(), &vec!["grandchild".to_string()]);
    }

    #[test]
    fn test_nodes_by_key() {
        let graph = create_complex_graph();

        let nodes_by_key = graph.nodes_by_key();
        assert_eq!(nodes_by_key.len(), 5);
        assert!(nodes_by_key.contains_key(&vec!["root1"]));
        assert!(nodes_by_key.contains_key(&vec!["root2"]));
        assert!(nodes_by_key.contains_key(&vec!["child1"]));
        assert!(nodes_by_key.contains_key(&vec!["child2"]));
        assert!(nodes_by_key.contains_key(&vec!["grandchild"]));

        assert!(
            nodes_by_key
                .get(&vec!["root1"])
                .unwrap()
                .get_parent()
                .is_none(),
        );
        assert_eq!(
            nodes_by_key.get(&vec!["root1"]).unwrap().get_children(),
            vec![Node::new(vec!["child1"], None)]
        );
    }
}
