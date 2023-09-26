use std::io::{ErrorKind, Read};
use std::fs::File;

#[allow(unused, dead_code)]
fn prac_2_11_2() {
    
    // let f = match File::open("./file/hello.txt") {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error);
    //     },
    // };
    let path = "src\\file\\file.txt";
    let f: Result<File, std::io::Error> = File::open(path);
    let mut f: File = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {}", e),
            },
            other_error => panic!("Problem creating file: {}", other_error),
        },
    };
    let mut text = String::new();
    let file_size = f.read_to_string(&mut text).unwrap();
    println!("file size:{}, file content: {}", file_size, text);
    fn first(arr: &[i32]) -> Option<&i32> {
        arr.get(0)
    }

    // 填空并修复错误
    use std::num::ParseIntError;

    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>()?;
        let n2 = n2_str.parse::<i32>()?;
        Ok(n1 * n2)
    }

    let result = multiply("10", "2").unwrap();
    assert_eq!(result, 20);

    let result = multiply("4", "2").unwrap();
    assert_eq!(result, 8);

    // 使用两种方式填空: map, and then
    fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
        n_str.parse::<i32>().map(|i| i + 2)
    }

    fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        n1_str.parse::<i32>().and_then(|num1| {
            n2_str.parse::<i32>().map(|num2| num1 * num2)
        })
    }

    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!")
}

#[allow(unused, dead_code)]
fn prac_2_11_1() {
    fn drink(beverage: &str) {
        if beverage == "lemonade" {
            println!("Success!");
            panic!("Filad exec code, beacuse beverage == \"lemonade\"");
        }
        println!("Exercise Failed if printing out this line!")
    }
    println!("Exercise Failed if printing out this line!");
    // 修复所有的 panic，让代码工作
    assert_eq!("abc".as_bytes(), [97, 98, 99]);

    let v = vec![1, 2, 3];
    let ele = v[2];
    let ele = v.get(2).unwrap();

    // 大部分时候编译器是可以帮我们提前发现溢出错误，并阻止编译通过。但是也有一些时候，这种溢出问题直到运行期才会出现
    let v = production_rate_per_hour(2);

    divide(15, 0);

    println!("Success!");

    fn divide(x: u8, y: u8) {
        if y == 0u8 {
            println!("{}", 0u8);
        } else {
            println!("{}", x / y);
        }
    }

    fn production_rate_per_hour(speed: u8) -> f64 {
        let cph: u8 = 221;
        match speed {
            1..=4 => speed as f64 * cph as f64,
            5..=8 => speed as f64 * cph as f64 * 0.9,
            9..=10 => speed as f64 * cph as f64 * 0.77,
            _ => 0 as f64,
        }
    }

    pub fn working_items_per_minute(speed: u8) -> u32 {
        (production_rate_per_hour(speed) / 60 as f64) as u32
    }
}
