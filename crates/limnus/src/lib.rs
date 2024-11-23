/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/swamp
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
pub mod prelude;

use swamp_app::prelude::{App, AppReturnValue, Plugin};
use swamp_asset_registry::AssetRegistryPlugin;
use swamp_assets_loader::AssetLoaderRegistryPlugin;
use swamp_loader_plugin::LoaderPlugin;
use swamp_log::LogPlugin;
use swamp_wgpu_window::WgpuWindowPlugin;

#[cfg(feature = "audio")]
use limnus_audio_mixer::AudioMixerPlugin;
#[cfg(feature = "audio")]
use limnus_audio_stream::AudioStreamPlugin;
#[cfg(feature = "audio")]
use swamp_audio::AudioPlugin;
#[cfg(feature = "audio")]
use swamp_audio_sample::AudioSamplePlugin;

pub struct Main;

impl Main {
    pub fn run() -> AppReturnValue {
        App::new().add_plugins(DefaultPlugins).run()
    }
}

pub struct WindowRunnerPlugin;

impl Plugin for WindowRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(swamp_window_runner::runner);
    }
}

pub struct DefaultPlugins;

impl Plugin for DefaultPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LogPlugin,
            LoaderPlugin,
            AssetLoaderRegistryPlugin,
            AssetRegistryPlugin,
        ));
        app.add_plugins((WindowRunnerPlugin, WgpuWindowPlugin));

        #[cfg(feature = "audio")]
        app.add_plugins((
            AudioPlugin,
            AudioSamplePlugin,
            AudioMixerPlugin,
            AudioStreamPlugin,
        ));
    }
}
