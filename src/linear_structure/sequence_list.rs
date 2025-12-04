/*
 * @Author: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @Date: 2025-11-25 00:10:15
 * @LastEditors: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @LastEditTime: 2025-12-04 09:05:43
 * @FilePath: \Rust-Data-Struct\src\linear_list\sequence_list.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

/*
    这里的Const会要求传入的N必须是一个常量，并且注意，不同的 N 值会产生不同的具体类型（SeqList<T, 10> 和 SeqList<T, 20> 是不同类型）
    // ✅ 正确：使用字面量常量
    let list: SeqList<i32, 10> = SeqList::new();

    // ✅ 正确：使用const声明的常量
    const CAPACITY: usize = 20;
    let list: SeqList<String, CAPACITY> = SeqList::new();

    // ❌ 错误：不能使用运行时变量
    let size = 10;
    let list: SeqList<i32, size> = SeqList::new(); // 编译错误
*/
#[derive(Debug)]
pub struct SeqList<T: std::fmt::Debug, const N: usize> {
    data: [Option<T>; N],
    len: usize,
}

/*
    前面的<T, const N: usize>是告诉编译器impl块有哪些泛型参数是适用的
    后面的<T, N>是指定我们正在为哪个具体类型实现方法

    let list1: SeqList<i32, 10> = SeqList::new();
    let list2: SeqList<i32, 20> = SeqList::new();
    // ❌ 编译错误：不同类型不能直接赋值
    list1 = list2; // Error!
*/
impl<T: std::fmt::Debug, const N: usize> SeqList<T, N> {
    // 构造方法
    pub fn new() -> Self {
        Self {
            // const {None}是一个常量表达式，创建一个Option<T>类型的None值
            // 类似的我们还可以创建长度为N，元素都为0的元素，Rust 要求数组重复初始化语法 [value; N] 中的 value 必须是常量表达式const {None} 明确地标记这是一个常量表达式
            // data: [0, N]
            data: [const { None }; N],
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == N
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        N
    }

    // 在末尾添加元素
    // Result<T, E>是Rust标准库中的一个枚举类型，用于处理可能失败的操作，另外一个是Option<T>，它同样也是Rust标准库中的一个枚举类型，用于表示可能不存在的值，Some(T)表示存在值，包装在Some中，None表示没有值
    // 'static是rust中的一个特殊的生命周期，数据在整个程序运行期间都是有效的，这里意味着，错误消息字符串在整个程序执行过程中都存在
    pub fn push(&mut self, element: T) -> Result<(), &'static str> {
        if self.is_full() {
            // Err(E)
            Err("List is full!")
        } else {
            self.data[self.len] = Some(element);
            self.len += 1;
            // Ok(T)
            Ok(())
        }
    }
    pub fn insert(&mut self, element: T, pos: usize) -> Result<(), &'static str> {
        if self.is_full() {
            Err("List is full")
        } else if pos > self.len + 1 || pos == 0 {
            Err("Out of List!")
        } else {
            // for循环也可以写，rev()方法可以翻转迭代顺序
            for i in ((pos - 1)..=(self.len - 1)).rev() {
                // 这样直接移动元素是不行的，没有实现Copy trait，应该使用更加安全的方法，获取元素并存放，这里使用的是take()方法
                // self.data[i+1] = self.data[i];
                self.data[i + 1] = self.data[i].take();
            }
            self.data[pos - 1] = Some(element);
            self.len += 1;
            Ok(())
        }
    }

    pub fn get(&mut self, pos: usize) -> Option<&T> {
        if pos > self.len || pos < 1 {
            None
        } else {
            self.data[pos - 1].as_ref()
        }
    }

    pub fn remove(&mut self, pos: usize) -> Result<Option<T>, &'static str> {
        if pos < 1 && pos >= self.len {
            Err("Out of List")
        } else if self.is_empty() {
            Err("List is empty")
        } else {
            let removed = self.data[pos - 1].take();
            for i in pos..=(self.len - 1) {
                self.data[i - 1] = self.data[i].take();
            }
            self.len -= 1;
            Ok(removed)
        }
    }

    pub fn print(&self) {
        println!("{:?}", self.data);
    }
}
