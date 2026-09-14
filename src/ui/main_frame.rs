use wxdragon::prelude::*;

use crate::ui::DefaultValuesDialog;

/// tssconfigurator's main window.
///
/// This struct represents the main window of the application.
pub struct MainFrame {
    frame: Frame,
}

impl MainFrame {
    /// Creates a new instance of `MainFrame`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let frame = MainFrame::new();
    /// frame.show();
    /// ```
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
        let frame = Frame::builder()
            .with_title("Slideshow Configurator")
            .with_size(Size::new(400, 200))
            .build();
        let main_frame = Self { frame };
        let start_button = create_start_button(main_frame);
        let quit_button = create_quit_button(&frame);
        let sizer = BoxSizer::builder(Orientation::Vertical).build();
        sizer.add(
            &start_button,
            0,
            SizerFlag::All | SizerFlag::AlignCenterHorizontal,
            5,
        );
        sizer.add(
            &quit_button,
            0,
            SizerFlag::All | SizerFlag::AlignCenterHorizontal,
            5,
        );
        frame.set_sizer(sizer, true);
        Self { frame }
    }

    /// Shows the main frame.
    ///
    /// # Example
    /// ```ignore
    /// let frame = MainFrame::new();
    /// frame.show();
    /// ```
    pub fn show(&self) {
        self.frame.show(true);
    }

    /// Centers the main frame on the screen.
    ///
    /// # Example
    /// ```ignore
    /// let frame = MainFrame::new();
    /// frame.show();
    /// frame.centre();
    /// ```
    pub fn centre(&self) {
        self.frame.centre();
    }

    /// Returns a reference to the underlying `Frame`.
    pub fn get_frame(&self) -> &Frame {
        &self.frame
    }
}

pub fn create_start_button(parent: MainFrame) -> Button {
    let button = Button::builder(parent.get_frame())
        .with_label("Start Configuration")
        .build();
    button.on_click(move |_| {
        let dialog = DefaultValuesDialog::new(&parent, "Default Values");
        match dialog.get_dialog().show_modal() {
            ID_OK => {
                println!("OK button clicked");
            }
            ID_CANCEL => {
                println!("Quit button clicked");
                std::process::exit(0);
            }
            _ => {
                // Handle other cases
            }
        }
        // Here you can add the logic to start the configuration process.
    });
    button
}

pub fn create_quit_button(parent: &Frame) -> Button {
    let button = Button::builder(parent).with_label("Quit").build();
    button.on_click(move |_| {
        std::process::exit(0);
    });
    button
}
