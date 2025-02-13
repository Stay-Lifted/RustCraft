use crate::helpers::global_to_local_position;

use crate::systems::asset::AssetService;
use crate::systems::chunk::ChunkSystem;
use bevy::prelude::*;

use crate::systems::chunk::builder::{RerenderChunkFlag, RerenderChunkFlagContext};
use nalgebra::Vector3;
use rc_networking::constants::CHUNK_SIZE;
use rc_networking::protocol::Protocol;
use rc_networking::types::ReceivePacket;

pub fn network_chunk_sync(
    mut event_reader: EventReader<ReceivePacket>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_service: Res<AssetService>,
    mut chunk_service: ResMut<ChunkSystem>,
    mut rerender_chunks: EventWriter<RerenderChunkFlag>,
) {
    for event in event_reader.iter() {
        match &event.0 {
            Protocol::PartialChunkUpdate(update) => {
                let location = Vector3::new(update.x, update.y, update.z);

                chunk_service.create_chunk(
                    location,
                    update.data,
                    &mut commands,
                    &asset_service,
                    &mut rerender_chunks,
                    &mut meshes,
                );
            }
            Protocol::BlockUpdate(update) => {
                let location = Vector3::new(update.x, update.y, update.z);

                // Locate chunk
                let (chunk_loc, inner_loc) = global_to_local_position(location);

                // Try find chunk
                if let Some(mut chunk) = chunk_service.chunks.get_mut(&chunk_loc) {
                    // Found chunk! Update block
                    chunk.world[inner_loc.x][inner_loc.y][inner_loc.z] = update.id;

                    // Rerender
                    rerender_chunks.send(RerenderChunkFlag {
                        chunk: chunk_loc,
                        context: RerenderChunkFlagContext::Surrounding,
                    });
                } else {
                    // Create chunk data
                    let mut chunk = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

                    // Set block
                    chunk[inner_loc.x][inner_loc.y][inner_loc.z] = update.id;

                    // Create chunk
                    chunk_service.create_chunk(
                        chunk_loc,
                        chunk,
                        &mut commands,
                        &mut asset_service,
                        &mut rerender_chunks,
                        &mut meshes,
                    );
                }
            }
            _ => {}
        }
    }
}
