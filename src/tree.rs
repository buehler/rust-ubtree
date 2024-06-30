use std::fmt::Display;

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

        for element in &set {
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
                if (*last_node).end_of_path_set.is_none() {
                    (*last_node).end_of_path_set = Some(set);
                }
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
                if node.end_of_path_set.is_some() {
                    return true;
                }

                current_children = &node.children;
            }
        }

        false
    }

    pub fn subsets(&self, query: &[T]) -> Vec<&[T]> {
        if query.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::new();

        let mut query = query.to_vec();
        query.sort();

        let mut current_children = &self.children;

        for element in query {
            if let Ok(index) = current_children.binary_search_by(|n| n.element.cmp(&element)) {
                let node = &current_children[index];
                if let Some(set) = &node.end_of_path_set {
                    result.push(set.as_slice());
                }

                current_children = &node.children;
            }
        }

        result
    }

    pub fn supersets(&self, query: &[T]) -> Vec<&[T]> {
        let mut query = query.to_vec();
        query.sort();

        let mut current_children = &self.children;

        for element in query {
            if let Ok(index) = current_children.binary_search_by(|n| n.element.cmp(&element)) {
                let node = &current_children[index];
                current_children = &node.children;
            }
        }

        current_children
            .iter()
            .flat_map(|c| c.get_all_eop_sets())
            .map(|s| s.as_slice())
            .collect()
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
    fn insert_single_set() {
        let set = vec![1, 2, 3];
        let mut tree = UBTree::new();
        tree.insert(&set);

        assert_eq!(tree.len(), 1);
        assert_eq!(tree.children[0].element, 1);
        assert_eq!(tree.children[0][0].element, 2);
        assert_eq!(tree.children[0][0][0].element, 3);
    }

    #[test]
    fn insert_multiple_sets() {
        let sets = vec![vec![0, 1, 2, 3], vec![0, 1, 3], vec![0, 1, 2], vec![2, 3]];

        let mut tree = UBTree::new();
        for set in sets {
            tree.insert(&set);
        }

        assert_eq!(tree.len(), 2);
        assert_eq!(tree.children[0].children.len(), 1);
        assert_eq!(tree.children[0][0].children.len(), 2);

        assert_eq!(tree.children[0][0][0][0].element, 3);

        assert!(tree.children[0][0][0][0].end_of_path_set.is_some());
        assert!(!tree.children[0][0].end_of_path_set.is_some());
        assert!(tree.children[0][0][1].end_of_path_set.is_some());
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
        assert_eq!(get_tree().subsets(&Vec::new()), Vec::<&Vec<i32>>::new());
    }

    #[test]
    fn non_empty_truthly_subsets() {
        let tree = get_tree();
        let result = tree.subsets(&vec![0, 1, 2, 3]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], &vec![0, 1, 2]);
        assert_eq!(result[1], &vec![0, 1, 2, 3]);
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
        assert_eq!(result[0], &vec![2, 3]);
    }
}
