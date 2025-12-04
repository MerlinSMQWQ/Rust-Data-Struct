/*
 * @Author: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @Date: 2025-11-24 22:58:14
 * @LastEditors: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @LastEditTime: 2025-12-04 09:34:31
 * @FilePath: \Rust-Data-Struct\src\main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

use crate::linear_structure::fibonacci;

pub mod linear_structure;

fn main() {
    let n = 13;
    let result = fibonacci(n);
    println!("{}", result.unwrap());
}
