#![allow(dead_code)]
#![allow(unused)]

use std::ops::Deref;
use std::string;
use std::{slice::from_raw_parts, str::from_utf8_unchecked};

fn get_memory_location() -> (usize, usize) {
    let string: &str = "Hello Wolrd";
    let point: usize = string.as_ptr() as usize;
    let length: usize = string.len();
    (point, length)
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

pub fn demo1() {
    let (pointer, length) = get_memory_location();
    let message: &str = get_str_at_location(pointer, length);
    println!(
        "The {} bytes as 0x{:X} stored: {}",
        length, pointer, message
    );

    // unsafe block
    // from_utf8_unchecked(from_raw_parts(pointer as *const u8, length));
}

// use std::fmt::Display;

// fn demo2() {
//     let r1;
//     let r2;
//     {
//         static STATIC_EXAMPLE: i32 = 42;
//         r1 = &STATIC_EXAMPLE;
//         let x = "&'static str";
//         r2 = x;
//         // r1 和 r2 持有的数据都是 'static 的，因此在花括号结束后，并不会被释放
//     }

//     println!("&'static i32: {}", r1); // -> 42
//     println!("&'static str: {}", r2); // -> &'static str

//     let r3: &str;

//     {
//         let s1: String = "String".to_string();

//         // s1 虽然没有 'static 生命周期，但是它依然可以满足 T: 'static 的约束
//         // 充分说明这个约束是多么的弱。。
//         static_bound(&s1);

//         // s1 是 String 类型，没有 'static 的生命周期，因此下面代码会报错
//         // r3 = &s1; // `s1` does not live long enough

//         // s1 在这里被 drop
//     }

//     // println!("{}", r3);
// }

// 参数是&T形式，所以他压根就不会关注T的实际类型
// fn static_bound<T: Display + 'static>(t: &T) {
//     println!("{}", t);
// }

use std::fmt::Debug;

#[derive(Debug)]
struct Test<'a> {
    a: i32,
    b: &'a i32,
}

fn print_it<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}
pub fn demo3() {
    // let n: i32 = 3;
    // let t = Test {a: 3, b: &n}; 报错，因为n的生命周期不够长（static）
    // 如果保持print_it对于T生命周期的'static要求的话，则
    const N: i32 = 3;
    let t = Test { a: 3, b: &N }; // 因为&N的生命周期满足&'static，满足T: 'static的要求
    print_it(&t);
}

// static 到底针对谁
//     大家有没有想过，到底是 &'static 这个引用还是该引用指向的数据活得跟程序一样久呢？
//     答案是引用指向的数据
// "I'm in ready-only memory" 字符串在代码块中被分配，而当代码块作用区间结束时，其引用被释放，但是内存中分配的值没有被释放
// 当再次输出其内存地址的时候，会发现还是在同一个字符串池中
pub fn demo4() {
    {
        let static_string: &str = "I'm in ready-only memory";
        println!("static_string: {:#p}", static_string);
    }
    println!(
        "static_string reference remain alive: {:#p}",
        "I'm in ready-only memory"
    );
    // println!("static_string reference remain alive: {:#p}", static_string);
}

#[test]
/* 使用两种方法填空 */
fn demo5() {
    static V: &str = "hello";
    need_static(V);

    println!("Success!")
}

fn need_static(r: &'static str) {
    assert_eq!(r, "hello");
}

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}

static mut CONFIG: Option<&mut Config> = None;

/* 让代码工作，但不要修改函数的签名 */
// 使用Box::leak就可以将一个运行期的值转为'static
fn init() -> Option<&'static mut Config> {
    let c: Box<Config> = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });
    Some(Box::leak(c))
}

fn demo12() {
    unsafe {
        CONFIG = init();

        println!("{:?}", CONFIG)
    }
}

#[test]
fn demo13() {
    let mut p1: *const u8;
    {
        // 字符串字面量能跟程序活得一样久，因此 `static_string` 的生命周期是 `'static`
        let static_string: &str = "I'm in read-only memory";
        p1 = static_string.as_ptr();
        println!("static_string: {:#p}", static_string.as_ptr());

        // 当 `static_string` 超出作用域时，该引用就无法再被使用，但是引用指向的数据( 字符串字面量 ) 依然保存在二进制 binary 所占用的内存中
    }

    println!(
        "static_string reference remains alive: {:#p}",
        "I'm in read-only memory"
    );
    assert_eq!(p1, "I'm in read-only memory".as_ptr());
}

use std::fmt::Display;

#[test]
fn demo14() {
    let mut string = "First".to_owned();
    string.push_str(string.to_uppercase().as_str());
    let string: &'static mut String = Box::leak(Box::new(string.clone()));
    print_a(string);
    print_b(string);
    print_c(string); // Compilation error
    print_d(string); // Compilation error
    print_e(string);
    print_f(string);
    print_g(string); // Compilation error
}

fn print_a<T: Display + 'static>(t: &T) {
    println!("{}", t);
}

fn print_b<T>(t: &T)
where
    T: Display + 'static,
{
    println!("{}", t);
}

fn print_c(t: &'static dyn Display) {
    println!("{}", t)
}

fn print_d(t: &'static impl Display) {
    println!("{}", t)
}

fn print_e(t: &(dyn Display + 'static)) {
    println!("{}", t)
}

fn print_f(t: &(impl Display + 'static)) {
    println!("{}", t)
}

fn print_g(t: &'static String) {
    println!("{}", t);
}

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` 包含对泛型类型 `T` 的引用，该泛型类型具有
// 未知的生命周期 `'a`. `T` 是约定任何
// 引用在 `T` 必须大于 `'a` 。此外，在生命周期
// 里 `Ref` 不能超过 `'a`。

// 使用 `Debug` 特征打印的通用函数。
fn print<T>(t: T)
where
    T: Debug,
{
    println!("`print`: t is {:?}", t);
}

// 这里引用 `T` 使用 where `T` 实现
// `Debug` 和所有引用 `T` 都要比 `'a` 长
// 此外，`'a`必须要比函数声明周期长
fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("`print_ref`: t is {:?}", t);
}

fn demo15() {
    let x: i32 = 7;
    let ref_x: Ref<'_, i32> = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}

struct DoubleRef<'b, 'a: 'b, T> {
    r: &'b T,
    s: &'a T,
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

/* 添加 HRTB 使下面代码正常运行！ */
fn call_on_ref_zero<F>(f: F)
where
    for<'a> F: Fn(&'a i32), // impl for &any i32 (any lifetime)
{
    let zero: i32 = 0;
    f(&zero);
}
#[test]
fn demo16() {
    println!("Success!")
}

#[test]
fn demo17() {
    let mut s: String = String::from("hello");

    let r1: &String = &s;
    let r2: &String = &s;
    println!("{} and {}", r1, r2);

    let r3: &mut String = &mut s;
    println!("{}", r3);
}

#[test]
fn demo18() {
    // Rust 2015
    struct Ref1<'a, T: 'a> {
        field: &'a T,
    }

    // Rust 2018
    struct Ref2<'a, T> {
        field: &'a T,
    }
}

/* 使下面代码正常运行 */
struct Interface<'a, 'b> {
    manager: &'b mut Manager<'a>,
}

impl Interface<'_, '_> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'a, 'b>
    where
        'a: 'b,
    {
        Interface {
            manager: &mut self.manager,
        }
    }
}

#[test]
fn demo19() {
    let mut list: List<'_> = List {
        manager: Manager { text: "hello" },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}

