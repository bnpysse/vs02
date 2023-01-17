// You'll get warnings if you have unused variables, This gets rid of them.
#![allow(unused)]

//  ---------- LIBRARIES ----------   
// Define that you want to use the input/output library
use std::io;

// Generate random numbers (Add rand crate to Cargo.toml)
use rand::Rng;

// Used for working with files
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;

// Ordering compares values
use std::cmp::Ordering;

//----------Function----------  
fn say_hello(){
    println!("Hello");
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x+y);
}
fn get_sum_2(x: i32, y: i32) -> i32 {
    // 此表达式被返回
    // 如果使用了分号,将会得到一个错误,因为一个语句不能赋值给一个变量
    // error: mismatched types label: expected 'i32', found '()'
    x + y
}

fn get_sum_3(x: i32, y: i32) -> i32 {
    return x + y;
}

fn get_2(x: i32) -> (i32, i32) {
    // 如果要进行显示的话,可以使用 {:?} 来表达一个Tuple
    // println!("By get_2: {}, {:?}", 10, get_2(10));
    return (x+1, x+2);
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
   assert!(n != 0 && m != 0);
   while m != 0 {
    if m < n {
        let t = m; m = n; n = t;
    }
    m %= n;
   }
   n
}

use std::str::FromStr;
use std::env;

fn main() {
    get_sum(12, 10);
    println!("By get_sum_2: {} + {} = {}", 12, 10, get_sum_2(12, 10));
    println!("By get_sum_3: {} + {} = {}", 12, 10, get_sum_3(12, 10));
    println!("By get_2: {}, {:?}", 10, get_2(10));
    
    // Processing the command line,2023年1月16日21时52分53秒
    if false{
        let mut numbers = Vec::new();
        for arg in env::args().skip(1) {
            numbers.push(u64::from_str(&arg)
                        .expect("Error parsing argument."));
        }
        if numbers.len() == 0 {
            eprintln!("Usage: gcd NUMBER ...");
            std::process::exit(1);
        }

        let mut d = numbers[0];
        for m in &numbers[1..] {
            d = gcd(d, *m);
        }

        println!("The greatest common divisor of {:?} is {}", numbers, d);
    }

    // C 程序員读 & 为"取地址符",rust程序员则是 借用(borrow) 它.
    // 这是要学习的 Rust 的关键词.
    // 借用是编程中常见模式的名称,即:
    // 每当你通过 引用 传递(几乎问题发生在动态语言中)或在 C 中传递指针时,原始所有者所拥有的任何东西都被  借用  了
    
    // 数组用的并不多, 在Rust中常用的是 切片,可以将其看作是一个基本值数组的 快照,它们的行为很像一个数组,且 知道
    // 其尺寸. 必须使用 & 将其传递给函数
    // 如果想把数组作为一个切片传递,必须明确地使用 & 操作符

    // 你要返回,就不能加 分号(;)

    // 值可以通过  引用 方式传递,一个引用是由 & 创建的,还有用 * 解引用.
    // 如果想要一个函数来修改它的一个参数,那么请输入  可变引用, 也即是 mut,
    // 必须明确地传递参数(加上 & )和明确用 * 解引用,然后键入mut,因为它不是默认可变的

    // 不能以通常的方式 {} 打印一个数组,可以用{:?}做一个debug性质的打印输出
    let ints = [1, 2, 3];
    let floats = [1.2, 2.3, 3.4, 4.5];
    let strings = ["Hello", "World"];
    let ints_ints = [[1, 2], [10,20], [30,30],];
    println!("{:?}", ints);
    println!("{:?}", floats);
    println!("{:?}", strings);
    println!("{:?}", ints_ints);

    // 数组嵌套是没有问题的,但重要的是,数组包括的内容 只能是一个类型 
    // Slice 类似于 Python 的切片,但是有很大区别:
    //       从未有过任何数据的副本,这些 切片 都是 借用(borrow) 他们自己的数据
    

}

pub fn adder(x : i32, y : i32) -> i32 {
    x + y
}

pub fn sub(x: i32, y: i32) -> i32 {
    x - y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_gcd() {
        assert_eq!(gcd(14, 15), 1);
        assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                       3 * 7 * 11 * 13 * 19), 3 * 11);
        assert_eq!(gcd(27347, 28823), 41);
    }

    #[test] 
    fn test_add() {
        assert_eq!(adder(2, 3) , 5);
        let result = adder(23 ,11);
        assert_eq!(result ,34);
    }
    #[test]
    fn test_sub() {
        assert_eq!(sub(2, 3) ,-1 );
        let result = sub(23 ,11);
        assert_eq!(result, 12);
        let result = sub(2, -30);
        assert_eq!(result, 32);
    }
}