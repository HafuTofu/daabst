pub mod tree {
    use std::cell::RefCell;
    use std::ops::Add;
    use std::rc::{Rc, Weak};


    //all core structure mustn't be changed
    pub type NodeLink = Rc<RefCell<Node>>;

    //static t : Option<&NodeLink> = None;

    #[derive(Clone)]
    pub struct Node {
        pub leaves: i32,
        pub value: i32,
        pub parent: Option<Weak<RefCell<Node>>>,
        pub left: Option<NodeLink>,
        pub right: Option<NodeLink>,
    }

    //All existing function interface mustn't be changed
    //but you allowed to add new one
    impl Node {
        //private interface
        fn new(value: i32) -> Self {
            Node {
                leaves: 1,
                value: value,
                left: None,
                right: None,
                parent: None,
            }
        }

        /**
         * Generate new nodelink from a value
         */
        pub fn new_nodelink(value: i32) -> NodeLink {
            let currentnode = Node::new(value);
            let currentlink = Rc::new(RefCell::new(currentnode));
            currentlink
        }

        /**
         * Consumptive, this function can only be called once for the whole lifetime
         */
        fn get_node_link(self) -> NodeLink {
            Rc::new(RefCell::new(self))
        }

        //private interface
        fn new_with_parent(parent: &NodeLink, value: i32) -> NodeLink {
            let mut currentnode = Node::new(value);
            currentnode.add_parent(Rc::<RefCell<Node>>::downgrade(parent));
            Node::add_parents_leaves(&Rc::<RefCell<Node>>::downgrade(parent));
            let currentlink = Rc::new(RefCell::new(currentnode));
            currentlink
        }

        fn add_parents_leaves(parent: &Weak<RefCell<Node>>) {
            match parent.upgrade().as_ref() {
                higher_parent => {
                    parent.upgrade().as_ref().unwrap().borrow_mut().leaves += 1;
                    Node::add_parents_leaves(higher_parent.unwrap().borrow_mut().parent.as_ref().unwrap());
                }, 
                None => {
                    parent.upgrade().as_ref().unwrap().borrow_mut().leaves += 1;
                }
            }
        }

        /**
         * Add new left child node from value, parent of the node will be set to current_node_link
         */
        pub fn add_left_child(&mut self, current_node_link: &NodeLink, value: i32) {
            let new_node = Node::new_with_parent(current_node_link, value);
            current_node_link.borrow_mut().left = Some(new_node);
            Node::add_parents_leaves(&Rc::<RefCell<Node>>::downgrade(current_node_link));
        }

        /**
         * Add new right child node from value, parent of the node will be set to current_node_link
         */
        pub fn add_right_child(&mut self, current_node_link: &NodeLink, value: i32) {
            let new_node = Node::new_with_parent(current_node_link, value);
            current_node_link.borrow_mut().right = Some(new_node);
            Node::add_parents_leaves(&Rc::<RefCell<Node>>::downgrade(current_node_link));
        }

        fn add_parent(&mut self, node: Weak<RefCell<Node>>) {
            self.parent = Some(node);
        }

        /**
         * This function will return the node that match value
         * Let's assume the tree won't have any value duplicates
         */
        pub fn get_node_by_value(&self, value: i32) -> Option<NodeLink> {
            let node = self.clone();
            let nodelink = Rc::<RefCell<Node>>::downgrade(&node.get_node_link());
            let strongnode = nodelink.upgrade();

            

            if self.value == value {
                return strongnode;
            }
            

            None
        }

        /**
         * This function will return the node that matches all Nodelink Properties: 1). current node value, 2). node parent value, 3). both child values
         * Let's assume the tree won't have any value duplicates
         */
        pub fn get_node_by_full_property(&self, node: &NodeLink) -> Option<NodeLink> {
            //TODO
            None
        }

        /**
         * This function will discard a node that match the value, the whole node tree that match the description will be discarded
         * Along with its child
         */
        pub fn discard_node_by_value(&mut self, value: i32) -> bool {
            //TODO
            false
        }

        /**
         * Count the amount of nodes in the whole subtree, in the current node
         */
        pub fn count_nodes(&self) -> i32 {
            self.leaves
        }

        //the same as above except start the count from nodelink reference parameter
        pub fn count_nodes_by_nodelink(node: &NodeLink) -> i32 {
            node.borrow().leaves
        }

        //Count depth of the tree in the current node
        pub fn tree_depth(&self) -> i32 {
            //TODO
            -1
        }

        /**
         * a node is guaranteed to have two childs at most, since this is a binary tree
         * a sibling is a node which has same direct parent
         */
        pub fn get_sibling(nodelink: &NodeLink) -> Option<NodeLink> {
            let parent_link = nodelink.borrow().parent.as_ref()?.upgrade()?;
            let parent = parent_link.borrow();
            
            match (&parent.left, &parent.right) {
                (Some(left), Some(right)) => {
                    if Rc::ptr_eq(left, nodelink) {
                        Some(Rc::clone(right))
                    } else if Rc::ptr_eq(right, nodelink) {
                        Some(Rc::clone(left))
                    } else {
                        None
                    }
                },
                _ => None
            }
        }
    }
}
