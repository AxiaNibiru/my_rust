#![allow(unused)]
#![allow(dead_code)]

// 静态常量
// 常量可以在任意作用域进行定义，其生命周期贯穿整个程序的生命周期。
// 编译时编译器会尽可能将其内联到代码中，所以在不同地方对同一常量的引用并不能保证引用到相同的内存地址
// 对于变量出现重复的定义(绑定)会发生变量遮盖，后面定义的变量会遮住前面定义的变量，常量则不允许出现重复的定义
const MAX_NUM_DIV_2: usize = usize::MAX / 2;

#[test]
fn test1() {
    println!("{}", MAX_NUM_DIV_2);
}

#[test]
fn test2() {
    // 静态变量不会被内联，在整个程序中，静态变量只有一个实例，所有的引用都会指向同一个地址
    // 存储在静态变量中的值必须要实现 Sync trait
    static mut REQUEST_RECV: usize = 0;

    unsafe {
        REQUEST_RECV += 1;
        assert_eq!(REQUEST_RECV, 1);
    }
}

use std::string::ToString;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
static REQUEST_RECV: AtomicUsize = AtomicUsize::new(0);

#[test]
fn test3() {
    for _ in 1..=100 {
        REQUEST_RECV.fetch_add(1, Ordering::Relaxed);
    }
    thread::spawn(move || {
        println!("{:?}", REQUEST_RECV);
    });
    println!("当前用户请求：{:?}", REQUEST_RECV);
}

use std::sync::Mutex;

// cannot call non-const fn `<str as ToString>::to_string` in statics
// 无法使用函数进行静态初始化
// static NAMES: Mutex<String> = Mutex::new("asd".to_string()); // panic!

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env::args;
use std::iter::Once;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m: HashMap<u32, &'static str> = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

#[test]
fn test4() {
    println!("{} is {}", 0, *HASHMAP.get(&0).unwrap());
    println!("{} is {}", 1, *HASHMAP.get(&1).unwrap());
}

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut CONFIG: Option<&mut Config> = None;

#[test]
fn test5() {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    unsafe {
        // 将`c`从内存中泄漏，变成`'static`生命周期
        CONFIG = Some(Box::leak(c));
        println!("{:?}", CONFIG);
    }
}

use std::sync::OnceLock;

#[derive(Debug)]
struct Logger;

static LOGGER: OnceLock<Logger> = OnceLock::new();

impl Logger {
    fn global() -> &'static Logger {
        LOGGER.get_or_init(|| {
            println!("Logger is being created..."); // 初始化打印
            Logger
        })
    }

    fn log(&self, msg: String) {
        println!("{}", msg);
    }
}

#[test]
fn test6() {
    let handle = thread::spawn(move || {
        let logger = Logger::global();
        logger.log("thread msg".to_string());
    });

    // 主线程调用
    let logger = Logger::global();
    logger.log("some message".to_string());

    let logger2 = Logger::global();
    logger2.log("other message".to_string());

    handle.join().unwrap();
}
