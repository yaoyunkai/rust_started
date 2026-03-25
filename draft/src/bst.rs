/*
Binary Search Tree

use std::cmp::Ordering;

assert_eq!(5.cmp(&10), Ordering::Less);
assert_eq!(10.cmp(&5), Ordering::Greater);
assert_eq!(5.cmp(&5), Ordering::Equal);

functions of BST:

size()
contains(key): check Key in BST, return True if exists else false
get(key): return value if key exists, else return None
put(key,value): insert key-value to BST, update old value if key already in BST.

delete_min() : remove the smallest key and associated value from BST.
delete(key) : remove the specified key and associated value from BST, if key exists, else do nothing.
min() : return the smallest key.

delete_max()
max()

to_vector(): return vector, and element is Pair<K,V>
from_vector(vec_data): convert to BST

get_keys_iterator(): return a Iterator that return all keys from ascending order.

---------------
key can't be None for above functions, if None, do nothing or raise issue ?


*/
use std::cmp::Ordering;

struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    size: usize,
}

#[derive(Debug)]
struct Pair<K, V> {
    key: K,
    value: V,
}

struct BST<K: Ord, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
            size: 1,
        }
    }

    fn size(node: &Option<Box<Node<K, V>>>) -> usize {
        node.as_ref().map_or(0, |n| n.size)
    }

    fn update_size(&mut self) {
        self.size = 1 + Self::size(&self.left) + Self::size(&self.right);
    }
}

impl<K: Ord, V> BST<K, V> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn size(&self) -> usize {
        Node::size(&self.root)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut current = &self.root;
        while let Some(node) = current {
            match key.cmp(&node.key) {
                Ordering::Less => current = &node.left,
                Ordering::Greater => current = &node.right,
                Ordering::Equal => return Some(&node.value),
            }
        }
        None
    }

    pub fn put(&mut self, key: K, value: V) {
        Self::put_recursive(&mut self.root, key, value);
    }

    fn put_recursive(node_link: &mut Option<Box<Node<K, V>>>, key: K, value: V) {
        if let Some(node) = node_link {
            match key.cmp(&node.key) {
                Ordering::Less => Self::put_recursive(&mut node.left, key, value),
                Ordering::Greater => Self::put_recursive(&mut node.right, key, value),
                Ordering::Equal => node.value = value,
            }
            node.update_size();
        } else {
            *node_link = Some(Box::new(Node::new(key, value)));
        }
    }

    pub fn min(&self) -> Option<&K> {
        let mut current = &self.root;
        let mut min_key = None;
        while let Some(node) = current {
            min_key = Some(&node.key);
            current = &node.left;
        }
        min_key
    }

    pub fn max(&self) -> Option<&K> {
        let mut current = &self.root;
        let mut max_key = None;
        while let Some(node) = current {
            max_key = Some(&node.key);
            current = &node.right;
        }
        max_key
    }

    pub fn delete_min(&mut self) {
        Self::pop_min(&mut self.root);
    }

    fn pop_min(node_link: &mut Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        if node_link.is_none() {
            return None;
        }
        // 如果没有左子树，当前节点就是最小的
        if node_link.as_ref().unwrap().left.is_none() {
            let mut min_node = node_link.take().unwrap();
            *node_link = min_node.right.take(); // 用右子树替换当前位置
            return Some(min_node);
        }
        // 递归去左子树找
        let min_node = Self::pop_min(&mut node_link.as_mut().unwrap().left);
        node_link.as_mut().unwrap().update_size();
        min_node
    }

    pub fn delete_max(&mut self) {
        Self::pop_max(&mut self.root);
    }

    fn pop_max(node_link: &mut Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        if node_link.is_none() {
            return None;
        }
        if node_link.as_ref().unwrap().right.is_none() {
            let mut max_node = node_link.take().unwrap();
            *node_link = max_node.left.take();
            return Some(max_node);
        }
        let max_node = Self::pop_max(&mut node_link.as_mut().unwrap().right);
        node_link.as_mut().unwrap().update_size();
        max_node
    }

    pub fn delete(&mut self, key: &K) {
        Self::delete_recursive(&mut self.root, key);
    }

    fn delete_recursive(node_link: &mut Option<Box<Node<K, V>>>, key: &K) {
        // 使用 take() 暂时夺取节点的所有权，以便进行模式匹配和修改
        if let Some(mut node) = node_link.take() {
            match key.cmp(&node.key) {
                Ordering::Less => {
                    Self::delete_recursive(&mut node.left, key);
                    node.update_size();
                    *node_link = Some(node); // 放回节点
                }
                Ordering::Greater => {
                    Self::delete_recursive(&mut node.right, key);
                    node.update_size();
                    *node_link = Some(node); // 放回节点
                }
                Ordering::Equal => {
                    // 找到节点，准备删除
                    if node.right.is_none() {
                        *node_link = node.left;
                    } else if node.left.is_none() {
                        *node_link = node.right;
                    } else {
                        // 左右子树都存在：找到右子树中的最小节点代替当前节点
                        let mut successor = Self::pop_min(&mut node.right).unwrap();
                        successor.left = node.left;
                        successor.right = node.right;
                        successor.update_size();
                        *node_link = Some(successor);
                    }
                }
            }
        }
    }

    pub fn to_vector(self) -> Vec<Pair<K, V>> {
        let mut vec = Vec::with_capacity(self.size());
        Self::inorder_collect(self.root, &mut vec);
        vec
    }

    fn inorder_collect(node: Option<Box<Node<K, V>>>, vec: &mut Vec<Pair<K, V>>) {
        if let Some(n) = node {
            Self::inorder_collect(n.left, vec);
            vec.push(Pair {
                key: n.key,
                value: n.value,
            });
            Self::inorder_collect(n.right, vec);
        }
    }

    pub fn from_vector(vec_data: Vec<Pair<K, V>>) -> Self {
        let mut bst = BST::new();
        for pair in vec_data {
            bst.put(pair.key, pair.value);
        }
        bst
    }

    pub fn get_keys_iterator(&self) -> BSTKeysIter<'_, K, V> {
        let mut stack = Vec::new();
        let mut current = &self.root;
        // 初始化时，一直向左走到底
        while let Some(node) = current {
            stack.push(node.as_ref());
            current = &node.left;
        }
        BSTKeysIter { stack }
    }
}

pub struct BSTKeysIter<'a, K, V> {
    // 使用栈来模拟递归调用栈
    stack: Vec<&'a Node<K, V>>,
}

impl<'a, K, V> Iterator for BSTKeysIter<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        // 弹出栈顶元素（当前最小的元素）
        let node = self.stack.pop()?;

        // 如果有右子树，将右子树及其所有左子孙压入栈中
        let mut current = &node.right;
        while let Some(n) = current {
            self.stack.push(n.as_ref());
            current = &n.left;
        }

        Some(&node.key)
    }
}

pub fn run_bst() {
    let mut bst = BST::new();
    bst.put(5, "five");
    bst.put(3, "three");
    bst.put(7, "seven");
    bst.put(2, "two");
    bst.put(4, "four");

    println!("Size: {}", bst.size()); // 5
    println!("Contains 3: {}", bst.contains(&3)); // true
    println!("Get 7: {:?}", bst.get(&7)); // Some("seven")
    println!("Min: {:?}", bst.min()); // Some(&2)
    println!("Max: {:?}", bst.max()); // Some(&7)

    let keys: Vec<&i32> = bst.get_keys_iterator().collect();
    println!("Keys in order: {:?}", keys); // [2, 3, 4, 5, 7]

    bst.delete(&3);
    let keys_after_delete: Vec<&i32> = bst.get_keys_iterator().collect();
    println!("Keys after deleting 3: {:?}", keys_after_delete); // [2, 4, 5, 7]

    let vec = bst.to_vector();
    println!("To Vector: {:?}", vec);
}
