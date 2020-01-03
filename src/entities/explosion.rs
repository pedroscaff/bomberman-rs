use amethyst::core::math::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::ecs::{Entities, LazyUpdate};
use amethyst::renderer::SpriteRender;

use ncollide2d::bounding_volume::{AABB};

use crate::state::{
    AssetType, SpriteSheetList, TILE_WIDTH, TILE_HEIGHT,
};
use std::time::Instant;

pub struct Explosion {
    pub created_time: Instant,
    pub collision_polygon: AABB<f32>,
}

impl Component for Explosion {
    type Storage = DenseVecStorage<Self>;
}

pub fn create_explosion(
    entities: &Entities,
    lazy_update: &LazyUpdate,
    sprite_sheet_list: &SpriteSheetList,
    bbox: &AABB<f32>,
) {
    let explosion_entity = entities.create();
    let mut explosion_transform = Transform::default();
    let center = bbox.center();
    explosion_transform.set_translation_xyz(
        center.x,
        center.y,
        0.3,
    );
    let extents = bbox.extents();
    let scale = (extents.x / TILE_WIDTH, extents.y / TILE_HEIGHT);
    explosion_transform.set_scale(Vector3::new(scale.0, scale.1, 1.0));
    let sprite_number = if scale.0 > 1.0 {
        1
    } else {
        0
    };
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_list.get(AssetType::Explosion).unwrap().clone(),
        sprite_number,
    };
    lazy_update.insert(explosion_entity, sprite_render);
    lazy_update.insert(
        explosion_entity,
        Explosion {
            created_time: Instant::now(),
            collision_polygon: bbox.clone(),
        },
    );
    lazy_update.insert(explosion_entity, explosion_transform);
}

