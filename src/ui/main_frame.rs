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
