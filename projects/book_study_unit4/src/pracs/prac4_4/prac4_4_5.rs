#![allow(unused)]
#![allow(dead_code)]

#[cfg(test)]
mod test {

    use std::cell::RefCell;
    use std::cell::{Cell, Ref, RefMut};
    use std::thread::sleep;

    #[test]
    fn test1() {
        let c: Cell<&str> = Cell::new("asd");
        // println!("{}", c.get());
        let a = c.get(); // copy
        c.set("qwe");
        let b = c.get();
        println!("a: {}, b: {}", a, b); // &str impl Copy trait
    }

    #[test]
    fn test2() {
        let c = Cell::new(String::from("asd"));
        // let a = c.get(); // error: doesn't satisfy `String: Copy`
    }

    // Rust 规则	                      智能指针带来的额外规则
    // 一个数据只有一个所有者	          Rc/Arc让一个数据可以拥有多个所有者
    // 要么多个不可变借用，要么一个可变借用  RefCell实现编译期可变、不可变引用共存
    // 违背规则导致编译错误	             违背规则导致运行时panic
    #[test]
    fn test3() {
        let c = RefCell::new(String::from("asd"));
        let a: Ref<String> = c.borrow();
        let b: Ref<String> = c.borrow();
        let d: Ref<String> = c.borrow();
        // let b: RefMut<String> = c.borrow_mut(); //panic: already borrowed: BorrowMutError RefCell使用时违背借用规则会导致运行期的 panic

        // println!("{}, {}", a, b);
        println!("{}", a);
    }
    #[test]
    fn test4() {
        // code snipet 1
        let x = Cell::new(1);
        let y = &x;
        let z = &x;
        x.set(2);
        y.set(3);
        z.set(4);
        println!("{}", x.get());

        // code snipet 2
        // let mut x = 1;
        // let y = &mut x;
        // let z = &mut x;
        // x = 2;
        // *y = 3;
        // *z = 4;
        // println!("{}", x);
    }

    #[test]
    fn test5() {
        trait Messenger {
            // Messenger 为外部库中定义的结构体，无法更改
            fn send(&self, msg: String);
        }

        struct MsgQueue {
            msg_cache: RefCell<Vec<String>>,
        }

        impl Messenger for MsgQueue {
            fn send(&self, msg: String) {
                self.msg_cache.borrow_mut().push(msg);
            }
        }
    }

    use std::rc::Rc;
    #[test]
    fn test6() {
        // 智能指针 + RefCell
        let s: Rc<RefCell<String>> = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

        let s1: Rc<RefCell<String>> = s.clone();
        let s2: Rc<RefCell<String>> = s.clone();
        // let mut s2 = s.borrow_mut();
        s2.borrow_mut().push_str(", oh yeah!");

        println!("{:?}\n{:?}\n{:?}", s, s1, s2);
    }

    #[test]
    fn test7() {
        fn is_even(i: i32) -> bool {
            i % 2 == 0
        }

        // fn retain_even(nums: &mut Vec<i32>) {
        //     let mut i = 0;
        //     for num in nums.iter().filter(|&num| is_even(*num)) {
        //         nums[i] = *num;
        //         i += 1;
        //     }
        //     nums.truncate(i);
        // }

        // fn retain_even(nums: &mut Vec<i32>) {
        //     let mut i = 0;
        //     for j in 0..nums.len() {
        //         if is_even(nums[j]) {
        //             nums[i] = nums[j];
        //             i += 1;
        //         }
        //     }
        //     nums.truncate(i);
        // }

        fn retain_even(nums: &mut Vec<i32>) {
            // 使用 from_mut 将 &mut [i32] 转换成 &Cell<[i32]> 再使用as_slice_of_cells 将其转换成 &[Cell<i32>]
            let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..]).as_slice_of_cells();
            let mut i: usize = 0;
            for num in slice.iter().filter(|num| is_even(num.get())) {
                slice[i].set(num.get()); // 获取的不可变引用，使用unsafe进行替换
                i += 1;
            }
            nums.truncate(i);
        }
    }
}
