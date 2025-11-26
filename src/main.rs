/*
 * @Author: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @Date: 2025-11-24 22:58:14
 * @LastEditors: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @LastEditTime: 2025-11-25 21:27:23
 * @FilePath: \Rust-Data-Struct\src\main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */


use crate::linear_list::sequence_list::SeqList;

mod linear_list;

const N: usize = 20;
fn main() {
    let mut seq_list: SeqList<i32, N> = SeqList::new();
    for i in 1..=5 {
        seq_list.push(i);
    }
    seq_list.print();
    seq_list.insert(12, 6);
    seq_list.print();
    let e = seq_list.get(3).unwrap();
    println!("e = {}", e);
    let result = seq_list.remove(6);
    println!("result = {:?}", result);
    seq_list.print();
    println!("Max length of seq_list = {}", seq_list.capacity());
    println!("Length of seq_list = {}", seq_list.len());
}
