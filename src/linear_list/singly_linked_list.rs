/*
 * @Author: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @Date: 2025-11-26 17:28:42
 * @LastEditors: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @LastEditTime: 2025-11-30 21:51:42
 * @FilePath: \Rust-Data-Struct\src\linear_list\singly_linked_list.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

// NonNull是一个包装过的原始指针，可以保证指针部位null，可以喝Box配合来管理堆内存
use std::ptr::NonNull;

pub struct Node<T: std::fmt::Debug> {
    pub data: T,
    pub next: Option<NonNull<Node<T>>>,
}

impl<T: std::fmt::Debug> Node<T> {
    pub fn new(element: T) -> Self {
        Self {
            data: element,
            next: None,
        }
    }
}

pub struct SinglyLinkedList<T: std::fmt::Debug> {
    pub length: usize,
    pub next: Option<NonNull<Node<T>>>,
}

impl<T: std::fmt::Debug> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            next: None,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    // 头插法
    pub fn push_front(&mut self, element: T) {
        // 如果直接Node::new()，创建的node只是局部变量（在对应的栈上），一旦运行方法结束，就会回收，这样的话，self.next将会指向一个无意义的地址，引发内存安全问题，所以使用Box将它放在堆上，这样就不会被回收了
        let mut node = Box::new(Node::new(element));
        node.next = self.next;
        // Box::into_raw()将Box<T>转换成指针*mut T，同时转移所有权
        self.next = NonNull::new(Box::into_raw(node));
        self.length += 1;
    }

    // 尾插法
    pub fn push(&mut self, element: T) {
        let node = Box::new(Node::new(element));

        if self.next.is_none() {
            self.next = NonNull::new(Box::into_raw(node));
        } else {
            let mut current_ptr = self.next.unwrap();
            loop {
                // as_mut() 是 Rust 中 NonNull 类型的一个方法，用于将 NonNull<T> 转换为可变引用 &mut T。
                let current_node = unsafe { current_ptr.as_mut() };
                match current_node.next {
                    Some(next_ptr) => current_ptr = next_ptr,
                    None => {
                        current_node.next = NonNull::new(Box::into_raw(node));
                        break;
                    }
                }
            }
        }
        self.length += 1;
    }

    // 按位置插入元素
    pub fn insert(&mut self, element: T, pos: usize) -> Result<(), &'static str> {
        if pos < 1 || (pos > self.length + 1) {
            Err("out of list!")
        } else if pos == 1 {
            self.push_front(element);
            Ok(())
        } else if pos == self.len() + 1 {
            self.push(element);
            Ok(())
        } else {
            let mut node = Box::new(Node::new(element));

            // 添加空指针判断
            if self.next.is_none() {
                return Err("Invalid position for empty list!")
            }
            
            let mut current_ptr = self.next.unwrap();

            // 移动到 pos-1 位置（插入位置的前一个节点）
            for _ in 1..(pos - 1) {
                let current_node = unsafe { current_ptr.as_mut() };
                current_ptr = current_node.next.unwrap();
            }

            // 插入新节点
            let current_node = unsafe { current_ptr.as_mut() };
            node.next = current_node.next.take();
            current_node.next = NonNull::new(Box::into_raw(node));

            self.length += 1;
            Ok(())
        }
    }

    // 按位置删除元素
    pub fn delete(&mut self, pos: usize) -> Result<(), &'static str> {
        if self.len() == 0 {
            return Err("No elements in List!");
        }
        if pos < 1 || pos > self.len() {
            return Err("out of List!");
        }

        if pos == 1 {
            let head_ptr = self.next.take().unwrap();
            // from_raw是所有权交给Box，在Box就会自动调用drop，离开了域就会被销毁，into_raw是所有权交出去，因为box没有所有权也就无法调用drop了
            let head_node = unsafe { Box::from_raw(head_ptr.as_ptr()) };
            self.next = head_node.next;
            self.length -= 1;
            Ok(())
        } else {
            let mut current_ptr = self.next.unwrap();
            for _ in 1..(pos - 1) {
                let current_node = unsafe { current_ptr.as_mut() };
                current_ptr = current_node.next.unwrap();
            }

            let prev_node = unsafe { current_ptr.as_mut() };
            // take将会获得pre_node.next指向的节点的所有权，相当于断开目标节点
            let target_ptr = prev_node.next.take().unwrap();
            // 令prev_node的next指向target_ptr节点的下一个节点
            let target_node = unsafe { Box::from_raw(target_ptr.as_ptr()) };
            prev_node.next = target_node.next;
            self.length -= 1;
            Ok(())
        }
    }

    pub fn get(&self, pos: usize) -> Result<&T, &'static str> {
        if pos < 1 || pos > self.len() {
            Err("Out of list!")
        } else {
            if pos == 1 {
                let current_ptr = self.next.unwrap();
                // 使用as_ref，我们不需要改变节点，而是获取数据，使用引用即可
                let current_node = unsafe { current_ptr.as_ref() };
                Ok(&current_node.data)
            } else {
                let mut current_ptr = self.next.unwrap();
                let mut current_node = unsafe { current_ptr.as_ref() };
                for _ in 1..pos {
                    current_ptr = current_node.next.unwrap();
                    current_node = unsafe { current_ptr.as_ref() };
                }
                Ok(&current_node.data)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list() {
        let list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        assert_eq!(list.len(), 0);
        assert!(list.next.is_none());
    }

    #[test]
    fn test_push_front() {
        let mut list = SinglyLinkedList::new();
        list.push_front(1);
        assert_eq!(list.len(), 1);
        
        list.push_front(2);
        assert_eq!(list.len(), 2);
        
        // 验证顺序: 2 -> 1
        let first = list.get(1).unwrap();
        let second = list.get(2).unwrap();
        assert_eq!(*first, 2);
        assert_eq!(*second, 1);
    }

    #[test]
    fn test_push() {
        let mut list = SinglyLinkedList::new();
        list.push(1);
        assert_eq!(list.len(), 1);
        
        list.push(2);
        assert_eq!(list.len(), 2);
        
        // 验证顺序: 1 -> 2
        let first = list.get(1).unwrap();
        let second = list.get(2).unwrap();
        assert_eq!(*first, 1);
        assert_eq!(*second, 2);
    }

    #[test]
    fn test_insert() {
        let mut list = SinglyLinkedList::new();
        
        // 在空列表插入
        list.insert(1, 1).unwrap();
        assert_eq!(list.len(), 1);
        
        // 头部插入
        list.insert(0, 1).unwrap();
        assert_eq!(list.len(), 2);
        
        // 尾部插入
        list.insert(2, 3).unwrap();
        assert_eq!(list.len(), 3);
        
        // 中间插入
        list.insert(1, 2).unwrap();
        assert_eq!(list.len(), 4);
        
        // 验证顺序: 0 -> 1 -> 1 -> 2
        assert_eq!(*list.get(1).unwrap(), 0);
        assert_eq!(*list.get(2).unwrap(), 1);
        assert_eq!(*list.get(3).unwrap(), 1);
        assert_eq!(*list.get(4).unwrap(), 2);
    }

    #[test]
    fn test_get() {
        let mut list = SinglyLinkedList::new();
        list.push(10);
        list.push(20);
        list.push(30);
        
        // 正常获取
        assert_eq!(*list.get(1).unwrap(), 10);
        assert_eq!(*list.get(2).unwrap(), 20);
        assert_eq!(*list.get(3).unwrap(), 30);
        
        // 边界测试
        assert!(list.get(0).is_err());
        assert!(list.get(4).is_err());
    }

    #[test]
    fn test_delete() {
        let mut list = SinglyLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        
        // 删除头部
        list.delete(1).unwrap();
        assert_eq!(list.len(), 3);
        assert_eq!(*list.get(1).unwrap(), 2);
        
        // 删除中间
        list.delete(2).unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(*list.get(2).unwrap(), 4);
        
        // 删除尾部
        list.delete(2).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(*list.get(1).unwrap(), 2);
        
        // 删除最后一个
        list.delete(1).unwrap();
        assert_eq!(list.len(), 0);
        
        // 边界测试
        assert!(list.delete(1).is_err());
        assert!(list.delete(0).is_err());
    }

    #[test]
    fn test_mixed_operations() {
        let mut list = SinglyLinkedList::new();
        
        // 混合操作测试
        list.push_front(1);
        list.push(3);
        list.insert(2, 2).unwrap();
        
        assert_eq!(list.len(), 3);
        assert_eq!(*list.get(1).unwrap(), 1);
        assert_eq!(*list.get(2).unwrap(), 2);
        assert_eq!(*list.get(3).unwrap(), 3);
        
        list.delete(2).unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(*list.get(1).unwrap(), 1);
        assert_eq!(*list.get(2).unwrap(), 3);
    }
}