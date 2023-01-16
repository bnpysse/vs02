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
    println!("By get_2: {}, {:?}", 10, get_2(10));
    return (x+1, x+2);
}
fn main() {
    get_sum(12, 10);
    println!("By get_sum_2: {} + {} = {}", 12, 10, get_sum_2(12, 10));
    println!("By get_sum_3: {} + {} = {}", 12, 10, get_sum_3(12, 10));

    println!("By get_2: {}, {:?}", 10, get_2(10));
    
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