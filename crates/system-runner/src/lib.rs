/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
mod schedule;

use crate::schedule::Schedule;
use limnus_system::{IntoSystem, SystemParam};
use limnus_system_state::State;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum UpdatePhase {
    First,
    PreUpdate,
    Update,
    PostUpdate,
}

#[derive(Default)]
pub struct Runner {
    schedules: HashMap<UpdatePhase, Schedule>,
}

impl Runner {}

impl Runner {
    pub fn new() -> Self {
        let phases_in_order = [
            UpdatePhase::First,
            UpdatePhase::PreUpdate,
            UpdatePhase::Update,
            UpdatePhase::PostUpdate,
        ];

        let mut schedules = HashMap::default();

        for phase in phases_in_order {
            schedules.insert(phase, Schedule::new());
        }

        Self { schedules }
    }

    pub fn add_system<F, Params>(&mut self, update_phase: UpdatePhase, system: F)
    where
        F: IntoSystem<Params>,
        Params: SystemParam,
    {
        self.schedules
            .get_mut(&update_phase)
            .expect("tried to add to unknown phase")
            .add_system(system);
    }

    pub fn run_systems(&mut self, state: &mut State) {
        let phases_in_order = [
            UpdatePhase::First,
            UpdatePhase::PreUpdate,
            UpdatePhase::Update,
            UpdatePhase::PostUpdate,
        ];

        for phase in &phases_in_order {
            let schedule = self.schedules.get(phase).unwrap();

            schedule.run_systems(state);
        }
    }
}
