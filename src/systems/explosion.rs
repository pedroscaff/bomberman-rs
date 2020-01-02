use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{
    Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData, World, Write, WriteStorage,
};
use amethyst::renderer::SpriteRender;

use ncollide2d::bounding_volume::{BoundingVolume, AABB};
use ncollide2d::math::Point;

use std::time::Duration;

use crate::state::{
    AssetType, Map, SpriteSheetList, TileStatus, ARENA_HEIGHT, ARENA_WIDTH, TILE_COUNT_HORIZONTAL,
    TILE_COUNT_VERTICAL, TILE_HEIGHT, TILE_HEIGHT_HALF, TILE_WIDTH, TILE_WIDTH_HALF,
};

use crate::entities::bomb::Bomb;
use crate::entities::player::{Player, PLAYER_HEIGHT_HALF, PLAYER_WIDTH_HALF};

#[derive(SystemDesc)]
pub struct ExplosionSystem;

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
                entities.delete(entity).unwrap();
                let bomb_tile = map.get_tile(
                    bomb_transform.translation().x,
                    bomb_transform.translation().y,
                );
                let coordinates = bomb_tile.coordinates;
                let x = coordinates[0] as i32;
                let y = coordinates[1] as i32;
                let initial_coordinates = (
                    Point::new(
                        x as f32 * ARENA_WIDTH / TILE_COUNT_HORIZONTAL,
                        y as f32 * ARENA_HEIGHT / TILE_COUNT_VERTICAL,
                    ),
                    Point::new(
                        x as f32 * ARENA_WIDTH / TILE_COUNT_HORIZONTAL + TILE_WIDTH,
                        y as f32 * ARENA_HEIGHT / TILE_COUNT_VERTICAL + TILE_HEIGHT,
                    ),
                );
                for i in 0..4 {
                    // check all four directions
                    let collision_polygon = {
                        let mut collision_polygon =
                            AABB::new(initial_coordinates.0, initial_coordinates.1);
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
                                TileStatus::Wall => {
                                    map.update_tile(x as usize, y as usize, TileStatus::Free);
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
                                    break;
                                }
                                TileStatus::Free => {
                                    if i < 2 {
                                        collision_polygon = AABB::new(
                                            initial_coordinates.0,
                                            Point::new(
                                                x as f32 * ARENA_WIDTH / TILE_COUNT_HORIZONTAL
                                                    + TILE_WIDTH,
                                                y as f32 * ARENA_HEIGHT / TILE_COUNT_VERTICAL
                                                    + TILE_HEIGHT,
                                            ),
                                        );
                                    } else {
                                        collision_polygon = AABB::new(
                                            Point::new(
                                                x as f32 * ARENA_WIDTH / TILE_COUNT_HORIZONTAL,
                                                y as f32 * ARENA_HEIGHT / TILE_COUNT_VERTICAL,
                                            ),
                                            initial_coordinates.1,
                                        );
                                    }
                                }
                                TileStatus::PermanentWall => break,
                            };
                        }
                        collision_polygon
                    };
                    for (entity, player, transform) in
                        (&*entities, &mut players, &transforms).join()
                    {
                        if player.number == bomb.player_number {
                            player.num_bombs = 1;
                        }
                        let bbox = AABB::new(
                            Point::new(
                                transform.translation().x - PLAYER_WIDTH_HALF,
                                transform.translation().y - PLAYER_HEIGHT_HALF,
                            ),
                            Point::new(
                                transform.translation().x + PLAYER_WIDTH_HALF,
                                transform.translation().y + PLAYER_HEIGHT_HALF,
                            ),
                        );
                        let collided = collision_polygon.intersects(&bbox);
                        if collided {
                            entities.delete(entity).unwrap();
                            println!("player {} dead", player.number);
                        }
                    }
                }
            }
        }
    }
}
