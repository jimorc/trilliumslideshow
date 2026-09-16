use wxdragon::prelude::*;

#[derive(Copy, Clone)]
pub struct ImageSizeCtrl {
    ctrl: Option<TextCtrl>,
    value: i32,
}

const MIN_SIZE: i32 = 100;
const MAX_SIZE: i32 = 9999;

impl ImageSizeCtrl {
    pub fn get_ctrl(&self) -> Option<TextCtrl> {
        self.ctrl
    }

    pub fn get_value(&self) -> i32 {
        self.value
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
}
