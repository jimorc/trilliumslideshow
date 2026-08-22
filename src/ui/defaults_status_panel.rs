use wxdragon::prelude::*;

/// Defaults status panel
///
/// This panel is placed in the MainFrame when the MainFrame is created.
/// It contains status information about the loading or creation of the
/// DefaultValues object and defaults.toml file.
pub struct DefaultsStatusPanel {
    panel: Panel,
}

impl DefaultsStatusPanel {
    /// Creates a new instance of `DefaultsStatusPanel`.
    ///
    /// This is called from MainFrame::new and displays as the first panel in MainFrame.
    pub fn new(&frame: &Frame) -> Self {
        let panel = Panel::builder(&frame).build();

        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        let status = StaticText::builder(&panel).build();
        status.set_label("Attempting to load default values file");

        sizer.add(
            &status,
            1,
            SizerFlag::AlignCenterHorizontal | SizerFlag::AlignCenterVertical,
            0,
        );

        panel.set_sizer(sizer, true);

        Self { panel }
    }

    pub fn panel(&self) -> Panel {
        self.panel
    }
}
