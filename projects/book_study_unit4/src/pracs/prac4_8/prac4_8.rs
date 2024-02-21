#![allow(unused)]
#![allow(dead_code)]

#[cfg(test)]
mod test {
    use std::fmt::Formatter;
    use std::io::{Error, Read};
    use std::num::ParseIntError;
    use std::{fmt, num};

    #[test]
    fn test1() {
        let s1 = Some("some1");
        let s2 = Some("some2");
        let n: Option<&str> = None;

        let o1: Result<&str, &str> = Ok("ok1");
        let o2: Result<&str, &str> = Ok("ok2");
        let e1: Result<&str, &str> = Err("error1");
        let e2: Result<&str, &str> = Err("error2");

        assert_eq!(s1.or(s2), s1);
        assert_eq!(s1.or(n), s1);
        assert_eq!(n.or(s1), s1);
        assert_eq!(n.or(n), n);

        assert_eq!(o1.or(o2), o1);
        assert_eq!(o1.or(e1), o1);
        assert_eq!(e1.or(o1), o1); // Err or Ok = Ok
        assert_eq!(e1.or(e2), e2); // Err1 or Err2 = Err2

        assert_eq!(s1.and(s2), s2); // Some1 and Some2 = Some2
        assert_eq!(s1.and(n), n); // Some and None = None
        assert_eq!(n.and(s1), n); // None and Some = None
        assert_eq!(n.and(n), n); // None1 and None2 = None1

        assert_eq!(o1.and(o2), o2); // Ok1 and Ok2 = Ok2
        assert_eq!(o1.and(e1), e1); // Ok and Err = Err
        assert_eq!(e1.and(o1), e1); // Err and Ok = Err
        assert_eq!(e1.and(e2), e1); // Err1 and Err2 = Err1
    }

    #[test]
    fn test2() {
        // or_else with Option
        let s1 = Some("some1");
        let s2 = Some("some2");
        let fn_some: fn() -> Option<&'static str> = || Some("some2"); // 类似于: let fn_some = || -> Option<&str> { Some("some2") };

        let n: Option<&str> = None;
        let fn_none: fn() -> Option<&'static str> = || None;

        assert_eq!(s1.or_else(fn_some), s1); // Some1 or_else Some2 = Some1
        assert_eq!(s1.or_else(fn_none), s1); // Some or_else None = Some
        assert_eq!(n.or_else(fn_some), s2); // None or_else Some = Some
        assert_eq!(n.or_else(fn_none), None); // None1 or_else None2 = None2

        // or_else with Result
        let o1: Result<&str, &str> = Ok("ok1");
        let o2: Result<&str, &str> = Ok("ok2");
        let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

        let e1: Result<&str, &str> = Err("error1");
        let e2: Result<&str, &str> = Err("error2");
        let fn_err = |_| Err("error2");

        assert_eq!(o1.or_else(fn_ok), o1); // Ok1 or_else Ok2 = Ok1
        assert_eq!(o1.or_else(fn_err), o1); // Ok or_else Err = Ok
        assert_eq!(e1.or_else(fn_ok), o2); // Err or_else Ok = Ok
        assert_eq!(e1.or_else(fn_err), e2); // Err1 or_else Err2 = Err2

        let a: Result<&str, &str> = e1.or_else(|a| {
            let mut s = Box::new(a.to_string());
            s.push_str("asd");
            Ok(Box::leak(s))
        });
        println!("{:?}", a);
    }

    #[test]
    fn test3() {
        // and_then with Option
        let s1 = Some("some1");
        let s2 = Some("some2");
        let fn_some = |_| Some("some2"); // 类似于: let fn_some = |_| -> Option<&str> { Some("some2") };

        let n: Option<&str> = None;
        let fn_none = |_| None;

        assert_eq!(s1.and_then(fn_some), s2); // Some1 and_then Some2 = Some2
        assert_eq!(s1.and_then(fn_none), n); // Some and_then None = None
        assert_eq!(n.and_then(fn_some), n); // None and_then Some = None
        assert_eq!(n.and_then(fn_none), n); // None1 and_then None2 = None1

        // and_then with Result
        let o1: Result<&str, &str> = Ok("ok1");
        let o2: Result<&str, &str> = Ok("ok2");
        let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

        let e1: Result<&str, &str> = Err("error1");
        let e2: Result<&str, &str> = Err("error2");
        let fn_err = |_| Err("error2");

        assert_eq!(o1.and_then(fn_ok), o2); // Ok1 and_then Ok2 = Ok2
        assert_eq!(o1.and_then(fn_err), e2); // Ok and_then Err = Err
        assert_eq!(e1.and_then(fn_ok), e1); // Err and_then Ok = Err
        assert_eq!(e1.and_then(fn_err), e1); // Err1 and_then Err2 = Err1
    }

    #[test]
    fn test_filter() {
        let s1 = Some(3);
        let s2 = Some(6);
        let n = None;

        let fn_is_even = |x: &i8| x % 2 == 0;

        assert_eq!(s1.filter(fn_is_even), n); // Some(3) -> 3 is not even -> None
        assert_eq!(s2.filter(fn_is_even), s2); // Some(6) -> 6 is even -> Some(6)
        assert_eq!(n.filter(fn_is_even), n); // None -> no value -> None
    }

    #[test]
    fn test_map() {
        let s1 = Some("abcde");
        let s2 = Some(5);

        let n1: Option<&str> = None;
        let n2: Option<usize> = None;

        let o1: Result<&str, &str> = Ok("abcde");
        let o2: Result<usize, &str> = Ok(5);

        let e1: Result<&str, &str> = Err("abcde");
        let e2: Result<usize, &str> = Err("abcde");

        let fn_character_count = |s: &str| s.chars().count();

        assert_eq!(s1.map(fn_character_count), s2); // Some1 map = Some2
        assert_eq!(n1.map(fn_character_count), n2); // None1 map = None2

        assert_eq!(o1.map(fn_character_count), o2); // Ok1 map = Ok2
        assert_eq!(e1.map(fn_character_count), e2); // Err1 map = Err2
    }

    #[test]
    fn test_map_err() {
        let o1: Result<&str, &str> = Ok("abcde");
        let o2: Result<&str, isize> = Ok("abcde");

        let e1: Result<&str, &str> = Err("404");
        let e2: Result<&str, isize> = Err(404);

        let fn_character_count = |s: &str| -> isize { s.parse().unwrap() }; // 该函数返回一个 isize

        // map_err 修改 Result类型中 Err 的内容
        assert_eq!(o1.map_err(fn_character_count), o2); // Ok1 map = Ok2
        assert_eq!(e1.map_err(fn_character_count), e2); // Err1 map = Err2

        // 如果非None则调用fn_closure
        const V_DEFAULT: u32 = 1;

        let s: Result<u32, ()> = Ok(10);
        let n: Option<u32> = None;
        let fn_closure = |v: u32| v + 2;

        assert_eq!(s.map_or(V_DEFAULT, fn_closure), 12);
        assert_eq!(n.map_or(V_DEFAULT, fn_closure), V_DEFAULT);
    }

    #[test]
    fn test_map_or() {
        const V_DEFAULT: u32 = 1;

        let s: Result<u32, ()> = Ok(10);
        let n: Option<u32> = None;
        let fn_closure = |v: u32| v + 2;
        //非None则调用闭包，None则返回默认值
        assert_eq!(s.map_or(V_DEFAULT, fn_closure), 12);
        assert_eq!(n.map_or(V_DEFAULT, fn_closure), V_DEFAULT);

        let s = Some(10);
        let n: Option<i8> = None;

        let fn_closure = |v: i8| v + 2;
        let fn_default = || 1;

        // map_or_else 与 map_or 类似，但是它是通过一个闭包来提供默认值:
        assert_eq!(s.map_or_else(fn_default, fn_closure), 12);
        assert_eq!(n.map_or_else(fn_default, fn_closure), 1);

        let o = Ok(10);
        let e = Err(5);
        let fn_default_for_result = |v: i8| v + 1; // 闭包可以对 Err 中的值进行处理，并返回一个新值

        assert_eq!(o.map_or_else(fn_default_for_result, fn_closure), 12);
        assert_eq!(e.map_or_else(fn_default_for_result, fn_closure), 6);
    }

    #[test]
    fn test_ok() {
        const ERR_DEFAULT: &str = "error message";

        let s = Some("abcde");
        let n: Option<i32> = None;

        let o: Result<&str, &str> = Ok("abcde");
        let e: Result<i32, &str> = Err(ERR_DEFAULT);

        assert_eq!(s.ok_or(ERR_DEFAULT), o); // Some(T) -> Ok(T)
        assert_eq!(n.ok_or(ERR_DEFAULT), e); // None -> Err(default)

        let s = Some("abide");
        let n: Option<&str> = None;
        let fn_err_message = || "error message";

        let o: Result<&str, &str> = Ok("abide");
        let e: Result<&str, &str> = Err("error message");

        assert_eq!(s.ok_or_else(fn_err_message), o); // Some(T) -> Ok(T)
        assert_eq!(n.ok_or_else(fn_err_message), e); // None -> Err(default)
    }

    #[test]
    fn test_simple_error() {
        #[derive(Debug)]
        struct AppError;

        impl fmt::Display for AppError {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "An Error Occurred!, please Try Again!")
            }
        }

        fn produce_error() -> Result<(), AppError> {
            Err(AppError {})
        }

        match produce_error() {
            Err(e) => eprintln!("{}", e),
            _ => println!(" No Error!"),
        }

        eprintln!("{:?}", produce_error());
    }

    #[test]
    fn test_complex_error_fmt() {
        use std::fmt;

        struct AppError {
            code: usize,
            message: String,
        }

        // 根据错误码显示不同的错误信息
        impl fmt::Display for AppError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let err_msg = match self.code {
                    404 => "Sorry, Can not find the Page!",
                    _ => "Sorry, something is wrong! Please Try Again!",
                };

                write!(f, "{}", err_msg)
            }
        }

        impl fmt::Debug for AppError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "AppError {{ code: {}, message: {} }}",
                    self.code, self.message
                )
            }
        }

        fn produce_error() -> Result<(), AppError> {
            Err(AppError {
                code: 404,
                message: String::from("Page not found"),
            })
        }

        match produce_error() {
            Err(e) => eprintln!("{}", e), // 抱歉，未找到指定的页面!
            _ => println!("No error"),
        }

        eprintln!("{:?}", produce_error()); // Err(AppError { code: 404, message: Page not found })

        eprintln!("{:#?}", produce_error());
        // Err(
        //     AppError { code: 404, message: Page not found }
        // )
    }

    #[test]
    fn tet_from() {
        #[derive(Debug)]
        struct AppError {
            error_kind: String,
            message: String,
        }

        impl From<std::io::Error> for AppError {
            fn from(value: Error) -> Self {
                AppError {
                    error_kind: String::from("io"),
                    message: value.to_string(),
                }
            }
        }

        impl From<num::ParseIntError> for AppError {
            fn from(value: ParseIntError) -> Self {
                AppError {
                    error_kind: String::from("parse"),
                    message: value.to_string(),
                }
            }
        }

        fn main() -> Result<(), AppError> {
            let mut file = std::fs::File::open("not_exists_file")?; // ?将std::io::Error直接返回，然后通过From隐式转化为AppError

            let mut content = String::new();
            file.read_to_string(&mut content);

            let _num: i32 = content.parse()?;

            Ok(())
        }

        let res = main();
        println!("{:?}", res);
    }

    /// --------------- 上述代码运行后的可能输出 ---------------
    //
    // // 01. 若 hello_world.txt 文件不存在
    // Error: AppError { kind: "io", message: "No such file or directory (os error 2)" }
    //
    // // 02. 若用户没有相关的权限访问 hello_world.txt
    // Error: AppError { kind: "io", message: "Permission denied (os error 13)" }
    //
    // // 03. 若 hello_world.txt 包含有非数字的内容，例如 Hello, world!
    // Error: AppError { kind: "parse", message: "invalid digit found in string" }
    #[test]
    fn test_myself_error() {
        fn main() -> Result<(), MyError> {
            let html = render()?;
            println!("{}", html);
            Ok(())
        }

        fn render() -> Result<String, MyError> {
            let file = std::env::var("MARKDOWN")?;
            let source = std::fs::read_to_string(file)?;
            Ok(source)
        }

        #[derive(Debug)]
        enum MyError {
            EnvironmentVariableNotFound,
            IOError(std::io::Error),
        }

        impl From<std::env::VarError> for MyError {
            fn from(_: std::env::VarError) -> Self {
                Self::EnvironmentVariableNotFound
            }
        }

        impl From<std::io::Error> for MyError {
            fn from(value: std::io::Error) -> Self {
                Self::IOError(value)
            }
        }

        // 需要实现std::fmt::Display特征才能实现 std::error::Error
        impl std::error::Error for MyError {}

        impl std::fmt::Display for MyError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    MyError::EnvironmentVariableNotFound => {
                        write!(f, "Environment variable not found")
                    }

                    MyError::IOError(err) => write!(f, "IO Error: {}", err.to_string()),
                }
            }
        }
    }

    #[test]
    fn test_dyn_error() {
        fn main() -> Result<(), Box<dyn std::error::Error>> {
            let html = render()?;
            println!("{}", html);
            Ok(())
        }

        fn render() -> Result<String, Box<dyn std::error::Error>> {
            let file = std::env::var("MARKDOWN")?;
            let source = std::fs::read_to_string(file)?;
            Ok(source)
        }
    }
}

#[cfg(test)]
mod this_error {
    use std::fs::read_to_string;
    use thiserror::Error;

    #[derive(Debug, Error)]
    enum MyError {
        // 两种类型Error
        #[error("Environment variable not found")]
        EnvironmentVariableNotFound(#[from] std::env::VarError),
        // #[from] 生成 impl From<std::env::VarError> 隐式转换
        #[error(transparent)]
        IOError(#[from] std::io::Error),
    }

    #[test]
    fn test() {
        let res = main();
        println!("{:?}", res);
    }

    fn main() -> Result<(), MyError> {
        let html = render()?;
        println!("{}", html);
        Ok(())
    }

    fn render() -> Result<String, MyError> {
        let file = std::env::var("MARKDOWN")?;
        let source = read_to_string("")?;

        Ok(source)
    }
}

#[cfg(test)]
mod anyhow {
    use std::fs::read_to_string;
    use anyhow::Result;

    #[test]
    fn test() {
        let res = main();
        println!("{:?}", res);
    }

    fn main() -> Result<()> {
        let html = render()?;
        println!("{}", html);
        Ok(())
    }

    fn render() -> Result<String> {
        let file = std::env::var("asd")?;
        let html = read_to_string("not_exists")?;
        Ok(html)
    }
}
