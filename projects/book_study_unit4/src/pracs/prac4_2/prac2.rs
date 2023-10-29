#![allow(unused)]
#![allow(dead_code)]
#![feature(test)]

use super::prac1::demo14;

#[test]
fn demo1() {
    let rang = 11..=10;
    for i in rang.into_iter() {
        println!("{i}");
    }
}

#[test]
fn demo2() {
    let values = vec![1, 2, 3];

    for v in values.into_iter() {
        println!("{}", v)
    }

    // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
    // println!("{:?}",values);

    let values = vec![1, 2, 3];
    let _values_iter = values.iter();

    // 不会报错，因为 values_iter 只是借用了 values 中的元素
    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    // 对 values 中的元素进行可变借用
    let mut values_iter_mut = values.iter_mut();

    // 取出第一个元素，并修改为0
    if let Some(v) = values_iter_mut.next() {
        *v = -1;
    }

    // 输出[0, 2, 3]
    println!("{:?}", values);
}

// 消费者配适器和迭代器配适器
// 参照Spark的action算子和非action算子
#[test]
fn demo3() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // sum 获取迭代器的所有权，并不断调用next，对元素进行求和，将所有元素消耗
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // v1_iter 是借用了 v1，因此 v1 可以照常使用
    println!("{:?}", v1);

    // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
    // println!("{:?}",v1_iter);
    let v1_iter_2 = v1.iter();
    v1_iter_2.for_each(|x| println!("{}", x));
}

#[test]
fn demo4() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(9, v2.iter().sum());
}

use std::{collections::HashMap, process::id};
#[test]
fn demo5() {
    let names: [&str; 2] = ["sunface", "sunfei"];
    let ages: [i32; 2] = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

    println!("{:?}", folks);
}

#[test]
fn demo6() {
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Self {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new();
    // 每次调用next就会使count+1
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let sum: u32 = Counter::new() // [1, 2, 3, 4, 5]
        .zip(Counter::new().skip(1)) // [(1, 2),(2, 3),(3, 4),(4, 5)]
        .map(|(a, b)| a * b) // [2, 6, 12, 20]
        .filter(|x| x % 3 == 0) // [6, 12]
        .sum(); // 18
    assert_eq!(18, sum);
}

#[test]
fn demo7() {
    let v: Vec<u64> = vec![1u64, 2, 3, 4, 5, 6];
    let sum: u64 = v
        .iter()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .map(|(_, val)| val)
        .fold(0u64, |sum, acm| sum + acm);

    assert_eq!(9, sum);
}


fn sum_for(x: &[f64]) -> f64 {
    let mut result: f64 = 0f64;
    for i in 0..x.len() {
        result += x[i];
    }
    result
}

fn sum_iter(x: &[f64]) -> f64 {
    x.iter().sum::<f64>()
}

extern crate rand;
#[cfg(test)]
mod bentch {
    use super::rand;
    
}