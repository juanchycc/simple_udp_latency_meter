use std::net::{UdpSocket, SocketAddr};
use std::error::Error;
use std::time::{Instant, Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let server_ip = "0.0.0.0:12345"; // Cambia esto a la IP de la computadora B
    let socket = UdpSocket::bind(server_ip)?;

    println!("Server listening on {}", server_ip);

    let mut buf = [0; 1024];
    let mut start_time: Option<Instant> = None;

    loop {
        let (len, client_addr) = socket.recv_from(&mut buf)?;
        if start_time.is_none() {
            start_time = Some(Instant::now());
        } else {
            let end_time = Instant::now();
            let latency = end_time.duration_since(start_time.unwrap()).as_micros();
            println!("Latency: {} microseconds from client {}", latency, client_addr);
            start_time = None;
        }
    }
}
