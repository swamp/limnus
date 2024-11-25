/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use crate::low_level::Audio;
use swamp_app::prelude::{App, Plugin};
use tracing::error;

pub mod low_level;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        let result = Audio::new();
        if let Ok(audio) = result {
            app.insert_local_resource(audio);
        } else {
            error!(
                err = result.unwrap_err(),
                "could not initialize audio thread "
            );
        }
    }
}
