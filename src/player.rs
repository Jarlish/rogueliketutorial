use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use super::{Position, Player, TileType, xy_idx, State};
use std::cmp::{min, max};

//Function to move the player entity
pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
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
pub fn user_input(gs: &mut State, ctx: &mut Rltk) {
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
