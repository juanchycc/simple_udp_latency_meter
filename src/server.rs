use std::net::{UdpSocket, SocketAddr};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let server_ip = "IP_DEL_SERVIDOR:12345"; // Cambia esto a la IP del servidor
    let server_addr: SocketAddr = server_ip.parse()?;
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let message = "Hello, World!"; // El mensaje que quieres enviar

    let mut packet_number = 0;

    loop {
        let start_time = Instant::now();
        let packet = format!("{} {}", message, packet_number);
        socket.send_to(packet.as_bytes(), &server_addr)?;
        
        let mut buf = [0; 1024];
        socket.recv_from(&mut buf)?;
        
        let end_time = Instant::now();
        let latency = end_time.duration_since(start_time).as_micros();
        println!("Packet {} - Latency: {} microseconds", packet_number, latency);
        
        packet_number += 1;
    }

}
