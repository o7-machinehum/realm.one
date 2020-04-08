use crate::{
    components::Orientation,
    constants,
    key_bindings::{ActionBinding, MovementBindingTypes},
    resources::{Input, Inputs},
};
use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::{Read, System, SystemData, Write},
    input::InputHandler,
};

#[derive(SystemDesc)]
pub struct InputSystem {
    latched_actions: Vec<(ActionBinding, f64)>, //expiry time in f64
    typing_mode: bool,
}

impl InputSystem {
    ///Iterates on latched_actions & removes any expired action
    fn remove_expired_latches(&mut self, current_time: f64) {
        self.latched_actions.retain(|(_action, expiry_time)| {
            return current_time < *expiry_time;
        });
    }
    /// 1. Checks if the key is down
    /// 2. Checks that an action of the same type does not exist in latched_actions.
    /// 3. If new action is unique, pushes into latched_actions
    fn try_latch(&mut self, action: ActionBinding, expiry_time: f64) -> bool {
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
}

impl Default for InputSystem {
    fn default() -> Self {
        Self {
            latched_actions: Vec::new(),
            typing_mode: false,
        }
    }
}

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        Read<'s, InputHandler<MovementBindingTypes>>,
        Read<'s, Time>,
        Write<'s, Input>,
    ); //, Write<'s, Input>);

    fn run(&mut self, (input, time, mut input_res): Self::SystemData) {
        let t = time.absolute_real_time().as_secs_f64();
        self.remove_expired_latches(t);
        let action_delay = constants::MOVEMENT_DELAY_MS as f64 / 1000.; // should this really be action_delay?
        let typing_delay = constants::TYPING_DELAY_MS as f64 / 1000.;
        let commands = vec![
            (ActionBinding::TypingMode, None, action_delay),
            (
                ActionBinding::Move(Orientation::North),
                Some(Inputs::Move(Orientation::North)),
                action_delay,
            ),
            (
                ActionBinding::Move(Orientation::East),
                Some(Inputs::Move(Orientation::East)),
                action_delay,
            ),
            (
                ActionBinding::Move(Orientation::South),
                Some(Inputs::Move(Orientation::South)),
                action_delay,
            ),
            (
                ActionBinding::Move(Orientation::West),
                Some(Inputs::Move(Orientation::West)),
                action_delay,
            ),
            (ActionBinding::Melee, Some(Inputs::Melee), action_delay),
        ];
        for c in commands {
            let input_action = c.0.clone();
            if input.action_is_down(&c.0).unwrap() {
                if self.try_latch(c.0, t + c.2 as f64) {
                    if let Some(command) = c.1 {
                        log::info!("cmd: {:?}", command);
                        input_res.add(command);
                    } else if input_action == ActionBinding::TypingMode {
                        self.typing_mode = !self.typing_mode;
                        log::info!("cmd: TypingMode {}", self.typing_mode);
                    }
                }
            }
        }
    }
}
