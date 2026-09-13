use wxdragon::prelude::*;

use crate::ui::main_frame::MainFrame;

pub fn create_default_values_dialog(parent: &MainFrame) -> Dialog {
    let dialog = Dialog::builder(parent.get_frame(), "Default Values")
        .with_size(500, 600)
        .build();

    let sizer = BoxSizer::builder(Orientation::Vertical).build();

    // Add controls for default values here (e.g., text boxes, labels, etc.)

    dialog.set_sizer(sizer, true);
    dialog
}
