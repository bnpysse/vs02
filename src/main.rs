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

    // ---------- 字符串 ----------     
    println!("\n---------- 字符串 ----------");
    // 系统语言必须具有 两种字符串,即 分配的(或者称之为动态的) 与 静态的    
    // 在rust中,比如说 Vec,动态分配并可调整大小,但是一个程序还可能包含很多的
    // string literals(字符串常量),比如 "hello"
    // 所以说, "hello" 不是 String 类型,它是 &str 类型
    // 就像 C++ 中的 const char* 与 std::string 之间的区别
    // 事实上, &str 和 String 有一个很好的相似关系,即 &[T] 到 Vec<T>
    let text = "Hellow dolly";
    let s = text.to_string();
    dump_string(text);
    dump_string(&s);
    // 借用符号 & ,可以迫使 String 成为 &str ,就像 Vec<T> 能被迫使成 &[T] 一样.
    // 实际上, String 基本是一个 Vect<u8> , &str 是一个 &[u8], 那些 字节 必须 表示有效的 UTF-8 文本.
    // 那么像 Vec 一样, 对于 String 可以同样使用 push 一个字符, 以及 pop 出 String 的结尾.

    let mut s = String::new();
    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!";

    s.pop();
    assert_eq!(s, "Hello World");
    
    // to_string可以将许多类型转换为字符串
    let arr = array_to_str(&[10, 20, 30]);
    let res = format!("hello {}", arr);
    // 这个地方还得注意呢, format! 形成的格式化串,数组元素之间是不带空格的,如果在下面的语句中,进行比较,需要注意
    assert_eq!(res, "hello [10,20,30]");
    
    // 用于切片的 .. 也可以与字符串一起工作
    let text = "staic";
    let string = "dynamic".to_string();

    let text_s = &text[1..];
    let string_s = &string[2..4];

    println!("Though slices is {:?} {:?}", text_s, string_s);
    // 但在这种情况下,是不能索引字符串的!因为其使用的是唯一真正编码 UTF-8 ,其中的"character"有可能是一个字节数
    // let multilingual = "Hi! ¡Hola! привет!";
    let multilingual = "世界,你好！";
    for ch in multilingual.chars() {
        print!("'{}' ", ch);
    }
    println!("");
    println!("len {}", multilingual.len());
    println!("count {}", multilingual.chars().count());

    let maybe = multilingual.find('п');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi {}", hi);
    }
    println!("{:?}", multilingual.chars());
    // Rust 的 char 类型是一个 4 字节的 Unicode 代码点,所以字符串不是 字符 的数组!!!!!!!!
    // 那对于 Unicode 字符串,我如何访问呢?
    println!("{}", &multilingual[0..3]);
    println!("{}", &multilingual[3..6]);
    // 这个地方是一个半角的逗号,就是一个字符
    println!("{}", &multilingual[6..7]);
    println!("{}", &multilingual[7..10]);
    println!("{}", &multilingual[10..13]);
    println!("{}", &multilingual[13..16]);
    // 对于 Unicode 还直是挺麻烦呢.我相信可以找到一个通用的方法

    // 拆解字符串
    // split_whitespace 方法返回 一个迭代器 ,然后选择如何处理它,一个主要的做法即是需要 创建拆分子串 的 Vec
    let text = "the red fox adn the lazy dog";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("the words is : {:?}", words);
    // 或者也可以这样,传递迭代器到 扩展(extend) 方法
    let mut words = Vec::new();
    words.extend(text.split_whitespace());
    println!("Though words.extend() to see the words is : {:?}", words);

    // Vec 中的每个片段,都是从原始字符串中借用的,我们所分配的是持有切片的位置
    // 使用可爱的双线 || 
    let stripped: String = text.chars()
        .filter(|ch| ! ch.is_whitespace()).collect();
    println!("Look the result of filter().collect() is : {}", stripped);
    // filer 方法接受一个 闭包 函数,这是 Rust 当中的 lambdas 函数
    // 这样可以搞定 chars 的显式循环,将返回的字符切片推送到一个可变的向量中

    // ---------- 获取命令行参数 ----------     
    println!("\n---------- 获取命令行参数 ----------");
    // std::env::args 访问命令行参数 返回一个迭代器作为字符串的参数,包括程序名
    // 2023年1月20日20时52分2秒
    for arg in std::env::args() {
        println!("{}", arg);
    }
    
    // 返回一个 Vec ,使用 collect 制作迭代器,使用该向量的 skip 方法跳过程序名
    println!("\n使用 Vec 来解决命令行参数");
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 0 {
        for arg in args.iter() {
            println!("{}", arg);
        }
    }

    // 读取单个参数,比如说传递一个整数值 
    // let first = env::args().nth(1).expect("请输入一个参数.");
    // let n: i32 = first.parse().expect("非整形参数!!");
    
    //---------- 匹配 ----------
    println!("\n---------- 匹配 ----------");
    // 这里用到一个 俄语 的字符串
    let multilingual = "Hi! ¡Hola! привет!";
    // match 包括几个模式 patterns ,用一个匹配值+=>,然后用 , 分隔
    // 必须指定所有的可能性,所以必须处理 None
    match multilingual.find('п') {
        Some(idx) => {
            let hi = &multilingual[idx..];
            println!("Russian hi {}", hi);
        },
        None => println!("Couldn't find the greeting,"),
    };
    // 如果只想做一次匹配,具 只 对一个可能的结果感兴趣,那么使用 if let 会是一个较好的方法
    if let Some(idx) = multilingual.find('п') {
        println!("Russion hi {}", &multilingual[idx..]);
    }

    // ---------- 读取文件 ----------
    println!("\n---------- 读取文件 ----------");
    // expert 就像 unwrap 一样,但可自定义一个错误信息
    use std::env;
    use std::fs::File;
    use std::io::Read;
    let first = env::args().nth(1).expect("Please supply a filename");
    let mut file = File::open(&first).expect("Can't open the file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Can't read the file");
    println!("File had {} byes.", text.len());

    println!("{:?}", good_or_bad(true));
    println!("{:?}", good_or_bad(false));
    match good_or_bad(true) {
        Ok(n) => println!("Cool, I got {}", n),
        Err(e) => println!("Huh, I just got {}", e),
    }
   
    let file = env::args().nth(1).expect("please supply a filename");
    let text = read_to_string(&file).expect("bad file man!");
    println!("file had {} bytes.", text.len());
}

fn read_to_string(filename: &str) -> Result<String,io::Error> {
    use std::io::Read;
    
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}

fn good_or_bad(good: bool) -> Result<i32, String> {
    // Result 是由 二种类型参数 定义的,分别是 Ok 和 Err
    // Result 盒子有两个隔间,一个标签是 Ok , 一个标签是 Err
    if good {
        Ok(42)
    } else {
        Err("bad".to_string())
    }
}
fn array_to_str(arr: &[i32]) -> String {
    let mut res = '['.to_string();
    for v in arr {
        // & 在 v.to_string() 表示一个字符串切片,不是 String 自身
        // v 本身是 &i32 类型,通过 to_string() 转换为 String, 再通过 & 符号转变为 &str, 让 res 的 += 语法糖
        // (亦即 add_assign 方法)操作可以成功.
        res += &v.to_string();
        res.push(',');
    }
    // 这里应该是 pop 出一个逗号, 是上面的语句中添加上的
    res.pop();
    res.push(']');
    res
}
fn dump_string(s: &str) {
    println!("Though dump_string(s: &str) is : {}", s);
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