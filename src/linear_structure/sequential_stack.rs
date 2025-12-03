use std::result;

/*
 * @Author: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @Date: 2025-12-02 11:22:48
 * @LastEditors: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @LastEditTime: 2025-12-02 17:35:56
 * @FilePath: \Rust-Data-Struct\src\linear_list\sequential_stack.rs
 * @Description: 线性栈，但是实现的是底部入栈，也就是从data[0]处开始插入数据，实际上使用顶部入栈的实现方法会更好
 */

pub struct SequentialStack<T, const N: usize> {
    data: [Option<T>; N],
    len: usize,
}

impl<T, const N: usize> SequentialStack<T, N> {
    pub fn new() -> Self {
        Self {
            data: [const { None }; N],
            len: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.len == N
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn push(&mut self, element: T) -> Result<(), &'static str> {
        if self.is_full() {
            Err("Stack overflow!")
        } else {
            if self.len == 0 {
                self.data[0] = Some(element);
            } else {
                for i in (0..self.len).rev() {
                    self.data[i + 1] = self.data[i].take();
                }
                self.data[0] = Some(element);
            }
            self.len += 1;
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.is_empty() {
            Err("Stack is empty!")
        } else {
            let result = self.data[0].take().unwrap();
            
            for i in 0..(self.len-1) {
                self.data[i] = self.data[i+1].take();
            }

            self.len -= 1;
            Ok(result)
        }
    }

    pub fn get(&self) -> Result<&T, &'static str> {
        if self.is_empty() {
            Err("Stack is empty!")
        } else {
            Ok(self.data[0].as_ref().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack() {
        let stack: SequentialStack<i32, 5> = SequentialStack::new();
        
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.capacity(), 5);
        assert!(stack.is_empty());
        assert!(!stack.is_full());
    }

    #[test]
    fn test_push_single_element() {
        let mut stack: SequentialStack<i32, 3> = SequentialStack::new();
        
        let result = stack.push(42);
        assert!(result.is_ok());
        assert_eq!(stack.len(), 1);
        assert!(!stack.is_empty());
        assert!(!stack.is_full());
    }

    #[test]
    fn test_push_multiple_elements() {
        let mut stack: SequentialStack<i32, 3> = SequentialStack::new();
        
        assert!(stack.push(1).is_ok());
        assert!(stack.push(2).is_ok());
        assert!(stack.push(3).is_ok());
        
        assert_eq!(stack.len(), 3);
        assert!(stack.is_full());
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_push_overflow() {
        let mut stack: SequentialStack<i32, 2> = SequentialStack::new();
        
        assert!(stack.push(1).is_ok());
        assert!(stack.push(2).is_ok());
        let result = stack.push(3);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Stack overflow!");
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn test_pop_single_element() {
        let mut stack: SequentialStack<i32, 3> = SequentialStack::new();
        
        stack.push(42).unwrap();
        let result = stack.pop();
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(stack.len(), 0);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_pop_multiple_elements() {
        let mut stack: SequentialStack<i32, 3> = SequentialStack::new();
        
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        stack.push(3).unwrap();
        
        assert_eq!(stack.pop().unwrap(), 3);
        assert_eq!(stack.pop().unwrap(), 2);
        assert_eq!(stack.pop().unwrap(), 1);
        assert_eq!(stack.len(), 0);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_pop_empty_stack() {
        let mut stack: SequentialStack<i32, 3> = SequentialStack::new();
        
        let result = stack.pop();
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Stack is empty!");
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_get_top_element() {
        let mut stack: SequentialStack<i32, 3> = SequentialStack::new();
        
        stack.push(10).unwrap();
        stack.push(20).unwrap();
        
        let top = stack.get();
        assert!(top.is_ok());
        assert_eq!(*top.unwrap(), 20);
        assert_eq!(stack.len(), 2); // get should not modify stack
    }

    #[test]
    fn test_get_empty_stack() {
        let stack: SequentialStack<i32, 3> = SequentialStack::new();
        
        let result = stack.get();
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Stack is empty!");
    }

    #[test]
    fn test_lifo_order() {
        let mut stack: SequentialStack<String, 5> = SequentialStack::new();
        
        stack.push("first".to_string()).unwrap();
        stack.push("second".to_string()).unwrap();
        stack.push("third".to_string()).unwrap();
        
        assert_eq!(stack.pop().unwrap(), "third");
        assert_eq!(stack.pop().unwrap(), "second");
        assert_eq!(stack.pop().unwrap(), "first");
    }

    #[test]
    fn test_with_different_types() {
        // Test with String
        let mut string_stack: SequentialStack<String, 2> = SequentialStack::new();
        string_stack.push("hello".to_string()).unwrap();
        string_stack.push("world".to_string()).unwrap();
        assert_eq!(string_stack.pop().unwrap(), "world");
        
        // Test with custom struct
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        
        let mut point_stack: SequentialStack<Point, 2> = SequentialStack::new();
        point_stack.push(Point { x: 1, y: 2 }).unwrap();
        point_stack.push(Point { x: 3, y: 4 }).unwrap();
        assert_eq!(point_stack.pop().unwrap(), Point { x: 3, y: 4 });
    }

    #[test]
    fn test_capacity_edge_cases() {
        // Test with capacity 0
        let mut zero_stack: SequentialStack<i32, 0> = SequentialStack::new();
        assert!(zero_stack.is_full());
        assert!(zero_stack.is_empty());
        assert_eq!(zero_stack.capacity(), 0);
        assert_eq!(zero_stack.len(), 0);
        
        let push_result = zero_stack.push(1);
        assert!(push_result.is_err());
        assert_eq!(push_result.unwrap_err(), "Stack overflow!");
        
        let pop_result = zero_stack.pop();
        assert!(pop_result.is_err());
        assert_eq!(pop_result.unwrap_err(), "Stack is empty!");
        
        // Test with capacity 1
        let mut one_stack: SequentialStack<i32, 1> = SequentialStack::new();
        assert!(one_stack.push(42).is_ok());
        assert!(one_stack.is_full());
        assert!(!one_stack.is_empty());
        assert_eq!(one_stack.pop().unwrap(), 42);
        assert!(one_stack.is_empty());
        assert!(!one_stack.is_full());
    }

    #[test]
    fn test_mixed_operations() {
        let mut stack: SequentialStack<i32, 5> = SequentialStack::new();
        
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        assert_eq!(stack.pop().unwrap(), 2);
        stack.push(3).unwrap();
        stack.push(4).unwrap();
        assert_eq!(stack.pop().unwrap(), 4);
        assert_eq!(stack.pop().unwrap(), 3);
        assert_eq!(stack.pop().unwrap(), 1);
        assert!(stack.is_empty());
    }
}