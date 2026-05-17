use std::rc::Rc;
use slint::{ComponentHandle, Model, RenderingState, SharedString, Weak};
use tokio::sync::mpsc::UnboundedSender;
use crate::basic_core::ui_manager::{pages, validators, UIEvent};
use crate::AppWindow;
use crate::basic_core::core_executor::{CoreCommand};

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

    let tx_server = tx.clone();
    let weak_ui_server = weak_ui.clone();
    strong_ui.on_server_mode(move || {
        if let Some(ui) = weak_ui_server.upgrade() {
            ui.set_mode_button_is_active(false);
            let username = ui.get_username().to_string();
            let ip = ui.get_ip().to_string();
            let port = ui.get_port().to_string();

            log::info!("UI: sending command to the bridge...");
            let _ = tx_server.send(CoreCommand::StartServer { username, ip, port });
        }
    });

    let tx_client = tx.clone();
    let weak_ui_client = weak_ui.clone();
    strong_ui.on_client_mode(move || {
        if let Some(ui) = weak_ui_client.upgrade() {
            ui.set_mode_button_is_active(false);
            let username = ui.get_username().to_string();
            let ip = ui.get_ip().to_string();
            let port = ui.get_port().to_string();

            log::info!("UI: sending client connection command to the bridge...");
            let _ = tx_client.send(CoreCommand::StartClient { username, ip, port });
        }
    });


    let tx_msg = tx.clone();
    strong_ui.on_send_message_clicked(move |text| {
        if !text.is_empty() {
            let _ = tx_msg.send(CoreCommand::SendMessage { text: text.to_string() });
        }
    });
}

/// Handles events received from the executor and updates the UI accordingly.
/// Called from the UI event loop thread via `slint::invoke_from_event_loop`.
pub fn handle_ui_event(weak_ui: Weak<AppWindow>, event: UIEvent) {
    let Some(ui) = weak_ui.upgrade() else { return; };

    match event {
        UIEvent::ServerStarted { addr } => {
            log::info!("UI: Server started on {}", addr);
            ui.set_active_page(3);
        }
        UIEvent::ClientConnected { addr } => {
            log::info!("UI: Connected to {}", addr);
            ui.set_active_page(3);
        }
        UIEvent::MessageReceived { text } => {
            log::info!("UI: Rendering new message: {}", text);

            let message_history = ui.get_chat_messages();

            let mut messages: Vec<String> = message_history
                .iter()
                .map(|s| s.to_string())
                .collect();

            messages.push(text);

            let slint_messages = Rc::new(slint::VecModel::from(
                messages
                    .into_iter()
                    .map(SharedString::from)
                    .collect::<Vec<_>>()
            ));

            ui.set_chat_messages(slint_messages.into());
        }
        UIEvent::Error { message } => {
            log::error!("UI: Error — {}", message);

            ui.set_active_page(0);
            ui.set_ip_error(true);
            ui.set_port_error(true);
            ui.set_mode_button_is_active(true);
        }
    }
}
