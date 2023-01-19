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
    
    // 切片就像数组一样,可以 索引
    // Rust在编译时知道数组的大小,但只有在运行时才知道切片的大小
    // 所以 s[i] 在运行时会引起 超出界限错误 和 恐慌(panic)
    // 用以处理这种情况的是一种方法 get
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);
    println!("first: {:?}", first);
    println!("last: {:?}", last);
    // 上面的情况返回的 First: Some(1) 和 last: None
    // last返回一个叫 None的东西,
    println!("first.is_some: {}, first.is_none: {}", first.is_some(), first.is_none());
    println!("last.is_some: {}, last.is_none: {}", last.is_some(), last.is_none());
    println!("first value: {}", first.unwrap());

    //打开 {unwrap}last ,会得到一个panic,但至少可以调用 is_some ,如果默认你有一个没有值的变量的话
    let maybe_last = slice.get(5);
    let last = if maybe_last.is_some() {
        *maybe_last.unwrap()
    }else {
        -1
    };
    // 上面的表述比较繁琐,可以用 unwrap_or ,
    // let last = *slice.get(5).unwarp_or(&-1);

    // ---------- 向量 ----------     
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    let first = v[0];
    let maybe_first = v.get(0);
    println!("v is {:?}", v);
    println!("first is {:?}", first);
    println!("maybe_first is {:?}", maybe_first);

    dump(&v);
    let slice = &v[1..];
    println!("slice of v is {:?}", slice);
    // 借用符号 & 是为了迫使向量进入切片
    // 在系统语言中，程序存储器有两种：堆和栈。在栈上分配数据非常简单，
    // 但栈是有限的，通常以MB为单位
    // 堆可以是GB，不在同一个数量级上面，但分配成本较高。而且还有一个特点：
    // 就是这样的内存必须在 使用之后释放。
    // 在所谓的管理语言（比如java，Go和所谓的脚本语言）将这些细节都隐藏在 垃圾收集器 中

    // Panic 就是 内存安全 ，它们在任何非法访问内存之前发生。这是一个C中常见的安全问题
    // 因为所有的内存访问都是 不安全 的。
    
    // 当一个Vectors被修改或创建时，它由堆分配内存，并变成该内存的拥有者。
    // 切片从vectors的内存中借用(borrow)。当vectors死亡或者drops时，切片
    // 也会跟随vectors的动作。

    // ---------- 迭代器 ----------     
    println!("\n---------- 迭代器 ----------");
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);

    // 这是对数组的第一次迭代
    // 据 Gentle Intro Rust 讲,这里会出错
    //  = note: `[{integer}; 3]` is not an iterator; maybe try calling `.iter()` or a similar method
    //  = note: required by `std::iter::IntoIterator::into_iter` 
    // 但实际情况却是通过了编译 2023年1月19日22时59分8秒
    let arr = [10, 20, 30];
    for i in arr {
        println!("The arr is {}", i);
    }
    // 正常的操作
    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("Through arr.iter() is : {}", i);
    }
    // 切片隐式转换为迭代器
    let slice = &arr;
    for i in slice {
        println!("Through slice=&arr is : {}", i);
    }
    // 一系列整数求和的例子,涉及一个 mut变量 和 循环,以下是惯用的方式
    let sum: i32 = (0..5).sum();
    println!("Sum is {}", sum);
    let sum: i64 = [10, 20, 30].iter().sum();
    println!("Through arr.iter().sum() is : {}", sum);

    // 切片的一个 windows 方法,windows可以看成是一个 滑动 的 窗口,而 chunks 却是首尾相连
    let ints = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice = &ints;
    for s in slice.windows(6) {
        println!("slice.windows() is {:?}", s);
    }
    // 块(chunks)
    for s in slice.chunks(3) {
        println!("slice.chunks() is {:?}", s);
    }
    // 有用的宏 vec! 用于初始化变量,另外可以使用  pop 去除(remove)向量尾值 
    // 扩展(extend)一个兼容的迭代器的向量
    let mut v1 = vec![10, 20, 30, 40];
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    assert_eq!(v1, v2);

    v2.extend(0..2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]);

    // 验证向量,它们之间的每个对应值都相互比较,切片为值.可以将值插入到向量中的任意位置
    // {insert} 或者 {remove} ,但仍保留其旧容量,所以用 push 等来填充,只会当尺寸大于
    // 该容量时,才会重新分配容量.
    // vec 可以排序,然后可以删除重复 dedup 的值 .如果想复制,可使用 clone
    let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    v1.sort();
    v1.dedup();
    assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);


}

fn dump(arr: &[i32]) {
    println!("arr is {:?}", arr);
}
pub fn linear_search<T>(arr: &[T], target: &T) -> Option<usize> 
    where T:PartialEq {
    for (index, item) in arr.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}
pub fn linear_search_inline<T>(arr: &[T], obj: &T) -> Option<usize>
    where T: PartialEq {
    arr.iter().position(|x| x == obj)
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