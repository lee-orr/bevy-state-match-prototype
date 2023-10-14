use bevy::prelude::{App, StateTransition};

use crate::state::{apply_state_transition, run_enter_schedule, States};

pub trait StateMatchingApp {
    fn add_matchable_state<S: States>(&mut self) -> &mut Self;
}

impl StateMatchingApp for App {
    fn add_matchable_state<S: States>(&mut self) -> &mut Self {
        self.init_resource::<State<S>>()
            .init_resource::<NextState<S>>()
            .add_systems(
                StateTransition,
                (
                    run_enter_schedule::<S>.run_if(run_once_condition()),
                    apply_state_transition::<S>,
                )
                    .chain(),
            );

        self
    }
}
