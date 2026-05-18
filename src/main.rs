// Your Turn — Exercise 0.6, Phase 0 Deliverable
// 
// Build an async TCP echo server:
// 
// Requirements:
// 
//     Listens on 127.0.0.1:9001
//     Each connection gets its own tokio::spawn task
//     Reads data in a loop with a Vec<u8> buffer
//     Echoes every byte back to the sender
//     Prints "[PHANTOM] Peer connected from {addr}" on connect
//     Prints "[PHANTOM] Peer disconnected" when they close the connection
//     Zero warnings on cargo build
//
// Starter Skeleton: 

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();
    println!("[PHANTOM] Node listening on port 9001");

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("[PHANTOM] Peer connected from {}", addr);

        tokio::spawn(async move {
            let mut buf = vec![0u8; 1024];
            loop {
                let n = socket.read(&mut buf).await.unwrap_or(0);
                if n == 0 {
                    // connection closed — print disconnect and break
                    println!("[PHANTOM] Peer disconnected");
                    break;
                }
                // echo the bytes back
                if socket.write_all(&buf[..n]).await.is_err() {
                    println!("[PHANTOM] Peer disconnected abruptly");
                    break;
                }
            }
        });
    }
}

// $ echo "hello phantom" | nc 127.0.0.1 9001
