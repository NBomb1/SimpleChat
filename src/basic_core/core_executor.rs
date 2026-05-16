use tokio::sync::mpsc::UnboundedReceiver;
use crate::basic_core::network::NetworkManager;

pub enum CoreCommand {
    SaveConfig { username: Option<String>, ip: Option<String>, port: Option<String> },
    StartServer { username: String, ip: String, port: String },
    StartClient { username: String, ip: String, port: String },
    SendMessage { text: String },
    OnServerCreated { username: String, ip: String, port: String },
    OnClientCreated { username: String, ip: String, port: String },
}

pub async fn execute(mut receiver: UnboundedReceiver<CoreCommand>) {
    while let Some(cmd) = receiver.recv().await {
        match cmd {
            CoreCommand::StartServer { ip, port, .. } => {
                let addr = format!("{}:{}", ip, port);
                tokio::spawn(async move {
                    if let Err(e) = NetworkManager::start_server(addr).await {
                        log::error!("Server execution error: {}", e);
                    }
                });
            }
            CoreCommand::StartClient { ip, port, .. } => {
                let addr = format!("{}:{}", ip, port);
                tokio::spawn(async move {
                    if let Err(e) = NetworkManager::connect_to_server(addr).await {
                        log::error!("Client execution error: {}", e);
                    }
                });
            }
            CoreCommand::SendMessage { text } => {
                tokio::spawn(async move {
                    if let Err(e) = NetworkManager::send_message(text).await {
                        log::error!("Failed to route message: {}", e);
                    }
                });
            }
            CoreCommand::SaveConfig { .. } => {}
            CoreCommand::OnClientCreated { username, ip, port } => {

            }
            CoreCommand::OnServerCreated { username, ip, port } => {},
        }
    }
}

pub fn create_compact_bridge() -> tokio::sync::mpsc::UnboundedSender<CoreCommand> {
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel::<CoreCommand>();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to build tokio runtime.");
        rt.block_on(async {
            execute(receiver).await;
        });
    });
    sender
}