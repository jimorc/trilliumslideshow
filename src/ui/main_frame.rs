use crate::ui::defaults_status_panel::DefaultsStatusPanel;

use wxdragon::prelude::*;

/// The program's main window.
///
/// This struct represents the main window of the application.
///
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
    pub fn new() -> Self {
        SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
        let frame = Frame::builder()
            .with_title("Hello, World!")
            .with_size(Size::new(300, 200))
            .build();

        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        let panel = DefaultsStatusPanel::new(&frame);
        sizer.add(
            &panel.panel(),
            1,
            SizerFlag::AlignCenterHorizontal | SizerFlag::AlignCenterVertical,
            0,
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

    fn set_sizer(&self, sizer: BoxSizer, delete_old_sizer: bool) {
        self.frame.set_sizer(sizer, delete_old_sizer);
    }

    pub fn set_panel(&self, &panel: &Panel) {
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        sizer.add(
            &panel,
            1,
            SizerFlag::AlignCenterHorizontal | SizerFlag::AlignCenterVertical,
            0,
        );

        self.set_sizer(sizer, true);
    }
}
