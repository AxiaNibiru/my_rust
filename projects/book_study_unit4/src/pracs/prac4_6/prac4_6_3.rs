#![allow(unused)]
#![allow(dead_code)]

#[cfg(test)]
mod test {
    use std::sync::mpsc::{self, Receiver, Sender};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test1() {
        // mpsc::channel为异步通道，创建channel的子线程不会被阻塞
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            tx.send(1).unwrap();
        })
        .join()
        .unwrap();

        println!("revice {}", rx.recv().unwrap());
    }

    #[test]
    fn test2() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            tx.send(1).unwrap();
        })
        .join()
        .unwrap();

        println!("receive: {:?}", rx.try_recv());

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            tx.send(1).unwrap();
            thread::sleep(Duration::from_millis(10));
        });

        println!("receive: {:?}", rx.try_recv());
        let (tx, rx) = mpsc::channel::<i32>();

        thread::spawn(move || {
            drop(tx);
        });

        println!("receive: {:?}", rx.try_recv());
    }

    #[test]
    fn test3() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let s = String::from("我，飞走咯!");
            tx.send(s).unwrap();
            // println!("val is {}", s); // use s after move
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    #[test]
    fn test4() {
        let (tx, rx) = mpsc::channel::<String>();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for r in rx {
            // 阻塞直到tx被释放
            println!("Got : {}", r);
        }
    }

    #[test]
    fn test5() {
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            tx.send(String::from("hi from raw tx")).unwrap();
        });

        thread::spawn(move || {
            tx1.send(String::from("hi from cloned tx")).unwrap();
        });

        // channel sender和 receiver 的顺序为FIFO（先进先出）
        for received in rx {
            println!("Got: {}", received);
        }
    }

    #[test]
    fn test6() {
        //  mpsc::sync_channel(0) -> 0：该值可以用来指定同步通道的消息缓存条数，
        // 当你设定为N时，发送者就可以无阻塞的往通道中发送N条消息，当消息缓冲队列满了后，
        // 新的消息发送将被阻塞(如果没有接收者消费缓冲队列中的消息，那么第N+1条消息就将触发发送阻塞)。
        let (tx, rx) = mpsc::sync_channel(0);

        let handler = thread::spawn(move || {
            println!("before send");
            tx.send(1);
            println!("after send");
        });

        println!("before sleep");
        thread::sleep(Duration::from_secs(3));
        println!("after sleep");

        println!("receive {}", rx.recv().unwrap());
        handler.join().unwrap();
    }

    // 使用枚举的方式传输多种类型数据，但是enum会对内存占用最高的进行内存对齐。
    enum Fruit {
        Apple(u8),
        Orange(String),
    }

    #[test]
    fn test7() {
        let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();
        // product
        tx.send(Fruit::Apple(10u8)).unwrap();
        tx.send(Fruit::Orange("Orange".to_string())).unwrap();

        for _ in 0..2 {
            // customer
            match rx.recv().unwrap() {
                Fruit::Apple(num) => println!("{}", num),
                Fruit::Orange(name) => println!("{}", name),
            }
        }
    }

    #[test]
    fn test8() {
        let (send, recv) = mpsc::channel();
        let num_threads = 3;
        for i in 0..num_threads {
            let thread_send = send.clone();
            thread::spawn(move || {
                thread_send.send(i).unwrap();
                println!("thread {:?} finished", i);
            });
        }

        // Receiver.recv() 会阻塞，直到通道关闭，如果使用try_recv()则不会阻塞线程，如果没有值直接返回Err(None)
        // 通道关闭的两个条件：发送者全部drop或接收者被drop，要结束for循环显然是要求发送者全部drop，
        // 但是由于send自身没有被drop，会导致该循环永远无法结束，最终主线程会一直阻塞。
        // 在这里drop send...
        // drop(send); // 如果不drop那么main线程会一直阻塞

        thread::sleep(Duration::from_secs(1));

        for x in recv.try_iter() {
            println!("Got: {}", x);
        }
        println!("finished iterating");
    }
}
