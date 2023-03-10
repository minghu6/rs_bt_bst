
//! Normal BST except splayed recent accessed node.
//!

use std::ptr::{null, null_mut};

use super::*;
use crate::*;


pub struct Splay<K, V> {
    root: *mut SplayNode<K, V>,
}

pub struct SplayNode<K, V> {
    paren: *mut Self,
    left: *mut Self,
    right: *mut Self,
    key: *mut K,
    value: *mut V,
}



////////////////////////////////////////////////////////////////////////////////
//// Implement

impl<'a, K: CollKey + 'a, V: 'a> SplayNode<K, V> {
    pub fn new(key: K, value: V) -> *mut Self {
        Box::into_raw(box Self {
            left: null_mut(),
            right: null_mut(),
            paren: null_mut(),
            key: Box::into_raw(box key),
            value: Box::into_raw(box value),
        })
    }

    pub fn into_value(self) -> V {
        unsafe { *Box::from_raw(self.value) }
    }
}


impl<'a, K: CollKey + 'a, V: 'a> BTNode<'a, K, V> for SplayNode<K, V> {
    fn itself(&self) -> *const (dyn BTNode<'a, K, V> + 'a) {
        self as *const Self
    }

    fn null(&self) -> *const (dyn BTNode<'a, K, V> + 'a) {
        null::<Self>()
    }

    fn try_as_bst(&self) -> Result<*const (dyn BSTNode<'a, K, V> + 'a), ()> {
        Ok(self as *const Self)
    }

    fn order(&self) -> usize {
        2
    }

    fn child(&self, idx: usize) -> *mut (dyn BTNode<'a, K, V> + 'a) {
        if idx == 0 {
            self.left
        } else {
            self.right
        }
    }

    fn assign_child(
        &mut self,
        child: *mut (dyn BTNode<'a, K, V> + 'a),
        idx: usize,
    ) {
        match idx {
            0 => {
                self.left = child as *mut Self;
            }
            1 => {
                self.right = child as *mut Self;
            }
            _ => unreachable!(),
        }
    }

    fn assign_value(&mut self, value: V, _idx: usize) {
        self.value = Box::into_raw(box value);
    }

    fn assign_paren(&mut self, paren: *mut (dyn BTNode<'a, K, V> + 'a)) {
        self.paren = paren as *mut Self;
    }

    fn paren(&self) -> *mut (dyn BTNode<'a, K, V> + 'a) {
        self.paren as *mut (dyn BTNode<K, V> + 'a)
    }

    fn key(&self, idx: usize) -> Option<&K> {
        if idx == 0 {
            unsafe { Some(&*self.key) }
        } else {
            None
        }
    }

    fn key_ptr(&self, idx: usize) -> *mut K {
        if idx == 0 {
            self.key
        } else {
            null_mut::<K>()
        }
    }

    fn assign_key_ptr(&mut self, idx: usize, key_ptr: *mut K) {
        if idx == 0 {
            self.key = key_ptr;
        }
    }

    fn val_ptr(&self, idx: usize) -> *mut V {
        if idx == 0 {
            self.value
        } else {
            null_mut::<V>()
        }
    }

    fn assign_val_ptr(&mut self, idx: usize, val_ptr: *mut V) {
        if idx == 0 {
            self.value = val_ptr;
        }
    }

    fn height(&self) -> i32 {
        self.calc_height()
    }
}

impl<'a, K: CollKey + 'a, V: 'a> BSTNode<'a, K, V> for SplayNode<K, V> {}


impl<'a, K: CollKey + 'a, V: 'a> Splay<K, V> {
    pub fn new() -> Self {
        Self { root: null_mut() }
    }

    /// Rotate to root
    unsafe fn splay(&mut self, mut x: *mut SplayNode<K, V>) {
        while !(*x).paren.is_null() {
            let x_dir = (*x).dir();

            x = self.rotate((*x).paren, x_dir.reverse()) as *mut SplayNode<K, V>;
        }

    }

}

impl<'a, K: CollKey + 'a, V: 'a> Dictionary<K, V> for Splay<K, V> {
    fn insert(&mut self, key: K, value: V) -> bool {
        let new_node = SplayNode::new(key, value);

        unsafe {
            let key = BSTNode::key_bst(&*new_node);
            let approxi_node =
                (*self.search_approximately(&key)).try_as_bst_mut().unwrap();

            if !approxi_node.is_null() && BSTNode::key_bst(&*approxi_node) == key {
                return false;
            }

            // duplcate code for there is no guanrantee on Clone
            if approxi_node.is_null() {
                (*new_node).assign_paren(approxi_node);

                self.assign_root(new_node)
            } else if key < BSTNode::key_bst(&*approxi_node) {
                (*approxi_node).connect_left(new_node)
            } else {
                (*approxi_node).connect_right(new_node)
            }

            self.splay(new_node);

            true
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        unsafe {
            let approxi_node =
                (*self.search_approximately(&key)).try_as_bst_mut().unwrap()
                as *mut SplayNode<K, V>;

            if approxi_node.is_null() {
                return None;
            }

            if BSTNode::key_bst(&*approxi_node) != key {
                return None;
            }

            self.splay(approxi_node);

            if (*approxi_node).left().is_null() {
                self.subtree_shift(approxi_node, (*approxi_node).right())
            } else if (*approxi_node).right().is_null() {
                self.subtree_shift(approxi_node, (*approxi_node).left())
            } else {
                let y = BSTNode::successor_bst(&*approxi_node);
                // y should be leaf.

                if (*y).paren_bst() != approxi_node {
                    self.subtree_shift(y, (*y).right());
                    (*y).assign_right((*approxi_node).right());
                    (*(*y).right()).assign_paren(y);
                }
                self.subtree_shift(approxi_node, y);
                (*y).assign_left((*approxi_node).left());
                (*(*y).left()).assign_paren(y);
            }

            Some(Box::from_raw(approxi_node).into_value())
        }
    }

    fn modify(&mut self, key: &K, value: V) -> bool {
        unsafe {
            let app_node
            = (*self.search_approximately(key))
            .try_as_bst_mut().unwrap() as *mut SplayNode<K, V>;

            if app_node.is_null() {
                false
            } else {
                (*app_node).assign_value(value, 0);
                self.splay(app_node);

                true
            }
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        unsafe {
            let res = self.search_approximately(key) as *mut SplayNode<K, V>;

            if res.is_null() || (*res).key_bst() != key {
                None
            } else {
                let self_mut = &mut *(self as *const Splay<K, V> as *mut Splay<K, V>);

                self_mut.splay(res);
                Some(&*(*res).value)
            }
        }
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        unsafe {
            let res = self.search_approximately(key) as *mut SplayNode<K, V>;

            if res.is_null() || (*res).key_bst() != key {
                None
            } else {
                self.splay(res);
                Some(&mut *(*res).value)
            }
        }
    }

    fn self_validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.basic_self_validate()
    }
}



impl<'a, K: CollKey + 'a, V: 'a> BT<'a, K, V> for Splay<K, V> {
    fn order(&self) -> usize {
        2
    }

    fn root(&self) -> *mut (dyn BTNode<'a, K, V> + 'a) {
        self.root
    }

    fn assign_root(&mut self, root: *mut (dyn BTNode<'a, K, V> + 'a)) {
        self.root = root as *mut SplayNode<K, V>;
    }
}


impl<'a, K: CollKey + 'a, V: 'a> BST<'a, K, V> for Splay<K, V> {
    unsafe fn rotate_cleanup(
        &mut self,
        _x: *mut (dyn BSTNode<'a, K, V> + 'a),
        _z: *mut (dyn BSTNode<'a, K, V> + 'a),
    ) {}
}



#[cfg(test)]
pub(crate) mod tests {

    use crate::*;
    use super::*;


    #[test]
    pub(crate) fn test_splay_randomdata() {
        test_dict!(Splay::new());
    }


    #[test]
    fn test_splay_fixeddata_case_1() {
        let mut splay = Splay::<i32, ()>::new();

        let dict = &mut splay as &mut dyn Dictionary<i32, ()>;

        dict.insert(71, ());
        dict.insert(13, ());

        dict.remove(&71);
        assert!(dict.get(&71).is_none());

        BST::just_echo_stdout(&splay);
    }

}
