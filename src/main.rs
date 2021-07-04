use rltk::{Rltk, GameState, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

//Define components

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct Player {}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall, Floor
}

//Create game state
struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        //Clear the screen
        ctx.cls();

        //Read user input from RLTK
        user_input(self, ctx);

        //Run systems
        self.run_systems();

        //Draw the map
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        //Draw entities with a Position and a Renderable component to the screen
        let positions = self.ecs.read_storage::<Position>(); //Get read access to the ECS's Position component storage
        let renderables = self.ecs.read_storage::<Renderable>(); //Get read access to the ECS's Renderable component storage

        for (pos, render) in (&positions, &renderables).join() { //All entities with both a Position and a Renderable component
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph); //Draw the entities render properties at its position
        }
    }
}

//Function to get a unique index for each map tile from its position
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

//Constructor function for the map
fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];

    //Make the boundaries of the map walls
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    //Randomly place 400 walls
    //Obtain the thread-local RNG
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y); //Calculate the new walls location in the tile vector from its position
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

//Function to draw the map
fn draw_map(map: &[TileType], ctx : &mut Rltk) {
    let mut y = 0;
    let mut x = 0;

    for tile in map.iter() {
        //Render the tile based on its type
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_u8(80, 55, 10), RGB::from_u8(0, 0, 0), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_u8(160, 110, 20), RGB::from_u8(0, 0, 0), rltk::to_cp437('#'));
            }
        }

        //Move to the next tile
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

//Function to move the player entity
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>(); //Get write access to the ECS's Position component storage
    let mut players = ecs.write_storage::<Player>(); //Get write access to the ECS's Player component storage
    let map = ecs.fetch::<Vec<TileType>>(); //Fetch the tile map from the ECS

    for (_player, pos) in (&mut players, &mut positions).join() { //All entities with both a Player component and a Position component
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79 , max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

//Function to read user input from RLTK
fn user_input(gs: &mut State, ctx: &mut Rltk) {
    //Player movement
    match ctx.key {
        None => {} //Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

//Function to run systems
impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

//Main method
fn main() -> rltk::BError {
    //Set up the RLTK 80x50 windows
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .with_tile_dimensions(16, 16)
        .build()?;
    context.with_post_scanlines(true);
    //context.screen_burn_color(RGB::from_u8(170, 105, 50));
    context.screen_burn_color(RGB::from_u8(0, 0, 0));

    //Set the game state with a new ECS (World)
    let mut gs = State { 
        ecs: World::new()
    };
    //Tell the ECS (World) about the components so it can store them
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    //Add a new randomly generated map to the ECS as a resource
    gs.ecs.insert(new_map());

    //Create a player entity with Position and Renderable components and a Player tag component
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::WHITE),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

    //Start the RLTK main loop
    rltk::main_loop(context, gs)
}
