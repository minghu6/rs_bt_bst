#![feature(trait_alias)]
#![feature(box_syntax)]
#![feature(macro_metavar_expr)]
#![feature(is_sorted)]
#![feature(int_roundings)]
#![feature(trait_upcasting)]


//! B-Tree alias as M-ary Tree,
//! Bayer and McCreight never explained what, if anything, the B stands for: Boeing, balanced, broad, bushy, and Bayer have been suggested.
//! McCreight has said that "the more you think about what the B in B-trees means, the better you understand B-trees.
/// According to Knuth's definition, a B-tree of order m is a tree which satisfies the following properties:
/// 1. Every node has at most m children.
/// 1. Every non-leaf node (except root) has at least ⌈m/2⌉ child nodes.
/// 1. The root has at least two children if it is not a leaf node.
/// 1. A non-leaf node with k children contains k − 1 keys.
/// 1. All leaves appear in the same level and carry no information.
/// Here, It's not restrict B-Tree as it save key-value in internal node instead of leaf(nil),
/// we do it just for comparison convenience with other BT impl with Dictionary Trait


use std::{fmt::Debug, fmt::Write, collections::{VecDeque, BinaryHeap}};

use self::bst::{BSTNode, BST};

pub mod bst;
pub mod b3;
pub mod b4;
pub mod bstar4;
mod aux;

pub(crate) use aux::*;

////////////////////////////////////////////////////////////////////////////////
//// Common Trait


/// 1. add a pair to the collection;
/// 2. remove a pair from the collection;
/// 3. modify an existing pair;
/// 4. lookup a value associated with a particular key.
pub trait Dictionary<K: CollKey, V> {
    /// need update or else?
    ///
    /// , return instead of replace to be friendly for BST
    ///
    /// loopup is often cheap moreover
    fn insert(&mut self, key: K, value: V) -> bool;

    /// exist or else
    fn remove(&mut self, key: &K) -> Option<V>;

    /// exist or else
    fn modify(&mut self, key: &K, value: V) -> bool;

    fn get(&self, key: &K) -> Option<&V>;

    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    // check if dict's structure looks like it's expected.
    fn self_validate(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait CollKey = Ord + Debug;


pub trait Coll {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}


pub trait Heap<K: CollKey, T: CollKey>: Coll {
    // max for max-heap and min for min-heap respectly.
    fn top(&self) -> Option<&T>;

    fn pop(&mut self) -> Option<T>;

    fn push(&mut self, key: K, val: T);
}


/// B-Tree (not restrictly, storing info in the internal node)
pub trait BT<'a, K: CollKey + 'a, V: 'a>: Dictionary<K, V> {
    fn order(&self) -> usize;  // >= 2
    fn root(&self) -> *mut (dyn BTNode<'a, K, V> + 'a);
    fn assign_root(&mut self, root: *mut (dyn BTNode<'a, K, V> + 'a));
    fn reset_root(&mut self, root: *mut (dyn BTNode<'a, K, V> + 'a)) {
        unsafe {
            if !root.is_null() {
                (*root).assign_paren((*root).null_mut());
            }
        }

        self.assign_root(root);
    }

    /// alias as transplant
    fn subtree_shift(
        &mut self,
        u: *mut (dyn BTNode<'a, K, V> + 'a),
        v: *mut (dyn BTNode<'a, K, V> + 'a),
    ) {
        unsafe {
            let u_paren = (*u).paren();

            if u_paren.is_null() {
                self.assign_root(v);
            } else {
                let u_idx = (*u_paren).index_of_child(u);
                (*u_paren).assign_child(v, u_idx);
            }

            if !v.is_null() {
                (*v).assign_paren(u_paren)
            }
        }
    }

    // ////////////////////////////////////////////////////////////////////////////
    // //// Introspection
    // fn try_as_bst(&self) -> Result<*const (dyn BST<'a, K, V> + 'a), ()>;
    // fn try_as_bst_mut(&self) -> Result<*mut (dyn BST<'a, K, V> + 'a), ()> {
    //     if let Ok(p) = self.try_as_bst() {
    //         Ok(p as *mut (dyn BST<'a, K, V> + 'a))
    //     } else {
    //         Err(())
    //     }
    // }

    fn root_bst(&self) -> *mut (dyn BSTNode<'a, K, V> + 'a) {
        unsafe { (*self.root()).try_as_bst_mut().unwrap() }
    }

    fn minimum(&self) -> *mut (dyn BTNode<'a, K, V> + 'a) {
        unsafe{
            if self.root().is_null() {
                self.root()
            } else {
                (*self.root()).minimum()
            }
        }
    }


    fn maximum(&self) -> *mut (dyn BTNode<'a, K, V> + 'a) {
        unsafe {
            if self.root().is_null() {
                self.root()
            } else {
                (*self.root()).maximum()
            }
        }
    }


    fn search_approximately(
        &self,
        income_key: &K,
    ) -> *mut (dyn BTNode<'a, K, V> + 'a) {
        if !self.root().is_null() {
            unsafe { (*self.root()).search_approximately(income_key) }
        } else {
            self.root()
        }
    }

    /// BFS Echo
    fn echo_in_mm(
        &self,
        cache: &mut String,
        action: fn(
            *mut (dyn BTNode<'a, K, V> + 'a),
            &mut String,
        ) -> std::fmt::Result,
    ) -> std::fmt::Result {
        if self.root().is_null() {
            writeln!(cache, "ROOT: null")
        } else {
            unsafe {
                writeln!(cache, "ROOT: {:?}", (*self.root()).format_keys())?;

                (*self.root()).echo_in_mm(cache, action)
            }
        }
    }

    // fn bfs_do(
    //     &self,
    //     action: fn(
    //         *mut (dyn BSTNode<'a, K, V> + 'a),
    //     )
    // ) {
    //     if !self.root().is_null() {
    //         unsafe{ (*self.root_bst()).bfs_do(action) }
    //     }

    // }

    fn just_echo_stdout(&self) {
        if !self.root().is_null() {
            unsafe { (*self.root()).just_echo_stdout() }
        } else {
            println!("EMPTY.")
        }
    }

    fn calc_height(&self) -> i32 {
        if self.root().is_null() {
            return -1;
        }

        unsafe { (*self.root()).calc_height() }
    }

    fn height(&self) -> i32 {
        if self.root().is_null() {
            return -1;
        }

        unsafe { (*self.root()).height() }
    }

    fn total(&self) -> usize {
        if self.root().is_null() {
            0
        } else {
            unsafe { (*self.root()).total() }
        }
    }

    fn basic_lookup(
        &self,
        income_key: &K,
    ) -> Option<&V> {
        let res = self.search_approximately(income_key);

        if res.is_null() {
            None
        } else {
            unsafe {
                // println!("{:?}", (*res).format_keys());

                if let Some(idx) = (*res).find_pos_of_key(income_key) {
                    Some(&*(*res).val_ptr(idx))
                } else {
                    None
                }
            }
        }
    }

    fn basic_lookup_mut(
        &mut self,
        income_key: &K,
    ) -> Option<&mut V> {
        let res = self.search_approximately(income_key);

        if res.is_null() {
            None
        } else {
            unsafe {
                // println!("{:?}", (*res).format_keys());

                if let Some(idx) = (*res).find_pos_of_key(income_key) {
                    Some(&mut *(*res).val_ptr(idx))
                } else {
                    None
                }
            }
        }
    }

    fn basic_modify(&mut self, key: &K, value: V) -> bool {
        unsafe {
            let app_node
            = (*self.search_approximately(key)).try_as_bst_mut().unwrap();

            if app_node.is_null() {
                false
            } else if let Some(idx) = (*app_node).find_pos_of_key(key) {
                (*app_node).assign_value(value, idx);
                true
            } else {
                false
            }
        }
    }

    fn bfs_do(
        &self,
        action: fn(
            *mut (dyn BTNode<'a, K, V> + 'a),
        )
    ) {
        if !self.root().is_null() {
            unsafe{ (*self.root()).bfs_do(action) }
        }

    }

    fn basic_self_validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.root().is_null() {
            Ok(())
        } else {
            unsafe {
                (*self.root()).basic_self_validate()
            }
        }
    }

}


/// B-Tree Node
pub trait BTNode<'a, K: CollKey + 'a, V: 'a> {

    ////////////////////////////////////////////////////////////////////////////
    //// Introspection

    fn itself(&self) -> *const (dyn BTNode<'a, K, V> + 'a);
    fn itself_mut(&self) -> *mut (dyn BTNode<'a, K, V> + 'a) {
        self.itself() as *mut (dyn BTNode<'a, K, V> + 'a)
    }
    fn null(&self) -> *const (dyn BTNode<'a, K, V> + 'a);
    fn null_mut(&self) -> *mut (dyn BTNode<'a, K, V> + 'a) {
        self.null() as *mut (dyn BTNode<'a, K, V> + 'a)
    }


    fn try_as_bst(&self) -> Result<*const (dyn BSTNode<'a, K, V> + 'a), ()>;
    fn try_as_bst_mut(&self) -> Result<*mut (dyn BSTNode<'a, K, V> + 'a), ()> {
        if let Ok(p) = self.try_as_bst() {
            Ok(p as *mut (dyn BSTNode<'a, K, V> + 'a))
        } else {
            Err(())
        }
    }
    fn itself_bst(&self) -> *const (dyn BSTNode<'a, K, V> + 'a) {
        self.try_as_bst().unwrap()
    }
    fn itself_bst_mut(&self) -> *mut (dyn BSTNode<'a, K, V> + 'a) {
        self.try_as_bst_mut().unwrap()
    }

    fn order(&self) -> usize;  // >= 2

    /// 0 <= idx <= order, child(order) is temporary case.
    fn child(&self, idx: usize) -> *mut (dyn BTNode<'a, K, V> + 'a);
    fn child_first(&self) -> *mut (dyn BTNode<'a, K, V> + 'a) {
        self.child(0)
    }
    fn child_last(&self) -> *mut (dyn BTNode<'a, K, V> + 'a) {
        self.child(self.order() - 1)
    }

    /// 0 <= idx <= order, child(order) is temporary case.
    fn assign_child(&mut self, child: *mut (dyn BTNode<'a, K, V> + 'a), idx: usize);
    fn assign_value(&mut self, value: V, idx: usize);
    fn assign_paren(&mut self, paren: *mut (dyn BTNode<'a, K, V> + 'a));

    fn paren(&self) -> *mut (dyn BTNode<'a, K, V> + 'a);
    fn paren_bst(&self) -> *mut (dyn BSTNode<'a, K, V> + 'a) {
        unsafe { (*self.paren()).try_as_bst_mut().unwrap() }
    }

    fn key(&self, idx: usize) -> Option<&K> {
        if !self.key_ptr(idx).is_null() {
            Some(unsafe{ &*self.key_ptr(idx) })
        } else {
            None
        }
    }

    fn key_mut(&mut self, idx: usize) -> Option<&mut K> {
        if !self.key_ptr(idx).is_null() {
            Some(unsafe{ &mut *self.key_ptr(idx) })
        } else {
            None
        }
    }
    fn key_ptr(&self, idx: usize) -> *mut K;
    fn assign_key_ptr(&mut self, idx: usize, key_ptr: *mut K);

    fn value(&self, idx: usize) -> Option<&V> {
        if !self.val_ptr(idx).is_null() {
            Some(unsafe{ &*self.val_ptr(idx) })
        } else {
            None
        }
    }

    fn value_mut(&mut self, idx: usize) -> Option<&mut V> {
        if !self.val_ptr(idx).is_null() {
            Some(unsafe{ &mut *self.val_ptr(idx) })
        } else {
            None
        }
    }
    fn val_ptr(&self, idx: usize) -> *mut V;
    fn assign_val_ptr(&mut self, idx: usize, val_ptr: *mut V);

    fn connect_child(&mut self, child: *mut (dyn BTNode<'a, K, V> + 'a), idx: usize) {
        if !child.is_null() {
            unsafe{ (*child).assign_paren(self.itself_mut()) };
        }

        self.assign_child(child, idx);
    }

    // fn right_sibling(&self) -> *mut (dyn BTNode<'a, K, V> + 'a) {
    //     let paren = (*self).paren();

    //     if paren.is_null() {
    //         return paren;
    //     }

    //     unsafe {
    //         let idx = (*paren).index_of_child(self.itself_mut());

    //         (*paren).child(idx + 1)
    //     }

    // }


    // fn left_sibling(&self) -> *mut (dyn BTNode<'a, K, V> + 'a) {
    //     let paren = (*self).paren();

    //     if paren.is_null() {
    //         return paren;
    //     }

    //     unsafe {
    //         let idx = (*paren).index_of_child(self.itself_mut());

    //         if idx > 0 {
    //             (*paren).child(idx - 1)
    //         } else {
    //             self.null_mut()
    //         }
    //     }

    // }


    fn index_of_child(&self, child: *mut (dyn BTNode<'a, K, V> + 'a)) -> usize {
        for i in 0..self.order() {
            // as *const () just to ignore the vtable variant from the fat pointer
            if self.child(i) as *const () == child as *const () {
                return i;
            }

        }

        unreachable!()
    }

    /// key must in it!!
    fn index_of_key(&self, key: &K) -> usize {
        for i in 0..self.order() {
            if self.key(i).unwrap() == key {
                return i;
            }

        }

        unreachable!()
    }

    fn find_pos_of_key(&self, key: &K) -> Option<usize> {
        for i in 0..self.order() {
            if let Some(here_key) = self.key(i) {
                if here_key == key {
                    return Some(i);
                }
            }
        }

        None
    }

    /// If this node contains key (exclude the subtree)
    #[inline]
    fn node_contains(&self, key: &K) -> bool {
        for i in 0..self.order() {
            let key_opt = self.key(i);
            if key_opt.is_some() && key_opt.unwrap() == key {
                return true;
            }
        }

        false
    }

    fn node_last_key(&self) -> &K {
        self.key(self.key_num() - 1).unwrap()
    }

    fn node_first_key(&self) -> &K {
        self.key(0).unwrap()
    }

    /// How many key-values does this node contains?
    fn node_size(&self) -> usize {
        self.key_num()
    }

    fn key_num(&self) -> usize {
        for i in 0..self.order() {
            if self.key(i).is_none() {
                return i;  // i must be greater than one in this case.
            }
        }

        self.order()
    }

    fn key_iter(&'a self) -> Box<dyn Iterator<Item=&K> + 'a> {
        let mut i = -1i32;

        box std::iter::from_fn(move || -> Option<&K> {
            i += 1;
            self.key(i as usize)
        })
    }

    fn val_num(&self) -> usize {
        for i in 0..self.order() {
            if self.value(i).is_none() {
                return i;  // i must be greater than one in this case.
            }
        }

        self.order()
    }

    fn children_num(&self) -> usize {
        for i in 0..self.order() + 1 {
            if self.child(i).is_null() {
                return i;  // i must be greater than one in this case.
            }
        }

        self.order()
    }

    fn node_is_overfilled(&self) -> bool {
        self.node_size() >= self.order()
    }

    fn node_is_fullfilled(&self) -> bool {
        self.node_size() >= self.order() - 1
    }

    fn height(&self) -> i32;

    #[inline]
    fn calc_height(&self) -> i32 {
        (0..self.order())
        .into_iter()
        .map(|i| {
            if self.child(i).is_null() {
                -1
            } else {
                unsafe { (*self.child(i)).calc_height() }
            }
        }).max().unwrap() + 1

    }


    fn total(&self) -> usize {
        let mut total = 1;

        for i in 0..self.order() {
            let child = self.child(i);

            if !child.is_null() {
                unsafe{ total += (*child).total() + 1; }
            }
        }

        total
    }


    fn minimum(&self) -> *mut (dyn BTNode<'a, K, V> + 'a) {
        let mut x = self.itself_mut();

        while unsafe { !(*x).child_first().is_null() } {
            unsafe { x = (*x).child_first() }
        }

        x
    }


    fn maximum(&self) -> *mut (dyn BTNode<'a, K, V> + 'a) {
        let mut x = self.itself_mut();

        while unsafe { !(*x).child_last().is_null() } {
            unsafe { x = (*x).child_last() }
        }

        x
    }

    fn is_leaf(&self) -> bool {
        for i in 0..self.order() {
            if !self.child(i).is_null() {
                return false;
            }
        }

        true
    }

    /// successor of item whose key is key.
    fn successor(&self, key: &K) -> BTItem<'a, K, V> {
        let k_idx = self.index_of_key(key);

        unsafe {
            if self.is_leaf() {
                if self.key(k_idx + 1).is_none() {  // Goto parent
                    let mut x = self.itself_mut();
                    let mut y = (*x).paren();

                    while !y.is_null() {
                        let idx = (*y).index_of_child(x);

                        if (*y).key(idx).is_some() {
                            return BTItem::new(y, idx);
                        }

                        x = y;
                        y = (*x).paren();
                    }

                    BTItem::new(y, 0)

                } else {
                    BTItem::new(self.itself_mut(), k_idx + 1)
                }

            } else {

                BTItem::new((*self.child(k_idx + 1)).minimum(), 0)
            }
        }
    }


    /// precessor of item whose key is key.
    fn precessor(&self, key: &K) -> BTItem<'a, K, V> {
        let k_idx = self.index_of_key(key);

        unsafe {
            if self.is_leaf() {
                if k_idx == 0 {  // Goto parent
                    let mut x = self.itself_mut();
                    let mut y = (*x).paren();

                    while !y.is_null() {
                        let idx = (*y).index_of_child(x);

                        if idx > 0 {
                            return BTItem::new(y, idx - 1);
                        }

                        x = y;
                        y = (*x).paren();
                    }

                    BTItem::new(y, 0)

                } else {
                    BTItem::new(self.itself_mut(), k_idx - 1)
                }

            } else {
                let pre_ptr = (*self.child(k_idx)).maximum();

                BTItem::new(pre_ptr, (*pre_ptr).node_size() - 1)
            }
        }
    }


    #[inline]
    fn search_approximately(
        &self,
        income_key: &K,
    ) -> *mut (dyn BTNode<'a, K, V> + 'a) {
        let mut y = self.null_mut();
        let mut x = self.itself_mut();

        unsafe {
            while !x.is_null() {
                y = x;

                if (*x).node_contains(income_key) || (*x).is_leaf() {
                    break;
                }

                let mut i = 0;
                let mut encountered = false;
                loop {
                    if let Some(key) = (*x).key(i) {
                        if income_key < key {
                            x = (*x).child(i);
                            encountered = true;

                            break;
                        }
                    } else {
                        break;
                    }

                    i += 1;
                }

                if !encountered {
                    x = (*x).child(i);
                }
            }
        }

        y
    }

    fn swap_to_leaf(&mut self, idx: usize) -> BTItem<'a, K, V> {
        let mut item_x = BTItem::new(self.itself_mut(), idx);

        while let Ok(item_nxt) = item_x.swap_with_successor_until_leaf() {
            item_x = item_nxt;
        }

        // unsafe {
        //     if (*item_x.node).is_leaf() {
        //         return item_x;
        //     }
        // }

        // while let Ok(item_nxt) = item_x.swap_with_precessor_until_leaf() {
        //     item_x = item_nxt;
        // }

        item_x
    }

    // fn swap_to_valid(&mut self, idx: usize) -> BTItem<'a, K, V> {
    //     let mut item_x = BTItem::new(self.itself_mut(), idx);

    //     while let Ok(item_nxt) = item_x.swap_with_successor_until_valid() {
    //         item_x = item_nxt;
    //     }

    //     item_x
    // }

    fn just_echo_stdout(&self) {
        let mut cache = String::new();

        self.echo_in_mm(&mut cache, |_, _| Ok(())).unwrap();

        println!("{}", cache);
    }

    fn format_keys(&self) -> String {
        let mut keys_s = vec![];

        for i in 0..self.order() {
            let key_s = if let Some(key) = self.key(i) {
               format!("{:?}", key)
            } else {
                break;
            };

            keys_s.push(key_s)
        }

        format!("({})", keys_s.join(", "))
    }

    /// BFS Echo
    fn echo_in_mm(
        &self,
        cache: &mut String,
        action: fn(
            *mut (dyn BTNode<'a, K, V> + 'a),
            &mut String,
        ) -> std::fmt::Result,
    ) -> std::fmt::Result {
        unsafe {
            writeln!(cache, "Entry: {}", self.format_keys())?;

            let mut this_level_queue: VecDeque<
                *mut (dyn BTNode<'a, K, V> + 'a),
            > = VecDeque::new();

            this_level_queue
                .push_back(self.itself_mut());
            let mut level = 0;

            while !this_level_queue.is_empty() {
                writeln!(cache)?;
                writeln!(
                    cache,
                    "############ Level: {} #############",
                    level
                )?;
                writeln!(cache)?;

                let mut nxt_level_queue: VecDeque<
                    *mut (dyn BTNode<'a, K, V> + 'a),
                > = VecDeque::new();

                while !this_level_queue.is_empty() {
                    let x = this_level_queue.pop_front().unwrap();


                    action(x, cache)?;

                    writeln!(cache, "{}", (*x).format_keys() )?;
                    for i in 0..self.order() {
                        let child = (*x).child(i);

                        if !child.is_null() {
                            writeln!(
                                cache,
                                "{} -({})-> {}",
                                "  |",
                                i,
                                (*child).format_keys(),
                            )?;

                            nxt_level_queue.push_back(child)
                        } else {
                            writeln!(cache, "{} -({})-> null", "  |", i)?;
                        }
                    }

                    writeln!(cache)?;
                }

                this_level_queue = nxt_level_queue;
                level += 1;
            }

            writeln!(cache, "{}", "------------- end --------------")?;
            writeln!(cache)?;
        }


        Ok(())
    }

    fn bfs_do(
        &self,
        action: fn(
            *mut (dyn BTNode<'a, K, V> + 'a),
        )
    ) {
        let mut queue= VecDeque::new();

        queue.push_back(self.itself_mut());
        while !queue.is_empty() {
            let x = queue.pop_front().unwrap();

            action(x);

            unsafe {
                for i in 0..self.order() {
                    let child = BTNode::child (&*x, i);

                    if !child.is_null() {
                        queue.push_back(child);
                    } else {
                        break;
                    }
                }
            }

        }
    }

    /// 1. N(keys) = N(vals)
    /// 1. Keep infix-order
    /// 1. non-leaf node (except root) has at least ⌈m/2⌉ child nodes.
    /// 1. A non-leaf node with k children contains k − 1 keys (m >= 3).
    /// 1. The root has at least two children if it is not a leaf node.
    ///
    fn basic_self_validate(&'a self) -> Result<(), Box<dyn std::error::Error>> {
        if self.order() == 2 {
            unsafe {
                if !self.child(0).is_null() {
                    assert!((*self.child(0)).key(0).unwrap() < self.key(0).unwrap());

                    (*self.child(0)).basic_self_validate()?;
                }

                if !self.child(1).is_null() {
                    assert!((*self.child(1)).key(0).unwrap() > self.key(0).unwrap());

                    (*self.child(1)).basic_self_validate()?;
                }
            }

            return Ok(());
        }


        let key_num = self.key_num();
        assert_eq!(key_num, self.val_num());

        if self.is_leaf() {
            assert!(self.key_iter().is_sorted())
        } else {
            let children_num = self.children_num();

            assert_eq!(key_num + 1, children_num);

            if self.paren().is_null() {
                assert!(children_num >= 2);
            } else {
                assert!(children_num >= self.order().div_ceil(2));
            }

            for i in 0..self.key_num() {
                unsafe {
                    let cur_key = self.key(i).unwrap();
                    let lf_child = self.child(i);
                    let rh_child = self.child(i + 1);

                    if !lf_child.is_null() {
                        assert!((*lf_child).node_last_key() < cur_key);
                    }

                    if !rh_child.is_null() {
                        assert!((*rh_child).node_first_key() > cur_key);
                    }
                }
            }

            for i in 0..self.order() {
                if !self.child(i).is_null() {
                    unsafe { (*self.child(i)).basic_self_validate()?; }
                }
            }
        }


        Ok(())
    }

}


#[derive(Clone)]
pub struct BTItem<'a, K, V> {
    node: *mut (dyn BTNode<'a, K, V> + 'a),
    idx: usize
}

impl<'a, K: CollKey, V> BTItem<'a, K, V> {
    pub fn new(node: *mut (dyn BTNode<'a, K, V> + 'a), idx: usize) -> Self {
        Self {
            node,
            idx,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.node.is_null()
    }

    pub fn key(&self) -> *mut K {
        unsafe {
            (*self.node).key_ptr(self.idx)
        }
    }

    pub fn assign_key(&mut self, key: *mut K) {
        unsafe {
            (*self.node).assign_key_ptr(self.idx, key)
        }
    }

    pub fn assign_val(&mut self, val: *mut V) {
        unsafe {
            (*self.node).assign_val_ptr(self.idx, val)
        }
    }

    pub fn val(&self) -> *mut V {
        unsafe {
            (*self.node).val_ptr(self.idx)
        }
    }

    pub fn successor(&self) -> Self {
        unsafe {
            (*self.node).successor(&*self.key())
        }
    }

    pub fn precessor(&self) -> Self {
        unsafe {
            (*self.node).precessor(&*self.key())
        }
    }

    pub fn swap(x: &mut Self, y: &mut Self) {
        let tmp_key = y.key();
        let tmp_val = y.val();

        y.assign_key(x.key());
        y.assign_val(x.val());

        x.assign_key(tmp_key);
        x.assign_val(tmp_val);
    }

    pub fn swap_with_successor_until_leaf(&mut self) -> Result<Self, ()> {
        unsafe {
            if (*self.node).is_leaf() {
                return Err(())
            }

            let mut nxt_item = self.successor();

            if nxt_item.is_valid() {
                BTItem::swap(self, &mut nxt_item);

                Ok(nxt_item)
            } else {
                Err(())
            }

        }
    }

    /// END COND: is_leaf || key < successor's key
    pub fn swap_with_successor_until_valid(&mut self) -> Result<Self, ()> {
        unsafe {
            if (*self.node).is_leaf() {
                return Err(())
            }

            let mut nxt_item = self.successor();

            if nxt_item.is_valid() && self.key() > nxt_item.key() {
                BTItem::swap(self, &mut nxt_item);

                Ok(nxt_item)
            } else {
                Err(())
            }

        }
    }

    pub fn swap_with_precessor_until_leaf(&mut self) -> Result<Self, ()> {
        unsafe {
            if (*self.node).is_leaf() {
                return Err(())
            }

            let mut nxt_item = self.precessor();

            if nxt_item.is_valid() {
                BTItem::swap(self, &mut nxt_item);

                Ok(nxt_item)
            } else {
                Err(())
            }

        }
    }

}



////////////////////////////////////////////////////////////////////////////////
//// Unify Test

#[cfg(test)]
macro_rules! gen_data {
    ($get_one: ident, $group: expr, $num: expr) => {{
        let group = $group;
        let num = $num;

        let mut keys = std::collections::HashSet::new();
        let mut elems = vec![];

        for _ in 0..num {
            let mut k = $get_one();
            let mut j = 0;

            while j < group {
                k = k.wrapping_add(1);
                if keys.contains(&k) {
                    continue;
                }

                keys.insert(k);
                elems.push((k, k.wrapping_add(1000)));

                j += 1;
            }
        }

        elems
    }};
}
#[cfg(test)]
pub(crate) use gen_data;


#[cfg(test)]
macro_rules! test_dict {
    ($dict: expr) => {
        let get_one = || rand::random::<u64>();

        for _ in 0..20 {
            let mut dict = $dict;
            let mut elems = $crate::gen_data!(get_one, 10, 100);

            /* Verify Create */

            for (i, (k, v)) in elems.iter().cloned().enumerate() {
                assert!(
                    dict.insert(k, v),
                    "[dict insert] insert res invalid"
                );
                assert_eq!(
                    dict.get(&k), Some(&v),
                     "[dict insert] insert but query failed"
                );

                if i % 20 == 0 {
                    dict.self_validate().unwrap();
                }
                // println!("{i}. insert: ");
            }

            dict.self_validate().unwrap();

            /* Verify Update */

            for (i, (k, v)) in elems.clone().into_iter().enumerate() {
                assert_eq!(dict.get(&k), Some(&v));

                let newv = k + 500;

                assert!(dict.modify(&k, newv));

                elems[i] = (k, newv);

                assert_eq!(dict.get(&k), Some(&newv));
            }

            /* Verify Remove */

            use rand::{prelude::SliceRandom, thread_rng};

            elems.shuffle(&mut thread_rng());

            for (i, (k, v)) in elems.into_iter().enumerate() {
                assert_eq!(
                    dict.get(&k),
                    Some(&v),
                    "[dict remove] Assure get Some"
                );
                assert_eq!(
                    dict.remove(&k),
                    Some(v),
                    "[dict remove] Assert remove failed"
                );
                assert_eq!(
                    dict.get(&k),
                    None,
                    "[dict remove] Assure get None"
                );

                // println!("[dict remove]: {}", i);

                // sample to save time
                if i % 10 == 0 {
                    dict.self_validate().unwrap();
                }
            }
        }
    };
}

#[cfg(test)]
pub(crate) use test_dict;


/// Test heap push/pop
#[cfg(test)]
macro_rules! test_heap {
    ($heap:expr, $endian:ident) => {
        test_heap!($heap, $endian, push:push, pop:pop);
    };
    ($heap:expr, $endian:ident, push:$push:ident, pop:$pop:ident) => {
        use $crate::gen_unique;
        let get_one = || rand::random::<u64>();

        let non_dec = $crate::heap_endian_no_dec!($endian);

        for _ in 0..20 {
            /* Basic Test */

            let mut heap = $heap;
            let mut unique = gen_unique();

            let batch_num = 1000;

            for _ in 0..batch_num {
                let e = get_one();
                heap.push(unique(), e);
            }

            let mut res = vec![];

            for _ in 0..batch_num {
                res.push(heap.pop().unwrap());
            }

            if !non_dec {
                res.reverse();
            }

            assert!(res.is_sorted());

            /* Accompany Test */

            // In-In Out-Out, generate in/out sequence
            let mut seq = vec![];
            let mut rems = 0;

            // pad 25% of batch
            for _ in 0..batch_num / 4 {
                seq.push(true); // push
                rems += 1;
            }

            for _ in 0..(3 * batch_num) / 4 {
                if random::<usize>() % 2 == 0 {
                    seq.push(true);
                    rems += 1;
                } else {
                    seq.push(false);
                    rems -= 1;
                }

                if rems == 0 {
                    break;
                }
            }

            let mut refheap = $crate::union_heap!($endian);
            let mut testheap = $heap;
            let mut unique = gen_unique();

            for flag in seq {
                if flag {
                    let e = get_one();
                    let i = unique();

                    refheap.push(i, e.clone());
                    testheap.push(i, e);
                } else {
                    let target = refheap.pop();
                    assert_eq!(testheap.pop(), target);
                }
            }
        }

    }
}

#[cfg(test)]
pub(crate) use test_heap;


#[cfg(test)]
macro_rules! union_heap {
    (MAX) => {
        {
            $crate::MaxDictHeap::new()
        }
    };
    (MIN) => {
        {
            $crate::MinDictHeap::new()
        }
    }
}

#[cfg(test)]
pub(crate) use union_heap;


#[cfg(test)]
macro_rules! heap_endian_no_dec {
    (MAX) => {
        false
    };
    (MIN) => {
        true
    };
}

#[cfg(test)]
pub(crate) use heap_endian_no_dec;


pub struct MinHeap<T>(BinaryHeap<std::cmp::Reverse<T>>);


/// Fake Min Dict Heap
pub struct MinDictHeap<T> {
    inner: BinaryHeap<std::cmp::Reverse<T>>,
}


/// Fake Max Dict Heap
pub struct MaxDictHeap<T> {
    inner: BinaryHeap<T>,
}


impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    pub fn push(&mut self, v: T) {
        self.0.push(std::cmp::Reverse(v));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|r|r.0)
    }
}


impl<T: Ord> MinDictHeap<T> {
    pub fn new() -> Self {
        Self {
            inner: BinaryHeap::new(),
        }
    }

    pub fn push<I>(&mut self, _i: I, v: T) {
        self.inner.push(std::cmp::Reverse(v));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop().map(|r|r.0)
    }
}


impl<T: Ord> MaxDictHeap<T> {
    pub fn new() -> Self {
        Self {
            inner: BinaryHeap::new(),
        }
    }

    pub fn push<I>(&mut self, _i: I, v: T) {
        self.inner.push(v);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }
}
