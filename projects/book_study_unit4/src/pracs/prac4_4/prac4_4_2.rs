#![allow(unused)]
#![allow(dead_code)]

#[cfg(test)]
mod tests4_4_2 {
    use super::*;
    use crate::pracs::prac4_1::prac_1::Foo;
    use std::ops::Deref;

    #[test]
    fn test1() {
        let a = Box::new(1889);
        assert_eq!(*a, 1889);
    }

    #[test]
    fn test2() {
        // 智能指针
        struct SmartPointer<T>(T);
        impl<T> SmartPointer<T> {
            fn new(x: T) -> SmartPointer<T> {
                SmartPointer(x)
            }
        }

        impl<T> Deref for SmartPointer<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0 // 因为需要被*所解引用，所以返回&self.0而非*self.0
            }
        }

        let sp = SmartPointer::new(5);
        assert_eq!(*sp, 5); // 实际上会进行解引用，即调用deref -> *(sp.deref())
    }

    #[test]
    fn test3() {
        // Deref隐式转换
        fn display(s: &str) {
            println!("{}", s);
        }

        let s = String::from("helloWOlrd");
        display(&s); // 通过deref自动转换&String为&str
    }

    #[test]
    fn test4() {
        fn display(s: &str) {
            println!("{}", s);
        }

        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        let s: MyBox<String> = MyBox::new(String::from("hello world"));
        display(&s); // MyBox deref -> &String deref -> &str
        display(&(*s)[..]) // 在没有deref的情况下需要使用 &(*)转换成String，再通过[..]传入切片引用
    }

    #[test]
    fn test5() {
        use std::rc::Rc;

        fn foo(s: &str) {}

        // String 实现了 Deref<Target=str>
        let owned: String = "Hello".to_string();
        // 且 Rc 智能指针可以被自动脱壳为内部的 `owned` 引用： &String ，然后 &String 再自动解引用为 &str
        let counted: Rc<String> = Rc::new(owned);

        // 因此下面的函数可以正常运行:
        foo(&counted);

        struct Foo;

        impl Foo {
            fn foo(&self) -> &'static str {
                "foo"
            }
        }

        let f = Foo {};

        assert_eq!(f.foo(), "foo");
        assert_eq!((&f).foo(), "foo");
        assert_eq!((&&f).foo(), "foo");
        assert_eq!((&&&&&&&&f).foo(), "foo");
    }
}
