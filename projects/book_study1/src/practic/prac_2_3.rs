#[allow(unused)]
fn prac_2_3_2() {
    let mut s = String::from("hello");

    let p: &mut String = &mut s;

    p.push_str("string");

    let mut a = "a".to_string();
    let ref mut b = a;
}
#[allow(unused)]
fn tokio_demo() {}

#[allow(unused)]
fn practic2_3_1() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);

    let x = Box::new(5);
    let a = x;
    let t = ("hello,".to_string(), "world".to_string());
    let _s = t.0;

    println!("{:?}", "hello, ".to_string() + t.1.as_str());
}
#[allow(unused)]
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    let _s = s.clone().into_bytes();
    s
}

// 只能修改下面的代码!
fn take_ownership(s: String) -> String {
    println!("{}", &s);
    s
}
