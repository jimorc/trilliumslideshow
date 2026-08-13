pub mod config;
pub mod gui;

//use config::default_values::DefaultValues;
use gui::main_frame::MainFrame;

fn main() {
    //    let defaults = DefaultValues::new();

    let _ = wxdragon::main(|_| {
        let main_frame = MainFrame::new();
        main_frame.show();
        main_frame.centre();
    });
}
