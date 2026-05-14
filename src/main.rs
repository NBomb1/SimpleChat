mod basic_core;

use crate::basic_core::Core;

slint::include_modules!();

fn main() {
    let mut core = Core::new();
    core.setup();
}
