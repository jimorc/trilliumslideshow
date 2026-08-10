use wxdragon::prelude::*;

pub struct MainFrame {
    value: i32,
    frame: Frame,
}

impl MainFrame {
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

    pub fn show(&self) {
        self.frame.show(true);
    }

    pub fn centre(&self) {
        self.frame.centre();
    }
}