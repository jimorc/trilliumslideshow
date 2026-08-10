use wxdragon::prelude::*;

/// The program's main window.
///
/// This struct represents the main window of the application.
/// 
pub struct MainFrame {
    value: i32,
    frame: Frame,
}

impl MainFrame {
    /// Creates a new instance of `MainFrame`.
    /// 
    /// # Example
    /// 
    /// ```
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

        let button = Button::builder(&frame).with_label("Click me").build();

        button.on_click(|_| {
            println!("Button clicked");
        });

        sizer.add(
            &button,
            1,
            SizerFlag::AlignCenterHorizontal | SizerFlag::AlignCenterVertical,
            0,
        );

        frame.set_sizer(sizer, true);

        Self { value: 42, frame }
    }

    /// Shows the main frame.
    /// 
    /// # Example
    /// ```
    /// let frame = MainFrame::new();
    /// frame.show();
    /// ```
    pub fn show(&self) {
        self.frame.show(true);
    }

    /// Centers the main frame on the screen.
    /// 
    /// # Example
    /// ```
    /// let frame = MainFrame::new();
    /// frame.show();
    /// frame.centre();
    /// ```
    pub fn centre(&self) {
        self.frame.centre();
    }
}