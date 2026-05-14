mod start_animation;
mod center_main_window;
mod validators;
mod pages;

use slint::{ComponentHandle, RenderingState};
use crate::AppWindow;

pub(super) struct UIManager {
    start_window: AppWindow,
}

impl UIManager {
    pub fn new() -> UIManager {
        let ui_manager = AppWindow::new().unwrap();

        UIManager{
            start_window: ui_manager,
        }
    }

    pub fn setup(&mut self) {
        self.window_events();
        self.link_validators();

        let result = self.start_window.run();
        // in case if render goes wrong
        match result {
            Ok(_) => {}
            Err(_) => {
                log::error!("Couldn't run UI manager!");
            }
        }
    }

    fn window_events(&mut self){
        let weak_ui = self.start_window.as_weak();
        self.start_window.window().set_rendering_notifier(move |state, _graphics_api| {
            // creating variable to get current state of window
            let strong_ui = match weak_ui.upgrade() {
                Some(ui) => ui,
                None => return, // no processing if window is not available
            };

            // better to keep it "match" for the future improvements
            match state {

                RenderingState::AfterRendering => {
                    if !strong_ui.get_first_start() { return; };  // it might be called multiple times
                    center_main_window::center_main_window(&strong_ui);
                    start_animation::start_animation(&strong_ui);  // start animation changes first_start to false
                }

                _ => {}
            }
        }).expect("Couldn't create notifier.");
    }

    fn link_validators(&mut self){
        // Username
        let weak_ui_origin = self.start_window.as_weak();

        let weak_ui = weak_ui_origin.clone();
        self.start_window.on_validate_username(move |text| {
            let Some(strong_ui) = weak_ui.upgrade() else { return; };
            let is_valid = validators::validate_username(&text);
            strong_ui.set_username_error(!is_valid);
        });

        // IP
        let weak_ui = weak_ui_origin.clone();
        self.start_window.on_validate_ip(move |text| {
            let Some(strong_ui) = weak_ui.upgrade() else { return; };
            let is_valid = validators::validate_ip(&text);
            strong_ui.set_ip_error(!is_valid);
        });

        // Port
        let weak_ui = weak_ui_origin.clone();
        self.start_window.on_validate_port(move |text| {
            let Some(strong_ui) = weak_ui.upgrade() else { return; };
            let is_valid = validators::validate_port(&text);
            strong_ui.set_port_error(!is_valid);
        });

        // connection page
        let weak_ui = weak_ui_origin.clone();
        self.start_window.on_validate_connection_page(move || { pages::connection_page_validation(weak_ui.clone()); });

        let weak_ui = weak_ui_origin.clone();
        self.start_window.on_validate_username_page(move || { pages::username_page_validation(weak_ui.clone()); });
    }

}
