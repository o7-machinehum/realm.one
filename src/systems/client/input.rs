use crate::{
    components::Orientation,
    key_bindings::MovementBindingTypes,
    resources::{Command, CommandQueue},
    constants
};

use amethyst::{
    core::{bundle::SystemBundle, timing::Time, SystemDesc},
    derive::SystemDesc,
    ecs::{DispatcherBuilder, Read, System, SystemData, World, Write},
    input::{InputEvent, InputHandler, VirtualKeyCode},
    shrev::{EventChannel, ReaderId},
};

pub struct InputSystemBundle;
impl<'a, 'b> SystemBundle<'a, 'b> for InputSystemBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), amethyst::Error> {
        builder.add(InputSystemDesc::default().build(world), "inp_system", &[]);
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct InputSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, InputSystem> for InputSystemDesc {
    fn build(self, world: &mut World) -> InputSystem {
        <InputSystem as System<'_>>::SystemData::setup(world);
        let reader = world
            .fetch_mut::<EventChannel<InputEvent<MovementBindingTypes>>>()
            .register_reader();
        InputSystem::new(reader)
    }
}

#[derive(Debug, SystemDesc)]
pub struct InputSystem {
    reader: ReaderId<InputEvent<MovementBindingTypes>>,
    latched_actions: Vec<(Command, f64)>, //expiry time in f64
    latched_keys: Vec<(VirtualKeyCode, f64)>, //expiry time in f64
    typing_mode: bool,
    typed_input: Vec<char>,
}

impl InputSystem {
    pub fn new(reader: ReaderId<InputEvent<MovementBindingTypes>>) -> Self {
        Self {
            latched_actions: Vec::new(),
            latched_keys: Vec::new(),
            typing_mode: false,
            typed_input: Vec::new(),
            reader,
        }
    }
    ///Iterates on latched_actions & removes any expired action
    fn remove_expired_latches(&mut self, current_time: f64) {
        self.latched_actions.retain(|(_action, expiry_time)| {
            return current_time < *expiry_time;
        });
        self.latched_keys.retain(|(_action, expiry_time)| {
            return current_time < *expiry_time;
        });
    }
}

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        Read<'s, InputHandler<MovementBindingTypes>>,
        Read<'s, Time>,
        Read<'s, EventChannel<InputEvent<MovementBindingTypes>>>,
        Write<'s, CommandQueue>,
    );

    fn run(&mut self, (input, time, input_event_channel, mut command_queue): Self::SystemData) {
        let t = time.absolute_real_time().as_secs_f64();
        let action_expiry = t + constants::ACTION_DELAY_MS as f64 / 1000.;
        let typing_expiry = t + constants::TYPING_DELAY_MS as f64 / 1000.;
        self.remove_expired_latches(t);

        let commands = vec![
            //Command, send_command_to_queue?)
            (Command::TypingMode, false),
            (Command::Move(Orientation::North), true),
            (Command::Move(Orientation::East), true),
            (Command::Move(Orientation::South), true),
            (Command::Move(Orientation::West), true),
            (Command::Melee, true),
        ];

        let input_events = input_event_channel.read(&mut self.reader);
        if self.typing_mode {
            let return_key = 0x0d as char;
            let escape_key = 0x1b as char;
            let backspace_key = 0x08 as char;
            let del_key = 0x7f as char;
            for e in input_events {
                match e {
                    InputEvent::KeyTyped(key) => {
                        if key == &return_key {
                            let data = self.typed_input.iter().collect::<String>();
                            if data.len() > 0 {
                                log::info!("Command: TypedData({:?})", data);
                                command_queue.add(Command::TypedData(data));
                            }
                            self.typed_input = Vec::new();
                            self.typing_mode = false;
                            log::info!("TypingMode {}", self.typing_mode);
                            try_latch(
                                Command::TypingMode,
                                &mut self.latched_actions,
                                action_expiry,
                            );
                        } else if key == &escape_key {
                            self.typing_mode = false;
                            log::info!("TypingMode {}", self.typing_mode);
                        } else if key == &backspace_key || key == &del_key {
                            self.typed_input.pop();
                            log::info!("{:?}", self.typed_input.iter().collect::<String>());
                        } else {
                            self.typed_input.push(*key);
                            log::info!("{:?}", self.typed_input.iter().collect::<String>());
                        }
                    }
                    _ => (),
                }
            }
        } else {
            for c in commands {
                let input_action = c.0;
                let send_command = c.1;
                if input.action_is_down(&input_action).unwrap() {
                    if try_latch(
                        input_action.clone(),
                        &mut self.latched_actions,
                        action_expiry,
                    ) {
                        if send_command {
                            log::info!("Command: {:?}", input_action);
                            command_queue.add(input_action.clone());
                        } else if input_action == Command::TypingMode {
                            self.typing_mode = !self.typing_mode;
                            try_latch(
                                VirtualKeyCode::Return,
                                &mut self.latched_keys,
                                typing_expiry,
                            ); // ensure latched to prevent runaway entry
                            log::info!("TypingMode {}", self.typing_mode);
                        }
                    }
                }
            }
        }
    }
}
/// 1. Checks if the key is down
/// 2. Checks that an action of the same type does not exist in latched_actions.
/// 3. If new action is unique, pushes into latched_actions
fn try_latch<I: PartialEq>(val: I, vec: &mut Vec<(I, f64)>, expiry_time: f64) -> bool {
    let mut can_latch = !vec.iter().any(|a| a.0 == val);
    //When vec is empty, the closure above will return false
    if vec.len() == 0 {
        can_latch = true;
    }
    if can_latch {
        vec.push((val, expiry_time));
        //self.latched_keys.push((key, expiry_time));
    }
    return can_latch;
}
