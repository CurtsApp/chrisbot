use serde::{Deserialize, Serialize};

use crate::network_utils::UDPSender;

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum Button {
    TopButton,
    BottomButton,
    LeftButton,
    RightButton,
    LeftBumper,
    RightBumper,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Select,
    Start,
}

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum Axis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
    LeftTrigger,
    RightTrigger,
}

#[repr(u8)]
pub enum ControlType {
    Button,
    Axis,
}

pub struct ControllerState {
    bottom_button: bool,
    top_button: bool,
    left_button: bool,
    right_button: bool,
    left_bumper: bool,
    right_bumper: bool,
    d_up: bool,
    d_down: bool,
    d_left: bool,
    d_right: bool,
    select: bool,
    start: bool,
    left_stick_x: f32,
    left_stick_y: f32,
    right_stick_x: f32,
    right_stick_y: f32,
    left_trigger: f32,
    right_trigger: f32,
}

impl ControllerState {
    fn init() -> ControllerState {
        ControllerState {
            bottom_button: false,
            top_button: false,
            left_button: false,
            right_button: false,
            left_bumper: false,
            right_bumper: false,
            d_up: false,
            d_down: false,
            d_left: false,
            d_right: false,
            select: false,
            start: false,
            left_stick_x: 0.0,
            left_stick_y: 0.0,
            right_stick_x: 0.0,
            right_stick_y: 0.0,
            left_trigger: 0.0,
            right_trigger: 0.0,
        }
    }

    fn get_button_state(&self, button: &Button) -> bool {
        match button {
            Button::TopButton => self.top_button,
            Button::BottomButton => self.bottom_button,
            Button::LeftButton => self.left_button,
            Button::RightButton => self.right_button,
            Button::LeftBumper => self.left_bumper,
            Button::RightBumper => self.right_bumper,
            Button::DPadUp => self.d_up,
            Button::DPadDown => self.d_down,
            Button::DPadLeft => self.d_left,
            Button::DPadRight => self.d_right,
            Button::Select => self.select,
            Button::Start => self.start,
        }
    }

    fn get_axis_state(&self, axis: &Axis) -> f32 {
        match axis {
            Axis::LeftStickX => self.left_stick_x,
            Axis::LeftStickY => self.left_stick_y,
            Axis::RightStickX => self.right_stick_x,
            Axis::RightStickY => self.right_stick_y,
            Axis::LeftTrigger => self.left_trigger,
            Axis::RightTrigger => self.right_trigger,
        }
    }

    fn set_button_state(&mut self, button: &Button, state: bool) {
        match button {
            Button::TopButton => self.top_button = state,
            Button::BottomButton => self.bottom_button = state,
            Button::LeftButton => self.left_button = state,
            Button::RightButton => self.right_button = state,
            Button::LeftBumper => self.left_bumper = state,
            Button::RightBumper => self.right_bumper = state,
            Button::DPadUp => self.d_up = state,
            Button::DPadDown => self.d_down = state,
            Button::DPadLeft => self.d_left = state,
            Button::DPadRight => self.d_right = state,
            Button::Select => self.select = state,
            Button::Start => self.start = state,
        }
    }

    fn set_axis_state(&mut self, axis: &Axis, state: f32) {
        match axis {
            Axis::LeftStickX => self.left_stick_x = state,
            Axis::LeftStickY => self.left_stick_y = state,
            Axis::RightStickX => self.right_stick_x = state,
            Axis::RightStickY => self.right_stick_y = state,
            Axis::LeftTrigger => self.left_trigger = state,
            Axis::RightTrigger => self.right_trigger = state,
        }
    }
}

struct ButtonPressedListener {
    button: Button,
    callback: fn(&UDPSender),
}

struct AxisChangedListener {
    axis: Axis,
    callback: fn(&UDPSender, f32),
}
pub struct Controller {
    state: ControllerState,
    sender: UDPSender,
    button_press_listeners: Vec<ButtonPressedListener>,
    axis_change_listeners: Vec<AxisChangedListener>,
}

impl Controller {
    pub fn init(sender: UDPSender) -> Controller {
        Controller {
            sender,
            axis_change_listeners: vec![],
            button_press_listeners: vec![],
            state: ControllerState::init(),
        }
    }

    pub fn subscribe_button_press(&mut self, button: Button, callback: fn(&UDPSender)) {
        self.button_press_listeners
            .push(ButtonPressedListener { button, callback })
    }

    pub fn subscribe_axis_change(&mut self, axis: Axis, callback: fn(&UDPSender, f32) -> ()) {
        self.axis_change_listeners
            .push(AxisChangedListener { axis, callback })
    }

    pub fn update_button_state(&mut self, button: &Button, new_state: bool) {
        let prior_state = self.state.get_button_state(button);

        if prior_state != new_state {
            self.state.set_button_state(button, new_state);

            if prior_state == false {
                // Button was pressed call it's event handlers
                for listener in &self.button_press_listeners {
                    if &listener.button == button {
                        (listener.callback)(&self.sender)
                    }
                }
            }
        }
    }

    pub fn update_axis_state(&mut self, axis: &Axis, new_state: f32) {
        let prior_state = self.state.get_axis_state(axis);

        if prior_state != new_state {
            self.state.set_axis_state(axis, new_state);

            for listener in &self.axis_change_listeners {
                if &listener.axis == axis {
                    (listener.callback)(&self.sender, new_state)
                }
            }
        }
    }
}
