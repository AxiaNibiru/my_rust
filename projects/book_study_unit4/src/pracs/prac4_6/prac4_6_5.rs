#![allow(unused)]
#![allow(dead_code)]

#[cfg(test)]
mod test {
    use std::hint;
    use std::ops::Sub;
    use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread::{self, JoinHandle};
    use std::time::Instant;

    const N_TIMES: u64 = 10000000;

    const N_THREADS: usize = 10;

    static R: AtomicU64 = AtomicU64::new(0);

    fn add_n_times(n: u64) -> JoinHandle<()> {
        thread::spawn(move || {
            for _ in 0..n {
                // Ordering::Relaxed -> 控制原子操作使用的内存顺序
                R.fetch_add(1, Ordering::Relaxed);
            }
        })
    }

    #[test]
    fn test1() {
        let s = Instant::now();
        let mut threads: Vec<JoinHandle<()>> = Vec::with_capacity(N_THREADS);

        for _ in 0..N_THREADS {
            threads.push(add_n_times(N_TIMES));
        }

        for thread in threads {
            thread.join().unwrap();
        }

        assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
        println!("{:?}", Instant::now().sub(s));
    }

    static mut DATA: u64 = 0;
    static READY: AtomicBool = AtomicBool::new(false);

    fn reset() {
        unsafe {
            DATA = 0;
        }
        READY.store(false, Ordering::Relaxed);
    }

    fn producer() -> JoinHandle<()> {
        thread::spawn(move || {
            unsafe {
                DATA = 100; // A
            }
            READY.store(true, Ordering::Release); // B: 内存屏障 ↑
        })
    }

    fn consumer() -> JoinHandle<()> {
        thread::spawn(move || {
            while !READY.load(Ordering::Acquire) {} // C: 内存屏障 ↓

            assert_eq!(100, unsafe { DATA }); // D
        })
    }

    #[test]
    fn test2() {
        loop {
            reset();

            let t_producer = producer();
            let t_consumer = consumer();

            t_producer.join().unwrap();
            t_consumer.join().unwrap();
        }
    }

    #[test]
    fn test3() {
        let spinlock = Arc::new(AtomicUsize::new(1));
        let spinlock_clone = Arc::clone(&spinlock);

        let thread = thread::spawn(move || {
            spinlock_clone.store(0, Ordering::SeqCst);
        });

        while 0 != spinlock.load(Ordering::SeqCst) {
            hint::spin_loop();
        }

        if let Err(panic) = thread.join() {
            println!("Thread had an error: {:?}", panic);
        }
    }
}
