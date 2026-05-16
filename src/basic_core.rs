mod configuration_manager;
mod ui_manager;
mod network;
mod logger;
mod core_executor;

use crate::basic_core::configuration_manager::ConfigurationManager;
use crate::basic_core::core_executor::CoreCommand;
// use crate::basic_core::network::NetworkManager;
use crate::basic_core::ui_manager::UIManager;

pub struct Core {
    configurator_manager: ConfigurationManager,
    ui_manager: UIManager,
    // network: network,
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
        }
    }

    pub fn setup(&mut self) -> () {
        log::info!("Setting up core; Initializing UI.");
        self.ui_manager.setup();
    }

    fn close(&mut self) {
        log::info!("Closing core");
    }

    fn compact_bridge(&mut self) {
        // creating roles
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel::<CoreCommand>();


        std::thread::spawn(async move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { core_executor::execute(receiver) }).await;
        });
    }
}
