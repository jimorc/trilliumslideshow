use wxdragon::prelude::*;

use crate::values::DataValues;

pub struct DefaultValuesDialog {
    dialog: Dialog,
    data: DataValues,
}

impl<'a> DefaultValuesDialog {
    pub fn builder(
        parent: &'a dyn WxWidget,
        caption: &'a str,
        defaults: DataValues,
    ) -> DefaultValuesDialogBuilder<'a> {
        DefaultValuesDialogBuilder {
            parent,
            caption: "",
            defaults,
        }
    }

    /*        let defaults = parent.get_data().unwrap();
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
    }*/

    pub fn show_modal(&self) -> i32 {
        self.dialog.show_modal()
    }
}

pub struct DefaultValuesDialogBuilder<'a> {
    parent: &'a dyn WxWidget,
    caption: &'a str,
    defaults: DataValues,
}

impl<'a> DefaultValuesDialogBuilder<'a> {
    pub fn with_caption(&mut self, caption: &'a str) -> &Self {
        self.caption = caption;
        self
    }

    pub fn build(&self) -> DefaultValuesDialog {
        let dialog = Dialog::builder(self.parent, self.caption)
            .with_size(500, 600)
            .build();
        DefaultValuesDialog {
            dialog,
            data: self.defaults,
        }
    }
}
/*
fn create_size_box(&dialog: &Dialog, defaults: &crate::DataValues, border: i32) -> StaticBox {
    let size_box = StaticBox::builder(&dialog)
        .with_label("Maximum Slide Size")
        .build();
    let size_sizer = BoxSizer::builder(Orientation::Vertical).build();
    let horz_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    let width_label = StaticText::builder(&size_box).with_label("Width:").build();
    let width_text = TextCtrl::builder(&size_box)
        .with_value(&defaults.get_slide_width().to_string())
        .with_style(TextCtrlStyle::ProcessEnter)
        .build();
    width_text.set_tooltip(
        "Maximum width for slides.\nValue must be between 100 and 9999.\nOnly digits are accepted.",
    );
    // accept only digits
    width_text.on_key_down(|event| {
        if let WindowEventData::Keyboard(ref key_data) = event {
            event.skip(number_key_down(key_data));
        }
    });
    let height_label = StaticText::builder(&size_box).with_label("Height:").build();
    let height_text = TextCtrl::builder(&size_box)
        .with_value(&defaults.get_slide_height().to_string())
        .build();
    height_text.set_tooltip("Maximum height for slides.\nValue must be between 100 and 9999.\nOnly digits are accepted.");
    // accepts only digits
    height_text.on_key_down(|event: WindowEventData| {
        if let WindowEventData::Keyboard(ref key_data) = event {
            event.skip(number_key_down(key_data));
        }
    });
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

fn number_key_down(key_data: &KeyboardEvent) -> bool {
    const ZERO: i32 = 0x30;
    const NINE: i32 = 0x39;
    const BACKSPACE: i32 = 0x08;
    const DELETE: i32 = 0x7f;
    const TAB: i32 = 0x09;
    const LEFT: i32 = 314;
    const RIGHT: i32 = 316;
    const HOME: i32 = 313;
    const END: i32 = 312;
    const KEYPAD_ZERO: i32 = 324;
    const KEYPAD_NINE: i32 = 333;
    const KEYPAD_HOME: i32 = 375;
    const KEYPAD_END: i32 = 382;
    const KEYPAD_LEFT: i32 = 376;
    const KEYPAD_RIGHT: i32 = 378;
    match key_data.get_unicode_key() {
        Some(key) => {
            if key_data.alt_down()
                || key_data.cmd_down()
                || key_data.control_down()
                || key_data.meta_down()
                || key_data.shift_down()
            {
                return false;
            }
            (ZERO..=NINE).contains(&key) || key == BACKSPACE || key == DELETE || key == TAB
        }
        None => {
            let key_code = key_data.get_key_code().unwrap();
            key_code == LEFT
                || key_code == RIGHT
                || key_code == HOME
                || key_code == END
                || key_code == KEYPAD_HOME
                || key_code == KEYPAD_END
                || key_code == KEYPAD_LEFT
                || key_code == KEYPAD_RIGHT
                || (KEYPAD_ZERO..=KEYPAD_NINE).contains(&key_code)
        }
    }
}
*/
