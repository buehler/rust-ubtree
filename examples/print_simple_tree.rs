use rust_ubtree::UBTree;

fn get_tree() -> UBTree<i32> {
    let sets = vec![vec![0, 1, 2, 3], vec![0, 1, 3], vec![0, 1, 2], vec![2, 3]];

    let mut tree = UBTree::new();
    for set in sets {
        tree.insert(&set);
    }

    tree
}

fn main() {
    let tree = get_tree();
    println!("{}", tree);
}
