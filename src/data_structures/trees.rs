use std::cell::{RefCell, Ref};
use std::rc::Rc;

pub struct TreeNode<T> {
    pub val: T,
    children: Vec<Rc<RefCell<TreeNode<T>>>>
}

impl<T> TreeNode<T> {

    pub fn new(val:T) -> Self {
        TreeNode{val: val, children: Vec::new()}
    }
    pub fn next(&self, index: usize) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if index >= self.children.len() {
            None
        } else {
            Some(self.children[index].clone())
        }
    }

    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    pub fn add_child(&mut self, val: T) {
        self.children.push(Rc::new(RefCell::new(TreeNode::new(val))));
    }
}

pub struct Tree<T> {
    head: Rc<RefCell<TreeNode<T>>>,
    cursor: Rc<RefCell<TreeNode<T>>>,
    depth: usize
}

impl<T> Tree<T> {
    pub fn new(value: T) -> Self {
        let head = Rc::new(RefCell::new(TreeNode::new(value)));
        Tree{head: head.clone(), cursor: head, depth: 0}
    }
    //go to next node
    pub fn next(&mut self, index: usize) {
        let tmp = self.cursor.borrow().next(index);
        match tmp {
            Some(x) => self.cursor = x,
            None => (),
        }
        self.depth += 1;
    }
    //reset cursor to head
    pub fn reset(&mut self) {
        self.cursor = self.head.clone();
        self.depth = 0;
    }
    //return amount of children of headnode
    pub fn child_count(&self) -> usize {
        return self.cursor.borrow().child_count();
    }
    // push another child to the head node
    pub fn add_child(&mut self, value: T) {
        self.cursor.borrow_mut().add_child(value);
    }
    // get cursor value ref
    pub fn get(&self) -> Ref<TreeNode<T>> {
        self.cursor.borrow()
    }
}