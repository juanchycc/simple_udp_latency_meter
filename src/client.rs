use std::net::{UdpSocket, SocketAddr};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let server_ip = "IP_DE_LA_COMPUTADORA_B:12345"; // Cambia esto a la IP de la computadora B
    let server_addr: SocketAddr = server_ip.parse()?;
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let message = "Hello, World!"; // El mensaje que quieres enviar

    for _ in 0..10 { // Envía 10 paquetes uno detrás del otro
        socket.send_to(message.as_bytes(), &server_addr)?;

        // Espera un breve período de tiempo antes de enviar el siguiente paquete
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
