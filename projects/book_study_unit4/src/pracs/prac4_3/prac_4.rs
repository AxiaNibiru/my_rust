#![allow(unused)]
#![allow(dead_code)]

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[test]
fn test1() {
    enum MyEnum {
        A = 1,
        B,
        C,
    }

    let x = MyEnum::C as i32;

    // can not use i32 to enum
    /*    match x  {
        MyEnum::A => {},
        MyEnum::B => {},
        MyEnum::C => {},
    }*/
}

#[test]
fn test2(){
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("")
        .expect("file not find");
}

#[test]
fn test3() {
    #[derive(FromPrimitive)]
    enum MyEnum {
        A = 1,
        B,
        C,
    }

    let x = 2;

    match FromPrimitive::from_u32(x) {
        Some(MyEnum::A) => {}
        Some(MyEnum::B) => {}
        Some(MyEnum::C) => {}
        None => {}
    }
}

use std::convert::TryFrom;

enum MyEnum {
    A = 1,
    B,
    C,
}

impl TryFrom<i32> for MyEnum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == MyEnum::A as i32 => Ok(MyEnum::A),
            x if x == MyEnum::B as i32 => Ok(MyEnum::B),
            x if x == MyEnum::C as i32 => Ok(MyEnum::C),
            _ => Err(()),
        }
    }
}

#[test]
fn test4() {
    let x = 2;

    match MyEnum::try_from(x) {
        Ok(MyEnum::A) => {}
        Ok(MyEnum::B) => {}
        Ok(MyEnum::C) => {}
        Err(_) => {}
    }

    let x = MyEnum::C as i32;

    match x.try_into() {
        Ok(MyEnum::A) => println!("A!"),
        Ok(MyEnum::B) => println!("B!"),
        Ok(MyEnum::C) => println!("C!"),
        Err(_) => eprintln!("Error!"),
    }
}


#[macro_export]
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

back_to_enum! {
    enum MyEnum2 {
        A = 1,
        B,
        C,
    }
}