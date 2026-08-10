pub mod gui;

use gui::main_frame::MainFrame;

fn main() {
    let _ = wxdragon::main(|_| {
        let main_frame = MainFrame::new();
        main_frame.show();
        main_frame.centre();
    });
}
