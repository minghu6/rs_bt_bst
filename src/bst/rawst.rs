//! Unbalanced Search Tree
//!

use std::ptr::{null, null_mut};

use super::*;
use crate::*;


pub struct RawST<K, V> {
    root: *mut RawSTNode<K, V>,
}

pub struct RawSTNode<K, V> {
    paren: *mut Self,
    left: *mut Self,
    right: *mut Self,
    key: *mut K,
    value: *mut V,
}



////////////////////////////////////////////////////////////////////////////////
//// Implement

impl<'a, K: CollKey + 'a, V: 'a> RawSTNode<K, V> {
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


impl<'a, K: CollKey + 'a, V: 'a> BTNode<'a, K, V> for RawSTNode<K, V> {
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

impl<'a, K: CollKey + 'a, V: 'a> BSTNode<'a, K, V> for RawSTNode<K, V> {}


impl<'a, K: CollKey + 'a, V: 'a> RawST<K, V> {
    pub fn new() -> Self {
        Self { root: null_mut() }
    }
}

impl<'a, K: CollKey + 'a, V: 'a> Dictionary<K, V> for RawST<K, V> {
    fn insert(&mut self, key: K, value: V) -> bool {
        let new_node = RawSTNode::new(key, value);

        self.basic_insert(new_node)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(node) = self.basic_remove(key) {
            Some(unsafe {
                Box::from_raw(node as *mut RawSTNode<K, V>).into_value()
            })
        } else {
            None
        }
    }

    fn modify(&mut self, key: &K, value: V) -> bool {
        self.basic_modify(key, value)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.basic_lookup(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.basic_lookup_mut(key)
    }

    fn self_validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.basic_self_validate()
    }
}



impl<'a, K: CollKey + 'a, V: 'a> BT<'a, K, V> for RawST<K, V> {
    fn order(&self) -> usize {
        2
    }

    fn root(&self) -> *mut (dyn BTNode<'a, K, V> + 'a) {
        self.root
    }

    fn assign_root(&mut self, root: *mut (dyn BTNode<'a, K, V> + 'a)) {
        self.root = root as *mut RawSTNode<K, V>;
    }
}


impl<'a, K: CollKey + 'a, V: 'a> BST<'a, K, V> for RawST<K, V> {
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
    pub(crate) fn test_rawst_randomdata() {
        test_dict!(RawST::new());
    }

}
