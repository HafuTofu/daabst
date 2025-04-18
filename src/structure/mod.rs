use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub mod Tree {
    use std::{
        cell::RefCell,
        rc::{Rc, Weak},
    };

    pub struct BiTree {
        pub size: u32,
        pub root: Option<StrongNode>,
    }

    type StrongNode = Rc<RefCell<Node>>;
    type WeakNode = Weak<RefCell<Node>>;

    #[derive(Debug)]
    pub struct Node {
        pub value: i32,
        pub left: Option<StrongNode>,
        pub right: Option<StrongNode>,
        pub parent: Option<WeakNode>,
    }

    impl Node {
        pub fn new(value: i32) -> Node {
            Node {
                value: value,
                left: None,
                right: None,
                parent: None,
            }
        }

        pub fn getLeftChild(&self) -> Option<StrongNode> {
            return self.left.clone();
        }

        pub fn add_left_child(&mut self, node: Node) {
            self.left = Some(Rc::new(RefCell::new(node)));
        }

        pub fn add_right_child(&mut self, node: Node) {
            self.right = Some(Rc::new(RefCell::new(node)));
        }

        pub fn add_parent_child(&mut self, node: Node) {
            let weaking = Rc::new(RefCell::new(node));
            self.parent = Some(Rc::<RefCell<Node>>::downgrade(&weaking));
        }
    }
}
