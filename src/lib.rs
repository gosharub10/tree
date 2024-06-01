pub mod tests;

pub mod tree_node{
    #[derive(PartialEq, Clone, Debug)]
    pub struct TreeNode {
        pub value: i32,
        pub right: TreeNodeNext,
        pub left: TreeNodeNext,
    }

    type TreeNodeNext = Option<Box<TreeNode>>;

    impl TreeNode {
        pub fn new(value: i32) -> Self {
            TreeNode {
                value,
                right: None,
                left: None,
            }
        }
    }
}

pub mod tree{
    use core::cmp::Ordering;
    use crate::tree_node::TreeNode;
    #[derive(Debug)]    
    pub struct Tree {
        root: Option<TreeNode>,
    }

    impl Tree {
        pub fn new() -> Self {
            Tree { root: None }
        }
    
        pub fn insert_value(&mut self, value: i32) {
            let new_node = TreeNode::new(value);
            if self.root == None {
                self.root = Some(new_node);
                return;
            } else {
                Tree::insert_node(self.root.as_mut().unwrap(), &new_node)
            }
        }
    
        fn insert_node(current_node: &mut TreeNode, new_node: &TreeNode) {
            match new_node.value.cmp(&current_node.value) {
                Ordering::Less => {
                    if current_node.left == None {
                        current_node.left = Some(Box::new(new_node.clone()));
                    } else {
                        return Tree::insert_node(current_node.left.as_mut().unwrap(), new_node);
                    }
                }
                Ordering::Greater =>{
                    if current_node.right == None {
                        current_node.right = Some(Box::new(new_node.clone()));
                    } else {
                        return Tree::insert_node(current_node.right.as_mut().unwrap(), new_node);
                    }
                }
                Ordering::Equal => println!("I cant insert same value in tree")
            }
        }
    
        pub fn print_inorder(&self) {
            Tree::print_inorder_recursive(self.root.clone().map(Box::new));
        }
    
        fn print_inorder_recursive(node: Option<Box<TreeNode>>) {
            if let Some(current_node) = node {
                Tree::print_inorder_recursive(current_node.left);
                println!("{}", current_node.value);
                Tree::print_inorder_recursive(current_node.right);
            }
        }

        pub fn find_number(&self, number: i32) -> Option<i32> {
            Tree::search_node(self.root.clone().map(Box::new), number)
        }
    
        fn search_node(node: Option<Box<TreeNode>>, number: i32) -> Option<i32> {
            if let Some(current_node)= node{
                match current_node.value.cmp(&number) {
                    Ordering::Less => return Tree::search_node(current_node.right, number),
                    Ordering::Equal => return Some(current_node.value),
                    Ordering::Greater => return Tree::search_node(current_node.left, number),
                }
            }
            else {
                None
            }
        }
    }
}