#[allow(unused)]
fn test2_2_2() {
    let x: char = '中';
    let y: &str = "中";
    println!("size({}) is {}", x, std::mem::size_of_val(&x));
    println!("size({}) is {}", y, std::mem::size_of_val(&y));
}

// 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
// 如果使用 checked_* 方法时发生溢出，则返回 None 值
// 使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
// 使用 saturating_* 方法使值达到最小值或最大值
#[allow(unused)]
fn test2_2_1() {
    let a: u8 = 255;
    // 如果不使用 wrapping_* 方法，下面的代码会在编译时报错
    // 错误为：attempt to compute `u8::MAX + 20_u8`, which would overflow
    let b: u8 = a.wrapping_add(20);
    println!("{}", b);

    let x = (-43.0_f32).sqrt();
    println!("{}", x.is_nan());

    use num::complex::Complex;
    let a: Complex<f64> = Complex { re: 2.1, im: -1.2 };
    let b: Complex<f64> = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);

    let zw = String::from("中文");
    let owned_string: String = "中文".to_owned();
    let into_string: String = "中文".into();
    let zw2 = zw.to_owned();
    let zw3 = zw;

    let mut a = [1, 2, 3];
    let x = &mut a;
    {
        let mut c = || {
            (*x)[0] = 0;
            println!("{:#?}", (*x))
        }; // 因为闭包捕获了可变变量a并做了修改，所以没有实现Copy trait
           // let y = &x;
        c();
    }
    let z = *x;
}
