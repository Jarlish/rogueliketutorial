use specs::prelude::*;
use super::{Map, TileType, Position, Renderable};
use rltk::{Point, Rltk, RGB};

//Whether or not to draw the area outside of the map
const SHOW_BOUNDARIES : bool = true;

//Function to get the bounds of the camera
pub fn get_screen_bounds(ecs: &World, ctx : &mut Rltk) -> (i32, i32, i32, i32) {
    let player_pos = ecs.fetch::<Point>();
    let (x_chars, y_chars) = ctx.get_char_size();

    let center_x = (x_chars / 2) as i32;
    let center_y = (y_chars / 2) as i32;

    let min_x = player_pos.x - center_x;
    let max_x = min_x + x_chars as i32;
    let min_y = player_pos.y - center_y;
    let max_y = min_y + y_chars as i32;

    (min_x, max_x, min_y, max_y)
}

//Function to draw the part of the tile map / entities within the camera's bounds
pub fn render_camera(ecs: &World, ctx : &mut Rltk) {
    //Get the map from the ECS
    let map = ecs.fetch::<Map>();

    //Calculate the bounds of the camera
    let (min_x, max_x, min_y, max_y) = get_screen_bounds(ecs, ctx);

    //Draw the map
    let map_width = map.width - 1;
    let map_height = map.height - 1;

    let mut y = 0;
    for ty in min_y .. max_y {
        let mut x = 0;
        for tx in min_x .. max_x {
            if tx >= 0 && tx <= map_width && ty >= 0 && ty <= map_height {
                let idx = map.xy_idx(tx, ty);
                let (glyph, fg, bg) = get_tile_glyph(idx, &*map);
                ctx.set(x, y, fg, bg, glyph); //Draw the tile at its position
            } else if SHOW_BOUNDARIES {
                ctx.set(x, y, RGB::named(rltk::GRAY), RGB::named(rltk::BLACK), rltk::to_cp437('·')); //Draw an indicator outside of the map bounds if it is enabled
            }
            x += 1;
        }
        y += 1;
    }

    //Draw entities
    let positions = ecs.read_storage::<Position>(); //Get read access to the ECS's Position component storage
    let renderables = ecs.read_storage::<Renderable>(); //Get read access to the ECS's Renderable component storage
    let data = (&positions, &renderables).join().collect::<Vec<_>>(); //All entities with both a Position and a Renderable component

    for (pos, render) in data.iter() {
        let entity_screen_x = pos.x - min_x;
        let entity_screen_y = pos.y - min_y;
        if entity_screen_x >= 0 && entity_screen_x <= map_width && entity_screen_y >= 0 && entity_screen_y <= map_height {
            ctx.set(entity_screen_x, entity_screen_y, render.fg, render.bg, render.glyph); //Draw the entities render properties at its position
        }
    }
}

//Function to get the render properties of a tile from its position
fn get_tile_glyph(idx: usize, map : &Map) -> (rltk::FontCharType, RGB, RGB) {
    let glyph;
    let fg;
    let bg = RGB::from_u8(0, 0, 0);

    match map.tiles[idx] {
        TileType::Floor => {
            glyph = rltk::to_cp437('.');
            fg = RGB::from_u8(80, 55, 10);
        }
        TileType::Wall => {
            glyph = rltk::to_cp437('#');
            fg = RGB::from_u8(160, 110, 20);
        }
        TileType::DownStairs => {
            glyph = rltk::to_cp437('>');
            fg = RGB::from_u8(255, 255, 255);
        }
    }

    (glyph, fg, bg)
}
