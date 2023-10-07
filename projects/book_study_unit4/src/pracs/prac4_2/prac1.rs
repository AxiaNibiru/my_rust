#![allow(dead_code)]
#![allow(unused)]

/// 闭包 Closure
/// 

pub fn demo1() {
    let x: i32 = 3;
    let sum = |y: i32| x + y;
    assert_eq!(6, sum(3));
}
