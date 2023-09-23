use std::net::{UdpSocket, SocketAddr};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let local_ip = "IP_LOCAL_DE_LA_COMPUTADORA_A:0"; // Cambia esto a la IP local de la computadora A
    let local_addr: SocketAddr = local_ip.parse()?;
    let server_ip = "IP_DEL_SERVIDOR:12345"; // Cambia esto a la IP del servidor
    let server_addr: SocketAddr = server_ip.parse()?;
    let socket = UdpSocket::bind(local_addr)?;

    let message = "Hello, World!"; // El mensaje que quieres enviar

    for _ in 0..10 { // Envía 10 paquetes uno detrás del otro
        socket.send_to(message.as_bytes(), &server_addr)?;

        // Espera un breve período de tiempo antes de enviar el siguiente paquete
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
