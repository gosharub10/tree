#[cfg(test)]
mod tests {
    use crate::tree::Tree;
    
    #[test]
    fn test_file() {
        let mut b_tree = Tree::new();
        b_tree.insert_value(2);
        b_tree.insert_value(4);
        b_tree.insert_value(3);
        b_tree.insert_value(7);
        b_tree.insert_value(1);
    
        assert_eq!(b_tree.find_number(7), Some(7));
        assert_eq!(b_tree.find_number(8), None);
    }
}