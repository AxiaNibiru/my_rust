#![allow(dead_code)]
#![allow(unused)]

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
    let t = Test {a: 3, b: &N}; // 因为&N的生命周期满足&'static，满足T: 'static的要求
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
    println!("static_string reference remain alive: {:#p}", "I'm in ready-only memory");
    // println!("static_string reference remain alive: {:#p}", static_string);
}
