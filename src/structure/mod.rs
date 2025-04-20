pub mod tree {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    use std::cmp;


    //all core structure mustn't be changed
    pub type NodeLink = Rc<RefCell<Node>>;


    #[derive(Clone)]
    pub struct Node {
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
            let currentlink = Rc::new(RefCell::new(currentnode));
            currentlink
        }
        
        /**
         * Add new left child node from value, parent of the node will be set to current_node_link
         */
        pub fn add_left_child(&mut self, current_node_link: &NodeLink, value: i32) {
            if self.left.is_none() {
                let new_node = Node::new_with_parent(current_node_link, value);
                self.left = Some(new_node);
            } else {
                self.left.as_ref().unwrap().borrow_mut().add_left_child(current_node_link, value);
            }
        }

        /**
         * Add new right child node from value, parent of the node will be set to current_node_link
         */
        pub fn add_right_child(&mut self, current_node_link: &NodeLink, value: i32) {
            if self.right.is_none() {
                let new_node = Node::new_with_parent(current_node_link, value);
                self.right = Some(new_node);
            } else {
                self.right.as_ref().unwrap().borrow_mut().add_left_child(current_node_link, value);
            }
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

            if let Some(left) = &self.left {
                if left.borrow().value == value {
                    return Some(left.clone());
                } else if let leftval = left.borrow().get_node_by_value(value) {
                    if leftval.is_some() {
                        return leftval;
                    }
                }
            }
            
            if let Some(right) = &self.right {
                if right.borrow().value == value {
                    return Some(right.clone());
                } else if let rightval = right.borrow().get_node_by_value(value) {
                    if rightval.is_some() {
                        return rightval;
                    }
                }
            }

            None
        }

        /**
         * This function will return the node that matches all Nodelink Properties: 1). current node value, 2). node parent value, 3). both child values
         * Let's assume the tree won't have any value duplicates
         */
        pub fn get_node_by_full_property(&self, node: &NodeLink) -> Option<NodeLink> {
            let selfnode = self.clone();
            let nodelink = Rc::<RefCell<Node>>::downgrade(&selfnode.get_node_link());
            let strongnode = nodelink.upgrade();
            
            if Rc::ptr_eq(&Rc::new(RefCell::new(self.clone())), node) {
                return strongnode;
            }
            
            if let Some(left) = &self.left {
                if Rc::ptr_eq(&left.clone(), node) {
                    return Some(left.clone());
                } else if let leftval = left.borrow().get_node_by_full_property(node) {
                    if leftval.is_some() {
                        return leftval;
                    }
                }
            }
            
            if let Some(right) = &self.right {
                if Rc::ptr_eq(&right.clone(), node) {
                    return Some(right.clone());
                } else if let rightval = right.borrow().get_node_by_full_property(node) {
                    if rightval.is_some() {
                        return rightval;
                    }
                }
            }

            None
        }

        /**
         * This function will discard a node that match the value, the whole node tree that match the description will be discarded
         * Along with its child
         */
        pub fn discard_node_by_value(&mut self, value: i32) -> bool {
            if self.value == value {
                self.value = -1;
                self.left = None;
                self.right = None;
                if let Some(parent_weak) = &self.parent {
                    if let Some(parent) = parent_weak.upgrade() {
                        let mut parent_mut = parent.borrow_mut();
                        match (&parent_mut.left, &parent_mut.right) {
                            (Some(left), Some(right)) => {
                                if left.borrow().value == value {
                                    parent_mut.left = None;
                                } else if right.borrow().value == value {
                                    parent_mut.right = None;
                                }
                            },
                            (Some(left), None) => {
                                parent_mut.left = None;
                            },
                            (None, Some(right)) => {
                                parent_mut.right = None;
                            },
                            (None, None) => {}
                        }
                    }
                }
                self.parent = None;
                return true;
            }

            if let Some(left) = &self.left {
                if left.borrow().value == value {
                    self.left = None;
                    return true;
                }
            }
            
            if let Some(right) = &self.right {
                if right.borrow().value == value {
                    self.right = None;
                    return true;
                }
            }

            if let Some(left) = &mut self.left {
                if left.borrow_mut().discard_node_by_value(value) {
                    return true;
                }
            }

            if let Some(right) = &mut self.right {
                if right.borrow_mut().discard_node_by_value(value) {
                    return true;
                }
            }

            false
        }

        /**
         * Count the amount of nodes in the whole subtree, in the current node
         */
        pub fn count_nodes(&self) -> i32 {
            let mut count = 0;
            if let selfnode = Some(self.clone()) {
                count += 1;
                match (&self.left, &self.right) {
                    (Some(left), Some(right)) => {
                        count += left.borrow().count_nodes() + right.borrow().count_nodes();
                    },
                    (Some(left), None) => {
                        count += left.borrow().count_nodes();
                    },
                    (None, Some(right)) => {
                        count += right.borrow().count_nodes();
                    },
                    (None, None) => {}
                }
            }
            count
        }

        //the same as above except start the count from nodelink reference parameter
        pub fn count_nodes_by_nodelink(node: &NodeLink) -> i32 {
            let mut count = 0;
            if let selfnode = Some(node.clone()) {
                count += 1;
                match (&node.borrow().left, &node.borrow().right) {
                    (Some(left), Some(right)) => {
                        count += left.borrow().count_nodes() + right.borrow().count_nodes();
                    },
                    (Some(left), None) => {
                        count += left.borrow().count_nodes();
                    },
                    (None, Some(right)) => {
                        count += right.borrow().count_nodes();
                    },
                    (None, None) => {}
                }
            }
            count
        }

        //Count depth of the tree in the current node
        pub fn tree_depth(&self) -> i32 {
            let mut height = 0;
            if let selfnode = Some(self.clone()) {
                match (&self.left, &self.right) {
                    (Some(left), Some(right)) => {
                        height = cmp::max(left.borrow().tree_depth(), right.borrow().tree_depth()) + 1;
                    },
                    (Some(left), None) => {
                        height = left.borrow().tree_depth() + 1;
                    },
                    (None, Some(right)) => {
                        height = right.borrow().tree_depth() + 1;
                    },
                    (None, None) => {
                        height = 1;
                    }
                }
            }
            height
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
                        Some(right.clone())
                    } else if Rc::ptr_eq(right, nodelink) {
                        Some(left.clone())
                    } else {
                        None
                    }
                },
                _ => None
            }
        }
    }
}
