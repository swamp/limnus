/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use swamp_app::prelude::{App, Plugin};
use swamp_assets::prelude::{Asset, Id};
use swamp_local_resource::prelude::LocalResource;

pub type StereoSampleRef = Id<StereoSample>;

#[derive(Asset)]
pub struct StereoSample {
    #[allow(unused)]
    pub stereo_frames: Arc<oddio::Frames<[oddio::Sample; 2]>>, // Not sure why it needs to be wrapped in Arc
}

impl Debug for StereoSample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StereoSample ({})", self.stereo_frames.len())
    }
}

impl StereoSample {
    pub fn frames(&self) -> &Arc<oddio::Frames<[oddio::Sample; 2]>> {
        &self.stereo_frames
    }
}

#[derive(LocalResource)]
pub struct AudioMixer {
    //pub scene: Arc<Mutex<oddio::SpatialScene>>,
    //#[allow(dead_code)]
    //scene_control: oddio::SpatialSceneControl,
    #[allow(dead_code)]
    pub mixer: Arc<Mutex<oddio::Mixer<[f32; 2]>>>,
    #[allow(dead_code)]
    mixer_control: oddio::MixerControl<[f32; 2]>,
}

impl Debug for AudioMixer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mixer")
    }
}

impl Default for AudioMixer {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioMixer {
    pub fn new() -> Self {
        //let (scene_control, scene) = oddio::SpatialScene::new();
        let (mixer_control, mixer) = oddio::Mixer::<[f32; 2]>::new();

        Self {
            // scene: Arc::new(Mutex::new(scene)),
            // scene_control,
            mixer_control,
            mixer: Arc::new(Mutex::new(mixer)),
        }
    }

    #[allow(unused)]
    pub fn play(&mut self, stereo_sample: &StereoSample) {
        let signal = oddio::FramesSignal::from(stereo_sample.frames().clone());
        self.mixer_control.play(signal);
    }
}

pub struct AudioMixerPlugin;

impl Plugin for AudioMixerPlugin {
    fn build(&self, app: &mut App) {
        let mixer = AudioMixer::new();
        app.insert_local_resource(mixer);
    }
}
