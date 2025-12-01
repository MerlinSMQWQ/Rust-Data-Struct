use std::{marker::PhantomData, ptr::NonNull};

use crate::linear_list::singly_linked_list::Node;

pub struct DoubleLinkedList<T> {
    // 等同于使用裸指针，但是需要额外注意内存安全问题，Option增加了一定的安全性
    head: Option<NonNull<DoubleLinkedNode<T>>>,
    tail: Option<NonNull<DoubleLinkedNode<T>>>,
    len: usize,
    // marker说明这个数据结构对一个Box<Node<T>>持有所有权，并且会负责调用drop
    marker: PhantomData<Box<DoubleLinkedNode<T>>>,
}

struct DoubleLinkedNode<T> {
    next: Option<NonNull<DoubleLinkedNode<T>>>,
    prev: Option<NonNull<DoubleLinkedNode<T>>>,
    element: T,
}

impl<T> DoubleLinkedNode<T> {
    fn new(element: T) -> Self {
        Self {
            next: None,
            prev: None,
            element: element,
        }
    }

    // 我自己没有设想过，但是非常好的一个设计
    // 这个函数将会消耗Box，相当于将堆上的内容移动到栈上（释放堆内存并将element拷贝到栈上），需要注意的是，实现原理上和take方法并不相似，take方法是取出对应位置的数据并给原来位置上的数据设置为None，也就是说take会保留容器，而into_element是消费方法，会使得Box对应的堆空间释放
    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}

impl<T> DoubleLinkedList<T> {
    // 创建一个空的双链表
    pub const fn new() -> Self {
        Self { head: None, tail: None, len: 0, marker: PhantomData }
    }

    fn push_front_node(&mut self, mut node: Box<DoubleLinkedNode<T>>) {
        unsafe {
            // self.head是链表第一个有效节点
            node.next = self.head;
            node.prev = None;

            // leak()将 Box<DoubleLinkedNode<T>> 转换为 &mut DoubleLinkedNode<T>，into()方法将&mut DoubleLinkedNode<T>转换成NonNull<DoubleLinkedNode<T>>指针
            // 但是现在更好的写法会尽量避免使用leak()，而是这样写，后续使用from_raw()处理即可：let node = NonNull::new(Box::into_raw(node));
            let node = Some(Box::leak(node).into());

            match self.head {
                None => self.tail = node,
                Some(head) => (*head.as_ptr()).prev = node,
            }
            self.head = node;
            self.len += 1;
        }
    }

    pub fn push_front(&mut self, elt: T) {
        self.push_front_node(Box::new(DoubleLinkedNode::new(elt)));
    }

    fn pop_front_node(&mut self) -> Option<Box<DoubleLinkedNode<T>>> {
        // map方法用于对Option中的值进行转换处理，但传入的是Some<T>，会对将数据传入对应的函数进行处理，再返回新的Option，如果传入None，则依然返回None
        self.head.map(|node| unsafe {
            // 使用from_raw重新让node连接上生命周期
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            // 去掉了第一个有效节点，如果链表为空，则设置尾节点为None，否则，将下一个节点的前驱节点设置为None（令他变成头结点，头结点没有前驱节点）
            match self.head {
                None => self.tail = None,
                Some(head) => (*head.as_ptr()).prev = None,
            }

            self.len -= 1;
            node
        })
    }

    pub fn push_back(&mut self, elt: T) {
        self.push_back_node(Box::new(DoubleLinkedNode::new(elt)))
    }

    fn push_back_node(&mut self, mut node: Box<DoubleLinkedNode<T>>) {
        unsafe {
            node.next = None;
            node.prev = self.tail;
            let node = Some(Box::leak(node).into());

            match self.tail {
                Some(tail) => (*tail.as_ptr()).next = node,
                None => self.head = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(DoubleLinkedNode::into_element)
    }

    // 和删除头结点类似
    fn pop_back_node(&mut self) -> Option<Box<DoubleLinkedNode<T>>> {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;

            match self.tail {
                None => self.head = None,
                Some(tail) => (*tail.as_ptr()).next = None,
            }

            self.len -= 1;
            node
        })
    }
}