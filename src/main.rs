mod structure;
mod tool;

use crate::structure::tree::Node;
use crate::structure::tree::NodeLink;
use crate::tool::generate_dotfile;


fn main() {
    //create the nodelink of the root node
    let rootlink: NodeLink = Node::new_nodelink(88);

    //add a new left node value
    rootlink.clone().borrow_mut().add_left_child(&rootlink,48);
    //add a new right node value
    rootlink.clone().borrow_mut().add_right_child(&rootlink,888);
    
    
    //print the tree at this time
    let mut main_tree_path = "prime.dot";
    generate_dotfile(&rootlink, main_tree_path);
    
    //add new child values to the left subtree
    let leftsub = &rootlink.clone().borrow().left.as_ref().unwrap().clone();
    rootlink.clone().borrow_mut().add_right_child(&leftsub,84);

    //print the tree again, now been added with more values
    main_tree_path = "prime_t2";
    generate_dotfile(&rootlink, main_tree_path);

    //add new child values to the right subtree
    let rightsub = &rootlink.clone().borrow().right.as_ref().unwrap().clone();
    rootlink.clone().borrow_mut().add_left_child(&rightsub,848);

    //Call tree depth function at this time
    let treedepth = rootlink.clone().borrow_mut().tree_depth();
    println!("tree depth {}", treedepth);
    
    //Call count_nodes function 
    let mut countnodes = rootlink.clone().borrow_mut().count_nodes();
    println!("count nodes {}", countnodes);
    
    //Call count_nodes_by_nodelink function, supplied right subtree as parameter
    countnodes = Node::count_nodes_by_nodelink(&rightsub);
    println!("count nodes by nodelink {}", countnodes);
    
    //Get the sibling of the leftsubtree from parent
    let sibling =Node::get_sibling(&leftsub);
    println!("sibling of leftsubtree is {}", sibling.unwrap().borrow().value);
    
    //get the left subtree by value
    rootlink.clone().borrow_mut().get_node_by_value(48);
    
    //get the left subtree by full properties
    rootlink.clone().borrow_mut().get_node_by_full_property(&leftsub);
    
    //Discard the right subtree from parent
    main_tree_path = "prime_t3";
    generate_dotfile(&rootlink, main_tree_path);
    rootlink.clone().borrow_mut().discard_node_by_value(888);
    
    //print the tree again
    
    //Call tree depth function at this time
    let treedepth = rootlink.clone().borrow_mut().tree_depth();
    println!("tree depth {}", treedepth);

    //Call count_nodes function 
    countnodes = rootlink.clone().borrow_mut().count_nodes();
    println!("count nodes {}", countnodes);

    //print the tree again
    main_tree_path = "prime_t4";
    generate_dotfile(&rootlink, main_tree_path);
}

