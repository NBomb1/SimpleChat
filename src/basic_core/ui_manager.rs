mod validators;
mod pages;
mod center_new;

use slint::{ComponentHandle, RenderingState};
use crate::AppWindow;

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
        self.link_validators();
        self.window_events();

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
            let first_start = strong_ui.get_first_start();

            // better to keep it "match" for the future use
            match state {
                RenderingState::BeforeRendering => {
                    if !first_start { return; };  // it might be called multiple times
                    strong_ui.set_first_start(false);  // starts animation
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

        let weak_ui = weak_ui_origin.clone();
        self.start_window.on_client_mode(move || {
            let ui = weak_ui.upgrade().unwrap();
            let ip = ui.get_ip().to_string();
            let port = ui.get_port().parse::<u16>().unwrap_or(0);


        });

        let weak_ui = weak_ui_origin.clone();
        self.start_window.on_server_mode(move || {
            let ui = weak_ui.upgrade().unwrap();
            let ip = ui.get_ip().to_string();
            let port = ui.get_port().parse::<u16>().unwrap_or(0);

            std::thread::spawn(move || {
                // 3. Внутри этого потока запускаем Tokio Runtime
                let rt = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap();

                rt.block_on(async {
                    log::info!("Starting server from a new thread...");
                });
            });
        });
    }
}
