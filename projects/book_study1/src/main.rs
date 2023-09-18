#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use std::{
    fmt::{format, Display},
    vec,
};
use num::range;

// 不要修改 main 中的代码
fn main() {
    prac_2_8_1();
}

fn largest<T>(list: &[T]) -> &T
    where T: std::cmp::PartialOrd {
    let mut result = list.get(0).unwrap();
    for item in list.iter() {
        if item > result { result = item; }
    }
    &result
}

fn prac_2_8_1() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let largest =  largest(&arr[..]);
    println!("{}", largest);

}

#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

impl Rectangle2 {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 方法和关联函数
fn prac_2_7() {
    let rect1 = Rectangle2 { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    struct Point {
        x: f64,
        y: f64,
    }
    impl Point {
        // 关联函数的使用方法跟构造器非常类似
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        // 另外一个关联函数，有两个参数
        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        // 这是一个方法
        // `&self` 是 `self: &Self` 的语法糖
        // `Self` 是当前调用对象的类型，对于本例来说 `Self` = `Rectangle`
        fn area(&self) -> f64 {
            // 使用点操作符可以访问 `self` 中的结构体字段
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;


            // `abs` 是一个 `f64` 类型的方法，会返回调用者的绝对值
            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        // 该方法要求调用者是可变的，`&mut self` 是 `self: &mut Self` 的语法糖
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }

    // `Pair` 持有两个分配在堆上的整数
    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        // 该方法会拿走调用者的所有权
        // `self` 是 `self: Self` 的语法糖
        fn destroy(self) {
            let Pair(first, second) = self;

            println!("Destroying Pair({}, {})", first, second);

            // `first` 和 `second` 在这里超出作用域并被释放
        }
    }

    let rectangle = Rectangle {
        // 关联函数的调用不是通过点操作符，而是使用 `::`
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 0.0);

    let pair = Pair(Box::new(10), Box::new(10));
    pair.destroy();
}

struct Cricle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Cricle {
    fn new(x: f64, y: f64, radius: f64) -> Cricle {
        Cricle {
            x,
            y,
            radius,
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.x * self.y)
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn prac_2_6_4() {
    let x = 5;
    match x {
        1..=5 => println!("1..=5"),
        _ => ()
    }

    let p = Point { x: 64, y: 67 };
    // 结构体解构
    let Point { x: a, y: b } = p;
    println!("{}", a);

    enum Message {
        Hello { id: i32 }
    }

    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => { println!("Found in rang {}", id_variable) }
        Message::Hello { id: id_variable @ 10..=12 } => { println!("Found an id in another range") }
        Message::Hello { id } => { println!("other id: {}", id) }
    }

    let mut v = String::from("hello,");
    let r = &mut v;
    match *r {
        ref mut value => value.push_str(" world!")
    }
}

fn prac_2_6_3() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(i) = stack.pop() {
        println!("{}", i);
    }

    let v = vec!['a', 'b', 'c'];
    for (i, v) in v.iter().enumerate() {
        println!("{} is {}", i, v);
    }
}

fn prac_2_4_5() {
    let arr: [String; 8] = std::array::from_fn(|_x| String::from("elem"));
    let one = [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    for a in &arrays {
        println!("{:?}", a);
        for n in a.iter() {
            println!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for n in 0..a.len() {
            sum = sum + a[n];
        }
        println!("\t{:?}, {}", a, sum);
    }
}

use crate::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    // self所有权转移到Box内部（智能指针），返回新的头部
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref next_ref) => format!("{}, {}", head, next_ref.stringify()),
            Nil => format!("Nil"),
        }
    }
}

fn prac_2_4_4() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("len: {}", list.len());
    println!("{}", list.stringify());
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn prac_2_4_2() {
    let msg = Message::Move { x: 1, y: 2 };
    if let Message::Move { x: a, y: b } = msg {
        println!("1");
        // println!("{},{}", x, y);
    } else {
        println!("2");
    }

    let mut v = vec![1, 2, 3];

    let ar: [Message; 1] = [Message::Quit];
    // let en = ("a", "b");
    // if let (a, b) = en {
    //     println!("{},{}",a,b);
    // }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn prac_2_4_3() {
    let rec = Rectangle {
        width: 10u32,
        length: 10u32,
    };
    println!("{:?}", &rec);

    let rec = dbg!(rec); //dbg!拿走传入表达式的所有权，打印相应的文件名 ，行号等信息。并最终将表达式的所有权返回。
}

fn prac_2_4_1() {
    let a = String::from("1");
    let b = String::from("2");
    let result = a + &b;

    let c: Box<str> = "C".into();
}

fn prac_2_3_2() {
    let mut s = String::from("hello");

    let p: &mut String = &mut s;

    p.push_str("string");

    let mut a = "a".to_string();
    let ref mut b = a;
}

fn tokio_demo() {}

fn practic2_3_1() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);

    let x = Box::new(5);
    let a = x;
    let t = ("hello,".to_string(), "world".to_string());
    let _s = t.0;

    println!("{:?}", "hello, ".to_string() + t.1.as_str());
}

fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    let _s = s.clone().into_bytes();
    s
}

// 只能修改下面的代码!
fn take_ownership(s: String) -> String {
    println!("{}", &s);
    s
}

fn test2_2_2() {
    let x: char = '中';
    let y: &str = "中";
    println!("size({}) is {}", x, std::mem::size_of_val(&x));
    println!("size({}) is {}", y, std::mem::size_of_val(&y));
}

// 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
// 如果使用 checked_* 方法时发生溢出，则返回 None 值
// 使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
// 使用 saturating_* 方法使值达到最小值或最大值
fn test2_2_1() {
    let a: u8 = 255;
    // 如果不使用 wrapping_* 方法，下面的代码会在编译时报错
    // 错误为：attempt to compute `u8::MAX + 20_u8`, which would overflow
    let b: u8 = a.wrapping_add(20);
    println!("{}", b);

    let x = (-43.0_f32).sqrt();
    println!("{}", x.is_nan());

    use num::complex::Complex;
    let a: Complex<f64> = Complex { re: 2.1, im: -1.2 };
    let b: Complex<f64> = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);

    let zw = String::from("中文");
    let owned_string: String = "中文".to_owned();
    let into_string: String = "中文".into();
    let zw2 = zw.to_owned();
    let zw3 = zw;

    let mut a = [1, 2, 3];
    let x = &mut a;
    {
        let mut c = || {
            (*x)[0] = 0;
            println!("{:#?}", (*x))
        }; // 因为闭包捕获了可变变量a并做了修改，所以没有实现Copy trait
        // let y = &x;
        c();
    }
    let z = *x;
}
