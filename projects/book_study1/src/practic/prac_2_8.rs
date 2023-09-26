#[allow(dead_code)]
#[allow(unused)]
use std::mem::Discriminant;

#[allow(unused)]
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

    use std::fmt::{self, Display};
    struct Wrapper(Vec<String>);
    impl Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
}
#[allow(unused)]
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
#[allow(unused)]
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
#[allow(unused)]
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

    use std::fmt::Debug;
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

    use std::sync::Arc;
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
#[allow(unused)]
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

