use crate::voxel::Voxel;

use super::ChunkUpdateEvent;
use bevy::ecs::system::SystemParam;
use bevy::{prelude::*, utils::HashMap};
use building_blocks::core::Point3i;
use building_blocks::storage::{Array3x1, ChunkHashMap3x1, ChunkKey3};

pub type ChunkEntityMap = HashMap<Point3i, Entity>;

#[derive(SystemParam)]
pub struct ChunkMapReader<'a> {
    pub chunk_entities: Res<'a, ChunkEntityMap>,
    pub chunk_data: Res<'a, ChunkHashMap3x1<crate::voxel::Voxel>>,
}

#[derive(SystemParam)]
pub struct ChunkMapWriter<'a> {
    pub chunk_entities: ResMut<'a, ChunkEntityMap>,
    pub chunk_data: ResMut<'a, ChunkHashMap3x1<crate::voxel::Voxel>>,
    pub chunk_updates: EventWriter<'a, ChunkUpdateEvent>,
}

impl<'a> ChunkMapReader<'a> {
    #[inline]
    pub fn chunk_exists(&self, chunk_coords: Point3i) -> bool {
        self.chunk_entities.contains_key(&chunk_coords)
    }

    pub fn get_entity(&self, chunk_coords: Point3i) -> Option<Entity> {
        self.chunk_entities.get(&chunk_coords).map(|entity| *entity)
    }

    pub fn get_chunk_data(&self, chunk_coords: Point3i) -> Option<&Array3x1<crate::voxel::Voxel>> {
        self.chunk_data.get_chunk(ChunkKey3::new(0, chunk_coords))
    }
}

impl<'a> ChunkMapWriter<'a> {
    pub fn get_chunk_data_mut(
        &mut self,
        chunk_coords: Point3i,
    ) -> Option<&mut Array3x1<crate::voxel::Voxel>> {
        self.chunk_data
            .get_mut_chunk(ChunkKey3::new(0, chunk_coords))
    }

    pub fn mark_updated(&mut self, chunk_coords: Point3i) {
        let min = self
            .chunk_data
            .indexer
            .min_of_chunk_containing_point(chunk_coords);
        if let Some(entity) = self.chunk_entities.get(&min) {
            self.chunk_updates.send(ChunkUpdateEvent(*entity));
        }
    }

    pub fn write_voxel(&mut self, chunk_coords: Point3i, voxel: Voxel, update: bool) {
        *self.chunk_data.get_mut_point(0, chunk_coords) = voxel;
        if update {
            self.mark_updated(chunk_coords);
        }
    }
}
