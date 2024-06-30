use std::{fmt::Display, ops::Index};

use crate::node::Node;

pub struct UBTree<T: Clone + Ord> {
    children: Vec<Node<T>>,
}

impl<T: Clone + Ord> UBTree<T> {
    pub fn new() -> Self {
        UBTree {
            children: Vec::new(),
        }
    }

    pub fn new_with_data(data: &Vec<Vec<T>>) -> Self {
        let mut tree = UBTree::new();
        for set in data {
            tree.insert(set);
        }

        tree
    }

    pub fn insert(&mut self, set: &[T]) {
        if set.is_empty() {
            return;
        }

        let mut set = set.to_vec();
        set.sort();

        let mut current_children = &mut self.children;
        let mut last_node: *mut Node<T> = std::ptr::null_mut();

        for element in set {
            match current_children.binary_search_by(|n| n.element.cmp(&element)) {
                Err(index) => {
                    // the element is not found in the children list. create new node and insert it at
                    // the proposed index.
                    let node = Node::new(element.clone());
                    current_children.insert(index, node);
                    last_node = &mut current_children[index];
                    current_children = &mut current_children[index].children;
                }
                Ok(index) => {
                    // the element is found in the children list. move to the next level.
                    last_node = &mut current_children[index];
                    current_children = &mut current_children[index].children;
                }
            };
        }

        if !last_node.is_null() {
            unsafe {
                (*last_node).end_of_path = true;
            }
        }
    }

    pub fn exists(&self, query: &[T]) -> bool {
        if query.is_empty() {
            return false;
        }

        let mut query = query.to_vec();
        query.sort();

        let mut current_children = &self.children;

        for element in query {
            if let Ok(index) = current_children.binary_search_by(|n| n.element.cmp(&element)) {
                let node = &current_children[index];
                if node.end_of_path {
                    return true;
                }

                current_children = &node.children;
            }
        }

        false
    }

    pub fn subsets(&self, query: &[T]) -> Vec<Vec<T>> {
        if query.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::new();

        let mut query = query.to_vec();
        query.sort();

        let mut current_children = &self.children;
        let mut current_path = Vec::new();

        for element in query {
            if let Ok(index) = current_children.binary_search_by(|n| n.element.cmp(&element)) {
                let node = &current_children[index];
                current_path.push(node.element.clone());
                if node.end_of_path {
                    result.push(current_path.clone());
                }

                current_children = &node.children;
            }
        }

        result
    }

    pub fn supersets(&self, query: &[T]) -> Vec<Vec<T>> {
        let mut query = query.to_vec();
        query.sort();

        let mut current_children = &self.children;
        // defines whether at least one element was found in the query
        // but if the query is empty, then all children are supersets.
        let mut found = query.is_empty();

        // Find the node that corresponds to the last element in the query.
        // All children (recursively) of this node are supersets of the query (if they end of path).
        for element in &query {
            if let Ok(index) = current_children.binary_search_by(|n| n.element.cmp(element)) {
                let node = &current_children[index];
                current_children = &node.children;
                found = true;
            }
        }

        if !found {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut node_stack = Vec::new();
        node_stack.extend(current_children.iter().map(|c| (c, query.clone())));

        while !node_stack.is_empty() {
            let (node, mut path) = node_stack.pop().unwrap();
            path.push(node.element.clone());

            if node.end_of_path {
                result.push(path.clone());
            }

            node_stack.extend(node.children.iter().map(|c| (c, path.clone())));
        }

        result
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }
}

impl<T: Clone + Ord + Display> Display for UBTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stack = self
            .children
            .iter()
            .rev()
            .map(|c| (0, c))
            .collect::<Vec<(i32, &Node<T>)>>();

        while !stack.is_empty() {
            let (level, node) = stack.pop().unwrap();
            writeln!(f, "{:indent$}{}", "", node, indent = level as usize * 2)?;

            if !node.children.is_empty() {
                for child in node.children.iter().rev() {
                    stack.push((level + 1, child));
                }
            }
        }

        Ok(())
    }
}

impl<T: Clone + Ord> Index<usize> for UBTree<T> {
    type Output = Node<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.children[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_tree() -> UBTree<i32> {
        let sets = vec![vec![0, 1, 2, 3], vec![0, 1, 3], vec![0, 1, 2], vec![2, 3]];

        let mut tree = UBTree::new();
        for set in sets {
            tree.insert(&set);
        }

        tree
    }

    #[test]
    fn display_tree() {
        let result = r#"(0)
  (1)
    (2; EoP)
      (3; EoP)
    (3; EoP)
(2)
  (3; EoP)
"#;

        assert_eq!(format!("{}", get_tree()), result);
    }

    #[test]
    fn index_to_children() {
        let tree = get_tree();
        assert_eq!(*tree[0], 0);
    }

    #[test]
    fn insert_single_set() {
        let set = vec![1, 2, 3];
        let mut tree = UBTree::new();
        tree.insert(&set);

        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].element, 1);
        assert_eq!(tree[0][0].element, 2);
        assert_eq!(tree[0][0][0].element, 3);
    }

    #[test]
    fn insert_multiple_sets() {
        let sets = vec![vec![0, 1, 2, 3], vec![0, 1, 3], vec![0, 1, 2], vec![2, 3]];

        let mut tree = UBTree::new();
        for set in sets {
            tree.insert(&set);
        }

        assert_eq!(tree.len(), 2);
        assert_eq!(tree[0].children.len(), 1);
        assert_eq!(tree[0][0].children.len(), 2);

        assert_eq!(tree[0][0][0][0].element, 3);

        assert!(tree[0][0][0][0].end_of_path);
        assert!(!tree[0][0].end_of_path);
        assert!(tree[0][0][1].end_of_path);
    }

    #[test]
    fn empty_exists() {
        assert!(!get_tree().exists(&Vec::new()));
    }

    #[test]
    fn non_empty_truthly_exists() {
        assert!(get_tree().exists(&vec![0, 1, 2, 3]));
    }

    #[test]
    fn non_empty_falsely_exists() {
        assert!(!get_tree().exists(&vec![0, 1, 4]));
    }

    #[test]
    fn empty_subsets() {
        assert_eq!(get_tree().subsets(&Vec::new()), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn non_empty_truthly_subsets() {
        let tree = get_tree();
        let result = tree.subsets(&vec![0, 1, 2, 3]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec![0, 1, 2]);
        assert_eq!(result[1], vec![0, 1, 2, 3]);
    }

    #[test]
    fn non_empty_falsely_subsets() {
        let tree = get_tree();
        let result = tree.subsets(&vec![0, 1, 4]);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn empty_supersets() {
        let tree = get_tree();
        let result = tree.supersets(&vec![]);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn multiple_supersets() {
        let tree = get_tree();
        let result = tree.supersets(&vec![0, 1]);
        assert_eq!(result.len(), 3);

        let result = tree.supersets(&vec![2]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], vec![2, 3]);
    }
}
