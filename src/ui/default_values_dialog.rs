use wxdragon::prelude::*;

use crate::ui::MainFrame;

pub struct DefaultValuesDialog {
    dialog: Dialog,
}

impl DefaultValuesDialog {
    pub fn new(parent: &MainFrame, caption: &str) -> Self {
        let dialog = Dialog::builder(parent.get_frame(), caption)
            .with_size(500, 600)
            .build();

        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Add controls for default values here (e.g., text boxes, labels, etc.)

        dialog.set_sizer(sizer, true);
        Self { dialog }
    }

    pub fn get_dialog(&self) -> &Dialog {
        &self.dialog
    }

    pub fn show_modal(&self) {
        self.dialog.show_modal();
    }
}
