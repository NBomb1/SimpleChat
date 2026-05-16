mod configuration_manager;
mod ui_manager;
mod network;
mod logger;
mod core_executor;

use tokio::sync::mpsc::UnboundedSender;
use crate::basic_core::configuration_manager::ConfigurationManager;
use crate::basic_core::core_executor::CoreCommand;
use crate::basic_core::ui_manager::UIManager;

pub struct Core {
    configurator_manager: ConfigurationManager,
    ui_manager: UIManager,
    // network: network,
    command_tx: UnboundedSender<CoreCommand>
}

impl Core {
    pub fn new() -> Core {
        logger::init(); // setting up logger

        let ui = UIManager::new();  // ui
        let configuration_manager = ConfigurationManager::new();  // configs

        Core{
            configurator_manager: configuration_manager,
            ui_manager: ui,
            // network: (),
            command_tx: core_executor::create_compact_bridge(),
        }
    }

    pub fn setup(&mut self) -> () {
        log::info!("Setting up core; Initializing UI.");
        self.ui_manager.core_link_commands(self.command_tx.clone());
        self.ui_manager.setup();
    }

    fn close(&mut self) {
        log::info!("Closing core");
    }
    

}
