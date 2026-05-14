use i_slint_backend_winit::WinitWindowAccessor;
use slint::ComponentHandle;
use crate::AppWindow;

pub fn center_main_window(ui_manager: &AppWindow) {
    let weak_ui = ui_manager.as_weak();

    // AI copypasta
    if let Some(ui_manager) = weak_ui.upgrade() {
        ui_manager.window().with_winit_window(|winit_window: &i_slint_backend_winit::winit::window::Window| {
            let monitor_size = winit_window.current_monitor()
                .map(|m| m.size().to_logical::<f64>(winit_window.scale_factor()));

            let screen_size = monitor_size.unwrap_or(winit::dpi::LogicalSize::new(0.0, 0.0));

            let window_size = winit_window.inner_size().to_logical::<f64>(winit_window.scale_factor());

            let x = (screen_size.width - window_size.width) / 2.0;
            let y = (screen_size.height - window_size.height) / 2.0;

            winit_window.set_outer_position(winit::dpi::LogicalPosition::new(x, y));
        });
    }
}