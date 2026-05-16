use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub enum CoreCommand {
    SaveConfig {
        username: Option<String>,
        ip: Option<String>,
        port: Option<String>
    },
    StartServer { username: String, ip: String, port: String },
    StartClient { username: String, ip: String, port: String },
}

pub async fn execute(mut receiver: UnboundedReceiver<CoreCommand>) {
    while let Some(cmd) = receiver.recv().await {
        match cmd {
            //
            CoreCommand::SaveConfig { username, ip, port } => {
                log::info!("Saving config for {}", username.unwrap_or_default());
            }

            //
            CoreCommand::StartServer { username, ip, port } => {
                let addr = format!("{}:{}", ip, port);
                tokio::spawn(async move {
                    if let Err(e) = run_server_logic(addr).await {
                        log::error!("Server error: {}", e);
                    }
                });
            }

            //
            CoreCommand::StartClient { username, ip, port } => {
                let addr = format!("{}:{}", ip, port);
                tokio::spawn(async move {
                    if let Err(e) = run_client_logic(addr).await {
                        log::error!("Client connection error: {}", e);
                    }
                });
            }
        }
    }
}

async fn run_server_logic(addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(&addr).await?;
    log::info!("🔥 Server has been ran {}", addr);

    loop {
        // Ждем подключения клиента
        let (mut socket, client_addr) = listener.accept().await?;
        log::info!("New client connected: {}", client_addr);

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            loop {
                match socket.read(&mut buffer).await {
                    Ok(0) => {
                        log::info!("Client disconnected {}.", client_addr);
                        break;
                    }
                    Ok(n) => {
                        let msg = String::from_utf8_lossy(&buffer[..n]);
                        log::info!("[{}] Got: {}", client_addr, msg.trim());
                        // Эхо-ответ обратно клиенту (для теста)
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

async fn run_client_logic(addr: String) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("🔌 Trying connect to {}...", addr);
    let mut stream = TcpStream::connect(&addr).await?;
    log::info!("✅ Successfully connected!");

    // Отправим тестовый пакет при подключении
    stream.write_all(b"Hello from Rust Client!\n").await?;

    // Тут в будущем будет цикл чтения сообщений от сервера
    Ok(())
}

/// Creates channel to communicate between main core components.
pub fn create_compact_bridge() -> UnboundedSender<CoreCommand> {
    // creating roles
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel::<CoreCommand>();

    // Creating thread for core commands execution
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to build tokio runtime.");
        rt.block_on(async {
            execute(receiver).await;
        });
    });
    sender
}
