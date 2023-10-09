#![allow(dead_code)]
#![allow(unused)]

/// 闭包 Closure
///

pub fn demo1() {
    let x: i32 = 3;
    let sum = |y: i32| x + y;
    assert_eq!(6, sum(3));
}

use std::thread;
use std::time::Duration;

pub fn say(intensity: u32) -> u32 {
    println!("say...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn work_out(intensity: u32, random_number: u32) {
    let action = |input: u32| {
        println!("action...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };
    if intensity < 25 {
        println!("今天活力满满，先做 {} 个俯卧撑!", action(intensity));
        println!(
            "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
            action(intensity)
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
            action(intensity)
        );
    }
}

extern crate rand;
use rand::Rng;

pub fn demo2() {
    let intensity: u32 = rand::thread_rng().gen_range(1..=100);
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("{}, {}", intensity, random_number);
    work_out(intensity, random_number);
}

// 假设我们要实现一个简易缓存，功能是获取一个值，然后将其缓存起来，那么可以这样设计：
// 一个闭包用于获取值
// 一个变量，用于存储该值
struct Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    query: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher {
            query: query,
            value: None,
        }
    }

    fn value(&mut self, args: E) -> E {
        match self.value {
            None => {
                let v = (self.query)(args);
                self.value = Some(v);
                v
            }
            Some(value) => value,
        }
    }
}

#[test]
pub fn demo3() {
    let x = 4;
    let equal_to_x = |y| y == x;
    let z = 4;
    assert!(equal_to_x(4))
}

#[test]
pub fn demo4() {
    use std::thread;
    let v: Vec<i32> = vec![1, 2, 3];
    // 使用move将v的所有权转移到另一个线程中。
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

#[test]
pub fn demo5() {
    let mut s = String::new();
    // 这个闭包因为捕获了外部mut s，所以也必须被声明为mut的
    let mut update_string = |str: &str| s.push_str(str);
    update_string("hello");

    println!("{:?}", s);
}
#[test]
pub fn demo6() {
    let mut s: String = String::new();
    // 因为没有在当前作用域内使用update_string，所以能够通过编译。
    // 如果需要使用FnMut则需要将闭包声明为mut的。
    let update_string = |str| s.push_str(str);
    // update_string("hello"); 报错，因为这个作用域内闭包没有被声明为mut的
    exec(update_string);

    println!("{:?}", s);
}

// 这里以mut方式接收f闭包，所以可以使用FnMut闭包
// 默认情况下如果闭包所捕获的所有变量都实现了Copy trait则，这个闭包会实现Copy trait
fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}

// 不可变借用
#[test]
fn demo7() {
    let s = "hello, ".to_string();
    // str deref -> String
    // immutable update_string Fn(String)
    // 捕获的变量为s，实现了Copy trait ，所以整个闭包也实现了Copy trait，下面就可以调用两次
    let update_string = |str| println!("{},{}", s, str);

    exec2(update_string);
    exec2(update_string);

    println!("{:?}", s);
}
// F: Fn trait
fn exec2<'a, F: Fn(String) -> ()>(f: F) {
    f("world".to_string())
}

// 一个闭包实现了哪种Fn特征取决于如何使用捕获的变量
/*
所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
*/
#[test]
fn demo8() {
    let s = String::new();

    let update_string = move || {
        //  s.push_str("a"); // 这里如果对s进行了修改那么就会实现FnMut trait
        println!("{}", s);
    };

    exec3(update_string);
}

fn exec3<F: FnOnce() + Fn()>(f: F) {
    f()
}

#[test]
fn demo9() {
    let s = String::new();

    let update_string = || println!("{}", s); // Fn()

    _exec(update_string);
    _exec1(update_string);
    _exec2(update_string);
}

fn _exec<F: FnOnce()>(f: F) {
    f()
}

fn _exec1<F: FnMut()>(mut f: F) {
    f()
}

fn _exec2<F: Fn()>(f: F) {
    f()
}

/*
pub trait Fn<Args> : FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

pub trait FnMut<Args> : FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait FnOnce<Args> {
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
*/

#[test]
pub fn demo10() {
    fn factory() -> impl Fn(i32) -> i32 {
        let num = 5;

        move |x| x + num
    }

    let f = factory();

    let answer = f(1);
    assert_eq!(6, answer);
}


#[test]
pub fn demo11() {
    fn factory(x:i32) -> Box<dyn Fn(i32) -> i32> {

        let num = 5;
    
        if x > 1{
            Box::new(move |x| x + num)
        } else {
            Box::new(move |x| x - num)
        }
    }
    
}

/* Make it work 
- Dont use `_reborrow` and `_count_reborrowed`
- Dont modify `assert_eq`
*/
#[test]
pub fn demo12() {
    let mut count: i32 = 0;
    println!("{:#p}", &count);

    let mut inc/* impl FnMut() */ = move || {
        count += 1;
        println!("`count`: {}", count);
        println!("{:#p}", &count);
    };

    inc();


    let _reborrow: &i32 = &count;
    println!("{:#p}", _reborrow);

    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed: &mut i32 = &mut count; 
    println!("{:#p}", _count_reborrowed);

    assert_eq!(count, 0);
    let mut a: String = "asd".to_string();
    let b: &mut str = a.as_mut();
    let c: &[u8] = a.as_ref();
    let c: &str = a.as_ref();
    let d: String = "ad".to_string();
    
}
