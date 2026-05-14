mod start_animation;
mod center_main_window;

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

        // creating event trigger.
        let weak_ui = self.start_window.as_weak();
        self.start_window.window().set_rendering_notifier(move |state, _graphics_api| {
            // creating variable to get current state of window
            let strong_ui = match weak_ui.upgrade() {
                Some(ui) => ui,
                None => return, // no processing if window is not available
            };

            match state { // better to keep it "match" for the future improvements
                RenderingState::AfterRendering => {
                    if !strong_ui.get_first_start() { return; };
                    center_main_window::center_main_window(&strong_ui);
                    start_animation::start_animation(&strong_ui);
                }
                _ => {}
            }
        }).expect("Couldn't create notifier.");


        let result = self.start_window.run();
        // in case if render goes wrong
        match result {
            Ok(_) => {}
            Err(_) => {
                log::error!("Couldn't run UI manager!");
            }
        }
    }

}
