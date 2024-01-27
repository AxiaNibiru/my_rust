#![allow(unused)]
#![allow(dead_code)]

#[cfg(test)]
mod test {
    struct SelfRef<'a> {
        value: String,
        pointer_to_value: &'a str,
    }

    #[test]
    fn test1() {
        let s = "aaa".to_string();
        let i = &s.clone();
        let v = SelfRef {
            value: s,
            pointer_to_value: i,
        };
    }

    #[derive(Debug)]
    struct WhatAboutThis<'a> {
        name: String,
        nickname: Option<&'a str>,
    }

    impl<'a> WhatAboutThis<'a> {
        fn tie_the_knot(&'a mut self) {
            self.nickname = Some(&self.name[..])
        }
    }

    #[test]
    fn test2() {
        let mut tricky = WhatAboutThis {
            name: "ads".to_string(),
            nickname: None,
        };

        tricky.nickname = Some(&tricky.name[..]);
        println!("{:?}", tricky);
    }
}

#[cfg(test)]
mod test2 {
    #[derive(Debug)]
    struct SelfRef {
        value: String,
        pointer_to_value: *const String,
    }

    impl SelfRef {
        fn new(txt: &str) -> Self {
            SelfRef {
                value: String::from(txt),
                pointer_to_value: std::ptr::null(),
            }
        }

        fn init(&mut self) {
            let self_ref: *const String = &self.value;
            self.pointer_to_value = self_ref;
        }

        fn value(&self) -> &str {
            &self.value
        }

        fn pointer_to_value(&self) -> &String {
            assert!(
                !self.pointer_to_value.is_null(),
                "Test::b called without Test::init being called first"
            );
            unsafe { &*(self.pointer_to_value) }
        }
    }

    #[test]
    fn test1() {
        let mut t = SelfRef::new("hello");
        t.init();
        // 打印值和指针地址
        println!("{}, {:p}", t.value(), t.pointer_to_value());
    }
}

#[cfg(test)]
mod test3 {
    use std::marker::PhantomPinned;
    use std::pin::Pin;
    use std::ptr::{null, NonNull};

    // 下面是一个自引用数据结构体，因为 slice 字段是一个指针，指向了 data 字段
    // 我们无法使用普通引用来实现，因为违背了 Rust 的编译规则
    // 因此，这里我们使用了一个裸指针，通过 NonNull 来确保它不会为 null
    struct Unmovable {
        data: String,
        slice: NonNull<String>,
        _pin: PhantomPinned,
    }

    impl Unmovable {
        // 为了确保函数返回时数据的所有权不会被转移，我们将它放在堆上，唯一的访问方式就是通过指针
        fn new(data: String) -> Pin<Box<Self>> {
            let res = Unmovable {
                data,
                // 只有在数据到位时，才创建指针，否则数据会在开始之前就被转移所有权
                slice: NonNull::dangling(),
                _pin: PhantomPinned,
            };

            let mut boxed = Box::pin(res);

            let slice = NonNull::from(&boxed.data);
            unsafe {
                let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
                let mut_ref: Pin<&mut Unmovable> = Pin::as_mut(&mut boxed);
                Pin::get_unchecked_mut(mut_ref).slice = slice
            }

            boxed
        }
    }

    #[test]
    fn test1() {
        let unmoved = Unmovable::new("aaa".to_string());
        // 只要结构体没有被转移，那指针就应该指向正确的位置，而且我们可以随意移动指针
        let mut still_moved = unmoved;
        assert_eq!(still_moved.slice, NonNull::from(&still_moved.data))
        // 因为我们的类型没有实现 `Unpin` 特征，下面这段代码将无法编译
        // let mut new_unmoved = Unmovable::new("world".to_string());
        // std::mem::swap(&mut *still_unmoved, &mut *new_unmoved);
    }

    use ouroboros::self_referencing;

    #[self_referencing]
    struct SelfRef {
        value: String,
        #[borrows(value)]
        pointer_to_value: &'this str,
    }

    #[test]
    fn test2() {
        let v = SelfRefBuilder {
            value: "asd".to_string(),
            pointer_to_value_builder: |value: &String| value,
        }
        .build();

        let s = v.borrow_value();
        let p = v.borrow_pointer_to_value();

        assert_eq!(s, *p);
    }

    // ouroboros 官方用例：
    #[self_referencing]
    struct MyStruct {
        int_data: i32,
        float_data: f32,
        #[borrows(int_data)]
        int_reference: &'this i32,
        #[borrows(mut float_data)]
        float_reference: &'this mut f32,
    }

    #[test]
    fn test3() {
        let mut my_value = MyStructBuilder {
            int_data: 42,
            float_data: 3.14,
            int_reference_builder: |int_data: &i32| int_data,
            float_reference_builder: |float_data: &mut f32| float_data,
        }
        .build();

        // Prints 42
        println!("{:?}", my_value.borrow_int_data());
        // Prints 3.14
        println!("{:?}", my_value.borrow_float_reference());
        // Sets the value of float_data to 84.0
        my_value.with_mut(|fields| {
            **fields.float_reference = (**fields.int_reference as f32) * 2.0;
        });
        println!("{:?}", my_value.borrow_float_reference());

        // We can hold on to this reference...
        let int_ref = *my_value.borrow_int_reference();
        println!("{:?}", *int_ref);
        // As long as the struct is still alive.
        drop(my_value);
        // This will cause an error!
        // println!("{:?}", *int_ref);
    }

    struct Foo<'a> {
        a: &'a String,
        b: &'a mut String,
    }
}
