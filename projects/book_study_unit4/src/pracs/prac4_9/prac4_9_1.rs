#![allow(unused)]
#![allow(dead_code)]

#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        let mut num = 5;
        // 同时创建了可变和不可变的裸指针
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        // 创建裸指针是安全的行为，而解引用裸指针才是不安全的行为 :
        // println!("r1 is {}", *r1);
        unsafe {
            println!("r1 is {}", *r1);
        }
    }

    use std::{slice, slice::from_raw_parts, str::from_utf8_unchecked};

    // 获取合法地址
    fn get_memory_location() -> (usize, usize) {
        let string: &str = "Hello World";
        let pointer: usize = string.as_ptr() as usize;
        let length: usize = string.len();
        (pointer, length)
    }

    // 在内存中读取指定大小的字符串
    fn get_str_at_location(data: usize, length: usize) -> &'static str {
        unsafe { from_utf8_unchecked(from_raw_parts(data as *const u8, length)) }
    }

    #[test]
    fn test2() {
        let (pointer, length) = get_memory_location();
        let message = get_str_at_location(pointer, length);
        println!(
            "The {} bytes at 0x{:X} stored: {}",
            length, pointer, message
        );
        // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
        // let message = get_str_at_location(1000, 10);
        // println!("{}", message);
    }

    #[test]
    fn test3() {
        let a: Box<i32> = Box::new(32);
        let b: *const i32 = &*a as *const i32;
        let c: *const i32 = Box::into_raw(a);
    }

    // 使用unsafe分割可变数组
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid), // 以(初始地址+偏移量)的方式获取
                slice::from_raw_parts_mut(ptr.add(mid), len - mid), // ptr.add(mid) -> ptr+4*mid
            )
        }
    }

    #[test]
    fn test4() {
        let mut v: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

        let r: &mut [i32] = &mut v[..];

        let (a, b) = split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }
}
