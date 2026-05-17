use std::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;
use crate::basic_core::core_executor::UIEvent;

/// Manages network connections for both server and client modes.
/// Owns connection state.
pub struct NetworkManager {
    event_tx: Sender<UIEvent>,
    /// Client mode: the writer half of the connection to the server.
    client_writer: Option<OwnedWriteHalf>,
    /// Server mode: shared list of writers for every connected client.
    server_clients: Arc<Mutex<Vec<OwnedWriteHalf>>>,
}

impl NetworkManager {
    pub fn new(event_tx: Sender<UIEvent>) -> Self {
        Self {
            event_tx,
            client_writer: None,
            server_clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // ── Server logic ──────────────────────────────────────────────

    /// Starts a TCP server. The accept loop runs as a background tokio task.
    pub async fn start_server(&self, addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind(addr).await?;
        log::info!("Server running on {}", addr);

        let event_tx = self.event_tx.clone();
        let server_clients = Arc::clone(&self.server_clients);

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((socket, client_addr)) => {
                        log::info!("New client connected: {}", client_addr);
                        let tx = event_tx.clone();
                        let clients = Arc::clone(&server_clients);
                        tokio::spawn(Self::handle_client_connection(socket, client_addr, tx, clients));
                    }
                    Err(e) => {
                        log::error!("Failed to accept connection: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    /// Handles a single client connection on the server side.
    /// Incoming messages are broadcast to every connected client.
    async fn handle_client_connection(
        socket: TcpStream,
        client_addr: std::net::SocketAddr,
        event_tx: Sender<UIEvent>,
        server_clients: Arc<Mutex<Vec<OwnedWriteHalf>>>,
    ) {
        let (mut reader, writer) = socket.into_split();

        // Register this client's writer in the shared list.
        server_clients.lock().await.push(writer);

        let mut buffer = [0u8; 1024];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => {
                    log::info!("Client disconnected: {}", client_addr);
                    break;
                }
                Ok(n) => {
                    let msg = String::from_utf8_lossy(&buffer[..n]);
                    let msg_trimmed = msg.trim().to_string();
                    log::info!("[{}] Received: {}", client_addr, msg_trimmed);

                    // Show in the server's own UI.
                    let _ = event_tx.send(UIEvent::MessageReceived {
                        text: msg_trimmed.clone(),
                    });

                    // Broadcast to all connected clients.
                    let mut clients = server_clients.lock().await;
                    let broadcast = format!("{}\n", msg_trimmed);
                    let mut dead = Vec::new();
                    for (i, w) in clients.iter_mut().enumerate() {
                        if w.write_all(broadcast.as_bytes()).await.is_err() {
                            dead.push(i);
                        }
                    }
                    // Remove disconnected writers (iterate in reverse to preserve indices).
                    for i in dead.into_iter().rev() {
                        clients.remove(i);
                    }
                }
                Err(e) => {
                    log::error!("Read error from {}: {}", client_addr, e);
                    break;
                }
            }
        }
    }

    // ── Client logic ──────────────────────────────────────────────

    /// Connects to a remote server as a client.
    pub async fn connect_to_server(&mut self, addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        log::info!("Connecting to {}...", addr);
        let stream = TcpStream::connect(addr).await?;
        log::info!("Successfully connected to {}", addr);

        let (reader, writer) = stream.into_split();
        self.client_writer = Some(writer);

        let event_tx = self.event_tx.clone();
        tokio::spawn(async move {
            Self::read_from_server(reader, event_tx).await;
        });

        Ok(())
    }

    /// Reads incoming messages from the server (runs in a background task).
    async fn read_from_server(
        mut reader: tokio::net::tcp::OwnedReadHalf,
        event_tx: Sender<UIEvent>,
    ) {
        let mut buffer = [0u8; 1024];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => {
                    log::info!("Server closed connection.");
                    break;
                }
                Ok(n) => {
                    let msg = String::from_utf8_lossy(&buffer[..n]);
                    log::info!("Received from server: {}", msg.trim());
                    let _ = event_tx.send(UIEvent::MessageReceived {
                        text: msg.trim().to_string(),
                    });
                }
                Err(e) => {
                    log::error!("Error reading from server: {}", e);
                    let _ = event_tx.send(UIEvent::Error {
                        message: format!("Connection error: {}", e),
                    });
                    break;
                }
            }
        }
    }

    /// Sends a message through the active connection.
    ///
    /// - **Client mode**: writes to the server over TCP.
    /// - **Server mode**: broadcasts to every connected client and also
    ///   displays the message locally in the server's own UI.
    pub async fn send_message(&mut self, text: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref mut writer) = self.client_writer {
            // Client: send to server.
            writer.write_all(format!("{}\n", text).as_bytes()).await?;
        } else {
            // Server: broadcast to all clients.
            let broadcast = format!("{}\n", text);
            let mut clients = self.server_clients.lock().await;
            let mut dead = Vec::new();
            for (i, w) in clients.iter_mut().enumerate() {
                if w.write_all(broadcast.as_bytes()).await.is_err() {
                    dead.push(i);
                }
            }
            for i in dead.into_iter().rev() {
                clients.remove(i);
            }

            // Also show the message in the server's own UI.
            let _ = self.event_tx.send(UIEvent::MessageReceived { text });
        }
        Ok(())
    }
}
