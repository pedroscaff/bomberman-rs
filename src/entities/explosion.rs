use amethyst::core::math::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::ecs::{Entities, LazyUpdate};
use amethyst::renderer::SpriteRender;

use ncollide2d::bounding_volume::AABB;

use crate::state::{AssetType, SpriteSheetList, TILE_HEIGHT, TILE_WIDTH};
use std::f32::consts::PI;
use std::time::Instant;

pub struct Explosion {
    pub created_time: Instant,
    pub collision_polygon: AABB<f32>,
}

impl Component for Explosion {
    type Storage = DenseVecStorage<Self>;
}

fn create_entity(
    entities: &Entities,
    transform: Transform,
    lazy_update: &LazyUpdate,
    sprite_render: SpriteRender,
    bbox: &AABB<f32>,
) {
    let entity = entities.create();
    lazy_update.insert(entity, sprite_render);
    lazy_update.insert(
        entity,
        Explosion {
            created_time: Instant::now(),
            collision_polygon: bbox.clone(),
        },
    );
    lazy_update.insert(entity, transform);
}

pub fn create_explosion(
    entities: &Entities,
    lazy_update: &LazyUpdate,
    sprite_sheet_list: &SpriteSheetList,
    bboxes: &Vec<AABB<f32>>,
    center_bbox: &AABB<f32>,
) {
    for bbox in bboxes {
        let mut explosion_transform = Transform::default();
        let center = bbox.center();
        explosion_transform.set_translation_xyz(center.x, center.y, 0.3);
        let extents = bbox.extents();
        let scale = (extents.x / TILE_WIDTH, extents.y / TILE_HEIGHT);
        if scale.0 > 1.0 {
            let rotation = PI / 2.;
            explosion_transform.set_rotation_2d(rotation);
            explosion_transform.set_scale(Vector3::new(scale.1, scale.0, 1.0));
        } else {
            explosion_transform.set_scale(Vector3::new(scale.0, scale.1, 1.0));
        };
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_list.get(AssetType::Explosion).unwrap().clone(),
            sprite_number: 0,
        };
        create_entity(
            entities,
            explosion_transform,
            lazy_update,
            sprite_render,
            bbox,
        );
    }
    {
        let mut center_transform = Transform::default();
        let center = center_bbox.center();
        center_transform.set_translation_xyz(center.x, center.y, 0.4);
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_list.get(AssetType::Explosion).unwrap().clone(),
            sprite_number: 1,
        };
        create_entity(
            entities,
            center_transform,
            lazy_update,
            sprite_render,
            center_bbox,
        );
    }
}
