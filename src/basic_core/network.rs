use tokio::net::{TcpListener, TcpStream};
use std::error::Error;

pub struct NetworkManager;

impl NetworkManager {
    // Для сервера
    pub async fn start_server(port: u16) -> Result<(), Box<dyn Error>> {
        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(&addr).await?;
        log::info!("Server started on {}", addr);

        loop {
            let (socket, addr) = listener.accept().await?;
            log::info!("New connection from: {}", addr);
            // Тут будет логика чтения сообщений
        }
    }

    // Для клиента
    pub async fn connect_to(ip: String, port: u16) -> Result<(), Box<dyn Error>> {
        let addr = format!("{}:{}", ip, port);
        let _stream = TcpStream::connect(addr).await?;
        log::info!("Connected to server!");
        Ok(())
    }
}