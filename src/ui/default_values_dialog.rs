use wxdragon::{prelude::*, widgets::button};

use crate::ui::MainFrame;

pub struct DefaultValuesDialog {
    dialog: Dialog,
}

impl DefaultValuesDialog {
    pub fn new(parent: &MainFrame, caption: &str) -> Self {
        const BORDER: i32 = 5;
        let dialog = Dialog::builder(parent.get_frame(), caption)
            .with_size(500, 600)
            .build();

        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Add controls for default values here (e.g., text boxes, labels, etc.)
        let size_box = StaticBox::builder(&dialog)
            .with_label("Maximum Slide Size")
            .build();
        let size_sizer = BoxSizer::builder(Orientation::Vertical).build();
        let horz_sizer = BoxSizer::builder(Orientation::Horizontal).build();
        let width_label = StaticText::builder(&size_box).with_label("Width:").build();
        let width_text = TextCtrl::builder(&size_box).with_value("800").build();
        let height_label = StaticText::builder(&size_box).with_label("Height:").build();
        let height_text = TextCtrl::builder(&size_box).with_value("600").build();
        horz_sizer.add(
            &width_label,
            0,
            SizerFlag::All | SizerFlag::AlignCenterVertical,
            BORDER,
        );
        horz_sizer.add(
            &width_text,
            0,
            SizerFlag::All | SizerFlag::AlignCenterVertical,
            BORDER,
        );
        horz_sizer.add(
            &height_label,
            0,
            SizerFlag::All | SizerFlag::AlignCenterVertical,
            BORDER,
        );
        horz_sizer.add(
            &height_text,
            0,
            SizerFlag::All | SizerFlag::AlignCenterVertical,
            BORDER,
        );
        size_sizer.add_sizer(&horz_sizer, 0, SizerFlag::All, 0);
        let min_height = size_box.get_size().height + width_text.get_size().height + 2 * BORDER; // Add some extra space for padding
        size_box.set_sizer(size_sizer, true);
        size_box.set_min_size(Size::new(500, min_height));

        sizer.add(&size_box, 0, SizerFlag::Top, BORDER);

        let sep = StaticLine::builder(&dialog)
            .with_style(StaticLineStyle::Default)
            .with_size(Size::new(dialog.get_client_size().width, -1))
            .build();
        sizer.add(&sep, 0, SizerFlag::Bottom, BORDER);

        let button_sizer = StdDialogButtonSizerBuilder::new().build();
        let ok_button = button::Button::builder(&dialog)
            .with_label("OK")
            .with_id(ID_OK)
            .build();
        let quit_button = button::Button::builder(&dialog)
            .with_label("Quit")
            .with_id(ID_CANCEL)
            .build();

        button_sizer.add_button(&ok_button);
        button_sizer.add_button(&quit_button);
        button_sizer.realize();

        sizer.add_sizer(
            &button_sizer,
            0,
            SizerFlag::Bottom | SizerFlag::AlignRight,
            BORDER,
        );

        dialog.set_sizer(sizer, true);
        dialog.fit();
        Self { dialog }
    }

    pub fn get_dialog(&self) -> &Dialog {
        &self.dialog
    }

    pub fn show_modal(&self) {
        self.dialog.show_modal();
    }
}
