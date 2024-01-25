#![allow(unused)]
#![allow(dead_code)]


#[cfg(test)]
mod test {
    use rand::Rng;

    #[test]
    fn test1() {
        let mut rng = rand::thread_rng();

        let n1: u8 = rng.gen();
        let n2: u16 = rng.gen();
        println!("Rand u8 {}", n1);
        println!("Rand u16 {}", n2);
        println!("Rand u32 {}", rng.gen::<u32>());
        println!("Random i32: {}", rng.gen::<i32>());
        println!("Random float: {}", rng.gen::<f64>());
    }

    #[test]
    fn test2() {
        let mut rng = rand::thread_rng();
        println!("Integer: {}", rng.gen_range(0..10));
        println!("Float: {}", rng.gen_range(0.0..10.0));
    }
}

