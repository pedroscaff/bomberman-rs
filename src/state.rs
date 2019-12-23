use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

use log::info;

use crate::config::read_map;

pub const ARENA_HEIGHT: f32 = 160.0;
pub const ARENA_WIDTH: f32 = 160.0;

pub const PLAYER_WIDTH: f32 = 16.0;
pub const PLAYER_HEIGHT: f32 = 16.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileStatus {
    FREE,
    DESTROYED,
    WALL,
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub status: TileStatus,
    pub coordinates: [usize; 2],
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

pub struct Player {
    pub is_human: bool,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub struct Map {
    tiles: [[Tile; 10]; 10],
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
}

impl Map {
    pub fn get_tile(&self, x: f32, y: f32) -> Tile {
        let grid_x = (x / (ARENA_WIDTH / 10.)).floor() as usize;
        let grid_y = (y / (ARENA_HEIGHT / 10.)).floor() as usize;
        self.tiles[grid_x][grid_y]
    }
}

impl Default for Map {
    fn default() -> Self {
        Map {
            tiles: [[Tile {
                status: TileStatus::FREE,
                coordinates: [0, 0],
            }; 10]; 10],
        }
    }
}

pub struct MyState;

impl SimpleState for MyState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let tiles = read_map("resources/maps/default.txt").unwrap();
        world.insert(Map { tiles });

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);

        // Load our sprites and display them
        let sprites = load_sprites(world);
        init_sprites(world, &tiles, &sprites, &dimensions);
        init_players(world, &sprites);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/general.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/general.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    // Create our sprite renders. Each will have a handle to the texture
    // that it renders from. The handle is safe to clone, since it just
    // references the asset.
    (0..3)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}

fn init_sprites(
    world: &mut World,
    map: &[[Tile; 10]; 10],
    sprites: &[SpriteRender],
    dimensions: &ScreenDimensions,
) {
    for (i, row) in map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let x = (i as f32) * (ARENA_WIDTH / 10.) + 8.;
            let y = (j as f32) * (ARENA_HEIGHT / 10.) + 8.;
            let mut transform = Transform::default();
            transform.set_translation_xyz(x, y, 0.);

            let sprite = match col.status {
                TileStatus::WALL => sprites[0].clone(),
                TileStatus::FREE => sprites[1].clone(),
                _ => {
                    println!("not yet implemented status {:?}", col.status);
                    sprites[1].clone()
                }
            };

            // Create an entity for each sprite and attach the `SpriteRender` as
            // well as the transform. If you want to add behaviour to your sprites,
            // you'll want to add a custom `Component` that will identify them, and a
            // `System` that will iterate over them. See https://book.amethyst.rs/stable/concepts/system.html
            world.create_entity().with(sprite).with(transform).build();
        }
    }
}

fn init_players(world: &mut World, sprites: &[SpriteRender]) {
    for i in 0..4 {
        let x = if i % 2 == 0 { 8. } else { ARENA_WIDTH - 8. };
        let y = if i < 2 { 8. } else { ARENA_HEIGHT - 8. };
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.1);

        let is_human = if i == 0 { true } else { false };

        world
            .create_entity()
            .with(sprites[2].clone())
            .with(Player { is_human })
            .with(transform)
            .build();
    }
}
