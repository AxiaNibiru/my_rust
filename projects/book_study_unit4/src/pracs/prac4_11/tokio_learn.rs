#![allow(dead_code)]
#![allow(unused)]

use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

mod test {

    use mini_redis::{client, Result};

    #[tokio::test]
    async fn test1() -> Result<()> {
        let mut client = client::connect("127.0.0.1:6379").await?;
        client.set("hello", "world".into()).await?;

        let result = client.get("hello").await?;
        println!("got value from the server; result={:?}", result);

        Ok(())
    }

    use mini_redis::{Connection, Frame};
    use tokio::net::{TcpListener, TcpStream};

    // 监听程序
    #[tokio::test]
    async fn test2() {
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
        // 处理监听到的 Socket
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            process(socket).await;
        }
    }

    async fn process(socket: TcpStream) {
        // 创建 Connection
        let mut connection = Connection::new(socket);
        if let Some(frame) = connection.read_frame().await.unwrap() {
            println!("GOT: {:?}", frame);

            // Respond with an error
            let response = Frame::Error(format!("You send: {:?}", frame));
            let response = Frame::Simple(format!("You send: {:?}", frame));
            connection.write_frame(&response).await.unwrap();
        }
    }
    #[tokio::test]
    async fn test3() {
        let handle = tokio::spawn(async {
            println!("this is inner async spawn");
            "return value"
        });

        // Do some other work

        let out = handle.await.unwrap();
        println!("GOT {}", out);
    }
    
}
