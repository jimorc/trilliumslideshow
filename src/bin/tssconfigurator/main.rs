use trilliumslideshow::config::default_values::DefaultValues;
use trilliumslideshow::ui::main_frame::MainFrame;

fn main() {
    let _defaults = DefaultValues::new();

    let _ = wxdragon::main(|_| {
        let main_frame = MainFrame::new();
        main_frame.show();
        main_frame.centre();
    });
}
