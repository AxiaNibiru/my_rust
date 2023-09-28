#![allow(dead_code)]
#![allow(unused)]
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

pub fn prac_4_1_1() {
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
