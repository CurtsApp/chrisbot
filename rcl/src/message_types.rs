use serde::{Deserialize, Serialize};

use crate::controller_utils::{Axis, Button};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    ButtonState(ButtonState),
    ButtonPressed(ButtonPressedEvent),
    AxisState(AxisState),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonState {
    pub button: Button,
    pub state: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonPressedEvent {
    pub button: Button,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AxisState {
    pub axis: Axis,
    pub state: f32,
}