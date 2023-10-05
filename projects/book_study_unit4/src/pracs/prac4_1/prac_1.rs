#![allow(dead_code)]
#![allow(unused)]

use std::clone;
///!
/// # 深入生命周期
///

#[derive(Debug)]
pub struct Foo;

impl Foo {
    pub fn mutate_and_share<'a>(&'a mut self) -> &'a Self {
        &*self
    }
    pub fn share<'a>(&'a self) {}
}

pub fn demo1() {
    let mut foo = Foo;
    'c: {
        // 传入的&mut foo生命周期应该和返回的loan生命周期一样，为<'c>所以，传入的mut foo无法再使用。
        // 因为无法同时存在一个可变和其他不可变。
        let loan: &Foo = Foo::mutate_and_share(&mut foo);
        println!("{:?}", loan);
        // foo.share();
        // println!("{:?}", foo);
        println!("{:?}", loan);
    }
}

// pub fn demo1() {
//     'b: {
//         let mut foo: Foo = Foo;
//         'c: {
//             let loan: &'c Foo = Foo::mutate_and_share::<'c>(&'c mut foo);
//             'd: {
//                 Foo::share::<'d>(&'d foo);
//             }
//             println!("{:?}", loan);
//         }
//     }
// }

pub fn demo2() {
    use std::collections::HashMap;
    use std::hash::Hash;

    pub fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> Option<&'m mut V>
    where
        K: Clone + Eq + Hash,
        V: Default,
    {
        if !map.contains_key(&key) {
            map.insert(key.clone(), V::default());
        }
        map.get_mut(&key)
    }
}

pub fn demo3() {
    fn fn_elision(x: &i32) -> &i32 {
        x
    }
    // 编译器无法推测返回的引用和传入的引用谁活得更久！
    // let closure_slision = |x: &i32| -> &i32 { x };
}

struct Interface<'b, 'a: 'b> {
    manager: &'b mut Manager<'a>,
}

impl<'b, 'a: 'b> Interface<'b, 'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a>
    where
        'a: 'b,
    {
        Interface {
            manager: &mut self.manager,
        }
    }
}

pub fn demo4() {
    let mut list: List<'_> = List {
        manager: Manager { text: "hello" },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    // 下面的调用会失败，因为同时有不可变/可变借用
    // 但是Interface在之前调用完成后就应该被释放了
    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}

// 注意：debug 需要两个具有相同生命周期的参数
fn debug<'a>(a: &'a str, b: &'a str) {
    println!("a = {a:?} b = {b:?}");
}

pub fn demo5() {
    let hello: &'static str = "hello";
    {
        let world: String = String::from("world");
        let world: &String = &world; // 'world 的生命周期比 'static 短
        debug(hello, world); // 在这里'static 自动降级为 'world
    }
}

