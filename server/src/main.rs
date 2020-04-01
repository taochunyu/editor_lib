use std::error::Error;
use tokio::net::TcpListener;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let _ = listener.accept();
    }

    // while let Some(stream) = listener.incoming().next().await {
    //     match stream {
    //         Ok(stream) => {
    //             println!("123");
    //         }
    //         Err(_) => {}
    //     }
    // }

    Ok(())
}
