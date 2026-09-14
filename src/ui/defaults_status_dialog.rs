use wxdragon::prelude::*;

use crate::ui::MainFrame;
use crate::values::{DataValues, DefaultValuesStatus};

/// Defaults status dialog
///
/// This dialog displays status information about the loading or creation of the
/// DefaultValues object and defaults.toml file.
pub struct DefaultsStatusDialog {
    dialog: Dialog,
    defaults: DataValues,
}

impl DefaultsStatusDialog {
    /// Creates a new instance of `DefaultsStatusDialog`.
    ///
    /// This is called from MainFrame::new and displays as the first dialog in MainFrame.
    pub fn new(parent: &mut MainFrame, caption: &str) -> Self {
        const BORDER: i32 = 5;
        let dialog = Dialog::builder(parent.get_frame(), caption).build();

        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        let (defaults, statuses) = DataValues::from_config_file_if_exists();
        parent.set_defaults(Some(defaults));
        let status = Self::build_status_label(&dialog, statuses.clone());

        sizer.add(&status, 1, SizerFlag::Top, 5);

        let button_sizer = StdDialogButtonSizerBuilder::new().build();
        let edit_button = Button::builder(&dialog)
            .with_label("Edit Defaults")
            .with_id(ID_APPLY)
            .build();
        edit_button.on_click(move |_| {
            dialog.end_modal(ID_APPLY);
        });
        let quit_button = Button::builder(&dialog)
            .with_label("Quit")
            .with_id(ID_CANCEL)
            .build();
        let config_button = Button::builder(&dialog)
            .with_label("Configure Slideshow")
            .with_id(ID_OK)
            .build();

        button_sizer.add_button(&edit_button);
        button_sizer.add_button(&config_button);
        button_sizer.add_button(&quit_button);
        button_sizer.realize();

        let sep = StaticLine::builder(&dialog)
            .with_style(StaticLineStyle::Default)
            .with_size(Size::new(dialog.get_client_size().width, -1))
            .build();
        sizer.add(&sep, 0, SizerFlag::Bottom, BORDER);
        sizer.add_sizer(
            &button_sizer,
            0,
            SizerFlag::Bottom | SizerFlag::AlignRight,
            BORDER,
        );
        dialog.set_sizer(sizer, true);
        dialog.fit();
        dialog.set_sizer(sizer, true);

        Self { dialog, defaults }
    }

    pub fn get_dialog(&self) -> &Dialog {
        &self.dialog
    }

    pub fn data_values(&self) -> DataValues {
        self.defaults
    }

    fn build_status_label(&dialog: &Dialog, statuses: Vec<DefaultValuesStatus>) -> StaticText {
        let status = StaticText::builder(&dialog).build();

        let mut label = String::new();
        for (i, st) in statuses.iter().enumerate() {
            if i != 0 {
                label += "\n\n";
            }
            label += st.to_string().as_str();
        }
        status.set_label(label.as_str());

        status
    }
}
