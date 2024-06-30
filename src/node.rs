// maybe use BTreeSet instead of vec?

use std::{
    fmt::Display,
    ops::{Deref, Index},
};

pub struct Node<T> {
    pub(crate) children: Vec<Node<T>>,
    pub(crate) element: T,
    pub(crate) end_of_path_set: Option<Vec<T>>,
}

impl<T> Node<T> {
    pub fn new(element: T) -> Self {
        Node {
            children: Vec::new(),
            element,
            end_of_path_set: None,
        }
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.end_of_path_set.is_some() {
            write!(f, "({}; EoP)", self.element)
        } else {
            write!(f, "({})", self.element)
        }
    }
}

impl<T> Index<usize> for Node<T> {
    type Output = Node<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.children[index]
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_node_non_eop() {
        let node = Node {
            children: Vec::new(),
            element: 1,
            end_of_path_set: None,
        };
        assert_eq!(format!("{}", node), "(1)");
    }

    #[test]
    fn display_node_eop() {
        let node = Node {
            children: Vec::new(),
            element: 1,
            end_of_path_set: Some(vec![1, 2, 3]),
        };
        assert_eq!(format!("{}", node), "(1; EoP)");
    }

    #[test]
    fn deref_to_value() {
        let node = Node::new("hello world");
        assert_eq!(*node, "hello world");
    }

    #[test]
    fn index_to_children() {
        let node = Node {
            element: 1,
            children: vec![Node::new(2), Node::new(3)],
            end_of_path_set: None,
        };
        assert_eq!(node[0].element, 2);
    }
}
