use slint::{ComponentHandle, RenderingState, Weak};
use tokio::sync::mpsc::UnboundedSender;
use crate::basic_core::ui_manager::{pages, validators};
use crate::AppWindow;
use crate::basic_core::core_executor::CoreCommand;

pub(crate) fn window_events(weak_ui: &mut Weak<AppWindow>){
    let strong_ui = weak_ui.upgrade().unwrap();

    let weak_ui = strong_ui.as_weak();
    strong_ui.window().set_rendering_notifier(move |state, _graphics_api| {
        // creating variable to get current state of window
        let strong_ui = match weak_ui.upgrade() {
            Some(ui) => ui,
            None => return, // no processing if window is not available
        };
        let first_start = strong_ui.get_first_start();

        // better to keep it "match" for the future use
        match state {
            RenderingState::AfterRendering => {
                if !first_start { return; };  // it might be called multiple times
                strong_ui.set_first_start(false);  // starts animation
            }
            _ => {}
        }
    }).expect("Couldn't create notifier.");
}

pub(crate) fn ui_link_validators(weak_ui: &mut Weak<AppWindow>){
    let strong_ui = weak_ui.upgrade().unwrap();

    let weak_ui_origin = strong_ui.as_weak();

    let weak_ui = weak_ui_origin.clone();
    strong_ui.on_validate_username(move |text| {
        let Some(strong_ui) = weak_ui.upgrade() else { return; };
        let is_valid = validators::validate_username(&text);
        strong_ui.set_username_error(!is_valid);
    });

    // IP
    let weak_ui = weak_ui_origin.clone();
    strong_ui.on_validate_ip(move |text| {
        let Some(strong_ui) = weak_ui.upgrade() else { return; };
        let is_valid = validators::validate_ip(&text);
        strong_ui.set_ip_error(!is_valid);
    });

    // Port
    let weak_ui = weak_ui_origin.clone();
    strong_ui.on_validate_port(move |text| {
        let Some(strong_ui) = weak_ui.upgrade() else { return; };
        let is_valid = validators::validate_port(&text);
        strong_ui.set_port_error(!is_valid);
    });

    // connection page
    let weak_ui = weak_ui_origin.clone();
    strong_ui.on_validate_connection_page(move || { pages::connection_page_validation(weak_ui.clone()); });

    let weak_ui = weak_ui_origin.clone();
    strong_ui.on_validate_username_page(move || { pages::username_page_validation(weak_ui.clone()); });
}

pub fn core_link_commands(weak_ui: &mut Weak<AppWindow>, tx: UnboundedSender<CoreCommand>) {
    let strong_ui = weak_ui.upgrade().unwrap();

    let weak_ui = strong_ui.as_weak();

    // Клон канала для кнопки СЕРВЕРА
    let tx_server = tx.clone();
    let weak_ui_server = weak_ui.clone();
    strong_ui.on_server_mode(move || {
        if let Some(ui) = weak_ui_server.upgrade() {
            ui.set_mode_button_is_active(false);
            let username = ui.get_username().to_string();
            let ip = ui.get_ip().to_string();
            let port = ui.get_port().to_string();

            log::info!("UI: Отправляю команду запуска сервера в мост...");
            let _ = tx_server.send(CoreCommand::StartServer { username, ip, port });
        }
    });

    // Клон канала для кнопки КЛИЕНТА
    let tx_client = tx.clone();
    let weak_ui_client = weak_ui.clone();
    strong_ui.on_client_mode(move || {
        if let Some(ui) = weak_ui_client.upgrade() {
            ui.set_mode_button_is_active(false);
            let username = ui.get_username().to_string();
            let ip = ui.get_ip().to_string();
            let port = ui.get_port().to_string();

            log::info!("UI: Отправляю команду подключения клиента в мост...");
            let _ = tx_client.send(CoreCommand::StartClient { username, ip, port });
        }
    });
}
