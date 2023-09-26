
#[derive(Debug)]
#[allow(unused, dead_code)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

impl Rectangle2 {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[allow(unused, dead_code)]
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