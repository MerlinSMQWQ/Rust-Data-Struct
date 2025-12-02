use std::result;

/*
 * @Author: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @Date: 2025-12-02 11:22:48
 * @LastEditors: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @LastEditTime: 2025-12-02 15:04:08
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
