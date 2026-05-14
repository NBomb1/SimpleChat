mod configuration_manager;
mod ui_manager;
mod network;
mod logger;

use crate::basic_core::configuration_manager::ConfigurationManager;
use crate::basic_core::ui_manager::UIManager;

pub struct Core {
    configurator_manager: configuration_manager::ConfigurationManager,
    ui_manager: ui_manager::UIManager,
    // network: network,
}

impl Core {
    pub fn new() -> Core {
        logger::init();
        let ui = UIManager::new();

        Core{
            // configurator_manager: (),
            configurator_manager: ConfigurationManager::new(),
            ui_manager: ui,
            // network: (),
        }
    }

    pub fn setup(&mut self) -> () {
        self.ui_manager.setup();
    }
}
