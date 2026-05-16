mod validators;
mod pages;
mod center_new;
pub(crate) mod linkers;

use slint::ComponentHandle;
use tokio::sync::mpsc::UnboundedSender;
use crate::AppWindow;
use crate::basic_core::core_executor::CoreCommand;

pub struct UIManager {
    start_window: AppWindow,
}

impl UIManager {
    pub fn new() -> UIManager {
        let ui_manager = AppWindow::new().unwrap();

        // centering is a bit of complicated
        match center_new::center(ui_manager.as_weak()) {
            Ok(()) => {},
            Err(e) => { log::info!("Couldn't center window: {}", e);} }

        UIManager{
            start_window: ui_manager,
        }
    }

    pub fn setup(&mut self) {
        linkers::ui_link_validators(&mut self.start_window.as_weak());
        linkers::window_events(&mut self.start_window.as_weak());

        let result = self.start_window.run();
        // in case if render goes wrong
        match result {
            Ok(_) => {}
            Err(_) => {
                log::error!("Couldn't run UI manager!");
            }
        }
    }

    /// Mode buttons stands for Server and Client choice. Page index 2.
    pub fn switch_state_mode_buttons(&mut self, state: bool) { self.start_window.set_mode_button_is_active(state);}
        
    pub fn change_page(&mut self, page: i32){
        self.start_window.set_active_page(page);
    }

    pub(super) fn core_link_commands(&mut self, tx: UnboundedSender<CoreCommand>) {linkers::core_link_commands(&mut self.start_window.as_weak(), tx);}
}
