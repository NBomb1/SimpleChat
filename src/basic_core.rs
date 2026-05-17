mod configuration_manager;
mod ui_manager;
mod network;
mod logger;
pub(crate) mod core_executor;

use std::sync::mpsc;
use crate::basic_core::configuration_manager::ConfigurationManager;
use crate::basic_core::ui_manager::{UIEvent, UIManager};

pub struct Core {
    ui_manager: UIManager,
    command_tx: tokio::sync::mpsc::UnboundedSender<core_executor::CoreCommand>,
    event_rx: Option<mpsc::Receiver<UIEvent>>,
}

impl Core {
    pub fn new() -> Core {
        logger::init();

        let config = ConfigurationManager::new();
        let (ip, port, username) = (  // cache values before config is moved into the executor
            config.config.ip.clone(),
            config.config.port.clone(),
            config.config.username.clone(),);

        let (event_tx, event_rx) = mpsc::channel::<UIEvent>();
        
        // executes commands
        let command_tx = core_executor::spawn_executor(config, event_tx);

        let ui = UIManager::new(ip, port, username); // giving saving values

        Core {
            ui_manager: ui,
            command_tx,
            event_rx: Some(event_rx)
        }
    }

    pub fn setup(&mut self) {
        log::info!("Setting up core; Initializing UI.");
        self.ui_manager.core_link_commands(self.command_tx.clone());

        // Start listening for events from the executor → UI
        if let Some(event_rx) = self.event_rx.take() {
            self.ui_manager.start_event_listener(event_rx);
        }

        self.ui_manager.setup();  // creates loop
        self.close()  // After loop is gone
    }

    pub fn close(&mut self) {
        log::info!("Closing core");
    }
}
