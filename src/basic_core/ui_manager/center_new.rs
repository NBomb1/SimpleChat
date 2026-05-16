use display_info::DisplayInfo;
use slint::{ComponentHandle, Weak};
use crate::AppWindow;

pub(crate) fn center(ui: Weak<AppWindow>) -> Result<(), String> {
    // wasted 5h to come to this btw
    let ui = ui.upgrade().ok_or("Could not upgrade ui")?;
    let display_info = DisplayInfo::all()
        .map_err(|e| format!("Couldn't get display info: {e}"))?
        .into_iter()
        .next()
        .ok_or("No display were found")?;

    let phys_width = (ui.get_width_() as f32 * display_info.scale_factor) as u32;
    let phys_height = (ui.get_height_() as f32 * display_info.scale_factor) as u32;

    let x = (display_info.width as i32 - phys_width as i32) / 2;
    let y = (display_info.height as i32 - phys_height as i32) / 2;
    ui.window().set_position(slint::WindowPosition::Physical(
        slint::PhysicalPosition::new(x, y),
    ));
    Ok(())
}