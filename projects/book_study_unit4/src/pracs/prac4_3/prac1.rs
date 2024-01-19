#![allow(unused)]
#![allow(dead_code)]

use std::char::TryFromCharError;
use std::f32::consts::E;
use std::fmt::Debug;
use std::num::TryFromIntError;

#[test]
fn demo1() {
    let mut value: [i32; 2] = [1, 2];
    let p1: *mut i32 = value.as_mut_ptr();
    let first = p1 as usize;
    let second = first + 4;
    let p2 = second as *mut i32;
    unsafe {
        *p2 += 1;
    }
    assert_eq!(3, value[1]);
}

#[test]
fn demo2() {
    // use std::convert::TryInto;
    let a: u8 = 10;
    let b: u16 = 1500;
    // let b_: u8 = match b.try_into() {
    //     Ok(b1) => b1,
    //     Err(e) => {
    //         println!("{:?}", e.to_string());
    //         0
    //     }
    // };

    let b_: u8 = b.try_into().unwrap_or_else(|e: TryFromIntError| {
        println!("{:?}", e.to_string());
        0
    });

    if a < b_ {
        println!("Ten is less than one hundred.")
    }
}

fn do_stuff1<T: Clone>(value: &T) {
    let cloned: T = value.clone();
}
//
// fn do_stuff2<T: Debug>(value: &T){
//     let cloned: &T = value.clone();
//     println!("{:?}", cloned)
// }
//
// use std::sync::Arc;
// #[derive(Clone)]
// struct Container<T>(Arc<T>);
//
//
// fn clone_containers<T: Debug>(foo: &Container<i32>, bar: &Container<T>) {
//     let foo_cloned: Container<i32> = foo.clone();
//     let bar_cloned: &Container<T> = bar.clone(); // 因为T没有实现Clone Trait所以无法进行值调用，调用的是&.clone()，克隆了一个引用
//     println!("{:?}", foo_cloned.0);
//     println!("{:?}", bar_cloned.0);
// }
