use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use std::sync::OnceLock;
use tokio::sync::Mutex;

static CLIENT_WRITE_STREAM: OnceLock<Mutex<OwnedWriteHalf>> = OnceLock::new();

pub struct NetworkManager;

impl NetworkManager {
    pub fn new() -> NetworkManager {
        NetworkManager{

        }
    }

    // ЛОГИКА СЕРВЕРА
    pub async fn start_server(addr: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind(&addr).await?;
        log::info!("🔥 Server ran successfully on {}", addr);

        loop {
            let (mut socket, client_addr) = listener.accept().await?;
            log::info!("🔗 New client connected: {}", client_addr);

            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                loop {
                    match socket.read(&mut buffer).await {
                        Ok(0) => {
                            log::info!("❌ Client disconnected: {}.", client_addr);
                            break;
                        }
                        Ok(n) => {
                            let msg = String::from_utf8_lossy(&buffer[..n]);
                            log::info!("[{}] Got: {}", client_addr, msg.trim());

                            // Эхо-ответ обратно клиенту
                            let _ = socket.write_all(format!("Echo: {}", msg).as_bytes()).await;
                        }
                        Err(e) => {
                            log::error!("Couldn't read from socket {}: {}", client_addr, e);
                            break;
                        }
                    }
                }
            });
        }
    }

    // ЛОГИКА КЛИЕНТА
    pub async fn connect_to_server(addr: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        log::info!("🔌 Trying to connect to {}...", addr);
        let stream = TcpStream::connect(&addr).await?;
        log::info!("✅ Successfully connected!");

        let (mut reader, writer) = stream.into_split();
        let _ = CLIENT_WRITE_STREAM.set(Mutex::new(writer));

        // Цикл чтения сообщений от сервера
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            loop {
                match reader.read(&mut buffer).await {
                    Ok(0) => {
                        log::info!("❌ Server closed connection.");
                        break;
                    }
                    Ok(n) => {
                        let msg = String::from_utf8_lossy(&buffer[..n]);
                        log::info!("Received from server: {}", msg.trim());
                        // TODO: Сюда мы добавим проброс сообщения в UI Slint
                    }
                    Err(e) => {
                        log::error!("Error reading from server: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    // ОТПРАВКА СООБЩЕНИЯ
    pub async fn send_message(text: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(mutex) = CLIENT_WRITE_STREAM.get() {
            let mut guard = mutex.lock().await;
            guard.write_all(format!("{}\n", text).as_bytes()).await?;
            log::info!("Data successfully sent to socket: {}", text);
            Ok(())
        } else {
            Err("Connection is not established yet!".into())
        }
    }
}