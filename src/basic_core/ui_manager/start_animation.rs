use slint::ComponentHandle;
use crate::AppWindow;

pub fn start_animation(ui_manager: & AppWindow,){
    let weak_ui = ui_manager.as_weak();
    
    if let Some(ui_manager) = weak_ui.upgrade() {
        if ui_manager.get_first_start() {
            ui_manager.set_first_start(false);
        }
    }
    
}
