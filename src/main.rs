mod structure;
use crate::structure::Tree::Node;

fn main() {
    let mut node: Node= Node:: new(5);
    let mut lnode: Node = Node:: new(3);
    let mut rnode: Node = Node:: new(2);
    node.add_left_child(lnode);
    node.add_right_child(rnode);
    println!("{:?}",node);

    let leftval = node.getLeftChild().clone().unwrap().borrow().value;
    println!("{}",leftval);

    let mut nlnode: Node = Node:: new(3);
    let mut nrnode: Node = Node:: new(2);

    lnode.add_left_child(nlnode);
}
