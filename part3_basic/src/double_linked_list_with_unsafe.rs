/*

Box::into_raw：将数据分配在堆（Heap）上，并剥夺 Rust 的自动内存管理（所有权），将其转换为一个裸指针。
                这相当于 C 语言中的 malloc。

Box::from_raw：将裸指针重新转换回 Box。当这个 Box 离开作用域时，Rust 会自动释放这块内存。
                这相当于 C 语言中的 free。

*/
use std::ptr;

struct Node<T> {
    next: *mut Node<T>,
    prev: *mut Node<T>,
    val: T,
}

pub struct DoubleLinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> Drop for DoubleLinkedList<T> {
    fn drop(&mut self) {
        // 不断弹出元素，pop_back 内部的 Box::from_raw 会负责释放内存
        while let Some(_) = self.pop_back() {}
    }
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.len
    }

    pub fn push_back(&mut self, val: T) {
        let new_node = Box::into_raw(Box::new(Node {
            val,
            next: ptr::null_mut(),
            prev: self.tail, // 新节点的 prev 指向当前的 tail
        }));

        unsafe {
            if !self.tail.is_null() {
                // 如果链表不为空，将旧尾部的 next 指向新节点
                (*self.tail).next = new_node;
            } else {
                // 如果链表为空，新节点同时也是头节点
                self.head = new_node;
            }
            // 更新链表的 tail 指向新节点
            self.tail = new_node;
        }
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None; // 链表为空
        }

        unsafe {
            // 1. 将裸指针转回 Box，以便获取数据并在函数结束时自动释放内存
            let old_tail = Box::from_raw(self.tail);
            let val = old_tail.val;

            // 2. 将链表的 tail 指针前移
            self.tail = old_tail.prev;

            // 3. 更新新尾部的 next 指针
            if !self.tail.is_null() {
                (*self.tail).next = ptr::null_mut();
            } else {
                // 如果弹出后链表为空，头节点也置空
                self.head = ptr::null_mut();
            }

            self.len -= 1;
            Some(val)
        }
    }
}
