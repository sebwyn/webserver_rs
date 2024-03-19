use std::io;

use tokio::io::Interest;
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
        let _ = process_socket(socket).await;
    }
}

async fn process_socket(stream: TcpStream) -> io::Result<()> {
    let mut message: Option<String> = None;

    loop {
        let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await?;

        if ready.is_readable() {
            let mut data = vec![0; 1024];
            // Try to read data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match stream.try_read(&mut data) {
                Ok(n) => {
                    println!("read {} bytes", n);
                    message = Some(std::str::from_utf8(&data[0..n]).unwrap().to_string());
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }

        }

        if ready.is_writable() {
            if let Some(msg) = &message {
                match stream.try_write(msg.as_bytes()) {
                    Ok(n) => {
                        message = None;
                        println!("write {} bytes", n);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use std::time::Duration;

    use tokio_test::assert_ok;

    use super::launch_server;
    use tokio::{net::TcpSocket, task, time::{self, timeout}, io::{AsyncWriteExt, AsyncReadExt, self}};

    #[tokio::test]
    async fn test_launch_spawns_echo_server_on_localhost() {
        task::spawn(launch_server());

        time::sleep(Duration::from_secs(1)).await;

        //create a client
        let addr = assert_ok!("127.0.0.1:8080".parse());
        let socket = assert_ok!(TcpSocket::new_v4());

        let mut stream = assert_ok!(socket.connect(addr).await);
        
        let expected_message = String::from("Hello, World!");
        assert_ok!(stream.write(expected_message.as_bytes()).await);
        
        println!("about to read");

        let mut buf = vec![0u8; 1024];

        let _ = timeout(Duration::from_secs(1), stream.read(&mut buf)).await;
        
        assert!(false);

        // let msg = assert_ok!(std::str::from_utf8(&buf[0..len]).map(|n| n.to_string()));
        // assert_eq!(msg, expected_message);

    }
}
