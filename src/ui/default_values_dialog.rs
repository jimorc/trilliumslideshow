use wxdragon::prelude::*;

use crate::{ui::ImageSizeCtrl, values::DataValues};

// Border size around most widgets
const BORDER: i32 = 5;

pub struct DefaultValuesDialog {
    dialog: Dialog,
    data: DataValues,
    width_ctrl: ImageSizeCtrl,
    height_ctrl: ImageSizeCtrl,
}

impl<'a> DefaultValuesDialog {
    pub fn builder(
        parent: &'a dyn WxWidget,
        caption: &'a str,
        defaults: DataValues,
    ) -> DefaultValuesDialogBuilder<'a> {
        DefaultValuesDialogBuilder {
            parent,
            caption,
            width: -1,
            height: -1,
            border: BORDER,
            dialog: None,
            defaults,
            width_ctrl: None,
            height_ctrl: None,
        }
    }

    pub fn show_modal(&self) -> i32 {
        self.dialog.show_modal()
    }
}

pub struct DefaultValuesDialogBuilder<'a> {
    parent: &'a dyn WxWidget,
    caption: &'a str,
    width: i32,
    height: i32,
    border: i32,
    dialog: Option<Dialog>,
    defaults: DataValues,
    width_ctrl: Option<ImageSizeCtrl>,
    height_ctrl: Option<ImageSizeCtrl>,
}

impl<'a> DefaultValuesDialogBuilder<'a> {
    pub fn with_caption(&mut self, caption: &'a str) -> &Self {
        self.caption = caption;
        self
    }

    pub fn with_size(&mut self, width: i32, height: i32) -> &Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn build(&mut self) -> DefaultValuesDialog {
        let dialog = Dialog::builder(self.parent, self.caption)
            .with_size(self.width, self.height)
            .build();
        self.dialog = Some(dialog);
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Add controls for default values here (e.g., text boxes, labels, etc.)
        let size_box = self.create_size_box();
        sizer.add(&size_box, 0, SizerFlag::Top, BORDER);

        let sep = StaticLine::builder(&dialog)
            .with_style(StaticLineStyle::Default)
            .with_size(Size::new(dialog.get_client_size().width, -1))
            .build();
        sizer.add(&sep, 0, SizerFlag::Bottom, BORDER);

        let button_sizer = self.create_button_sizer();
        sizer.add_sizer(
            &button_sizer,
            0,
            SizerFlag::Bottom | SizerFlag::AlignRight,
            BORDER,
        );

        dialog.set_sizer(sizer, true);
        dialog.fit();
        DefaultValuesDialog {
            dialog: dialog,
            data: self.defaults,
            width_ctrl: self.width_ctrl.unwrap(),
            height_ctrl: self.height_ctrl.unwrap(),
        }
    }

    fn create_size_box(&mut self) -> StaticBox {
        let size_box = StaticBox::builder(&self.dialog.unwrap())
            .with_label("Maximum Slide Size")
            .build();
        let size_sizer = BoxSizer::builder(Orientation::Vertical).build();
        let horz_sizer = BoxSizer::builder(Orientation::Horizontal).build();
        let width_label = StaticText::builder(&size_box).with_label("Width:").build();
        self.width_ctrl =
            Some(ImageSizeCtrl::builder(&size_box, self.defaults.get_slide_width()).build());
        let height_label = StaticText::builder(&size_box).with_label("Height:").build();
        self.height_ctrl =
            Some(ImageSizeCtrl::builder(&size_box, self.defaults.get_slide_height()).build());
        horz_sizer.add(
            &width_label,
            0,
            SizerFlag::All | SizerFlag::AlignCenterVertical,
            self.border,
        );
        horz_sizer.add(
            self.width_ctrl.unwrap().get_ctrl(),
            0,
            SizerFlag::All | SizerFlag::AlignCenterVertical,
            self.border,
        );
        horz_sizer.add(
            &height_label,
            0,
            SizerFlag::All | SizerFlag::AlignCenterVertical,
            self.border,
        );
        horz_sizer.add(
            self.height_ctrl.unwrap().get_ctrl(),
            0,
            SizerFlag::All | SizerFlag::AlignCenterVertical,
            self.border,
        );

        size_sizer.add_sizer(&horz_sizer, 0, SizerFlag::All, 0);
        let min_height = size_box.get_size().height
            + self.width_ctrl.unwrap().get_size().height
            + 2 * self.border; // Add some extra space for padding
        size_box.set_sizer(size_sizer, true);
        size_box.set_min_size(Size::new(500, min_height));

        size_box
    }

    fn create_button_sizer(&mut self) -> StdDialogButtonSizer {
        let dialog = self.dialog.unwrap();
        let button_sizer = StdDialogButtonSizerBuilder::new().build();
        let config_button = self.create_config_button();
        let quit_button = self.create_quit_button();

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

    fn create_config_button(&self) -> Button {
        let dialog = self.dialog.unwrap();
        let config_button = Button::builder(&dialog)
            .with_label("Configure Slideshow")
            // TODO: change this to ID_OK when ID_SAVE added to wxDragon
            .with_id(ID_NO)
            .build();
        config_button.set_tooltip("Click to configure a slide show.");
        config_button.on_click(move |_| {
            dialog.end_modal(ID_NO);
        });
        config_button
    }

    fn create_quit_button(&self) -> Button {
        let dialog = self.dialog.unwrap();
        let quit_button = Button::builder(&dialog)
            .with_label("Quit")
            .with_id(ID_CANCEL)
            .build();
        quit_button.set_tooltip("Click to terminate the program.");
        quit_button
    }
}
