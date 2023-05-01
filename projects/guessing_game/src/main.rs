#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::arch::x86_64::_mm256_broadcast_pd;
use std::cmp::Ordering;
use std::fs::hard_link;
use std::io;
use std::path::Component::Prefix;
use std::process::exit;
use std::str::FromStr;
use rand::Rng;

const ONE_HOURS_IN_SECONDS: u32 = 60 * 60;

fn main() {
    read_array();
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
    let heart_eyed_cat: char= '😻';
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
                continue
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

        let guess : i32= match guess.trim().parse() {
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