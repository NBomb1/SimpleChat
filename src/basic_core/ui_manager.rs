mod validators;
mod pages;
mod center_new;
pub(crate) mod linkers;

use std::sync::mpsc::Receiver;
use slint::{ComponentHandle, SharedString};
use tokio::sync::mpsc::UnboundedSender;
use crate::AppWindow;
use crate::basic_core::core_executor::CoreCommand;

/// Events sent from the executor back to UI.
pub enum UIEvent {
    ServerStarted { addr: String },
    ClientConnected { addr: String },
    MessageReceived { text: String },
    Error { message: String },
}

pub struct UIManager {
    start_window: AppWindow,
}

impl UIManager {
    pub fn new(ip: String, port: String, username: String) -> UIManager {
        let ui_manager = AppWindow::new().unwrap();

        ui_manager.set_ip(SharedString::from(ip));
        ui_manager.set_port(SharedString::from(port));
        ui_manager.set_username(SharedString::from(username));

        // Centering is a bit complicated
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
        // propagate render errors to the log
        match result {
            Ok(_) => {}
            Err(_) => {
                log::error!("Couldn't run UI manager!");
            }
        }
    }

    /// Receiver
    pub fn start_event_listener(&self, event_rx: Receiver<UIEvent>) {
        let weak_ui = self.start_window.as_weak();
        std::thread::Builder::new()
            .name("ui-event-listener".into())
            .spawn(move || {
                while let Ok(event) = event_rx.recv() {
                    let weak = weak_ui.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        linkers::handle_ui_event(weak, event);
                    });
                }
                log::info!("UI event listener stopped.");
            })
            .expect("Failed to spawn UI event listener thread");
    }

    pub(super) fn core_link_commands(&mut self, tx: UnboundedSender<CoreCommand>) {linkers::core_link_commands(&mut self.start_window.as_weak(), tx);}
}
