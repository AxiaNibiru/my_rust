#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

mod practic;
use crate::practic::prac_2_13;
use prac_2_13::kinds::PrimaryColor;
use prac_2_13::utils::mix;
fn main() {
    let blue = PrimaryColor::Blue;
    let yellow = PrimaryColor::Yellow;
    println!("{:?}",mix(blue, yellow));
}


