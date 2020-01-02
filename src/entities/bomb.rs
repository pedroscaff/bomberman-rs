use amethyst::renderer::SpriteRender;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::ecs::{Entities, LazyUpdate};
use amethyst::core::math::Vector3;

use std::time::Instant;
use crate::state::{ARENA_WIDTH, ARENA_HEIGHT, TILE_COUNT_HORIZONTAL, TILE_COUNT_VERTICAL, TILE_WIDTH_HALF, TILE_HEIGHT_HALF, Map, AssetType, SpriteSheetList};

pub struct Bomb {
    pub created_time: Instant,
    pub power: u8,
}

impl Component for Bomb {
    type Storage = DenseVecStorage<Self>;
}

pub fn spawn_bomb(entities: &Entities, transform: &Transform, lazy_update: &LazyUpdate, sprite_sheet_list: &SpriteSheetList, map: &Map) {
    let bomb_entity = entities.create();
    let mut bomb_transform = Transform::default();
    let tile = map.get_tile(transform.translation().x, transform.translation().y);
    bomb_transform.set_translation_xyz(
        tile.coordinates[0] as f32 * (ARENA_WIDTH / TILE_COUNT_HORIZONTAL) + TILE_WIDTH_HALF,
        tile.coordinates[1] as f32 * (ARENA_HEIGHT / TILE_COUNT_VERTICAL) + TILE_HEIGHT_HALF,
        0.2,
    );
    bomb_transform.set_scale(Vector3::new(0.75, 0.75, 1.0));
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_list.get(AssetType::Bomb).unwrap().clone(),
        sprite_number: 3,
    };
    lazy_update.insert(bomb_entity, sprite_render);
    lazy_update.insert(
        bomb_entity,
        Bomb {
            created_time: Instant::now(),
            power: 1,
        },
    );
    lazy_update.insert(bomb_entity, bomb_transform);
}
