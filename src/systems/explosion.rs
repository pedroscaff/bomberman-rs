use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{
    Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData, World, Write, WriteStorage,
};
use amethyst::renderer::SpriteRender;

use std::time::Duration;

use crate::state::{
    AssetType, Map, SpriteSheetList, TileStatus, ARENA_HEIGHT, ARENA_WIDTH, TILE_COUNT_HORIZONTAL,
    TILE_COUNT_VERTICAL, TILE_HEIGHT_HALF, TILE_WIDTH_HALF,
};

use crate::entities::bomb::Bomb;
use crate::entities::player::{
    Player, PLAYER_HEIGHT, PLAYER_HEIGHT_HALF, PLAYER_WIDTH, PLAYER_WIDTH_HALF,
};

#[derive(SystemDesc)]
pub struct ExplosionSystem;

//fn determinant(vec1: &[f32, f32], vec2: &[f32, f32]) -> f32 {
//    return vec1.x * vec2.y - vec1.y * vec2.x;
//}

////one edge is a-b, the other is c-d
//Vector2D edgeIntersection(Vector2D a, Vector2D b, Vector2D c, Vector2D d){
//    double det = determinant(b - a, c - d);
//    double t   = determinant(c - a, c - d) / det;
//    double u   = determinant(b - a, c - a) / det;
//    if ((t < 0) || (u < 0) || (t > 1) || (u > 1)) {
//     return NO_INTERSECTION;
//    } else {
//     return a * (1 - t) + t * b;
//    }
//}

// fn check_collisions((bomb_x, bomb_y): (f32, f32), power: u8, player_bboxes: Vec<(f32, f32, f32, f32)>, map: &mut Map) {
//     // if mouse_world_position.x > min_x
//     //     && mouse_world_position.x < max_x
//     //     && mouse_world_position.y > min_y
//     //     && mouse_world_position.y < max_y
//     for i in 0..power {

//     }
// }

impl<'s> System<'s> for ExplosionSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        Read<'s, SpriteSheetList>,
        Write<'s, Map>,
        WriteStorage<'s, Bomb>,
    );

    fn run(
        &mut self,
        (entities, lazy_update, transforms, mut players, sprite_sheet_list, mut map, mut bombs): Self::SystemData,
    ) {
        let three_secs = Duration::from_secs(3);
        for (entity, bomb, bomb_transform) in (&*entities, &mut bombs, &transforms).join() {
            if bomb.created_time.elapsed() >= three_secs {
                let player_bboxes = {
                    let mut bboxes = Vec::with_capacity(4);
                    for (player, transform) in (&mut players, &transforms).join() {
                        bboxes.push((
                            transform.translation().x - (PLAYER_WIDTH_HALF),
                            transform.translation().x + (PLAYER_WIDTH_HALF),
                            transform.translation().y - (PLAYER_HEIGHT_HALF),
                            transform.translation().y + (PLAYER_HEIGHT_HALF),
                        ));
                    }
                    bboxes
                };

                let bomb_tile = map.get_tile(
                    bomb_transform.translation().x,
                    bomb_transform.translation().y,
                );
                for i in 0..4 {
                    // check all four directions
                    let coordinates = bomb_tile.coordinates;
                    let x = coordinates[0] as i32;
                    let y = coordinates[1] as i32;
                    for j in 1..(bomb.power + 1) {
                        let j = j as i32;
                        let (x, y) = match i {
                            0 => (x, y + j),
                            1 => (x + j, y),
                            2 => (x, y - j),
                            3 => (x - j, y),
                            _ => panic!("HOW?"),
                        };
                        if x < 0 || x > 12 || y < 0 || y > 10 {
                            continue;
                        }
                        let next_tile = map.get_tile_by_key(x as usize, y as usize);
                        match next_tile.status {
                            TileStatus::WALL => {
                                map.update_tile(x as usize, y as usize, TileStatus::FREE);
                                let new_tile_entity = entities.create();
                                let mut new_tile_transform = Transform::default();
                                new_tile_transform.set_translation_xyz(
                                    x as f32 * ARENA_WIDTH / TILE_COUNT_HORIZONTAL
                                        + TILE_WIDTH_HALF,
                                    y as f32 * ARENA_HEIGHT / TILE_COUNT_VERTICAL
                                        + TILE_HEIGHT_HALF,
                                    0.1,
                                );
                                let sprite_render = SpriteRender {
                                    sprite_sheet: sprite_sheet_list
                                        .get(AssetType::Bomb)
                                        .unwrap()
                                        .clone(),
                                    sprite_number: 1,
                                };
                                lazy_update.insert(new_tile_entity, sprite_render);
                                lazy_update.insert(new_tile_entity, new_tile_transform);
                            }
                            // TileStatus::FREE => check_collisions
                            _ => {}
                        };
                    }
                }
                entities.delete(entity);

                // check_collisions((bomb_transform.translation().x, bomb_transform.translation().y), bomb.power, player_bboxes, &mut map);
            }
        }
    }
}
