use crate::{
    components::Orientation,
    constants,
    key_bindings::MovementBindingTypes,
    resources::{Input, Inputs},
};
use amethyst::{
    core::{bundle::SystemBundle, timing::Time, SystemDesc},
    derive::SystemDesc,
    ecs::{DispatcherBuilder, Read, System, SystemData, World, Write},
    input::{InputHandler, VirtualKeyCode},
    shrev::{EventChannel, ReaderId},
    winit::{Event, WindowEvent},
};

#[derive(Debug, SystemDesc)]
pub struct InputSystem {
    latched_actions: Vec<(Inputs, f64)>,        //expiry time in f64
    latched_typing: Vec<(VirtualKeyCode, f64)>, //expiry time in f64
    typing_mode: bool,
    typed_input: Vec<char>,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            latched_actions: Vec::new(),
            latched_typing: Vec::new(),
            typing_mode: false,
            typed_input: Vec::new(),
        }
    }
    ///Iterates on latched_actions & removes any expired action
    fn remove_expired_latches(&mut self, current_time: f64) {
        self.latched_actions.retain(|(_action, expiry_time)| {
            return current_time < *expiry_time;
        });
        self.latched_typing.retain(|(_action, expiry_time)| {
            return current_time < *expiry_time;
        });
    }
    /// 1. Checks if the key is down
    /// 2. Checks that an action of the same type does not exist in latched_actions.
    /// 3. If new action is unique, pushes into latched_actions
    fn try_latch_action(&mut self, action: Inputs, expiry_time: f64) -> bool {
        let mut can_latch = !self.latched_actions.iter().any(|a| a.0 == action);
        //When vec is empty, the closure above will return false
        if self.latched_actions.len() == 0 {
            can_latch = true;
        }
        if can_latch {
            self.latched_actions.push((action, expiry_time));
        }
        return can_latch;
    }
    //todo: refactor
    fn try_latch_key(&mut self, key: VirtualKeyCode, expiry_time: f64) -> bool {
        let mut can_latch = !self.latched_typing.iter().any(|a| a.0 == key);
        //When vec is empty, the closure above will return false
        if self.latched_typing.len() == 0 {
            can_latch = true;
        }
        if can_latch {
            self.latched_typing.push((key, expiry_time));
        }
        return can_latch;
    }
}

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        Read<'s, InputHandler<MovementBindingTypes>>,
        Read<'s, Time>,
        Write<'s, Input>,
        Read<'s, EventChannel<Event>>,
    ); //, Write<'s, Input>);

    fn run(&mut self, (input, time, mut input_res, events): Self::SystemData) {
        let t = time.absolute_real_time().as_secs_f64();
        self.remove_expired_latches(t);
        let action_delay = constants::MOVEMENT_DELAY_MS as f64 / 1000.; // should this really be ACTION_DELAY?
        let typing_delay = constants::TYPING_DELAY_MS as f64 / 1000.;
        let commands = vec![
            //Command, send_command_to_resource, delay)
            (Inputs::TypingMode, false, action_delay),
            (Inputs::Move(Orientation::North), true, action_delay),
            (Inputs::Move(Orientation::East), true, action_delay),
            (Inputs::Move(Orientation::South), true, action_delay),
            (Inputs::Move(Orientation::West), true, action_delay),
            (Inputs::Melee, true, action_delay),
        ];
        if self.typing_mode {
            let keys: Vec<VirtualKeyCode> = input.keys_that_are_down().collect();
            // if (keys.len() > 0) {
            //     log::warn!("{:?}", keys);
            // }
            for key in keys {
                if self.try_latch_key(key, t + typing_delay) {
                    match key {
                        VirtualKeyCode::Return => {
                            log::info!(
                                "Command: TypedData({:?})",
                                self.typed_input.iter().collect::<String>()
                            );
                            input_res.add(Inputs::TypedData(
                                self.typed_input.iter().collect::<String>(),
                            ));
                            self.typed_input = Vec::new();
                            self.typing_mode = false;
                            self.try_latch_action(Inputs::TypingMode, t + action_delay);
                            log::info!("TypingMode {}", self.typing_mode);
                        }
                        VirtualKeyCode::Back => {
                            self.typed_input.pop();
                        }
                        _ => {
                            let ch = keycode_to_char(key);
                            if ch.is_some() {
                                self.typed_input.push(ch.unwrap());
                                log::info!("{:?}", self.typed_input.iter().collect::<String>());
                            }
                        }
                    }
                }
            }
        } else {
            for c in commands {
                let input_action = c.0;
                let send_command = c.1;
                let expiry_delay = c.2;
                if input.action_is_down(&input_action).unwrap() {
                    if self.try_latch_action(input_action.clone(), t + expiry_delay as f64) {
                        if send_command {
                            log::info!("Command: {:?}", input_action);
                            input_res.add(input_action.clone());
                        } else if input_action == Inputs::TypingMode {
                            self.typing_mode = !self.typing_mode;
                            self.try_latch_key(VirtualKeyCode::Return, t + typing_delay); // ensure latched to prevent runaway entry
                            log::info!("TypingMode {}", self.typing_mode);
                        }
                    }
                }
            }
        }
    }
}
//this seems terrible, todo to find a better implementation:
fn keycode_to_char(key: VirtualKeyCode) -> Option<char> {
    match key {
        VirtualKeyCode::A => Some('a'),
        VirtualKeyCode::B => Some('b'),
        VirtualKeyCode::C => Some('c'),
        VirtualKeyCode::D => Some('d'),
        VirtualKeyCode::E => Some('e'),
        VirtualKeyCode::F => Some('f'),
        VirtualKeyCode::G => Some('g'),
        VirtualKeyCode::H => Some('h'),
        VirtualKeyCode::I => Some('i'),
        VirtualKeyCode::J => Some('j'),
        VirtualKeyCode::K => Some('k'),
        VirtualKeyCode::L => Some('l'),
        VirtualKeyCode::M => Some('m'),
        VirtualKeyCode::N => Some('n'),
        VirtualKeyCode::O => Some('o'),
        VirtualKeyCode::P => Some('p'),
        VirtualKeyCode::Q => Some('q'),
        VirtualKeyCode::R => Some('r'),
        VirtualKeyCode::S => Some('s'),
        VirtualKeyCode::T => Some('t'),
        VirtualKeyCode::U => Some('u'),
        VirtualKeyCode::V => Some('v'),
        VirtualKeyCode::W => Some('w'),
        VirtualKeyCode::X => Some('x'),
        VirtualKeyCode::Y => Some('y'),
        VirtualKeyCode::Z => Some('z'),
        VirtualKeyCode::Key1 => Some('1'),
        VirtualKeyCode::Key2 => Some('2'),
        VirtualKeyCode::Key3 => Some('3'),
        VirtualKeyCode::Key4 => Some('4'),
        VirtualKeyCode::Key5 => Some('5'),
        VirtualKeyCode::Key6 => Some('6'),
        VirtualKeyCode::Key7 => Some('7'),
        VirtualKeyCode::Key8 => Some('8'),
        VirtualKeyCode::Key9 => Some('9'),
        VirtualKeyCode::Key0 => Some('0'),
        VirtualKeyCode::Slash => Some('/'),
        VirtualKeyCode::Space => Some(' '),
        VirtualKeyCode::Period => Some('.'),
        VirtualKeyCode::Comma => Some(','),
        VirtualKeyCode::Colon => Some(':'),
        VirtualKeyCode::Semicolon => Some(';'),
        _ => None,
    }
}
