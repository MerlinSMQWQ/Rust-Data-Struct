use std::ptr::NonNull;

struct StackNode<T> {
    data: T,
    next: Option<NonNull<StackNode<T>>>,
}

struct LinkedStack<T> {
    len: usize,
    head: Option<NonNull<StackNode<T>>>
}

impl<T> StackNode<T> {
    fn new(element: T) -> Self {
        Self { data: element, next: None }
    }
}

impl<T> LinkedStack<T> {
    fn new() -> Self {
        Self { len: 0, head: None }
    }

    fn push(&mut self, element: T) {
        let mut node = Box::new(StackNode::new(element));
        node.next = self.head;
        self.head = NonNull::new(Box::into_raw(node));
        self.len += 1;
    }

    fn pop(&mut self) -> Result<T, &'static str> {
        if self.len == 0 {
            Err("Stack is empty!")
        } else {
            let node = unsafe { Box::from_raw(self.head.unwrap().as_mut()) };
            self.head = node.next;
            self.len -= 1;
            Ok(node.data)
        }
    }

    fn get(&self) -> Result<&T, &'static str> {
        if self.len == 0 {
            Err("Stack is empty!")
        } else {
            let node = unsafe { self.head.unwrap().as_ref() };
            Ok(&node.data)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack() {
        let stack: LinkedStack<i32> = LinkedStack::new();
        assert_eq!(stack.len, 0);
        // 验证新创建的栈是空的
    }

    #[test]
    fn test_push_and_pop() {
        let mut stack = LinkedStack::new();
        
        // 测试入栈
        stack.push(1);
        assert_eq!(stack.len, 1);
        
        stack.push(2);
        assert_eq!(stack.len, 2);
        
        stack.push(3);
        assert_eq!(stack.len, 3);
        
        // 测试出栈（后进先出）
        assert_eq!(stack.pop().unwrap(), 3);
        assert_eq!(stack.len, 2);
        
        assert_eq!(stack.pop().unwrap(), 2);
        assert_eq!(stack.len, 1);
        
        assert_eq!(stack.pop().unwrap(), 1);
        assert_eq!(stack.len, 0);
    }

    #[test]
    fn test_get_top_element() {
        let mut stack = LinkedStack::new();
        
        // 空栈获取元素应该失败
        assert!(stack.get().is_err());
        
        // 添加元素后能够获取栈顶元素
        stack.push(10);
        assert_eq!(*stack.get().unwrap(), 10);
        assert_eq!(stack.len, 1); // get不应该改变栈的状态
        
        stack.push(20);
        assert_eq!(*stack.get().unwrap(), 20);
        assert_eq!(stack.len, 2);
        
        stack.pop().unwrap();
        assert_eq!(*stack.get().unwrap(), 10);
        assert_eq!(stack.len, 1);
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_stack_operations() {
        let mut stack: LinkedStack<i32> = LinkedStack::new();
        
        // 空栈出栈应该返回错误
        assert!(stack.pop().is_err());
        assert_eq!(stack.pop().unwrap_err(), "Stack is empty!");
        
        // 空栈获取元素应该返回错误
        assert!(stack.get().is_err());
        assert_eq!(stack.get().unwrap_err(), "Stack is empty!");
    }

    #[test]
    fn test_single_element_stack() {
        let mut stack = LinkedStack::new();
        
        stack.push(42);
        assert_eq!(stack.len, 1);
        assert_eq!(*stack.get().unwrap(), 42);
        
        let popped = stack.pop().unwrap();
        assert_eq!(popped, 42);
        assert_eq!(stack.len, 0);
        
        // 再次操作空栈
        assert!(stack.pop().is_err());
        assert!(stack.get().is_err());
    }

    #[test]
    fn test_large_number_of_elements() {
        let mut stack = LinkedStack::new();
        const COUNT: usize = 1000;
        
        // 大量入栈
        for i in 0..COUNT {
            stack.push(i as i32);
        }
        assert_eq!(stack.len, COUNT);
        assert_eq!(*stack.get().unwrap(), (COUNT - 1) as i32);
        
        // 大量出栈并验证顺序
        for i in (0..COUNT).rev() {
            assert_eq!(stack.pop().unwrap(), i as i32);
        }
        assert_eq!(stack.len, 0);
        assert!(stack.pop().is_err());
    }
}

#[cfg(test)]
mod type_compatibility_tests {
    use super::*;

    #[test]
    fn test_string_elements() {
        let mut stack = LinkedStack::new();
        
        stack.push("Hello".to_string());
        stack.push("World".to_string());
        
        assert_eq!(stack.get().unwrap(), "World");
        assert_eq!(stack.pop().unwrap(), "World");
        assert_eq!(stack.pop().unwrap(), "Hello");
    }

    #[test]
    fn test_complex_struct_elements() {
        #[derive(Debug, PartialEq, Clone)]
        struct TestStruct {
            id: u32,
            name: String,
        }
        
        let mut stack = LinkedStack::new();
        
        let s1 = TestStruct { id: 1, name: "First".to_string() };
        let s2 = TestStruct { id: 2, name: "Second".to_string() };
        
        stack.push(s1.clone());
        stack.push(s2.clone());
        
        assert_eq!(stack.get().unwrap(), &s2);
        assert_eq!(stack.pop().unwrap(), s2);
        assert_eq!(stack.pop().unwrap(), s1);
    }

    #[test]
    fn test_option_elements() {
        let mut stack = LinkedStack::new();
        
        stack.push(Some(1));
        stack.push(None);
        stack.push(Some(3));
        
        assert_eq!(stack.get().unwrap(), &Some(3));
        assert_eq!(stack.pop().unwrap(), Some(3));
        assert_eq!(stack.pop().unwrap(), None);
        assert_eq!(stack.pop().unwrap(), Some(1));
    }
}

#[cfg(test)]
mod memory_safety_tests {
    use super::*;

    #[test]
    fn test_drop_semantics() {
        let mut stack = LinkedStack::new();
        
        // 创建一些拥有资源的对象
        stack.push(vec![1, 2, 3]);
        stack.push(vec![4, 5]);
        
        // 正常弹出应该释放资源
        let vec1 = stack.pop().unwrap();
        assert_eq!(vec1, vec![4, 5]);
        
        let vec2 = stack.pop().unwrap();
        assert_eq!(vec2, vec![1, 2, 3]);
        
        // 栈应该为空
        assert_eq!(stack.len, 0);
        assert!(stack.pop().is_err());
    }

    #[test]
    fn test_multiple_operations_memory_safety() {
        let mut stack = LinkedStack::new();
        
        // 混合的推入和弹出操作
        for i in 0..100 {
            stack.push(i);
            if i % 3 == 0 {
                let _ = stack.pop();
            }
        }
        
        // 最终应该有一些元素剩余
        assert!(stack.len > 0);
        
        // 清空栈
        while stack.pop().is_ok() {}
        assert_eq!(stack.len, 0);
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_performance_basic() {
        let mut stack = LinkedStack::new();
        const COUNT: usize = 10000;
        
        let start = Instant::now();
        
        // 推入操作
        for i in 0..COUNT {
            stack.push(i);
        }
        
        // 弹出操作
        for _ in 0..COUNT {
            let _ = stack.pop();
        }
        
        let duration = start.elapsed();
        println!("Performance test took: {:?}", duration);
        
        assert_eq!(stack.len, 0);
    }
}