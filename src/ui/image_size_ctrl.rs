use wxdragon::{event::KeyboardEvent, prelude::*};

#[derive(Copy, Clone)]
pub struct ImageSizeCtrl {
    // Option only to allow testing
    ctrl: Option<TextCtrl>,
    value: i32,
    ignore: bool,
}

const MIN_SIZE: i32 = 100;
const MAX_SIZE: i32 = 9999;

impl<'a> ImageSizeCtrl {
    pub fn builder(parent: &'a dyn WxWidget, value: i32) -> ImageSizeCtrlBuilder<'a> {
        ImageSizeCtrlBuilder { parent, value }
    }
    pub fn get_ctrl(&self) -> &TextCtrl {
        self.ctrl.as_ref().unwrap()
    }

    pub fn get_value(&mut self) -> i32 {
        if self.value < MIN_SIZE {
            self.set_value(MIN_SIZE);
        } else if self.value > MAX_SIZE {
            self.set_value(MAX_SIZE);
        }
        self.value
    }

    pub fn get_value_as_string(&mut self) -> String {
        let value = self.get_value();
        value.to_string()
    }

    pub fn set_value(&mut self, val: i32) {
        if !(MIN_SIZE..=MAX_SIZE).contains(&val) {
            panic!(
                "Programming error: Attempted to store the value {} in an ImageSizeCtrl. The value was not between {} and {}.",
                val, MIN_SIZE, MAX_SIZE
            );
        }
        self.value = val;
    }

    pub fn get_ignore(&self) -> bool {
        self.ignore
    }

    pub fn set_ignore(&mut self, ignore: bool) {
        self.ignore = ignore;
    }

    pub fn get_size(&self) -> Size {
        self.ctrl.unwrap().get_size()
    }

    pub fn validate(self) -> bool {
        let ctrl_value: i32 = self.ctrl.unwrap().get_value().parse().unwrap();
        if (MIN_SIZE..=MAX_SIZE).contains(&ctrl_value) {
            false
        } else {
            let msg = format!("Value must be between {} and {}", MIN_SIZE, MAX_SIZE);
            let msg_dialog = MessageDialog::builder(self.get_ctrl(), &msg, "Invalid Value")
                .with_style(MessageDialogStyle::IconError | MessageDialogStyle::OK)
                .build();
            msg_dialog.show_modal();
            self.get_ctrl().set_focus();
            true
        }
    }
}

pub struct ImageSizeCtrlBuilder<'a> {
    parent: &'a dyn WxWidget,
    value: i32,
}

impl<'a> ImageSizeCtrlBuilder<'a> {
    pub fn build(&self) -> ImageSizeCtrl {
        let text = TextCtrl::builder(self.parent)
            .with_value(&self.get_value_as_string())
            .with_style(TextCtrlStyle::ProcessEnter)
            .build();
        text.set_tooltip(
        "Maximum width for slides.\nValue must be between 100 and 9999.\nOnly digits are accepted.",
    );
        // accept only digits
        text.on_key_down(|event| {
            if let WindowEventData::Keyboard(ref key_data) = event {
                event.skip(should_skip_key_down(key_data));
            }
        });
        let ctrl = ImageSizeCtrl {
            ctrl: Some(text),
            value: self.value,
            ignore: false,
        };

        text.on_kill_focus(move |_| {
            if !ctrl.get_ignore() {
                ctrl.validate();
            }
        });
        ctrl
    }

    pub fn with_value(&mut self, val: i32) -> &ImageSizeCtrlBuilder<'a> {
        self.value = val;
        self
    }

    pub fn get_value_as_string(&self) -> String {
        self.value.to_string()
    }
}

/// should_skip_key_down determines if the event should be passed on or skipped.
/// Only digits and a few unicode keys or key codes are are passed.
///
/// Note: calls to event.skip seem to be logically backward. That is, calling event.skip(false) actually
/// stops key down processing.
///
/// Note: Because KeyboardEvent is a thin wrapper around a C++ pointer, it is not possible to create a
/// KeyboardEvent object purely in rust to test this function.
fn should_skip_key_down(key_data: &KeyboardEvent) -> bool {
    const ZERO: i32 = 0x30;
    const NINE: i32 = 0x39;
    const BACKSPACE: i32 = 0x08;
    const DELETE: i32 = 0x7f;
    const TAB: i32 = 0x09;
    const LEFT: i32 = 314;
    const RIGHT: i32 = 316;
    const HOME: i32 = 313;
    const END: i32 = 312;
    const KEYPAD_ZERO: i32 = 324;
    const KEYPAD_NINE: i32 = 333;
    const KEYPAD_HOME: i32 = 375;
    const KEYPAD_END: i32 = 382;
    const KEYPAD_LEFT: i32 = 376;
    const KEYPAD_RIGHT: i32 = 378;
    match key_data.get_unicode_key() {
        Some(key) => {
            if key_data.alt_down()
                || key_data.cmd_down()
                || key_data.control_down()
                || key_data.meta_down()
                || key_data.shift_down()
            {
                return false;
            }
            (ZERO..=NINE).contains(&key) || key == BACKSPACE || key == DELETE || key == TAB
        }
        None => {
            let key_code = key_data.get_key_code().unwrap();
            key_code == LEFT
                || key_code == RIGHT
                || key_code == HOME
                || key_code == END
                || key_code == KEYPAD_HOME
                || key_code == KEYPAD_END
                || key_code == KEYPAD_LEFT
                || key_code == KEYPAD_RIGHT
                || (KEYPAD_ZERO..=KEYPAD_NINE).contains(&key_code)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_set_value_one_less_than_min() {
        let mut ctrl = ImageSizeCtrl {
            ctrl: None,
            value: 1000,
            ignore: false,
        };
        ctrl.set_value(MIN_SIZE - 1);
    }

    #[test]
    fn test_set_value_at_min() {
        let mut ctrl = ImageSizeCtrl {
            ctrl: None,
            value: 1000,
            ignore: false,
        };
        ctrl.set_value(MIN_SIZE);
        assert_eq!(ctrl.get_value(), MIN_SIZE);
    }

    #[test]
    #[should_panic]
    fn test_set_value_one_greater_than_max() {
        let mut ctrl = ImageSizeCtrl {
            ctrl: None,
            value: 1000,
            ignore: false,
        };
        ctrl.set_value(MAX_SIZE + 1);
    }

    #[test]
    fn test_set_value_at_max() {
        let mut ctrl = ImageSizeCtrl {
            ctrl: None,
            value: 1000,
            ignore: false,
        };
        ctrl.set_value(MAX_SIZE);
        assert_eq!(ctrl.get_value(), MAX_SIZE);
    }

    #[test]
    fn test_get_value_with_value_greater_than_max() {
        let mut ctrl = ImageSizeCtrl {
            ctrl: None,
            value: MAX_SIZE + 1,
            ignore: false,
        };
        assert_eq!(MAX_SIZE, ctrl.get_value());
    }

    #[test]
    fn test_get_value_with_value_less_than_min() {
        let mut ctrl = ImageSizeCtrl {
            ctrl: None,
            value: MIN_SIZE - 1,
            ignore: false,
        };
        assert_eq!(MIN_SIZE, ctrl.get_value());
    }
}
