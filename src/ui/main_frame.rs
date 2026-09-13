use wxdragon::prelude::*;

/// tssconfigurator's main window.
///
/// This struct represents the main window of the application.
pub struct MainFrame {
    pub frame: Frame,
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
        let start_button = create_start_button(&frame);
        let exit_button = create_exit_button(&frame);
        let sizer = BoxSizer::builder(Orientation::Vertical).build();
        sizer.add(
            &start_button,
            0,
            SizerFlag::All | SizerFlag::AlignCenterHorizontal,
            5,
        );
        sizer.add(
            &exit_button,
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
}

pub fn create_start_button(parent: &Frame) -> Button {
    Button::builder(parent)
        .with_label("Start Configuration")
        .build()
}

pub fn create_exit_button(parent: &Frame) -> Button {
    Button::builder(parent).with_label("Exit").build()
}
