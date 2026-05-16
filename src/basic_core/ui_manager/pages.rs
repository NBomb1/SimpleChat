use slint::Weak;
use crate::AppWindow;
use crate::basic_core::ui_manager::validators::{validate_ip, validate_port, validate_username};

pub fn change_page(strong_ui: AppWindow, page: i32){
    strong_ui.set_active_page(page);
    log::info!("Switching to page {page}.")
}

/// Validating connection info page before proceeding to the username page.
pub fn connection_page_validation(ui: Weak<AppWindow>) {
    let Some(strong_ui) = ui.upgrade() else { return; };

    let (ip_shared, port_shared) = (strong_ui.get_ip(), strong_ui.get_port());
    let (ip_raw, port_raw) = (ip_shared.as_str(), port_shared.as_str());
    let (ip, port) = (validate_ip(ip_raw), validate_port(port_raw));

    strong_ui.set_ip_error(!ip);
    strong_ui.set_port_error(!port);

    if ip && port{
        log::info!("Successfully validated connection info ({ip_raw}, {port_raw})");
        change_page(strong_ui, 1);
    }
    else{
        log::info!("Failed to validate connection info ({ip_raw}, {port_raw})");
    }
}

pub fn username_page_validation(ui: Weak<AppWindow>) {
    let Some(strong_ui) = ui.upgrade() else { return; };

    let username_raw = strong_ui.get_username();

    if validate_username(&*username_raw){
        log::info!("Successfully validated username ({username_raw})");
        change_page(strong_ui, 2);
    }
    else{
        log::info!("Failed to validate username ({username_raw})");
    }
}
