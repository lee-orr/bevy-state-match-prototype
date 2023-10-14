use bevy::{
    ecs::schedule::{SystemConfigs, SystemSetConfigs},
    prelude::{
        run_once, App, IntoSystemConfigs, IntoSystemSetConfigs, State, StateTransition, States,
    },
};

use crate::{
    state::{apply_state_transition, run_enter_schedule, MatchableState},
    NextMatchableState, StateMatcher, StateMatcherSystem,
};

/// A trait adding support for state matching to a bevy `App`
pub trait StateMatchingApp {
    /// Add a state that support state matching to the application
    fn add_matchable_state<S: MatchableState>(&mut self) -> &mut Self;
}

impl StateMatchingApp for App {
    fn add_matchable_state<S: MatchableState>(&mut self) -> &mut Self {
        self.init_resource::<State<S>>()
            .init_resource::<NextMatchableState<S>>()
            .add_systems(
                StateTransition,
                (
                    run_enter_schedule::<S>.run_if(run_once()),
                    apply_state_transition::<S>,
                )
                    .chain(),
            );

        self
    }
}

/// A trait for adding `run_in` to systems
pub trait StateMatchingSystems<C, Marker> {
    /// Run a state if in a matching state
    fn run_in<S: States, M: 'static, Sm: StateMatcher<S, M>>(self, matcher: Sm) -> C;
}

impl<T: IntoSystemConfigs<Marker>, Marker> StateMatchingSystems<SystemConfigs, Marker> for T {
    fn run_in<S: States, M: 'static, Sm: StateMatcher<S, M>>(self, matcher: Sm) -> SystemConfigs {
        let system = Into::<StateMatcherSystem<S, M, Sm>>::into(matcher);
        self.run_if(system)
    }
}

impl<T: IntoSystemSetConfigs> StateMatchingSystems<SystemSetConfigs, ()> for T {
    fn run_in<S: States, M: 'static, Sm: StateMatcher<S, M>>(
        self,
        matcher: Sm,
    ) -> SystemSetConfigs {
        let system = Into::<StateMatcherSystem<S, M, Sm>>::into(matcher);
        IntoSystemSetConfigs::run_if(self, system)
    }
}
