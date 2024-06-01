use tree::tree::Tree;
fn main(){

    let mut b_tree = Tree::new();
    b_tree.insert_value(2);
    b_tree.insert_value(4);
    b_tree.insert_value(3);
    b_tree.insert_value(7);
    b_tree.insert_value(1);
    b_tree.insert_value(1);

    println!("{:?}", b_tree);

    b_tree.print_inorder();

    println!("Finded value: {}", b_tree.find_number(1).unwrap());
}