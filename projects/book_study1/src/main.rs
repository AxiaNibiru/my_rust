#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use std::collections::btree_map::Values;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::{ErrorKind, Read};
use std::ops::Add;
use std::ptr::write;
use std::sync::Arc;
use std::{
    fmt::{format, Display},
    vec,
};

fn main() {
    prac_2_11_2();
}

fn prac_2_11_2() {
    use std::fs::File;
    // let f = match File::open("./file/hello.txt") {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error);
    //     },
    // };
    let path = "src\\file\\file.txt";
    let f = File::open(path);
    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {}", e),
            },
            other_error => panic!("Problem creating file: {}", other_error),
        },
    };
    let mut text = String::new();
    let file_size = f.read_to_string(&mut text).unwrap();
    println!("file size:{}, file content: {}", file_size, text);
    fn first(arr: &[i32]) -> Option<&i32> {
        arr.get(0)
    }

    // 填空并修复错误
    use std::num::ParseIntError;

    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>();
        let n2 = n2_str.parse::<i32>();
        Ok(n1.unwrap() * n2.unwrap())
    }

    let result = multiply("10", "2");
    assert_eq!(result.unwrap(), 20);

    let result = multiply("t", "2");
    assert_eq!(result.unwrap(), 8);

    // 使用两种方式填空: map, and then
    fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
        n_str.parse::<i32>().map(|i| i + 2)
    }

    fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        n1_str.parse::<i32>().and_then(|num1| {
            n2_str.parse::<i32>().map(|num2| num1 * num2)
        })
    }

    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!")
}

fn prac_2_11_1() {
    fn drink(beverage: &str) {
        if beverage == "lemonade" {
            println!("Success!");
            panic!("Filad exec code, beacuse beverage == \"lemonade\"");
        }
        println!("Exercise Failed if printing out this line!")
    }
    println!("Exercise Failed if printing out this line!");
    // 修复所有的 panic，让代码工作
    assert_eq!("abc".as_bytes(), [97, 98, 99]);

    let v = vec![1, 2, 3];
    let ele = v[2];
    let ele = v.get(2).unwrap();

    // 大部分时候编译器是可以帮我们提前发现溢出错误，并阻止编译通过。但是也有一些时候，这种溢出问题直到运行期才会出现
    let v = production_rate_per_hour(2);

    divide(15, 0);

    println!("Success!");

    fn divide(x: u8, y: u8) {
        if y == 0u8 {
            println!("{}", 0u8);
        } else {
            println!("{}", x / y);
        }
    }

    fn production_rate_per_hour(speed: u8) -> f64 {
        let cph: u8 = 221;
        match speed {
            1..=4 => speed as f64 * cph as f64,
            5..=8 => speed as f64 * cph as f64 * 0.9,
            9..=10 => speed as f64 * cph as f64 * 0.77,
            _ => 0 as f64,
        }
    }

    pub fn working_items_per_minute(speed: u8) -> u32 {
        (production_rate_per_hour(speed) / 60 as f64) as u32
    }
}

fn prac_2_10() {
    fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
        if str1.len() > str2.len() {
            str1
        } else {
            str2
        }
    }
    let string1 = String::from("abc");
    let string2 = "xyz";
    let result = longest(&string1, string2);

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    struct ImportantExcerpt<'a> {
        party: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(",").next().expect("Could not find '.'");
    let i = ImportantExcerpt {
        party: first_sentence,
    };

    impl<'a> ImportantExcerpt<'a> {
        fn ann<'b>(&'a self, ann: &'b str) -> &'a str {
            println!("{}", ann);
            self.party
        }
    }

    impl<'a: 'b, 'b> ImportantExcerpt<'a> {
        fn bnn(&'a self, bnn: &'b str) -> &'b str {
            println!("{}", bnn);
            self.party
        }
    }

    impl<'a> ImportantExcerpt<'a> {
        fn cnn<'b>(&'a self, bnn: &'b str) -> &'b str
        where
            'a: 'b,
        {
            println!("{}", bnn);
            self.party
        }
    }

    fn longest_with<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: std::fmt::Display,
    {
        println!("{}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

fn prac_2_9_2() {
    use std::collections::HashMap;
    let mut hash_map: HashMap<&str, i32> = HashMap::new();
    hash_map.insert("key1", 1);
    hash_map.insert("key2", 2);
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let teams_map: HashMap<String, i32> = teams_list.into_iter().collect();
    Some(&12).copied(); // 12 -> i32，copied内容需要实现Copy trait

    let text = "hello world wonderful world";
    let mut map: HashMap<&str, usize> = HashMap::new();
    for word in text.split_ascii_whitespace() {
        let count: &mut usize = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    use std::hash::BuildHasherDefault;
    use twox_hash::XxHash64;

    let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    hash.insert(42, "the answer");
    assert_eq!(hash.get(&42), Some(&"the answer"));
    println!("{:?}", hash);

    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // 使用两种方法实现 team_map2
    // 提示:其中一种方法是使用 `collect` 方法
    // let teams_map2: HashMap<&str, i32> = teams.into_iter().collect();
    let mut teams_map2: HashMap<&str, i32> = HashMap::with_capacity(3);
    teams.into_iter().for_each(|(k, v)| {
        teams_map2.insert(k, v);
    });

    assert_eq!(teams_map1, teams_map2);

    teams_map1.entry("").or_insert_with(|| 45);
    println!("Success!");

    let mut map = HashMap::new();
    map.insert("a".to_string(), "asd");
    map.get("a");

    struct A {}
    impl A {
        fn a(a: &str) {}
        //fn a(a: String) {} Error
    }

    #[derive(PartialEq, Eq, Hash)]
    struct Viking {
        name: String,
        country: String,
    }
    impl Viking {
        fn new(name: &str, country: &str) -> Viking {
            Viking {
                name: name.to_string(),
                country: country.to_string(),
            }
        }
    }
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    assert_eq!("a", "a".to_string());
}

fn prac_2_9_1() {
    let v = vec![1, 2, 3];
    if let Some(value) = v.get(2) {
        println!("{value}");
    }

    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);

    // 实现 Ord 需要我们实现 Ord、Eq、PartialEq、PartialOrd 这些属性
    // 默认的Ord排序规则为按字段顺序来进行排序，下面的话就是先按name排序，如果相等则使用age进行排序
    #[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn new(name: String, age: u32) -> Self {
            Person {
                name: name,
                age: age,
            }
        }
    }

    let mut people = vec![
        Person::new("Zoe".to_string(), 20),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    people.sort_unstable_by(|a, b| b.age.cmp(&a.age));
    println!("{:?}", &people);
    people.sort_unstable();
    println!("{:?}", people);
    let arr: [u32; 3] = [1, 2, 3];
    arr.to_vec();
    let mut v1: Vec<u32> = Vec::new();
    arr.map(|a| v1.push(a));
    println!("{:?}", arr);
    println!("{:?}", v1);
    v1.extend(v.iter());
    let v2: Vec<u32> = arr.into();
    // let str_vec: Vec<u32> = "String".to_string().into();
    Vec::from("");

    // 修复错误并实现缺失的代码
    let mut v = Vec::from([1, 2, 3, 4, 5]);
    for i in 0..5 {
        println!("{:?}", v[i])
    }

    for i in 0..5 {
        v[i] += 1;
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!")
}

fn prac_2_8_4() {
    trait DemoTrait<T> {
        fn run(&self, a: &T);
    }

    use std::ops::Add;
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, rhs: Meters) -> Self::Output {
            Millimeters(self.0 + (rhs.0 * 1000))
        }
    }

    trait A {
        fn a(&self);
    }

    trait B: A {
        fn a(&self);
    }

    use std::fmt;
    struct Wrapper(Vec<String>);
    impl Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
}

fn prac_2_8_3() {
    trait Draw {
        fn draw(&self) {}
    }

    struct Screen<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T> Screen<T>
    where
        T: Draw,
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    trait Bird {
        fn quack(&self);
    }

    struct Duck;
    impl Duck {
        fn fly(&self) {
            println!("Look, the duck is flying")
        }
    }
    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }

    impl Bird for Duck {
        fn quack(&self) {
            println!("{}", "duck duck");
        }
    }

    impl Bird for Swan {
        fn quack(&self) {
            println!("{}", "swan swan");
        }
    }

    // 填空
    let birds: [Box<dyn Bird>; 2] = [Box::new(Duck {}), Box::new(Swan {})];

    for bird in birds {
        bird.quack();
        // 当 duck 和 swan 变成 bird 后，它们都忘了如何翱翔于天际，只记得该怎么叫唤了。。
        // 因此，以下代码会报错
        // bird.fly();
    }
}

fn largest<T>(list: &[T]) -> &T
where
    T: std::cmp::PartialOrd,
{
    let mut result = list.get(0).unwrap();
    for item in list.iter() {
        if item > result {
            result = item;
        }
    }
    &result
}

fn prac_2_8_2() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug, Clone)]
    pub struct Post {
        pub title: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{}，作者是{}", self.title, self.author)
        }
    }

    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了{}", self.username, self.content)
        }
    }

    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content: "Rust棒极了!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "好像微博没Tweet好用".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());

    pub fn notify(item: &impl Summary) {
        println!("Breaking news{}", item.summarize());
    }

    pub fn make_one<T: Copy>(a: &[T]) -> T {
        a[0]
    }

    let a: i32 = 10;
    let b: u16 = 100;
    let b_ = b.try_into().unwrap();
    let a_: u16 = a.try_into().unwrap();
    if a < b_ {
        println!("a less than b")
    }
    if a_ < b {
        println!("a less than b")
    }

    let mut a = 64;
    let a_ = &mut a;
    *a_ = 34;

    fn do_1<T: Clone + Debug>(a: &T) {
        println!("{:?}", a);
        let a_: &T = a;
        let b: T = a.clone();
        let c: T = (*a).clone();
    }

    fn do_2<T>(a: &T) {
        let a_: &T = a;
        let b: &T = a.clone();
    }
    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content: "Rust棒极了!".to_string(),
    };
    do_1(&post);
    do_2(&post);

    #[derive(Clone)]
    struct Container<T>(Arc<T>);

    fn clone_containers<T>(foo: &Container<i32>, bar: &Container<T>) {
        let foo_cloned = foo.clone();
        let bar_cloned = bar.clone();
    }

    use std::ops::Add;
    #[derive(Debug)]
    struct Point<T: Add<Output = T>> {
        x: T,
        y: T,
    }

    impl<T: Add<Output = T>> Add for Point<T> {
        type Output = Point<T>;

        fn add(self, p: Point<T>) -> Point<T> {
            Point {
                x: self.x + p.x,
                y: self.y + p.y,
            }
        }
    }

    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };
    println!("{:?}", p1 + p2);

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", p3.add(p4));
}

fn prac_2_8_1() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let largest = largest(&arr[..]);
    println!("{}", largest);

    fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }

    struct Point<T, U> {
        x: T,
        y: U,
    }
    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<V, U> {
            Point {
                x: other.x,
                y: self.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = Point::mixup(p1, p2);
    println!("{}, {}", p3.x, p3.y);

    pub fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }

    let arr = [1, 2, 3];
    let arr2 = [1, 2];
    display_array(arr);
    display_array(arr2);

    struct A; // 具体的类型 `A`.
    struct S(A); // 具体的类型 `S`.
    struct SGen<T>(T); // 泛型 `SGen`.

    fn reg_fn(_s: S) {}

    fn gen_spec_t(_s: SGen<A>) {}

    fn gen_spec_i32(_s: SGen<i32>) {}

    fn generic<T>(_s: SGen<T>) {}

    // 使用非泛型函数
    reg_fn(S(A {})); // 具体的类型
    gen_spec_t(SGen(A {})); // 隐式地指定类型参数  `A`.
    gen_spec_i32(SGen(5)); // 隐式地指定类型参数`i32`.

    // 显式地指定类型参数 `char`
    generic::<char>(SGen('a'));

    // 隐式地指定类型参数 `char`.
    generic(SGen('a'));
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
    let rect1 = Rectangle2 {
        width: 30,
        height: 50,
    };
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
        Cricle { x, y, radius }
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
        _ => (),
    }

    let p = Point { x: 64, y: 67 };
    // 结构体解构
    let Point { x: a, y: b } = p;
    println!("{}", a);

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found in rang {}", id_variable)
        }
        Message::Hello {
            id: id_variable @ 10..=12,
        } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("other id: {}", id)
        }
    }

    let mut v = String::from("hello,");
    let r = &mut v;
    match *r {
        ref mut value => value.push_str(" world!"),
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

use num::complex::{ComplexFloat, ParseComplexError};

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
