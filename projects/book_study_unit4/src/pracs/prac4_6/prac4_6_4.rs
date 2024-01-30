#![allow(unused)]
#![allow(dead_code)]

#[cfg(test)]
mod test {

    use std::cell::RefCell;
    use std::sync::mpsc::{self, Receiver, Sender};
    use std::sync::{Arc, Mutex, MutexGuard};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test1() {
        let x = Mutex::new(5);
        {
            // m.lock() 会申请一个lock，阻塞直到申请成功
            let mut num: MutexGuard<i32> = x.lock().unwrap(); // deref -> &mut i32
            *num = 6;
        }

        println!("x = {}", x.lock().unwrap());
    }

    use std::rc::Rc;
    use std::thread::{sleep, JoinHandle};

    #[test]
    fn test2() {
        let counter = Arc::new(Mutex::new(0));
        let mut handlers = Vec::<JoinHandle<()>>::with_capacity(10);

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            handlers.push(thread::spawn(move || {
                *(counter).lock().unwrap() += 1;
            }));
        }

        // handlers.iter_mut().map(|f| {
        //     let counter = Arc::clone(&counter);
        //     thread::spawn(move || *(counter.lock().unwrap()) += 1)
        // });

        // 等待所有子线程完成
        for handle in handlers {
            handle.join().unwrap();
        }

        // 输出最终的计数结果
        println!("Result: {}", *counter.lock().unwrap());
    }

    use lazy_static::lazy_static;
    use ouroboros::macro_help::unbox;

    lazy_static! {
        static ref MUTEX1: Mutex<i64> = Mutex::new(0);
        static ref MUTEX2: Mutex<i64> = Mutex::new(0);
    }

    #[test]
    fn test3() {
        let mut children: Vec<JoinHandle<()>> = vec![];

        for i_thread in 0..2 {
            children.push(thread::spawn(move || {
                for _ in 0..1 {
                    if i_thread % 2 == 0 {
                        let guard = MUTEX1.lock().unwrap();

                        println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                        sleep(Duration::from_millis(10));

                        let guard = MUTEX2.try_lock().unwrap();
                        println!("线程 {} 获取 MUTEX2 锁的结果: {:?}", i_thread, guard);
                    } else {
                        let _guard = MUTEX2.lock().unwrap();

                        println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1 !", i_thread);

                        let guard: MutexGuard<i64> = MUTEX1.try_lock().unwrap();
                        println!("线程 {} 获取 MUTEX1 锁的结果: {:?}", i_thread, guard);
                    }
                }
            }))
        }

        // 等子线程完成
        for child in children {
            let _ = child.join();
        }

        println!("死锁没有发生");
    }

    use std::sync::RwLock;

    #[test]
    fn test4() {
        let lock: RwLock<i32> = RwLock::new(5);
        {
            let r1 = lock.read().unwrap();
            let r2 = lock.read().unwrap();
            assert_eq!(*r1, 5);
            assert_eq!(*r2, 5)
            // r1, r2 drop。
        }

        {
            let mut w = lock.write().unwrap();
            *w += 1;
            assert_eq!(*w, 6);

            // 以下代码会阻塞发生死锁，因为读和写不允许同时存在
            // 写锁w直到该语句块结束才被释放，因此下面的读锁依然处于`w`的作用域中
            // let r1 = lock.read();
            // println!("{:?}",r1);
        }
    }

    use std::sync::Condvar;
    use tokio::join;

    #[test]
    fn test5() {
        // 创建 Mutex竞争变量（信号）
        let flag = Arc::new(Mutex::new(false));
        // 创建 条件变量Condvar
        let cond = Arc::new(Condvar::new());
        let cloned_flag = flag.clone();
        let cloned_cond = cond.clone();

        let handler = thread::spawn(move || {
            // 获取竞争变量锁
            let mut lock = cloned_flag.lock().unwrap();
            let mut counter = 0;

            while counter < 3 {
                while !*lock {
                    // wait方法会接收一个MutexGuard<'a, T>，且它会自动地暂时释放这个锁，使其他线程可以拿到锁并进行数据更新。
                    // 同时当前线程在此处会被阻塞，直到被其他地方notify后，它会将原本的MutexGuard<'a, T>还给我们，即重新获取到了锁，同时唤醒了此线程。
                    // 使用条件变量将子线程挂起，直到被唤醒
                    lock = cloned_cond.wait(lock).unwrap();
                }

                *lock = false;
                counter += 1;

                println!("inner counter: {}", counter);
            }
        });

        let mut counter = 0;
        loop {
            sleep(Duration::from_secs(1));
            // 获取锁并修改信号变量
            *flag.lock().unwrap() = true;
            counter += 1;
            if counter > 3 {
                break;
            }
            println!("outside counter: {}", counter);
            cond.notify_one();
        }
        handler.join().unwrap();

        println!("{:?}", flag)
    }

    use tokio::sync::{OwnedSemaphorePermit, Semaphore};

    #[tokio::main]
    #[test]
    async fn test6() {
        // 创建信号变量，值为3
        let semaphore: Arc<Semaphore> = Arc::new(Semaphore::new(3));
        let mut join_handles = Vec::new();

        for _ in 0..5 {
            // 申请信号量
            let permit: OwnedSemaphorePermit = semaphore.clone().acquire_owned().await.unwrap();

            join_handles.push(tokio::spawn(async move {
                println!("{:?}", permit);
                drop(permit);
            }));
        }

        for handle in join_handles {
            handle.await.unwrap();
        }
    }
}
