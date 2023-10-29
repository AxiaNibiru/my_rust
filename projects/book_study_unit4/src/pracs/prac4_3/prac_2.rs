#![allow(dead_code)]
#![allow(unused)]

use std::sync::WaitTimeoutResult;

#[test]
fn demo1() {
    use std::fmt;
    struct Wrapprer(Vec<String>);

    impl fmt::Display for Wrapprer {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(","))
        }
    }

    let w = Wrapprer(vec!["hello".to_owned(), "world".to_owned()]);
    println!("{w}");
}

#[test]
fn demo2() {
    fn generic<T: ?Sized>(t: &T){}
    let a: Box<str> = "helloWOrld".into();
    
}
