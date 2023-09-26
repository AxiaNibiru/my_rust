#[allow(unused)]
#[allow(dead_code)]
use num::complex::{ComplexFloat, ParseComplexError};

#[allow(unused)]
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


#[allow(unused)]
enum List {
    Cons(u32, Box<List>),
    Nil,
}

use self::List::*;
#[allow(unused)]
#[allow(dead_code)]
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

#[allow(unused)]
#[allow(dead_code)]
fn prac_2_4_4() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("len: {}", list.len());
    println!("{}", list.stringify());
}

#[derive(Debug)]
#[allow(unused)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[allow(unused, dead_code)]
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
#[allow(unused, dead_code)]
struct Rectangle {
    width: u32,
    length: u32,
}

#[allow(unused, dead_code)]
fn prac_2_4_3() {
    let rec = Rectangle {
        width: 10u32,
        length: 10u32,
    };
    println!("{:?}", &rec);

    let rec = dbg!(rec); //dbg!拿走传入表达式的所有权，打印相应的文件名 ，行号等信息。并最终将表达式的所有权返回。
}

#[allow(unused, dead_code)]
fn prac_2_4_1() {
    let a = String::from("1");
    let b = String::from("2");
    let result = a + &b;

    let c: Box<str> = "C".into();
}
