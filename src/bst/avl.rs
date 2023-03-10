//!
//! By two Soviet inventors, Georgy Adelson-Velsky and Evgenii Landis(1962)
//!
//! ref 1: https://en.wikipedia.org/wiki/AVL_tree
//!
//! ref 2: https://en.wikipedia.org/wiki/Binary_search_tree
//!

use std::{
    cmp::max,
    fmt::{self, Write},
    ptr::{null, null_mut},
};

use either::Either;

use super::*;
use crate::*;

////////////////////////////////////////////////////////////////////////////////
//// Struct
////

pub struct AVL<K, V> {
    root: *mut AVLNode<K, V>,
}

struct AVLNode<K, V> {
    left: *mut Self,
    right: *mut Self,
    paren: *mut Self,
    height: i32, // using C style int, as it's default for Rust
    key: *mut K,
    value: *mut V,
}

////////////////////////////////////////////////////////////////////////////////
//// Implement

impl<'a, K: CollKey + 'a, V: 'a> AVLNode<K, V> {
    pub fn new(key: K, value: V) -> *mut Self {
        Box::into_raw(box Self {
            left: null_mut(),
            right: null_mut(),
            paren: null_mut(),
            height: 0,
            key: Box::into_raw(box key),
            value: Box::into_raw(box value),
        })
    }

    pub fn into_value(self) -> V {
        unsafe { *Box::from_raw(self.value) }
    }

    fn bf(&self) -> i32 {
        self.right_height() - self.left_height()
    }

    fn calc_bf(&self) -> i32 {
        self.calc_right_height() - self.calc_left_height()
    }

    pub fn self_validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let bf = self.calc_bf();

        assert!(bf.abs() < 2, "BF: {}", bf);

        Ok(())
    }

    pub fn echo_in_mm(&self, cache: &mut String) -> fmt::Result {
        unsafe {
            BSTNode::echo_in_mm(self, cache, |x, cache| {
                let x_self = x as *mut AVLNode<K, V>;

                let check_res = if (*x_self).calc_bf().abs() >= 2 {
                    "failed"
                } else {
                    "pass"
                };

                writeln!(
                    cache,
                    "BF: {:?}, H(LF): {}, H(RH): {},  {}",
                    (*x_self).calc_bf(),
                    (*x).calc_left_height(),
                    (*x).calc_right_height(),
                    check_res
                )
            })
        }
    }

    pub fn echo_stdout(&self) {
        let mut cache = String::new();

        self.echo_in_mm(&mut cache).unwrap();

        println!("{}", cache);
    }
}

impl<'a, K: CollKey + Clone + 'a, V: Clone + 'a> Clone for AVLNode<K, V> {
    /// Expensive Implements.
    ///
    /// **WARNING: The Field `paren` isn't set!, it should be set manually!**
    fn clone(&self) -> Self {
        unsafe {
            let key = Box::into_raw(box (*self.key).clone());
            let value = Box::into_raw(box (*self.value).clone());
            let height = self.height;

            // let paren = self.paren;
            let paren = null_mut();
            let left = if self.left.is_null() {
                null_mut()
            } else {
                Box::into_raw(box (*self.left).clone())
            };
            let right = if self.right.is_null() {
                null_mut()
            } else {
                Box::into_raw(box (*self.right).clone())
            };

            Self {
                left,
                right,
                paren,
                height,
                key,
                value,
            }
        }
    }
}

impl<'a, K: CollKey + 'a, V: 'a> BTNode<'a, K, V> for AVLNode<K, V> {
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

    fn assign_child(&mut self, child: *mut (dyn BTNode<'a, K, V> + 'a), idx: usize) {
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

    fn height(&self) -> i32 {
        self.height
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
}

impl<'a, K: CollKey + 'a, V: 'a> BSTNode<'a, K, V> for AVLNode<K, V> {}

impl<'a, K: CollKey + 'a, V: 'a> AVL<K, V> {
    pub fn new() -> Self {
        #[cfg(test)]
        {
            unsafe {
                ROTATE_NUM = 0;
            }
        }

        Self { root: null_mut() }
    }

    pub fn echo_stdout(&self) {
        if !self.root.is_null() {
            unsafe { (*self.root).echo_stdout() }
        }
    }

    // unsafe fn insert_retracing(&mut self, new_node: *mut AVLNode<K, V>) {
    //     let mut y = new_node;
    //     let mut z = (*y).paren;

    //     while !z.is_null() {
    //         (*z).height = 1 + max((*z).left_height(), (*z).right_height());

    //         let x = (*z).paren;

    //         if !x.is_null() && (*x).bf().abs() > 1 {
    //             let direction = if z == (*x).right {
    //                 Either::Left(())
    //             } else {
    //                 Either::Right(())
    //             };

    //             if y == BSTNode::child(
    //                 &*z,
    //                 direction.reverse(),
    //             ) as *mut AVLNode<K, V>
    //             {
    //                 self.rotate(x, direction);
    //             } else {
    //                 self.double_rotate(x, direction);
    //             }
    //         }

    //         y = z;
    //         z = x;
    //     }
    // }

    unsafe fn insert_retracing(&mut self, new_node: *mut AVLNode<K, V>) {
        self.remove_retracing(new_node);
    }

    unsafe fn remove_retracing(&mut self, unbalanced_root: *mut AVLNode<K, V>) {
        let mut p = unbalanced_root;

        while !p.is_null() {
            (*p).height = 1 + max((*p).left_height(), (*p).right_height());

            if (*p).bf().abs() > 1 {
                let x = p;

                let direction = if (*x).right_height() > (*x).left_height() {
                    Either::Left(())
                } else {
                    Either::Right(())
                };

                let same_direction_height =
                    (*BSTNode::child(&*x, direction.reverse())).child_height(direction.reverse());

                let reverse_direction_height =
                    (*BSTNode::child(&*x, direction.reverse())).child_height(direction);

                p = if same_direction_height >= reverse_direction_height {
                    self.rotate(x, direction)
                } else {
                    self.double_rotate(x, direction)
                } as *mut AVLNode<K, V>;
            }

            p = (*p).paren;
        }
    }
}

impl<'a, K: CollKey + Clone + 'a, V: Clone + 'a> Clone for AVL<K, V> {
    fn clone(&self) -> Self {
        if self.root.is_null() {
            return Self { root: null_mut() };
        }

        unsafe {
            let root = Box::into_raw(box (*self.root).clone());

            (*root).bfs_do(|x| {
                let x = (*x).try_as_bst_mut().unwrap();

                if !(*x).left().is_null() {
                    (*(*x).left()).assign_paren((*x).itself_mut());
                }

                if !(*x).right().is_null() {
                    (*(*x).right()).assign_paren((*x).itself_mut());
                }
            });

            Self { root }
        }
    }
}

impl<'a, K: CollKey + 'a, V: 'a> Dictionary<K, V> for AVL<K, V> {
    fn insert(&mut self, key: K, value: V) -> bool {
        let new_node = AVLNode::new(key, value);

        if !self.basic_insert(new_node) {
            return false;
        }

        // self.echo_stdout();

        unsafe {
            self.insert_retracing(new_node);
        }

        true
    }

    ///
    /// case-3
    ///       z
    ///      / \
    ///         y
    ///        / \
    ///     null  x
    ///          / \
    ///
    fn remove(&mut self, key: &K) -> Option<V> {
        let z = self.search_approximately(&key) as *mut AVLNode<K, V>;
        if z.is_null() {
            return None;
        }

        unsafe {
            if BSTNode::key_bst(&*z) != key {
                return None;
            }

            let retracing_entry;
            if (*z).left().is_null() {
                retracing_entry = (*z).paren;
                self.subtree_shift(z, (*z).right());
            } else if (*z).right().is_null() {
                retracing_entry = (*z).paren;
                self.subtree_shift(z, (*z).left());
            } else {
                let y = BSTNode::successor_bst(&*z);
                retracing_entry = if (*y).paren() != z {
                    (*y).paren_bst()
                } else {
                    y
                } as *mut AVLNode<K, V>;

                if (*y).paren() != z {
                    self.subtree_shift(y, (*y).right());

                    (*y).assign_right((*z).right());
                    (*(*y).right()).assign_paren(y);
                }

                self.subtree_shift(z, y);
                (*y).assign_left((*z).left());
                (*(*y).left()).assign_paren(y);
            }
            self.remove_retracing(retracing_entry);

            let origin_node = Box::from_raw(z as *mut AVLNode<K, V>);

            Some(origin_node.into_value())
        }
    }

    fn modify(&mut self, key: &K, value: V) -> bool {
        self.basic_modify(key, value)
    }

    fn get(&self, income_key: &K) -> Option<&V> {
        self.basic_lookup(income_key)
    }

    fn get_mut(&mut self, income_key: &K) -> Option<&mut V> {
        self.basic_lookup_mut(income_key)
    }

    fn self_validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.basic_self_validate()?;

        if !self.root.is_null() {
            unsafe { (*self.root).self_validate()? }
        }

        Ok(())
    }
}

impl<'a, K: CollKey + 'a, V: 'a> BT<'a, K, V> for AVL<K, V> {
    fn order(&self) -> usize {
        2
    }

    fn root(&self) -> *mut (dyn BTNode<'a, K, V> + 'a) {
        self.root
    }

    fn assign_root(&mut self, root: *mut (dyn BTNode<'a, K, V> + 'a)) {
        self.root = root as *mut AVLNode<K, V>;
    }
}

impl<'a, K: CollKey + 'a, V: 'a> BST<'a, K, V> for AVL<K, V> {
    unsafe fn rotate_cleanup(
        &mut self,
        x: *mut (dyn BSTNode<'a, K, V> + 'a),
        z: *mut (dyn BSTNode<'a, K, V> + 'a),
    ) {
        (*(x as *mut AVLNode<K, V>)).height = 1 + max((*x).left_height(), (*x).right_height());
        (*(z as *mut AVLNode<K, V>)).height = 1 + max((*z).left_height(), (*z).right_height());
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use crate::*;
    use super::*;

    #[test]
    pub(crate) fn test_avl_randomdata() {
        test_dict!(AVL::new());

        println!("rotate numer: {}", unsafe { ROTATE_NUM })
    }

    #[test]
    fn test_avl_fixeddata_case_0() {
        let mut avl = AVL::<i32, ()>::new();

        let dict = &mut avl as &mut dyn Dictionary<i32, ()>;

        dict.insert(10, ());
        assert!(dict.self_validate().is_ok());

        dict.insert(5, ());
        dict.self_validate().unwrap();

        dict.insert(12, ());
        dict.self_validate().unwrap();

        dict.insert(13, ());
        dict.self_validate().unwrap();

        dict.insert(14, ());
        dict.self_validate().unwrap();

        dict.insert(18, ());
        dict.self_validate().unwrap();

        dict.insert(7, ());
        dict.self_validate().unwrap();

        dict.insert(9, ());
        dict.self_validate().unwrap();

        dict.insert(11, ());
        dict.self_validate().unwrap();

        dict.insert(22, ());
        dict.self_validate().unwrap();

        assert!(dict.get(&10).is_some());
        assert!(dict.get(&5).is_some());
        assert!(dict.get(&12).is_some());
        assert!(dict.get(&13).is_some());
        assert!(dict.get(&14).is_some());
        assert!(dict.get(&18).is_some());
        assert!(dict.get(&7).is_some());
        assert!(dict.get(&9).is_some());
        assert!(dict.get(&11).is_some());
        assert!(dict.get(&22).is_some());

        assert!(dict.remove(&10).is_some());
        assert!(dict.get(&10).is_none());
        dict.self_validate().unwrap();

        assert!(dict.remove(&5).is_some());
        assert!(dict.get(&5).is_none());
        dict.self_validate().unwrap();

        assert!(dict.remove(&12).is_some());
        dict.self_validate().unwrap();

        assert!(dict.remove(&13).is_some());
        dict.self_validate().unwrap();

        assert!(dict.remove(&14).is_some());
        dict.self_validate().unwrap();

        assert!(dict.remove(&18).is_some());
        dict.self_validate().unwrap();

        assert!(dict.remove(&7).is_some());
        dict.self_validate().unwrap();

        assert!(dict.remove(&9).is_some());
        dict.self_validate().unwrap();

        assert!(dict.remove(&11).is_some());
        dict.self_validate().unwrap();

        assert!(dict.remove(&22).is_some());

        avl.self_validate().unwrap();
        avl.echo_stdout();
    }

    #[test]
    fn test_avl_fixeddata_case_1() {
        let mut avl = AVL::<u16, ()>::new();

        let dict = &mut avl as &mut dyn Dictionary<u16, ()>;

        dict.insert(52, ());
        assert!(dict.get(&52).is_some());

        dict.insert(47, ());
        assert!(dict.get(&47).is_some());

        dict.insert(3, ());
        assert!(dict.get(&3).is_some());

        dict.insert(35, ());
        assert!(dict.get(&35).is_some());

        dict.insert(24, ());
        assert!(dict.get(&24).is_some());

        // avl.echo_stdout();
    }

    #[test]
    fn test_avl_fixeddata_case_2() {
        let mut avl = AVL::<u16, ()>::new();

        let dict = &mut avl as &mut dyn Dictionary<u16, ()>;

        dict.insert(6, ());
        dict.insert(29, ());
        dict.insert(26, ());
        dict.insert(10, ());
        dict.insert(17, ());
        dict.insert(18, ());
        dict.insert(12, ());

        // avl.echo_stdout();
    }
}
