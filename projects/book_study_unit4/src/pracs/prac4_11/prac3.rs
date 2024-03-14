#![allow(unused)]
#![allow(dead_code)]

/**
## 总结
* 若 T: Unpin ( Rust 类型的默认实现)，那么 Pin<'a, T> 跟 &'a mut T 完全相同，也就是 Pin 将没有任何效果, 该移动还是照常移动
* 绝大多数标准库类型都实现了 Unpin ，事实上，对于 Rust 中你能遇到的绝大多数类型，该结论依然成立 ，其中一个例外就是：async/await 生成的 Future 没有实现 Unpin
* 你可以通过以下方法为自己的类型添加 !Unpin 约束：
    * 使用文中提到的 std::marker::PhantomPinned
    * 使用nightly 版本下的 feature flag
* 可以将值固定到栈上，也可以固定到堆上
    * 将 !Unpin 值固定到栈上需要使用 unsafe
    * 将 !Unpin 值固定到堆上无需 unsafe ，可以通过 Box::pin 来简单的实现
* 当固定类型 T: !Unpin 时，你需要保证数据从被固定到被 drop 这段时期内，其内存不会变得非法或者被重用
 */
mod test {
    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
    }

    impl Test {
        fn new(txt: &str) -> Self {
            Test {
                a: String::from(txt),
                b: std::ptr::null(),
            }
        }

        fn init(&mut self) {
            let self_ref: *const String = &self.a;
            self.b = self_ref;
        }

        fn a(&self) -> &str {
            &self.a
        }

        fn b(&self) -> &String {
            assert!(
                !self.b.is_null(),
                "Test::b called without Test::init being called first"
            );
            unsafe { &*(self.b) }
        }
    }
}

mod test2 {
    use std::marker::PhantomPinned;
    use std::pin::Pin;

    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    impl Test {
        fn new(txt: &str) -> Self {
            Test {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned, // 这个标记可以让我们的类型自动实现特征`!Unpin`
            }
        }

        fn init(self: Pin<&mut Self>) {
            let self_ptr: *const String = &self.a;
            let this = unsafe { self.get_unchecked_mut() };
            this.b = self_ptr;
        }

        fn a(self: Pin<&Self>) -> &str {
            &self.get_ref().a
        }

        fn b(self: Pin<&Self>) -> &String {
            assert!(
                !self.b.is_null(),
                "Test::b called without Test::init being called first"
            );
            unsafe { &*(self.b) }
        }
    }
}

mod test3 {
    use std::marker::PhantomPinned;
    use std::pin::Pin;

    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    // 将一个 !Unpin 类型的值固定到堆上，会给予该值一个稳定的内存地址，它指向的堆中的值在 Pin 后是无法被移动的。
    // 而且与固定在栈上不同，我们知道堆上的值在整个生命周期内都会被稳稳地固定住。
    impl Test {
        fn new(txt: String) -> Pin<Box<Self>> {
            let t = Test {
                a: txt,
                b: std::ptr::null(),
                _marker: PhantomPinned, // 标记!Unpin
            };
            let mut boxed = Box::pin(t); // 被Pin标记之后，会被固定到堆上
            let self_ptr: *const String = &boxed.as_ref().a;
            unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

            boxed
        }

        fn a(self: Pin<&Self>) -> &str {
            &self.get_ref().a
        }

        fn b(self: Pin<&Self>) -> &String {
            unsafe { &*(self.b) }
        }
    }

    #[test]
    pub fn test() {
        let test1: Pin<Box<Test>> = Test::new("test1".to_string());
        let test2: Pin<Box<Test>> = Test::new("test2".to_string());

        println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
        println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
    }
}
