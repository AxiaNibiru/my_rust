#![allow(dead_code)]
#![allow(unused)]

#[cfg(test)]
mod test {
    use std::any::type_name;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test1() {
        // 线程往往是轮流执行的，但是这一点无法被保证！线程调度的方式往往取决于你使用的操作系统。
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn test2() {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || println!("Here's a vector: {:?}", v));

        handle.join().unwrap();
    }

    #[test]
    fn test3() {
        let new_thread = thread::spawn(move || {
            thread::spawn(move || loop {
                println!("I am a bad !");
            })
        });
        new_thread.join().unwrap();
        println!("Child thread is over!");

        thread::sleep(Duration::from_millis(100));
    }

    use rand::{thread_rng, Rng};
    use std::collections::HashMap;
    use std::sync::{Arc, Barrier, Condvar, Mutex};

    #[test]
    fn test4() {
        let mut handles = Vec::with_capacity(6);
        let barrier = Arc::new(Barrier::new(6));

        for _ in 0..6 {
            let b = barrier.clone();
            handles.push(thread::spawn(move || {
                println!("before wait");
                b.wait(); // 线程屏障
                println!("after wait");
            }))
        }

        for x in handles {
            x.join().unwrap();
        }

        println!("{}", current().name().unwrap());
    }

    use crate::pracs::prac4_1::prac_1::Foo;
    use chrono::Local;
    use num_traits::AsPrimitive;
    use std::cell::{Cell, RefCell};
    use std::thread::{current, JoinHandle};

    #[test]
    fn test5() {
        // 每个线程初次访问FOO时都会作为初始值开始，每个线程之间是隔离的
        thread_local! (static FOO: RefCell<i32> = RefCell::new(1));
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 2;
            println!("FOO value is {}", *f.borrow());
        });
        let t = thread::spawn(move || {
            FOO.with(|f| {
                assert_eq!(*f.borrow(), 1);
                *f.borrow_mut() = 3;
                println!("FOO value is {}", *f.borrow());
            });
        });

        t.join().unwrap();

        FOO.with(|f| {
            assert_eq!(*f.borrow(), 2);
            println!("FOO value is {}", *f.borrow());
        });
    }

    #[test]
    fn test6() {
        use std::cell::RefCell;

        struct Foo;
        impl Foo {
            thread_local! {
                static FOO: RefCell<usize> = RefCell::new(0);
            }
        }

        fn main() {
            Foo::FOO.with(|x| println!("{:?}", x));
        }

        main()
    }

    #[test]
    fn test7() {
        use std::cell::RefCell;
        use std::thread::LocalKey;

        thread_local! {
            static FOO: RefCell<usize> = RefCell::new(0);
        };

        struct Bar {
            foo: &'static LocalKey<RefCell<usize>>,
        }

        impl Bar {
            fn constructor() -> Bar {
                Self { foo: &FOO }
            }
        }
    }

    use thread_local::ThreadLocal;

    #[test]
    fn test8() {
        let tls = Arc::new(ThreadLocal::<Cell<i32>>::new());
        let mut v: Vec<JoinHandle<()>> = vec![];
        for _ in 0..5 {
            let tls2 = tls.clone();
            let handle = thread::spawn(move || {
                // 将计数器加1
                // 请注意，由于线程 ID 在线程退出时会被回收，因此一个线程有可能回收另一个线程的对象
                // 这只能在线程退出后发生，因此不会导致任何竞争条件
                let cell = tls2.get_or(|| Cell::new(0));
                cell.set(cell.get() + 1);
            });
            v.push(handle);
        }

        for h in v {
            h.join().unwrap();
        }

        let tls = Arc::try_unwrap(tls).unwrap();
        // 完成所有线程后，收集计数器值并返回所有线程本地计数器值的总和
        let total = tls.into_iter().fold(0, |x, y| {
            // 打印每个线程局部变量中的计数器值，发现不一定有5个线程，
            // 因为一些线程已退出，并且其他线程会回收退出线程的对象
            println!("x: {}, y: {}", x, y.get());
            x + y.get()
        });

        assert_eq!(total, 5);
    }

    #[test]
    fn test9() {
        let pair: Arc<(Mutex<bool>, Condvar)> =
            Arc::new((Mutex::<bool>::new(false), Condvar::new()));
        let pair2 = pair.clone();
        thread::spawn(move || {
            let (lock, cvar) = &*pair2; // *pair -> tuple &*pair -> ref tuple
            let mut started = lock.lock().unwrap();
            println!("changing started!");
            *started = true;
            cvar.notify_one();
        });
        // thread::sleep(Duration::from_secs(1));

        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            println!("waiting");
            // 挂起并等待子线程的通知，释放锁started
            started = cvar.wait(started).unwrap();
        }
        println!("started changed");
    }

    use std::sync::Once;

    static mut VAL: usize = 0;
    static INIT: Once = Once::new();

    #[test]
    fn test10() {
        let handler1 = thread::spawn(move || {
            // call_one 初始化一次，并且执只执行一次
            INIT.call_once(|| unsafe {
                VAL = 1;
            })
        });

        let handler2 = thread::spawn(move || {
            INIT.call_once(|| unsafe {
                VAL = 2;
            })
        });

        // 这里的handler1和handler2不保证执行顺序
        handler1.join().unwrap();
        handler2.join().unwrap();

        println!("{}", unsafe { VAL });
    }
}
