#![allow(dead_code)]
#![allow(unused)]

#[cfg(test)]
mod test {
    // 本质上Rc<T>是指向底层数据的不可变引用
    use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    fn test1() {
        let s = String::from("aaa");
        let a = Box::new(s);
        // let b = Box::new(s); // error
    }

    #[test]
    fn test2() {
        let s: Rc<String> = Rc::new(String::from("hello world")); // 创建智能指针
        let b: Rc<String> = Rc::clone(&s); // 复制智能指针
        let c: Rc<String> = Rc::clone(&s);

        assert_eq!(3, Rc::strong_count(&s));
        assert_eq!(Rc::strong_count(&s), Rc::strong_count(&b));
    }

    #[test]
    fn test3() {
        let a = Rc::new(String::from("test ref counting"));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Rc::clone(&a);
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Rc::clone(&a);
            println!("count after creating c = {}", Rc::strong_count(&c));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    #[test]
    fn test4() {
        struct Owner {
            name: String,
            // ...其它字段
        }

        struct Gadget {
            id: i32,
            owner: Rc<Owner>,
            // ...其它字段
        }

        // 创建一个基于引用计数的 `Owner`.
        let gadget_owner: Rc<Owner> = Rc::new(Owner {
            name: "Gadget Man".to_string(),
        });

        // 创建两个不同的工具，它们属于同一个主人
        let gadget1 = Gadget {
            id: 1,
            owner: Rc::clone(&gadget_owner),
        };
        let gadget2 = Gadget {
            id: 2,
            owner: Rc::clone(&gadget_owner),
        };

        // 释放掉第一个 `Rc<Owner>`
        drop(gadget_owner);

        // 尽管在上面我们释放了 gadget_owner，但是依然可以在这里使用 owner 的信息
        // 原因是在 drop 之前，存在三个指向 Gadget Man 的智能指针引用，上面仅仅
        // drop 掉其中一个智能指针引用，而不是 drop 掉 owner 数据，外面还有两个
        // 引用指向底层的 owner 数据，引用计数尚未清零
        // 因此 owner 数据依然可以被使用
        println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
        println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

        // 在函数最后，`gadget1` 和 `gadget2` 也被释放，最终引用计数归零，随后底层
        // 数据也被清理释放
    }

    #[test]
    fn test5() {
        let s = Rc::new(String::from("new "));
        for _ in 1..10 {
            let s = Rc::clone(&s);
            // let handle = std::thread::spawn(move || {
            //     println!("{}", s); // `Send` is not implemented for `Rc<String>`
            // })
        }
    }

    #[test]
    fn test6() {
        use std::sync::Arc;
        let s: Arc<String> = Arc::new(String::from("asd"));
        for _ in 1..10 {
            let s: Arc<String> = Arc::clone(&s);
            let handler = std::thread::spawn(move || {
                println!("{}", s);
            });
        }
    }

    #[test]
    fn test7() {
        let rc_examples = "Rc examples".to_string();
        {
            println!("--- rc_a is created ---");

            let rc_a: Rc<String> = Rc::new(rc_examples.clone());
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            {
                println!("--- rc_a is cloned to rc_b ---");

                let rc_b: Rc<String> = Rc::clone(&rc_a);
                println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
                println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

                // Two `Rc`s are equal if their inner values are equal
                println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

                // We can use methods of a value directly
                println!("Length of the value inside rc_a: {}", rc_a.len());
                println!("Value of rc_b: {}", rc_b);

                println!("--- rc_b is dropped out of scope ---");
            }

            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            println!("--- rc_a is dropped out of scope ---");
            drop(rc_a);
        }

        // Error! `rc_examples` already moved into `rc_a`
        // And when `rc_a` is dropped, `rc_examples` is dropped together
        println!("rc_examples: {}", rc_examples);
    }
}
