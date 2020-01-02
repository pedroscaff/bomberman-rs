use amethyst::core::math::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::SpriteRender;

use crate::state::{ARENA_HEIGHT, ARENA_WIDTH};

pub const PLAYER_WIDTH: f32 = 12.0;
pub const PLAYER_HEIGHT: f32 = 12.0;

pub const PLAYER_WIDTH_HALF: f32 = PLAYER_WIDTH / 2.0;
pub const PLAYER_HEIGHT_HALF: f32 = PLAYER_HEIGHT / 2.0;

pub struct Player {
    pub is_human: bool,
    pub number: u8,
    pub num_bombs: u8,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_players(world: &mut World, sprites: &[SpriteRender]) {
    for i in 0..4 {
        let x = if i % 2 == 0 {
            PLAYER_WIDTH_HALF
        } else {
            ARENA_WIDTH - PLAYER_WIDTH_HALF
        };
        let y = if i < 2 {
            PLAYER_HEIGHT_HALF
        } else {
            ARENA_HEIGHT - PLAYER_HEIGHT_HALF
        };
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.4);
        transform.set_scale(Vector3::new(0.75, 0.75, 1.0));

        let is_human = if i == 0 { true } else { false };

        world
            .create_entity()
            .with(sprites[2].clone())
            .with(Player {
                is_human,
                number: i,
                num_bombs: 1,
            })
            .with(transform)
            .build();
    }
}
