use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::state::{
    Map, Player, TileStatus, ARENA_HEIGHT, ARENA_WIDTH, PLAYER_HEIGHT, PLAYER_WIDTH,
};

#[derive(SystemDesc)]
pub struct MovementSystem;

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
                    let grid_y = (player_y / (ARENA_HEIGHT / 10.));
                    if grid_y - grid_y.floor() > 0.5 {
                        return;
                    }
                    let target_tile =
                        map.get_tile(player_x + scaled_amount + PLAYER_WIDTH * 0.5, player_y);
                    if target_tile.status == TileStatus::FREE {
                        transform.set_translation_x(
                            (player_x + scaled_amount)
                                .min(ARENA_WIDTH - PLAYER_WIDTH * 0.5)
                                .max(PLAYER_WIDTH * 0.5),
                        );
                    }
                }
            }
            if let Some(mv_amount) = movement_y {
                if mv_amount != 0. {
                    let scaled_amount = 1.2 * mv_amount as f32;
                    let player_x = transform.translation().x;
                    let player_y = transform.translation().y;
                    let grid_x = (player_x / (ARENA_WIDTH / 10.));
                    if grid_x - grid_x.floor() > 0.5 {
                        return;
                    }
                    let target_tile =
                        map.get_tile(player_x, player_y + scaled_amount + PLAYER_HEIGHT * 0.5);
                    if target_tile.status == TileStatus::FREE {
                        transform.set_translation_y(
                            (player_y + scaled_amount)
                                .min(ARENA_HEIGHT - PLAYER_HEIGHT * 0.5)
                                .max(PLAYER_HEIGHT * 0.5),
                        );
                    }
                }
            }
        }
    }
}
