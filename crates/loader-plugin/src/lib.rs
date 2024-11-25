/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use message_channel::{Channel, Receiver, Sender};
use swamp_app::prelude::*;
use swamp_loader::Blob;
use swamp_resource::prelude::Resource;
use tracing::debug;

#[derive(Debug, Resource)]
pub struct LoaderReceiver {
    pub receiver: Receiver<Blob>,
}

#[derive(Debug, Resource)]
pub struct LoaderSender {
    pub sender: Sender<Blob>,
}

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        debug!("creating a blob channel");
        let (sender, receiver) = Channel::<Blob>::create();
        app.insert_resource(LoaderReceiver { receiver });
        app.insert_resource(LoaderSender { sender });
    }
}
