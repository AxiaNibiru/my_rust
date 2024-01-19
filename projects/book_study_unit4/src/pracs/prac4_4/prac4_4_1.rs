#![allow(dead_code)]
#![allow(unused)]

#[test]
pub fn test1() {
    let a = String::from("heloWorld");
    let b: Box<String> = Box::new(a);
}

#[test]
pub fn test2() {
    let a: Box<i32> = Box::new(3);
    println!("{}", a);
    // let b = a + 1; can not deref
    let b = *a + 1; // deref with *
}

#[test]
pub fn test3() {
    // enum List {
    //     Cons(i32, List), // DST 类型
    //     Nil,
    // }

    enum ListB {
        Cons(i32, Box<ListB>),
        Nil,
    }
}
#[test]
pub fn test4() {
    trait Draw {
        fn draw(&self);
    }

    struct Button {
        id: u32,
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("这是屏幕上第{}号按钮", self.id)
        }
    }

    struct Select {
        id: u32,
    }

    impl Draw for Select {
        fn draw(&self) {
            println!("这个选择框贼难用{}", self.id)
        }
    }

    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];

    for e in elems {
        e.draw()
    }
}

#[test]
pub fn test5() {
    let arr: [Box<i32>; 2] = [Box::new(3), Box::new(4)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;
    println!("{}", sum);
}

#[test]
pub fn test6() {
    fn get_static_str() -> &'static str {
        let mut s = String::from("running str");
        s.push_str("append");

        Box::leak(s.into_boxed_str())
    }
    let s = get_static_str();
    println!("{}", s);
}



