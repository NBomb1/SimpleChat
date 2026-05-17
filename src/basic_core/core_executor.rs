use std::sync::mpsc::Sender;
use tokio::sync::mpsc::UnboundedReceiver;
use crate::basic_core::configuration_manager::ConfigurationManager;
use crate::basic_core::network::NetworkManager;
pub(crate) use crate::basic_core::ui_manager::UIEvent;

/// Commands sent from UI to the executor.
pub enum CoreCommand {
    StartServer { username: String, ip: String, port: String },
    StartClient { username: String, ip: String, port: String },
    SendMessage { text: String },
}

/// The core executor processes commands from the UI in a dedicated
/// tokio runtime thread, coordinating between network and config managers.
struct CoreExecutor {
    command_rx: UnboundedReceiver<CoreCommand>,
    network: NetworkManager,
    config: ConfigurationManager,
    event_tx: Sender<UIEvent>,
}

impl CoreExecutor {
    fn new(
        command_rx: UnboundedReceiver<CoreCommand>,
        config: ConfigurationManager,
        event_tx: Sender<UIEvent>,
    ) -> Self {
        Self {
            command_rx,
            network: NetworkManager::new(event_tx.clone()),
            config,
            event_tx,
        }
    }

    /// Main loop: receives commands and dispatches them.
    async fn run(&mut self) {
        log::info!("CoreExecutor started, waiting for commands...");
        while let Some(cmd) = self.command_rx.recv().await {
            self.handle_command(cmd).await;
        }
        log::info!("CoreExecutor shutting down (command channel closed).");
    }

    async fn handle_command(&mut self, cmd: CoreCommand) {
        match cmd {
            CoreCommand::StartServer { ip, port, username } => {
                save(&mut self.config, username.clone(), ip.clone(), port.clone());
                let addr = format!("{}:{}", ip, port);
                log::info!("Executor: starting server on {} as '{}'", addr, username);
                match self.network.start_server(&addr).await {
                    Ok(()) => {
                        let _ = self.event_tx.send(UIEvent::ServerStarted { addr });
                    }
                    Err(e) => {
                        log::error!("Server start error: {}", e);
                        let _ = self.event_tx.send(UIEvent::Error {
                            message: format!("Failed to start server: {}", e),
                        });
                    }
                }
            }
            CoreCommand::StartClient { ip, port, username } => {
                save(&mut self.config, username.clone(), ip.clone(), port.clone());
                let addr = format!("{}:{}", ip, port);
                log::info!("Executor: connecting to {} as '{}'", addr, username);
                match self.network.connect_to_server(&addr).await {
                    Ok(()) => {
                        let _ = self.event_tx.send(UIEvent::ClientConnected { addr });
                    }
                    Err(e) => {
                        log::error!("Client connection error: {}", e);
                        let _ = self.event_tx.send(UIEvent::Error {
                            message: format!("Failed to connect: {}", e),
                        });
                    }
                }
            }
            CoreCommand::SendMessage { text } => {
                if let Err(e) = self.network.send_message(text).await {
                    log::error!("Failed to send message: {}", e);
                    let _ = self.event_tx.send(UIEvent::Error {
                        message: format!("Send failed: {}", e),
                    });
                }
            }
        }
    }
}


fn save(config: &mut ConfigurationManager, username: String, ip: String, port: String) {
    log::info!("Executor: saving config (username={:?}, ip={:?}, port={:?})",
    username, ip, port);
    config.save(username, ip, port);
}

/// Spawns the executor in a dedicated thread with its own tokio runtime.
/// Returns the command sender for UI to use.
pub fn spawn_executor(
    config: ConfigurationManager,
    event_tx: Sender<UIEvent>,
) -> tokio::sync::mpsc::UnboundedSender<CoreCommand> {
    let (command_tx, command_rx) = tokio::sync::mpsc::unbounded_channel();

    std::thread::Builder::new()
        .name("core-executor".into())
        .spawn(move || {
            let rt = tokio::runtime::Runtime::new()
                .expect("Failed to build tokio runtime for executor");
            let mut executor = CoreExecutor::new(command_rx, config, event_tx);
            rt.block_on(executor.run());
        })
        .expect("Failed to spawn executor thread");

    command_tx
}