use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData, World};
use amethyst::input::{InputHandler, StringBindings};

use crate::state::{Map, SpriteSheetList};

use crate::entities::bomb::spawn_bomb;
use crate::entities::player::Player;

#[derive(SystemDesc)]
pub struct ActionsSystem;

impl<'s> System<'s> for ActionsSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, SpriteSheetList>,
        Read<'s, Map>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (entities, lazy_update, transforms, players, sprite_sheet_list, map, input): Self::SystemData,
    ) {
        let fire_input = input.action_is_down("fire").unwrap();
        if fire_input {
            for (player, transform) in (&players, &transforms).join() {
                if player.number != 0 {
                    return;
                }
                spawn_bomb(
                    &entities,
                    &transform,
                    &lazy_update,
                    &sprite_sheet_list,
                    &map,
                );
            }
        }
    }
}
