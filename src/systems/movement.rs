use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::state::{
    Map, TileStatus, ARENA_HEIGHT, ARENA_WIDTH,
};

use crate::entities::player::{
    Player, PLAYER_HEIGHT_HALF, PLAYER_WIDTH_HALF,
};

#[derive(SystemDesc)]
pub struct MovementSystem;

fn clamp_to_arena_vertical_boundaries(value: f32) -> f32 {
    value.min(ARENA_HEIGHT - PLAYER_HEIGHT_HALF).max(PLAYER_HEIGHT_HALF)
}

fn clamp_to_arena_horizontal_boundaries(value: f32) -> f32 {
    value.min(ARENA_WIDTH - PLAYER_WIDTH_HALF).max(PLAYER_WIDTH_HALF)
}

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, Map>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, players, map, input): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            if !player.is_human {
                return;
            }
            let movement_x = input.axis_value("leftright");
            let movement_y = input.axis_value("updown");
            if let Some(mv_amount) = movement_x {
                if mv_amount != 0. {
                    let scaled_amount = 1.2 * mv_amount as f32;
                    let player_x = transform.translation().x;
                    let player_y = transform.translation().y;
                    let target_tile = {
                        if scaled_amount > 0.0 {
                            let x = clamp_to_arena_horizontal_boundaries(player_x + scaled_amount + PLAYER_WIDTH_HALF);
                            let target_tile_top_right = map.get_tile(
                                x, clamp_to_arena_vertical_boundaries(player_y + PLAYER_HEIGHT_HALF)
                            );
                            let target_tile_top_left = map.get_tile(
                                x, clamp_to_arena_vertical_boundaries(player_y - PLAYER_HEIGHT_HALF)
                            );
                            if (target_tile_top_left.status != TileStatus::Free
                                || target_tile_top_right.status != TileStatus::Free)
                                && target_tile_top_right != target_tile_top_left
                            {
                                return;
                            }
                            target_tile_top_right
                        } else {
                            let x = clamp_to_arena_horizontal_boundaries(player_x + scaled_amount - PLAYER_WIDTH_HALF);
                            let target_tile_bottom_right = map.get_tile(
                                x, clamp_to_arena_vertical_boundaries(player_y + PLAYER_HEIGHT_HALF)
                            );
                            let target_tile_bottom_left = map.get_tile(
                                x, clamp_to_arena_vertical_boundaries(player_y - PLAYER_HEIGHT_HALF)
                            );
                            if (target_tile_bottom_left.status != TileStatus::Free
                                || target_tile_bottom_right.status != TileStatus::Free)
                                && target_tile_bottom_right != target_tile_bottom_left
                            {
                                return;
                            }
                            target_tile_bottom_right
                        }
                    };

                    if target_tile.status == TileStatus::Free {
                        transform.set_translation_x(
                            clamp_to_arena_horizontal_boundaries(player_x + scaled_amount)
                        );
                    }
                }
            }
            if let Some(mv_amount) = movement_y {
                if mv_amount != 0. {
                    let scaled_amount = 1.2 * mv_amount as f32;
                    let player_x = transform.translation().x;
                    let player_y = transform.translation().y;
                    let target_tile = {
                        if scaled_amount > 0.0 {
                            let y = clamp_to_arena_vertical_boundaries(player_y + scaled_amount + PLAYER_HEIGHT_HALF);
                            let target_tile_top_right = map.get_tile(
                                clamp_to_arena_horizontal_boundaries(player_x + PLAYER_WIDTH_HALF), y
                            );
                            let target_tile_top_left = map.get_tile(
                                clamp_to_arena_horizontal_boundaries(player_x - PLAYER_WIDTH_HALF), y
                            );
                            if (target_tile_top_left.status != TileStatus::Free
                                || target_tile_top_right.status != TileStatus::Free)
                                && target_tile_top_right != target_tile_top_left
                            {
                                return;
                            }
                            target_tile_top_right
                        } else {
                            let y = clamp_to_arena_vertical_boundaries(player_y + scaled_amount - PLAYER_HEIGHT_HALF);
                            let target_tile_bottom_right = map.get_tile(
                                clamp_to_arena_horizontal_boundaries(player_x + PLAYER_WIDTH_HALF), y
                            );
                            let target_tile_bottom_left = map.get_tile(
                                clamp_to_arena_horizontal_boundaries(player_x - PLAYER_WIDTH_HALF), y
                            );
                            if (target_tile_bottom_left.status != TileStatus::Free
                                || target_tile_bottom_right.status != TileStatus::Free)
                                && target_tile_bottom_right != target_tile_bottom_left
                            {
                                return;
                            }
                            target_tile_bottom_right
                        }
                    };

                    if target_tile.status == TileStatus::Free {
                        transform.set_translation_y(
                            clamp_to_arena_vertical_boundaries(player_y + scaled_amount)
                        );
                    }
                }
            }
        }
    }
}
