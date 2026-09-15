use wxdragon::prelude::*;

use crate::ui::MainFrame;

pub struct DefaultValuesDialog {
    dialog: Dialog,
}

impl DefaultValuesDialog {
    pub fn new(parent: &MainFrame, caption: &str) -> Self {
        const BORDER: i32 = 5;
        let defaults = parent.get_data().unwrap();
        let dialog = Dialog::builder(parent.get_frame(), caption)
            .with_size(500, 600)
            .build();

        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Add controls for default values here (e.g., text boxes, labels, etc.)
        let size_box = create_size_box(&dialog, &defaults, BORDER);

        sizer.add(&size_box, 0, SizerFlag::Top, BORDER);

        let sep = StaticLine::builder(&dialog)
            .with_style(StaticLineStyle::Default)
            .with_size(Size::new(dialog.get_client_size().width, -1))
            .build();
        sizer.add(&sep, 0, SizerFlag::Bottom, BORDER);

        let button_sizer = create_button_sizer(&dialog);
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

fn create_size_box(&dialog: &Dialog, defaults: &crate::DataValues, border: i32) -> StaticBox {
    let size_box = StaticBox::builder(&dialog)
        .with_label("Maximum Slide Size")
        .build();
    let size_sizer = BoxSizer::builder(Orientation::Vertical).build();
    let horz_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    let width_label = StaticText::builder(&size_box).with_label("Width:").build();
    let width_text = TextCtrl::builder(&size_box)
        .with_value(&defaults.get_slide_width().to_string())
        .build();
    width_text.set_tooltip("Maximum width for slides.\nValue must be between 100 and 9999.");
    let height_label = StaticText::builder(&size_box).with_label("Height:").build();
    let height_text = TextCtrl::builder(&size_box)
        .with_value(&defaults.get_slide_height().to_string())
        .build();
    height_text.set_tooltip("Maximum height for slides.\nValue must be between 100 and 9999.");
    horz_sizer.add(
        &width_label,
        0,
        SizerFlag::All | SizerFlag::AlignCenterVertical,
        border,
    );
    horz_sizer.add(
        &width_text,
        0,
        SizerFlag::All | SizerFlag::AlignCenterVertical,
        border,
    );
    horz_sizer.add(
        &height_label,
        0,
        SizerFlag::All | SizerFlag::AlignCenterVertical,
        border,
    );
    horz_sizer.add(
        &height_text,
        0,
        SizerFlag::All | SizerFlag::AlignCenterVertical,
        border,
    );

    size_sizer.add_sizer(&horz_sizer, 0, SizerFlag::All, 0);
    let min_height = size_box.get_size().height + width_text.get_size().height + 2 * border; // Add some extra space for padding
    size_box.set_sizer(size_sizer, true);
    size_box.set_min_size(Size::new(500, min_height));

    size_box
}

fn create_button_sizer(&dialog: &Dialog) -> StdDialogButtonSizer {
    let button_sizer = StdDialogButtonSizerBuilder::new().build();
    let config_button = Button::builder(&dialog)
        .with_label("Configure Slideshow")
        // TODO: change this to ID_OK when ID_SAVE added to wxDragon
        .with_id(ID_NO)
        .build();
    config_button.set_tooltip("Click to configure a slide show.");
    config_button.on_click(move |_| {
        dialog.end_modal(ID_NO);
    });

    let quit_button = Button::builder(&dialog)
        .with_label("Quit")
        .with_id(ID_CANCEL)
        .build();
    quit_button.set_tooltip("Click to terminate the program.");

    let save_button = Button::builder(&dialog)
        .with_label("Save")
        // TODO: change this to ID_SAVE when ID_SAVE added to wxDragon
        .with_id(ID_YES)
        .build();
    save_button.set_tooltip("Click to save changes.");
    save_button.enable(false);
    save_button.on_click(move |_| {
        println!("save button clicked.");
    });

    button_sizer.add_button(&config_button);
    button_sizer.add_button(&quit_button);
    button_sizer.add_button(&save_button);
    button_sizer.realize();

    button_sizer
}
