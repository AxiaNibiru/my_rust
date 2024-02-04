#![allow(unused)]
#![allow(dead_code)]

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use std::sync::mpsc::Sender;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test1() {
        let v = Rc::new(5);
        let t = thread::spawn(move || {
            // the trait `Send` is not implemented for `Rc<i32>`
            // println!("{}", v); // `std::rc::Rc<i32>` cannot be sent between threads safely
        });

        t.join().unwrap();
    }

    // Rc和Arc对比：
    //// Rc源码片段
    // impl<T: ?Sized> !marker::Send for Rc<T> {}
    // impl<T: ?Sized> !marker::Sync for Rc<T> {}
    //
    // // Arc源码片段
    // unsafe impl<T: ?Sized + Sync + Send> Send for Arc<T> {}
    // unsafe impl<T: ?Sized + Sync + Send> Sync for Arc<T> {}
    #[test]
    fn test2() {
        let v = Arc::new(5);
        let t = thread::spawn(move || {
            println!("{}", v);
        });

        t.join().unwrap();
    }

    #[derive(Debug)]
    struct MyBox(*mut u8);

    unsafe impl Send for MyBox {}

    #[test]
    fn test3() {
        // let p = 5 as *mut u8;
        // let t = thread::spawn(move || {
        //     println!("{:?}", p); // `*mut u8` cannot be sent between threads safely
        // });

        let p = MyBox(5 as *mut u8);
        let t = thread::spawn(move || {
            println!("{:?}", p);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn test4() {
        let v = 5;
        let t = thread::spawn(move || {
            println!("{:?}",&v);
        });

        t.join().unwrap();
    }




}
