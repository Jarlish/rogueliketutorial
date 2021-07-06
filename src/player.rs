use rltk::{VirtualKeyCode, Rltk, Point};
use specs::prelude::*;
use super::{Position, Player, TileType, Map, State, RunState};
use std::cmp::{min, max};

//Function to move the player entity
pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>(); //Get write access to the ECS's Position component storage
    let mut players = ecs.write_storage::<Player>(); //Get write access to the ECS's Player component storage
    let map = ecs.fetch::<Map>(); //Fetch the map from the ECS

    for (_player, pos) in (&mut players, &mut positions).join() { //All entities with both a Player component and a Position component
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(map.width - 1 , max(0, pos.x + delta_x));
            pos.y = min(map.height - 1, max(0, pos.y + delta_y));

            //Update the Point tracking the player's position
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

//Function to attempt interacting with an object on the map
pub fn attempt_interact(ecs: &mut World) -> RunState {
    let player_pos = ecs.fetch::<Point>(); //Get the Point tracking the player's position
    let map = ecs.fetch::<Map>(); //Fetch the current map from the ECS
    let player_idx = map.xy_idx(player_pos.x, player_pos.y);
    if map.tiles[player_idx] == TileType::DownStairs {
        //If the player interacts with a down stairs, move to the next map level
        RunState::NextLevel
    } else {
        //If there is no interactable object at the player's location, do nothing
        RunState::Paused
    }
}

//Function to read user input from RLTK
pub fn user_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    //Player movement
    match ctx.key {
        None => { return RunState::Paused } //Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::A |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::D |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up |
            VirtualKeyCode::W |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::S |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),

            VirtualKeyCode::E => {
                return attempt_interact(&mut gs.ecs);
            }            
            _ => { return RunState::Paused }
        },
    }
    RunState::Running
}
