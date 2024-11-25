/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use chunk_reader::{ChunkReader, ResourceId};
use message_channel::Sender;
use std::fmt::Debug;
use swamp_assets::prelude::{AssetName, RawWeakId};

pub struct Blob {
    pub path: AssetName,
    pub content: Vec<u8>,
    pub id: RawWeakId,
}

impl Debug for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ name:{} size:{} }}", self.path, self.content.len())
    }
}

pub async fn load(
    reader: Box<dyn ChunkReader>,
    sender: &Sender<Blob>,
    asset_name: AssetName,
    id: RawWeakId,
) {
    if let Ok(octets) = reader
        .fetch_octets(ResourceId::from(asset_name.value()))
        .await
    {
        let blob = Blob {
            path: asset_name,
            content: octets,
            id,
        };
        sender.send(blob).expect("could not send blob to channel");
    }
}
