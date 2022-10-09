mod binary_tree;

fn main() {
    let mut bst: binary_tree::BinaryTree<i32> = binary_tree::BinaryTree::new();
    bst.insert(5);
    bst.insert(7);
    bst.insert(99);
    bst.insert(1);
    bst.insert(2);
    bst.insert(0);
    println!("{:#?}", bst)
}
