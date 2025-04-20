use crate::structure::tree::NodeLink;
use std::fs::File;
use std::io::Write;

/**
 * @root: root node of the tree in NodeLink Type
 * @output_path: write the graphviz structure to output_path
 * Generate graphviz dot file given a NodeLink, you will traverse from root to all leaves incrementally, 
 * as you proceed wrote the progress to dot file
 */
pub fn generate_dotfile(root: &NodeLink, output_path: &str){
    let mut string = String::from("Tree Node {\n");

    let mut rootvec = vec![root.clone()];
    while let Some(node) = rootvec.pop() {
        let refnode = node.borrow();
        let label = refnode.value;

        if let Some(left) = &refnode.left {
            let left_val = left.borrow().value;
            string.push_str(&format!("    P{} -> L{};\n", label, left_val));
            rootvec.push(left.clone());
        }

        if let Some(right) = &refnode.right {
            let right_val = right.borrow().value;
            string.push_str(&format!("    P{} -> R{};\n", label, right_val));
            rootvec.push(right.clone());
        }
    }

    string.push_str("}\n");

    if let Ok(mut file) = File::create(output_path) {
        let _ = file.write_all(string.as_bytes());
    }
}

/**
 * Similar to above, but instead of store the graph in a dot file, print the graph directly to terminal in graphviz notation
 */
pub fn print_graph(root: &NodeLink){
    let mut string = String::from("Tree Node {\n");

    let mut rootvec = vec![root.clone()];
    while let Some(node) = rootvec.pop() {
        let refnode = node.borrow();
        let label = refnode.value;

        if let Some(left) = &refnode.left {
            let left_val = left.borrow().value;
            string.push_str(&format!("    P{} -> L{};\n", label, left_val));
            rootvec.push(left.clone());
        }

        if let Some(right) = &refnode.right {
            let right_val = right.borrow().value;
            string.push_str(&format!("    P{} -> R{};\n", label, right_val));
            rootvec.push(right.clone());
        }
    }

    string.push_str("}\n");

    println!("{}", string);
}