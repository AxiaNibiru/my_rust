#![allow(unused)]
#![allow(dead_code)]

#[test]
fn demo1() {
    let rang = 11..=10;
    for i in rang.into_iter() {
        println!("{i}");
    }
}