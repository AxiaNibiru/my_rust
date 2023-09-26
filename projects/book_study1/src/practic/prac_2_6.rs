#[allow(unused, dead_code)]
struct Cricle {
    x: f64,
    y: f64,
    radius: f64,
}

#[allow(unused, dead_code)]
impl Cricle {
    fn new(x: f64, y: f64, radius: f64) -> Cricle {
        Cricle { x, y, radius }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.x * self.y)
    }
}

#[allow(unused, dead_code)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(unused, dead_code)]
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

#[allow(unused, dead_code)]
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