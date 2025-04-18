use crate::structure::tree::NodeLink;

/**
 * @root: root node of the tree in NodeLink Type
 * @output_path: write the graphviz structure to output_path
 * Generate graphviz dot file given a NodeLink, you will traverse from root to all leaves incrementally, 
 * as you proceed wrote the progress to dot file
 */
pub fn generate_dotfile(root: &NodeLink, output_path: &str){
    //TODO
}

/**
 * Similar to above, but instead of store the graph in a dot file, print the graph directly to terminal in graphviz notation
 */
pub fn print_graph(root: &NodeLink){
    //TODO
}