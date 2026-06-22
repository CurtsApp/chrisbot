use gilrs::{Axis as GilAxis, Button as GilButton, Event, EventType, Gilrs};
use rcl::{
    controller_utils::{Axis, Button, Controller},
    network_utils::*,
};

pub fn watch_controller() {
    let tx_landbot = match UDPSender::init(DASHBOARD_TX, LANDBOT_RX) {
        Some(tx_landbot) => tx_landbot,
        None => {
            println!("Failed to open landbot TX socket");
            return;
        }
    };

    let mut controller = Controller::init(tx_landbot);
    controller.subscribe_button_press(Button::BottomButton, |tx_landbot| {
        tx_landbot.send_button_pressed(Button::BottomButton);
    });

    controller.subscribe_axis_change(Axis::LeftStickX, |tx_landbot, state| {
        tx_landbot.send_axis_state(Axis::LeftStickX, state);
    });

    controller.subscribe_axis_change(Axis::RightTrigger, |tx_landbot, state| {
        tx_landbot.send_axis_state(Axis::RightTrigger, state);
    });

    controller.subscribe_axis_change(Axis::LeftTrigger, |tx_landbot, state| {
        tx_landbot.send_axis_state(Axis::LeftTrigger, state);
    });

    let mut gilrs = Gilrs::new().unwrap();
    loop {
        // Examine new events
        while let Some(Event {
            id, event, time, ..
        }) = gilrs.next_event()
        {
            println!("{:?} New event from {}: {:?}", time, id, event);

            controller = handle_controller_event(event, controller);
        }
    }
}

fn handle_controller_event(event: EventType, mut controller: Controller) -> Controller {
    match event {
        EventType::AxisChanged(gil_axil, value, _code) => {
            match translate_axis(gil_axil) {
                Some(axis) => controller.update_axis_state(&axis, value),
                // Unhandled axis types are ignored
                None => (),
            }
        }
        EventType::ButtonPressed(gil_button, _code) => {
            match translate_button(gil_button) {
                Some(button) => controller.update_button_state(&button, true),
                // Unhandled axis types are ignored
                None => (),
            }
        }
        EventType::ButtonReleased(gil_button, _code) => {
            match translate_button(gil_button) {
                Some(button) => controller.update_button_state(&button, false),
                // Unhandled axis types are ignored
                None => (),
            }
        }
        EventType::ButtonChanged(gil_button, value, _code) => {
            match translate_button_as_axis(gil_button) {
                Some(axis) => controller.update_axis_state(&axis, value),
                // Unhandled axis types are ignored
                None => (),
            }
        }
        // Unhandled event types are ignored
        _ => (),
    }

    controller
}

fn translate_axis(axis: GilAxis) -> Option<Axis> {
    match axis {
        GilAxis::LeftStickX => Some(Axis::LeftStickX),
        GilAxis::LeftStickY => Some(Axis::LeftStickY),
        GilAxis::RightStickX => Some(Axis::RightStickX),
        GilAxis::RightStickY => Some(Axis::RightStickY),
        _ => None,
    }
}

fn translate_button_as_axis(button: GilButton) -> Option<Axis> {
    match button {
        GilButton::LeftTrigger2 => Some(Axis::LeftTrigger),
        GilButton::RightTrigger2 => Some(Axis::RightTrigger),
        _ => None,
    }
}

fn translate_button(button: GilButton) -> Option<Button> {
    match button {
        GilButton::North => Some(Button::TopButton),
        GilButton::South => Some(Button::BottomButton),
        GilButton::West => Some(Button::LeftButton),
        GilButton::East => Some(Button::RightButton),
        GilButton::LeftTrigger => Some(Button::LeftBumper),
        GilButton::RightTrigger => Some(Button::RightBumper),
        GilButton::DPadUp => Some(Button::DPadUp),
        GilButton::DPadDown => Some(Button::DPadDown),
        GilButton::DPadLeft => Some(Button::DPadLeft),
        GilButton::DPadRight => Some(Button::DPadRight),
        GilButton::Select => Some(Button::Select),
        GilButton::Start => Some(Button::Start),
        _ => None,
    }
}
