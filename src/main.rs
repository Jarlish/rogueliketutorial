use rltk::{Rltk, GameState, RGB, Point, RandomNumberGenerator};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
pub mod camera;
mod rect;
pub use rect::Rect;
mod visibility_system;
use visibility_system::VisibilitySystem;

//Create game state
#[derive(PartialEq, Copy, Clone)]
pub enum RunState {Paused, Running, NextLevel}

pub struct State {
    pub ecs: World,
    pub runstate : RunState
}

impl State {
    //Function to run systems
    fn run_systems(&mut self) {
        //Run the visibility system
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);

        self.ecs.maintain();
    }

    //Function to get all the entities that need to be removed from the ECS when the player enters a new map
    fn entities_to_remove_on_level_change(&mut self) -> Vec<Entity> {
        let entities = self.ecs.entities(); //Get all entities from the ECS
        let player = self.ecs.read_storage::<Player>(); //Get the player entity from the ECS

        let mut to_delete : Vec<Entity> = Vec::new();
        for entity in entities.join() {
            let mut should_delete = true;

            //Don't delete the player
            let p = player.get(entity);
            if let Some(_p) = p { //If the current entity is the player, don't delete it
                should_delete = false;
            }

            if should_delete {
                to_delete.push(entity);
            }
        }

        to_delete
    }

    //Function to go to the next map
    fn goto_next_level(&mut self) {
        //Delete all entities other than the player
        let to_delete = self.entities_to_remove_on_level_change(); //Get all entities other than the player
        for target in to_delete {
            self.ecs.delete_entity(target).expect("Unable to delete entity"); //Delete the entities from the ECS
        }

        //Generate a new map
        let worldmap;
        {
            let mut worldmap_resource = self.ecs.write_resource::<Map>();
            let current_depth = worldmap_resource.depth;

            //Create a new random number generator
            let mut rng = RandomNumberGenerator::new();
            if rng.range(0, 3) == 1 {
                *worldmap_resource = Map::new_map_cellular_automata(current_depth + 1, 100, 100);
            }else {
                *worldmap_resource = Map::new_map_rooms_and_corridors(current_depth + 1, 100, 100);
            }

            worldmap = worldmap_resource.clone();
        }

        //Place the player in the first room of the new map
        let (player_x, player_y) = (worldmap.starting_position_x, worldmap.starting_position_y); //Set the player's start position in the new map
        let mut player_position = self.ecs.write_resource::<Point>(); //Get the Point tracking the player's position
        *player_position = Point::new(player_x, player_y); //Update the Point tracking the player's position
        let mut position_components = self.ecs.write_storage::<Position>();
        let player_entity = self.ecs.fetch::<Entity>();
        let player_pos_comp = position_components.get_mut(*player_entity); //Get the Position component associated with the player entity
        if let Some(player_pos_comp) = player_pos_comp { //Update the player's entity's position
            player_pos_comp.x = player_x;
            player_pos_comp.y = player_y;
        }

        //Mark the player's viewshed as dirty so visibility is recalculated
        let mut viewshed_components = self.ecs.write_storage::<Viewshed>(); //Get write access to the ECS's Viewshed component storage
        let viewshed = viewshed_components.get_mut(*player_entity); //Get the Viewshed component associated with the player entity
        if let Some(vs) = viewshed {
            vs.dirty = true;
        }        
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        //Clear the screen
        ctx.cls();

        //Run the game if it isn't paused, otherwise wait for input
        if self.runstate == RunState::Running {
            //Run systems
            self.run_systems();
            self.runstate = RunState::Paused;
        }else if self.runstate == RunState::Paused {
            //Read user input from RLTK
            self.runstate = user_input(self, ctx);
        }else if self.runstate == RunState::NextLevel {
            self.goto_next_level();
            self.run_systems();
            self.runstate = RunState::Paused;
        }        

        //Draw the map
        camera::render_camera(&self.ecs, ctx);

        //Draw a simple HUD with the current map depth
        let map = self.ecs.fetch::<Map>();
        let depth = format!("Depth: {}", map.depth);
        ctx.print_color(1, 48, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &depth);
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
        ecs: World::new(),
        runstate : RunState::Running
    };
    //Tell the ECS (World) about the components so it can store them
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    //Add a new randomly generated map to the ECS as a resource
    let map = Map::new_map_cellular_automata(1, 100, 100);
    let (player_x, player_y) = (map.starting_position_x, map.starting_position_y); //Set the player's start position in the new map
    gs.ecs.insert(map);

    //Create a player entity with Position and Renderable components and a Player tag component
    let player_entity = gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::WHITE),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles : Vec::new(), range : 8, dirty: true })
        .build();
    gs.ecs.insert(player_entity);

    //Keep track of the player's position with a Point
    gs.ecs.insert(Point::new(player_x, player_y));

    //Start the RLTK main loop
    rltk::main_loop(context, gs)
}
