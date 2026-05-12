use slint::RenderingState;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let ui = AppWindow::new()?;
    let weak_ui = ui.as_weak();

    ui.window().set_rendering_notifier(move |state, _graphics_api| {
        match state {
            RenderingState::AfterRendering => {
                if let Some(ui) = weak_ui.upgrade() {
                    if ui.get_first_start() {
                        ui.set_first_start(false);
                    }
                }
            }
            _ => {}
        }
    }).expect("Couldn't create notifier.");

    ui.run()?;
    Ok(())
}
