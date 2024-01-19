#![allow(dead_code)]
///!
/// # 格式化输出
/// 
pub struct Person {
    name: String,
    age: u8,
}

use std::fmt;
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "大佬在上，请受我一拜，小弟姓名{}，年芳{}，家里无田又无车，生活苦哈哈",
            self.name, self.age
        )
    }
}


pub fn prac_2_14() {
    let p: Person = Person {
        name: "sunface".to_string(),
        age: 18,
    };
    println!("{}", p);

    println!("Hello {:5}!", "x");
    // 使用参数5来指定宽度
    println!("Hello {:1$}!", "x", 5);
    // 使用x作为占位符输出内容，同时使用5作为宽度
    println!("Hello {1:0$}!", 5, "x");
    // 使用有名称的参数作为宽度
    println!("Hello {:width$}!", "x", width = 5);
    //-----------------------------------

    // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
    println!("Hello {:1$}!{}", "x", 5);

    let width: usize = 50;
    println!("{:=^1$}","格式化字符串，居中对齐", width);
    println!("{:=^width$}","格式化字符串，居中对齐", width=width);

    // let pi: f64 = std::f64::consts::PI;
    let v: f64 = 3.1415926;
    // 保留小数点后两位 => 3.14
    println!("{:.2}", v);
    // 带符号保留小数点后两位 => +3.14
    println!("{:+.2}", v);
    // 不带小数 => 3
    println!("{:.0}", v);
    // 通过参数来设定精度 => 3.1416，相当于{:.4}
    println!("{:.1$}", v, 4);

    let s: &str = "hi我是Sunface孙飞";
    // 保留字符串前三个字符 => hi我
    println!("{:.3}", s);
    // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值 => Hello abc!
    println!("Hello {:.*}!", 3, "abcdefg");
}
