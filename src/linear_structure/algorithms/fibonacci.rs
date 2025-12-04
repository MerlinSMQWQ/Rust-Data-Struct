/*
 * @Author: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @Date: 2025-12-04 08:39:09
 * @LastEditors: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @LastEditTime: 2025-12-04 09:34:57
 * @FilePath: \Rust-Data-Struct\src\linear_structure\algorithms\Fibonacci.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
//  经典递归算法解决的问题斐波那契数列
pub fn fibonacci(n: usize) -> Result<usize, &'static str>{
    if n >= 50 {
        Err("问题规模太大了")
    } else {
        if n == 0 {
            Ok(0)
        } else if n == 1 {
            Ok(1)
        } else {
            Ok(fibonacci(n-1).unwrap() + fibonacci(n-2).unwrap())
        }
    }
}
