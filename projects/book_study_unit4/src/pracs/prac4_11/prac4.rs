#![allow(unused)]
#![allow(dead_code)]

mod test {
    use futures::executor::block_on;
    use std::thread::current;
    use std::time::Duration;
    use tokio::io::join;
    use tokio::time::Instant;

    async fn wait5() {
        async_std::task::sleep(Duration::from_secs(2)).await;
        println!("wait {:?}", Instant::now());
    }

    async fn print3() {
        println!("3 {:?}", Instant::now());
    }

    #[test]
    fn test1() {
        let f = async {
            // async内部是有序的
            println!("1 {:?}", Instant::now());
            wait5().await;
            println!("2 {:?}", Instant::now());
            print3().await;

            println!("---------------------------");

            futures::join!(print3(), wait5()); // async 函数在同一async线程交替执行
        };

        block_on(f);
    }
}

mod test2 {
    use chrono::format;
    use chrono::format::{DelayedFormat, StrftimeItems};
    use chrono::{DateTime, Local};
    use futures::{executor::block_on, SinkExt, StreamExt};
    use std::future::Future;
    use std::sync::mpsc;
    use std::time::Instant;
    #[test]
    fn test() {
        // let instant = Instant::now();
        let local_time: DateTime<Local> = Local::now();
        let fmt_time: StrftimeItems = StrftimeItems::new("%Y-%m-%d %H:%M:%S");
        let formatted_time: DelayedFormat<StrftimeItems> = local_time.format_with_items(fmt_time);
        println!("{formatted_time}");
    }

    async fn foo(x: &u8) -> u8 {
        *x
    }

    // 上面的函数跟下面的函数是等价的:
    fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
        async move { *x }
    }

    async fn send_recv() {
        const BUFFER_SIZE: usize = 0;
        let (mut tx, mut rx) = mpsc::channel::<i32>();
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        drop(tx);

        // `StreamExt::next` 类似于 `Iterator::next`, 但是前者返回的不是值，而是一个 `Future<Output = Option<T>>`，
        // 因此还需要使用`.await`来获取具体的值
        let rx = &mut rx.iter();
        assert_eq!(Some(1), rx.next());
        assert_eq!(Some(2), rx.next());
        assert_eq!(None, rx.next());
    }

    #[test]
    fn test2() {
        block_on(send_recv());
    }
}

mod test_stream {
    use futures::{io, Stream, StreamExt, TryStreamExt};
    use std::pin::Pin;

    async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
        let mut sum: i32 = 0;
        while let Some(item) = stream.next().await {
            sum += item;
        }
        sum
    }

    async fn sum_with_try_next(
        mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
    ) -> Result<i32, io::Error> {
        let mut sum = 0;
        while let Some(item) = stream.try_next().await? {
            sum += item;
        }
        Ok(sum)
    }

    async fn jump_around(
        mut stream: Pin<&mut dyn Stream<Item = Result<u8, io::Error>>>,
    ) -> Result<(), io::Error> {
        use futures::stream::TryStreamExt; // 引入 `try_for_each_concurrent`
        const MAX_CONCURRENT_JUMPERS: usize = 100;

        stream
            .try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
                jump_n_times(num).await?;
                report_n_jumps(num).await?;
                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn report_n_jumps(p0: u8) -> Result<u8, io::Error> {
        todo!()
    }

    async fn jump_n_times(p0: u8) -> Result<u8, io::Error> {
        todo!()
    }
}
