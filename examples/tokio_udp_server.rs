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
