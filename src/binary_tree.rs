use std::fmt::Debug;
use std::cmp::Ordering;


pub type BTreeNode<T> = Option<Box<Node<T>>>;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Node <T> {
    data: T,
    left: BTreeNode<T>,
    right: BTreeNode<T>
}


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BinaryTree<T> {
    root: BTreeNode<T>
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self { root: None }
     }
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        Default::default()
    }
    
    pub fn from_value(val: T) -> Self {
        let root = Box::new(
            Node{
                data: val,
                left: None,
                right: None
            }
        );
        Self { root: Some(root) }    
    }
    
 }

impl<T> BinaryTree<T>
where
    T: Ord,
{
    pub fn insert(&mut self, val: T){
        let new_node = Box::new(
            Node{
                data: val,
                left: None,
                right: None
            }
        );
        Self::insert_node(new_node, &mut self.root);
    }
    fn insert_node(new_node: Box<Node<T>>, current_node: &mut BTreeNode<T>){
        if let Some(node) = current_node {
            match node.data.cmp(&new_node.data){
                Ordering::Less | Ordering::Equal => Self::insert_node(new_node, &mut node.left),
                Ordering::Greater => Self::insert_node(new_node, &mut node.right)
            }
        } else {
            current_node.insert(new_node);    
        }
    }
}