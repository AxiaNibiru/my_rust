#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::cmp::Ordering;
use std::io;
use std::path::Component::Prefix;
use std::str::FromStr;
use rand::Rng;

fn main() {
    guess3()
}

fn  guess3() {
    println!("guess: ");

    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess : i32 = guess.trim().parse().expect("Please type a number!");

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

    match input_number.cmp(&secret_number.to_string()){
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