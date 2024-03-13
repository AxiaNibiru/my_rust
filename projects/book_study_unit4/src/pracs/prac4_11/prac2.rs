#![allow(unused)]
#![allow(dead_code)]

// Future Test

#[cfg(test)]
mod future_test {
    struct Socket {
        data: Vec<u8>,
    }

    impl Socket {
        fn has_data_to_read(&self) -> bool {
            self.data.is_empty()
        }

        fn read_buf(&mut self) -> Vec<u8> {
            self.data.clone()
        }

        fn set_readable_callback(&self, p0: fn()) {
            p0()
        }
    }

    trait SimpleFuture {
        type Output;

        fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
    }

    enum Poll<T> {
        Ready(T),
        Pending,
    }

    struct SocketRead<'a> {
        socket: &'a mut Socket,
    }

    impl SimpleFuture for SocketRead<'_> {
        type Output = Vec<u8>;

        fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
            if self.socket.has_data_to_read() {
                Poll::Ready(self.socket.read_buf())
            } else {
                self.socket.set_readable_callback(wake);
                Poll::Pending
            }
        }
    }

    /// 一个SimpleFuture，它会并发地运行两个Future直到它们完成
    ///
    /// 之所以可以并发，是因为两个Future的轮询可以交替进行，一个阻塞，另一个就可以立刻执行，反之亦然
    pub struct Join<FutureA, FutureB> {
        // 结构体的每个字段都包含一个Future，可以运行直到完成.
        // 等到Future完成后，字段会被设置为 `None`. 这样Future完成后，就不会再被轮询
        a: Option<FutureA>,
        b: Option<FutureB>,
    }

    impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
    where
        FutureA: SimpleFuture<Output = ()>,
        FutureB: SimpleFuture<Output = ()>,
    {
        type Output = ();
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
            // 尝试去完成一个 Future `a`
            if let Some(a) = &mut self.a {
                if let Poll::Ready(()) = a.poll(wake) {
                    self.a.take();
                }
            }

            // 尝试去完成一个 Future `b`
            if let Some(b) = &mut self.b {
                if let Poll::Ready(()) = b.poll(wake) {
                    self.b.take();
                }
            }

            if self.a.is_none() && self.b.is_none() {
                // 两个 Future都已完成 - 我们可以成功地返回了
                Poll::Ready(())
            } else {
                // 至少还有一个 Future 没有完成任务，因此返回 `Poll::Pending`.
                // 当该 Future 再次准备好时，通过调用`wake()`函数来继续执行
                Poll::Pending
            }
        }
    }
}

mod future_test2 {

    use std::{
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
        thread,
        time::Duration,
    };

    pub struct TimerFuture {
        shared_state: Arc<Mutex<SharedState>>,
    }

    /// 在Future和等待的线程间共享状态
    struct SharedState {
        /// 定时(睡眠)是否结束
        completed: bool,

        /// 当睡眠结束后，线程可以用`waker`通知`TimerFuture`来唤醒任务
        waker: Option<Waker>,
    }

    impl Future for TimerFuture {
        type Output = ();
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            // 通过检查共享状态，来确定定时器是否已经完成
            let mut shared_state = self.shared_state.lock().unwrap();
            if shared_state.completed {
                Poll::Ready(())
            } else {
                // 设置`waker`，这样新线程在睡眠(计时)结束后可以唤醒当前的任务，接着再次对`Future`进行`poll`操作,
                //
                // 下面的`clone`每次被`poll`时都会发生一次，实际上，应该是只`clone`一次更加合理。
                // 选择每次都`clone`的原因是： `TimerFuture`可以在执行器的不同任务间移动，如果只克隆一次，
                // 那么获取到的`waker`可能已经被篡改并指向了其它任务，最终导致执行器运行了错误的任务
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }

    impl TimerFuture {
        /// 创建一个新的`TimerFuture`，在指定的时间结束后，该`Future`可以完成
        pub fn new(duration: Duration) -> Self {
            let shared_state = Arc::new(Mutex::new(SharedState {
                completed: false,
                waker: None,
            }));

            // 创建新线程
            let thread_shared_state = shared_state.clone();
            thread::spawn(move || {
                // 睡眠指定时间实现计时功能
                thread::sleep(duration);
                let mut shared_state = thread_shared_state.lock().unwrap();
                // 通知执行器定时器已经完成，可以继续`poll`对应的`Future`了
                shared_state.completed = true;
                if let Some(waker) = shared_state.waker.take() {
                    waker.wake() // 唤醒
                }
            });

            TimerFuture { shared_state }
        }
    }
}

mod executor_test {
    use {
        // 引入之前实现的定时器模块
        super::future_test2::TimerFuture,
        futures::{
            future::{BoxFuture, FutureExt},
            task::{waker_ref, ArcWake},
        },
        std::{
            future::Future,
            sync::mpsc::{sync_channel, Receiver, SyncSender},
            sync::{Arc, Mutex},
            task::{Context, Poll},
            time::Duration,
        },
    };

    /// 任务执行器，负责从通道中接收任务然后执行
    struct Executor {
        ready_queue: Receiver<Arc<Task>>,
    }

    /// `Spawner`负责创建新的`Future`然后将它发送到任务通道中
    #[derive(Clone)]
    struct Spawner {
        task_sender: SyncSender<Arc<Task>>,
    }

    /// 一个Future，它可以调度自己(将自己放入任务通道中)，然后等待执行器去`poll`
    struct Task {
        /// 进行中的Future，在未来的某个时间点会被完成
        ///
        /// 按理来说`Mutex`在这里是多余的，因为我们只有一个线程来执行任务。但是由于
        /// Rust并不聪明，它无法知道`Future`只会在一个线程内被修改，并不会被跨线程修改。因此
        /// 我们需要使用`Mutex`来满足这个笨笨的编译器对线程安全的执着。
        ///
        /// 如果是生产级的执行器实现，不会使用`Mutex`，因为会带来性能上的开销，取而代之的是使用`UnsafeCell`
        future: Mutex<Option<BoxFuture<'static, ()>>>,

        /// 可以将该任务自身放回到任务通道中，等待执行器的poll
        task_sender: SyncSender<Arc<Task>>,
    }

    fn new_executor_and_spawner() -> (Executor, Spawner) {
        // 任务通道允许的最大缓冲数(任务队列的最大长度)
        // 当前的实现仅仅是为了简单，在实际的执行中，并不会这么使用
        const MAX_QUEUED_TASKS: usize = 10_000;
        let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
        (Executor { ready_queue }, Spawner { task_sender })
    }
    
    impl Spawner {
        fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
            let future = future.boxed();
            let task = Arc::new(Task {
                future: Mutex::new(Some(future)),
                task_sender: self.task_sender.clone(),
            });
            self.task_sender.send(task).expect("任务队列已满");
        }
    }


}
