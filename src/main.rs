use rltk::{Rltk, GameState, RGB, Point};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
pub mod camera;

//Create game state
pub struct State {
    pub ecs: World
}

impl State {
    //Function to run systems
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
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
        camera::render_camera(&self.ecs, ctx);
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
    gs.ecs.insert(Map::new(100, 100));

    //Create a player entity with Position and Renderable components and a Player tag component
    gs.ecs
        .create_entity()
        .with(Position { x: 50, y: 50 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::WHITE),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

    //Keep track of the player's position with a Point
    gs.ecs.insert(Point::new(50, 50));

    //Start the RLTK main loop
    rltk::main_loop(context, gs)
}
