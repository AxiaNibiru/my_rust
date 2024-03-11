#![allow(unused)]
#![allow(dead_code)]

#[cfg(test)]
mod test {
    use futures::executor::block_on;

    async fn do_something() {
        hello_cat().await;
        println!("go go go");
    }

    async fn hello_cat() {
        println!("hello cat!");
    }

    #[test]
    fn test_async() {
        let future = do_something();
        block_on(future);
    }
}

#[cfg(test)]
mod test2 {
    use futures::executor::block_on;
    use std::thread::sleep;
    use std::time::Duration;

    struct Song {
        author: String,
        name: String,
    }

    async fn learn_song() -> Song {
        Song {
            author: "周杰伦".to_string(),
            name: String::from("《菊花台》"),
        }
    }

    async fn sing_song(song: Song) {
        println!(
            "给大家献上一首{}的{} ~ 菊花残，满地伤~ ~",
            song.author, song.name
        );
    }

    async fn dance() {
        println!("唱到情深处，身体不由自主的动了起来~ ~");
    }

    async fn learn_and_sing() {
        // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
        let song = learn_song().await;

        async_std::task::sleep(Duration::from_secs(1)).await;
        // tokio::time::sleep(std::time::Duration::from_secs_f32(0.0001)).await;

        // 唱歌必须要在学歌之后
        sing_song(song).await;
    }

    async fn async_main() {
        // let song = block_on(learn_song());
        // block_on(sing_song(song));
        // block_on(dance());
        // learn_and_sing固定了学歌与唱歌的顺序，但是跳舞并不受学歌与唱歌影响
        let f1 = learn_and_sing();
        let f2 = dance();

        // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。
        // 若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
        // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
        futures::join!(f1, f2);
    }

    #[test]
    fn main() {
        block_on(async_main());
    }
}
