// maybe use BTreeSet instead of vec?

use std::{fmt::Display, ops::Index};

pub(crate) struct Node<T> {
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

    pub fn get_all_eop_sets(&self) -> Vec<&Vec<T>> {
        let mut result: Vec<&Vec<T>> = self
            .children
            .iter()
            .flat_map(|c| c.get_all_eop_sets())
            .collect();

        if let Some(set) = &self.end_of_path_set {
            result.insert(0, set);
        }

        result
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
