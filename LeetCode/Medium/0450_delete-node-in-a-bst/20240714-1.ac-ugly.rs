use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        if root.as_ref().unwrap().borrow().left.is_none()
            && root.as_ref().unwrap().borrow().right.is_none()
            && root.as_ref().unwrap().borrow().val == key
        {
            return None;
        }

        let mut dummy = Rc::new(RefCell::new(TreeNode::new(0)));
        dummy.borrow_mut().left = root.as_ref().map(|t| Rc::clone(t));
        dummy.borrow_mut().right = root.as_ref().map(|t| Rc::clone(t));

        let mut ptr_parent: Option<Rc<RefCell<TreeNode>>> = Some(Rc::clone(&dummy));
        let mut ptr = Rc::clone(root.as_ref().unwrap());

        loop {
            if key == ptr.borrow().val {
                if ptr.borrow().left.is_some() {
                    let ptr_left = Rc::clone(ptr.borrow().left.as_ref().unwrap());
                    if ptr_parent.is_some() {
                        if key < ptr_parent.as_ref().unwrap().borrow().val {
                            ptr_parent.as_mut().unwrap().borrow_mut().left =
                                Some(Rc::clone(&ptr_left));
                        } else {
                            ptr_parent.as_mut().unwrap().borrow_mut().right =
                                Some(Rc::clone(&ptr_left));
                        }
                    }

                    if ptr.borrow().right.is_some() {
                        let ptr_right = Rc::clone(ptr.borrow().right.as_ref().unwrap());
                        if ptr_left.borrow().right.is_some() {
                            let ptr_left_right =
                                Rc::clone(ptr_left.borrow().right.as_ref().unwrap());
                            ptr_left.borrow_mut().right = Some(Rc::clone(&ptr_right));

                            let mut ptr_tmp = Rc::clone(&ptr_right);
                            loop {
                                if ptr_tmp.borrow().left.is_none() {
                                    break;
                                }

                                let z = ptr_tmp
                                    .borrow()
                                    .left
                                    .as_ref()
                                    .map(|t| Rc::clone(t))
                                    .unwrap();
                                ptr_tmp = z;
                            }
                            ptr_tmp.borrow_mut().left = Some(ptr_left_right);
                        } else {
                            ptr_left.borrow_mut().right = Some(ptr_right);
                        }
                    }
                } else if ptr.borrow().right.is_some() {
                    let ptr_right = Rc::clone(ptr.borrow().right.as_ref().unwrap());
                    if ptr_parent.is_some() {
                        if key < ptr_parent.as_ref().unwrap().borrow().val {
                            ptr_parent.as_mut().unwrap().borrow_mut().left =
                                Some(Rc::clone(&ptr_right));
                        } else {
                            ptr_parent.as_mut().unwrap().borrow_mut().right =
                                Some(Rc::clone(&ptr_right));
                        }
                    } else {
                    }
                } else {
                    if ptr_parent.is_some() {
                        if key < ptr_parent.as_ref().unwrap().borrow().val {
                            ptr_parent.as_mut().unwrap().borrow_mut().left = None;
                        } else {
                            ptr_parent.as_mut().unwrap().borrow_mut().right = None;
                        }
                    } else {
                        return None;
                    }
                }
                break;
            } else if key > ptr.borrow().val {
                if ptr.borrow().right.is_none() {
                    break;
                }

                ptr_parent = Some(Rc::clone(&ptr));
                let p_tmp = ptr.borrow().right.as_ref().map(|t| Rc::clone(t)).unwrap();
                ptr = p_tmp;
            } else {
                if ptr.borrow().left.is_none() {
                    break;
                }

                ptr_parent = Some(Rc::clone(&ptr));
                let p_tmp = ptr.borrow().left.as_ref().map(|t| Rc::clone(t)).unwrap();
                ptr = p_tmp;
            }
        }

        if dummy.borrow().left.as_ref().unwrap().borrow().val
            == dummy.borrow().right.as_ref().unwrap().borrow().val
        {
            root
        } else {
            return if dummy.borrow().left.as_ref().unwrap().borrow().val
                == root.as_ref().unwrap().borrow().val
            {
                dummy.borrow().right.as_ref().map(|t| Rc::clone(t))
            } else {
                dummy.borrow().left.as_ref().map(|t| Rc::clone(t))
            };
        }
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
