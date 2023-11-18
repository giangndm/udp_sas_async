# udp_sas_async

[![Crates.io](https://img.shields.io/crates/v/udp_sas_async.svg)](https://crates.io/crates/udp_sas_async)

This project implement async support with both tokio or async_std for udp_sas crate (https://crates.io/crates/udp_sas)

Example with tokio

```rust
use udp_sas::tokio::UdpSocketSas;

#[tokio::main]
async fn main() {
    let socket = UdpSocketSas::bind("0.0.0.0:0".parse().unwrap()).unwrap();
    println!("Running port on 0.0.0.0:{}", socket.local_port());
    let mut buf = [0; 1500];
    while let Ok((size, remote, dest)) = socket.recv_sas(&mut buf).await {
        println!("received {} from {} to {}", size, remote, dest);
        socket.send_sas(&buf[..size], dest, remote).await.unwrap();
    }
}

```

Example with async_std

```rust
use udp_sas::async_std::UdpSocketSas;

#[tokio::main]
async fn main() {
    let socket = UdpSocketSas::bind("0.0.0.0:0".parse().unwrap()).unwrap();
    println!("Running port on 0.0.0.0:{}", socket.local_port());
    let mut buf = [0; 1500];
    while let Ok((size, remote, dest)) = socket.recv_sas(&mut buf).await {
        println!("received {} from {} to {}", size, remote, dest);
        socket.send_sas(&buf[..size], dest, remote).await.unwrap();
    }
}
```