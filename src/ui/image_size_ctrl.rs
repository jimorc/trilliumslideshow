use wxdragon::prelude::*;

#[derive(Copy, Clone)]
pub struct ImageSizeCtrl {
    ctrl: Option<TextCtrl>,
    value: i32,
}

const MIN_SIZE: i32 = 100;
const MAX_SIZE: i32 = 9999;

impl<'a> ImageSizeCtrl {
    pub fn builder(parent: &'a dyn WxWidget) -> ImageSizeCtrlBuilder<'a> {
        ImageSizeCtrlBuilder {
            parent,
            value: MIN_SIZE,
        }
    }
    pub fn get_ctrl(&self) -> Option<TextCtrl> {
        self.ctrl
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
        /*    text.on_key_down(|event| {
            if let WindowEventData::Keyboard(ref key_data) = event {
                event.skip(number_key_down(key_data));
            }
        });*/
        ImageSizeCtrl {
            ctrl: Some(text),
            value: self.value,
        }
    }

    pub fn with_value(&mut self, val: i32) -> &ImageSizeCtrlBuilder<'a> {
        self.value = val;
        self
    }

    pub fn get_value_as_string(&self) -> String {
        self.value.to_string()
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
        };
        ctrl.set_value(MIN_SIZE - 1);
    }

    #[test]
    fn test_set_value_at_min() {
        let mut ctrl = ImageSizeCtrl {
            ctrl: None,
            value: 1000,
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
        };
        ctrl.set_value(MAX_SIZE + 1);
    }

    #[test]
    fn test_set_value_at_max() {
        let mut ctrl = ImageSizeCtrl {
            ctrl: None,
            value: 1000,
        };
        ctrl.set_value(MAX_SIZE);
        assert_eq!(ctrl.get_value(), MAX_SIZE);
    }

    #[test]
    fn test_get_value_with_value_greater_than_max() {
        let mut ctrl = ImageSizeCtrl {
            ctrl: None,
            value: MAX_SIZE + 1,
        };
        assert_eq!(MAX_SIZE, ctrl.get_value());
    }

    #[test]
    fn test_get_value_with_value_less_than_min() {
        let mut ctrl = ImageSizeCtrl {
            ctrl: None,
            value: MIN_SIZE - 1,
        };
        assert_eq!(MIN_SIZE, ctrl.get_value());
    }
}
