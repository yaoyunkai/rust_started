use std::cell::RefCell;
use std::rc::Rc;

// 为了方便阅读，我们定义一个类型别名
// 每个节点都被 Rc 和 RefCell 包裹，Option 用于处理空指针的情况
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

// 定义链表节点
struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

// 定义双向链表结构体
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> DoublyLinkedList<T> {
    // 创建一个空链表
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    // 在链表尾部添加元素
    pub fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                // 如果链表不为空，将旧尾部的 next 指向新节点
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                // 将新节点的 prev 指向旧尾部
                new_node.borrow_mut().prev = Some(old_tail);
                // 更新链表的 tail
                self.tail = Some(new_node);
            }
            None => {
                // 如果链表为空，head 和 tail 都指向新节点
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }
    }

    // 在链表头部添加元素
    pub fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&new_node));
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
            }
        }
    }

    // 从链表尾部弹出元素
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    // 将新的尾节点的 next 置空
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    // 如果没有 prev，说明链表空了
                    self.head = None;
                }
            }
            // 此时 old_tail 的引用计数应该为 1，我们可以安全地解包并取出里面的值
            Rc::try_unwrap(old_tail)
                .ok()
                .expect("引用计数异常")
                .into_inner()
                .value
        })
    }

    // 从链表头部弹出元素
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            Rc::try_unwrap(old_head)
                .ok()
                .expect("引用计数异常")
                .into_inner()
                .value
        })
    }
}

pub fn foo() {
    let mut list = DoublyLinkedList::new();

    // 测试尾部插入
    list.push_back(1);
    list.push_back(2);

    // 测试头部插入
    list.push_front(0);

    // 此时链表应该是: 0 <-> 1 <-> 2

    // 测试弹出
    println!("Pop front: {:?}", list.pop_front()); // 输出: Some(0)
    println!("Pop back: {:?}", list.pop_back()); // 输出: Some(2)
    println!("Pop back: {:?}", list.pop_back()); // 输出: Some(1)
    println!("Pop back: {:?}", list.pop_back()); // 输出: None (链表已空)
}
