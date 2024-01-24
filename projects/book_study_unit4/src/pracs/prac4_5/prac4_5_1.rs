#![allow(unused)]
#![allow(dead_code)]

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use num_traits::clamp;
    use std::rc::Weak;

    #[test]
    fn test1() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
        println!("a指向的节点 = {:?}", a.tail());

        // 创建`b`到`a`的引用
        // List b.tail -> &a
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a)); // List a 被两个指针指向
        println!("b的初始化rc计数 = {}", Rc::strong_count(&b)); // 1
        println!("b指向的节点 = {:?}", b.tail());

        // 利用RefCell的可变性，创建了`a`到`b`的引用
        if let Some(link) = a.tail() {
            // a.tail = &b
            // 循环引用
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b)); // 2
        println!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a)); // 2

        // 下面一行println!将导致循环引用
        // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
        // println!("a next item = {:?}", a.tail());
    }

    #[test]
    fn test2() {
        let five = Rc::new(5);
        let weak_five = Rc::downgrade(&five);

        let strong_five: Option<Rc<i32>> = weak_five.upgrade();
        assert_eq!(5, *strong_five.unwrap());

        drop(five);
        let strong_five: Option<Rc<_>> = weak_five.upgrade();
        assert_eq!(None, strong_five);
    }

    // master
    struct Owner {
        name: String,
        gadgets: RefCell<Vec<Weak<Gadget>>>,
    }

    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
    }

    #[test]
    fn test3() {
        let gadget_owner: Rc<Owner> = Rc::new(Owner {
            name: "Axia Nibiru".to_string(),
            gadgets: RefCell::new(Vec::new()),
        });

        // Owner 和 Gadget互相包含，所以需要使用 Weak，否则就会循环引用
        let gadget1 = Rc::new(Gadget {
            id: 1,
            owner: gadget_owner.clone(),
        });

        let gadget2 = Rc::new(Gadget {
            id: 2,
            owner: gadget_owner.clone(),
        });

        for gadget_opt in gadget_owner.gadgets.borrow().iter() {
            let gadget = gadget_opt.upgrade().unwrap();
            println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
        }

        println!("{}", Rc::strong_count(&gadget_owner));
        drop(gadget1);
        drop(gadget2);
        println!("{}", Rc::strong_count(&gadget_owner));
    }

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    #[test]
    fn test4() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}", // 1, 0
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![leaf.clone()]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}", // 1, 1
                Rc::strong_count(&branch),
                Rc::weak_count(&branch)
            );

            println!(
                "leaf strong = {}, weak = {}", // 2, 0
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf)
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None
        println!(
            "leaf strong = {}, weak = {}", // 1, 0
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }
}
