#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::arch::x86_64::{__cpuid_count, _mm256_broadcast_pd};
use std::cmp::Ordering;
use std::fs::hard_link;
use std::io;
use std::path::Component::Prefix;
use std::process::exit;
use std::str::FromStr;
use rand::Rng;

const ONE_HOURS_IN_SECONDS: u32 = 60 * 60;

fn main() {

}

fn calculate_length_driver() {
    let s1 = String::from("Hello啊");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.chars().count()
}

// generate a function to study rust reference
fn rust_reference() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);
}

fn rust_move2() {
    let mut a = String::from("hello");
    let b = move_test2(&mut a); // &mut 表示的是指向可变变量a的借用
    b.push_str(" last");
    println!("{}", b);
}

fn move_test2(s: &mut String) -> &mut String {
    s.push_str(", world");
    s // move
}

fn rust_move() {
    let a = String::from("hello");
    let mut b = move_test(a); // 表示将a的所有权转移到move_test中
    b.push_str(" last");
    println!("{}", b);
}

fn move_test(mut s: String) -> String { // 以可变变量的方式传入a的所有权
    s.push_str(", world");
    s // move
}


// 对于存储在堆上的数据，由于有栈上的引用指向堆中的数据，为了不在对运行时性能造成影响，Rust永远不会自动创建数据的深拷贝
fn memory() {
    let a = String::from("hello");
    let mut b = a; // a moved here, 拷贝了指向String的引用，String的所有权被转移给了b
    let mut c = b.clone(); // copy trait -> clone，拷贝了引用和指向的Vec对象(String)
    b.push_str(", world1");
    c.push_str(", world2");
    // println!("{a}"); // value borrowed here after move
    println!("{b}");
    println!("{c}");
}

fn rust_loos() {
    let mut counter = 0;
    let loop_result = loop {
        counter += 1;
        if counter == 10 {
            break counter *2;
        }
    };
    println!("The result is {loop_result}");

    let mut counter2 = 0;
    'counting_up: loop {
        let mut  reaming = 10;
        loop {
            println!("reaming = {reaming}");
            if reaming == 9 {
                break;
            }
            if counter2 == 2 {
                break 'counting_up;
            }
            reaming -= 1;
        }
        counter2 += 1;
    }
    println!("End count = {counter2}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!");

    let a: [i32; 3] = [1, 2, 3];
    for (index, num) in a.iter().enumerate() {
        println!("index {index} number is {num}");
    }

    for i in (1..3).rev() {
        println!("{i}");
    }
    println!("LIFTOFF!!");
}

fn is_else_code() {
    let num = 6;
    let six = if num > 5 && num < 7 { num } else { 0 };
    println!("num is {}", num);
}

fn read_array() {
    let array = [1, 2, 3, 4, 5];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            print!("Please type a number!");
            exit(-1);
        }
    };

    // Rust will panic when input index out of index(error to visit memory)
    println!("{}", array[index]);
}

fn array() {
    let arr = [0; 100];
    println!("{}", arr[0]);
}

fn tup() {
    let tup: (i32, f64, i32) = (1, 6.4, 2);
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);
}

fn chars() {
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat: char = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);
}

fn variables() {
    let x = 5;
    println!("the value of x is : {}", x);
    // x = 6; error because x is immutable
    // x is i32 and i32 is Copy
    println!("the value of x is : {}", x);
}

fn last_guess() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..10);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
    }
}

//ai code guess game
fn ai_guess() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {}", guess);
}

fn guess4() {
    let mut guess = String::new();

    loop {
        println!("Guess your number:");
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let number: i32 = rand::thread_rng().gen_range(1..=10);

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num, // 成功时返回转换成功的i32 num
            Err(_) => { // 如果转化为i32失败就输出提示，并继续进行循环
                println!("Please type a number!");
                continue;
            }
        };
        // .expect("Please type a number!");

        println!("the answer is {}", number);

        match guess.cmp(&number) {
            Ordering::Greater => println!("Greater"),
            Ordering::Less => println!("Less"),
            Ordering::Equal => {
                println!("Equal");
                println!("You win!");
                break;
            }
        }
    }
}

fn guess3() {
    println!("guess: ");

    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    let answer: i32 = rand::thread_rng().gen_range(1..=100);
    println!("the answer is {}", &answer);

    match guess.cmp(&answer) {
        Ordering::Greater => println!("Greater"),
        Ordering::Less => println!("Less"),
        Ordering::Equal => println!("Equal")
    }
}

fn guess2() {
    println!("Guess the number!");

    println!("Please input your guess number: ");

    let mut input_number: String = String::new();

    std::io::stdin()
        .read_line(&mut input_number)
        .expect("Failed read line");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("the answer is {}", secret_number);

    match input_number.cmp(&secret_number.to_string()) {
        Ordering::Greater => println!("too Greater"),
        Ordering::Less => println!("too Less"),
        Ordering::Equal => println!("Equals")
    }
}


fn guess1() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess: String = String::new();

    io::stdin() // Mutex::new(BufReader::with_capacity(stdio::STDIN_BUF_SIZE, stdin_raw())) 返回BufReader带缓冲的阅读器
        .read_line(&mut guess) // io::Result<usize> 以行的方式将输入附加传递给 str guess
        .expect("Failed to read line"); // Result<T, E> 处理潜在的错误，成员包括Ok和Err

    print!("You guessed: {guess}");
    print!("You guessed: {}", guess); // 占位符打印
}