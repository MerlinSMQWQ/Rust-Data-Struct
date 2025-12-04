/*
 * @Author: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @Date: 2025-12-02 18:09:02
 * @LastEditors: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @LastEditTime: 2025-12-04 09:37:34
 * @FilePath: \Rust-Data-Struct\src\linear_list\sequential_double_stack.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

pub struct SqDoubleStack<T, const N: usize> {
    data: [Option<T>; N],
    top0: usize,
    top1: usize
}

pub enum StackNum {
    First = 0,
    Second = 1,
}

impl<T, const N: usize> SqDoubleStack<T, N> {
    pub fn new() -> Self {
        Self {
            data: [const {None}; N],
            top0: 0,
            top1: N-1,
        }
    }

    pub fn push(&mut self, element: T, stack_num: StackNum) -> Result<(), &'static str>{
        if self.top0 + 1 == self.top1 {
            Err("stack is full!")
        } else {
            match stack_num {
                StackNum::First => {
                    self.top0+=1;
                    self.data[self.top0] = Some(element);
                }
                StackNum::Second => {
                    self.top1-=1;
                    self.data[self.top1] = Some(element);
                }
            }
            Ok(())
        }
    }

    // TODO 逻辑扁平化 
    pub fn pop(&mut self, stack_num: StackNum) -> Result<T, &'static str> {
        match stack_num {
            StackNum::First => {
                if self.top0 == 0 && self.data[self.top0].is_none() {
                    Err("Left stack is empty!")
                } else {
                    if self.top0 == 0 {
                        Ok(self.data[self.top0].take().unwrap())
                    } else {
                        self.top0 -= 1;
                        Ok(self.data[self.top0+1].take().unwrap())
                    }
                }
            }
            StackNum::Second => {
                if self.top1 == N-1 && self.data[self.top1].is_none() {
                    Err("Right stack is empty!")
                } else {
                    if self.top1 == N-1 {
                        Ok(self.data[self.top1].take().unwrap())
                    } else {
                        self.top1 += 1;
                        Ok(self.data[self.top1-1].take().unwrap())
                    }
                }
            }
        }
    }

    // TODO 逻辑扁平化
    pub fn get(&self, stack_num: StackNum) -> Result<&T, &'static str> {
        match stack_num {
            StackNum::First => {
                if self.top0 == 0 && self.data[self.top0].is_none() {
                    Err("Left stack is empty!")
                } else {
                    Ok(self.data[self.top0].as_ref().unwrap())
                }
            }
            StackNum::Second => {
                if self.top1 == N-1 && self.data[self.top1].is_none() {
                    Err("Right stack is empty!")
                } else {
                    Ok(self.data[self.top1].as_ref().unwrap())
                }
            }
        }
    }
}