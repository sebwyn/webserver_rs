use std::io;

use tokio::task;
use tokio::net::{TcpListener, TcpStream};

fn main() -> io::Result<()> {
    task::spawn_blocking(launch_server);

    Ok(())
}

async fn launch_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
}

async fn process_socket(_socket: TcpStream) {
}


#[cfg(test)]
mod test {
    use std::time::Duration;

    use tokio_test::assert_ok;

    use super::launch_server;
    use tokio::{net::TcpSocket, task, time};

    #[tokio::test]
    async fn test_launch_spawns_listener_on_localhost() {
        task::spawn(launch_server());

        //wait for a moment
        let _ = time::sleep(Duration::from_secs(1));

        //create a client
        let addr = assert_ok!("127.0.0.1:8080".parse());
        let socket = assert_ok!(TcpSocket::new_v4());

        let stream = assert_ok!(socket.connect(addr).await);
        println!("{:?}", stream);
    }
}
