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

#[derive(Component)]
struct LeftMover {}

#[derive(Component, Debug)]
struct Player {}

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

        //Print hello world to the screen
        ctx.print(1, 1, "Hello Rust World");

        //Draw entities with a Position and a Renderable component to the screen
        let positions = self.ecs.read_storage::<Position>(); //Get read access to the ECS's Position component storage
        let renderables = self.ecs.read_storage::<Renderable>(); //Get read access to the ECS's Renderable component storage

        for (pos, render) in (&positions, &renderables).join() { //All entities with both a Position and a Renderable component
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph); //Draw the entities render properties at its position
        }
    }
}

//Function to move the player entity
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>(); //Get write access to the ECS's Position component storage
    let mut players = ecs.write_storage::<Player>(); //Get write access to the ECS's Player component storage

    for (_player, pos) in (&mut players, &mut positions).join() { //All entities with both a Player component and a Position component
        pos.x = min(79 , max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
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

//LeftMover system
struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, //System needs read access to the ECS's LeftMover component storage
                    WriteStorage<'a, Position>); //System needs write access to the ECS's Position component storage

    fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() { //All entities with both a LeftMover component and a Position component
            pos.x -= 1;
            if pos.x < 0 { pos.x = 79; }
        }
    }
}

//Function to run systems
impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

//Main method
fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State{ 
        ecs: World::new()
    };
    //Tell the ECS (World) about the components so it can store them
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    //Create a player entity with Position and Renderable components and a Player tag component
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();
    
    //Add some more entities
    for i in 0..10 {
        gs.ecs
        .create_entity()
        .with(Position { x: i * 7, y: 20 })
        .with(Renderable {
            glyph: rltk::to_cp437('☺'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(LeftMover{})
        .build();
    }
    
    //Start the RLTK main loop
    rltk::main_loop(context, gs)
}
