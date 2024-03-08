#![allow(unused)]
#![allow(dead_code)]

#[cfg(test)]
mod test {
    use hello_macro_derive::HelloMacro;
    #[macro_export]
    macro_rules! my_vec {
    ($( $e: expr ),*) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($e);
            )*
            vec
        }
    };
}
    // 
    // #[test]
    // pub fn test1() {
    //     let mut v: Vec<i32> = my_vec!(1, 2, 3);
    // }
    // 
    // #[derive(HelloMacro)]
    // struct Foo;
    // 
    // // impl HelloMacro for Foo {
    // //     fn hello_macro() {
    // //         println!("Hello Macro! My Name is Foo");
    // //     }
    // // }
    // 
    // #[derive(HelloMacro)]
    // struct  Boo;
    // 
    // // impl HelloMacro for Boo {
    // //     fn hello_macro() {
    // //         println!("Hello Macro! My name is Boo")
    // //     }
    // // }
    // 
    // #[test]
    // fn test_macro() {
    //     Foo::hello_macro();
    // }

    
}
