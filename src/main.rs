use rltk::{Rltk, GameState, RGB};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;

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
