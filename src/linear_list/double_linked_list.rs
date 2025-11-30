use std::ptr::NonNull;

pub struct Node<T: std::fmt::Debug> {
    pub pre: Option<NonNull<Node<T>>>,
    pub data: T,
    pub next: Option<NonNull<Node<T>>>,
}

impl<T: std::fmt::Debug> Node<T> {
    pub fn new(element: T) -> Self {
        Self {
            pre: None,
            data: element,
            next: None,
        }
    }
}

impl<T: std::fmt::Debug + Default> Default for Node<T> {
    fn default() -> Self {
        Self {
            pre: None,
            data: T::default(),
            next: None,
        }
    }
}

pub struct DoubleLinkedList<T: std::fmt::Debug> {
    pub head: Option<NonNull<Node<T>>>,
    pub length: usize,
    pub tail: Option<NonNull<Node<T>>>,
}

impl<T: std::fmt::Debug + Default> DoubleLinkedList<T> {
    // 双链表的哨兵头结点
    pub fn new() -> Self {
        let head_node = Box::new(Node {
            pre: None,
            data: T::default(),
            next: None,
        });
        let head_ptr = NonNull::new(Box::into_raw(head_node));
        Self {
            head: head_ptr,
            length: 0,
            tail: None,
        }
    }

    pub fn push_front(&mut self, element: T) {
        let mut node = Box::new(Node::new(element));
        let head_node = unsafe { self.head.unwrap().as_mut() };
        node.pre = self.head;
        node.next = head_node.next;
        let node_ptr = NonNull::new(Box::into_raw(node));

        if let Some(mut next_ptr) = head_node.next {
            unsafe { next_ptr.as_mut().pre = node_ptr };
        } else {
            self.tail = node_ptr;
        }

        head_node.next = node_ptr;
        self.length += 1;
    }

    pub fn push(&mut self, element: T) {
        if self.length == 0 {
            self.push_front(element);
        } else {
            let mut node = Box::new(Node::new(element));
            node.pre = self.tail;
            let tail_node = unsafe { self.tail.unwrap().as_mut() };
            let node_ptr = NonNull::new(Box::into_raw(node));
            tail_node.next = node_ptr;
            self.tail = node_ptr;
            self.length += 1;
        }
    }

    pub fn insert(&mut self, element: T, pos: usize) -> Result<(), &'static str> {
        if pos < 1 || pos > (self.length + 1) {
            Err("Out of List!")
        } else {
            if pos == 1 {
                self.push_front(element);
            } else if pos == self.length + 1 {
                self.push(element);
            } else {
                let mut node = Box::new(Node::new(element));
                let mut current_node = unsafe { self.head.unwrap().as_mut() };
                let mut current_ptr = self.head;
                for _ in 1..(pos - 1) {
                    current_ptr = current_node.next;
                    current_node = unsafe { current_ptr.unwrap().as_mut() };
                }
                node.pre = current_ptr;
                node.next = current_node.next;
                let node_ptr = NonNull::new(Box::into_raw(node));
                unsafe { current_node.next.unwrap().as_mut().pre = node_ptr };
                current_node.next = node_ptr;
                self.length += 1;
            }
            Ok(())
        }
    }
}
