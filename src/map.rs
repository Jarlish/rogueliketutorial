  use rltk::{ RandomNumberGenerator };
use super::{Rect};
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor, DownStairs
}

#[derive(Default, Clone)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub rooms : Vec<Rect>,
    pub width : i32,
    pub height : i32,
    pub depth : i32
}

impl Map { 
    //Function to get a unique index for each map tile from its position
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    //Function to create a new map with square rooms and corridors
    pub fn new_map_rooms_and_corridors(new_depth : i32, width: i32, height: i32) -> Map {
        //Calculate the total number of tiles in the map based on its width and height
        let map_tile_count = (width * height) as usize;

        //Create the map struct
        let mut map = Map {
            tiles : vec![TileType::Wall; map_tile_count],
            rooms : Vec::new(),
            width,
            height,
            depth: new_depth    
        };
    
        //Set the map's generation properties
        const MAX_ROOMS : i32 = 100;
        const MIN_SIZE : i32 = 10;
        const MAX_SIZE : i32 = 18;

        //Create a new random number generator
        let mut rng = RandomNumberGenerator::new();

        //Apply random rooms to the map
        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, width - w - 1) - 1;
            let y = rng.roll_dice(1, height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() { //Check each existing room
                if new_room.intersect(other_room) { //If the new room intersects with an existing room, cancel it
                    ok = false;
                }
            }
            if ok { //If the room didn't run in to any problems, apply it to the map
                map.apply_room_to_map(&new_room); //Apply the room    

                //Apply tunnels connecting the new room to the previous one
                if !map.rooms.is_empty() { //Make sure there is an existing previous room to join the new room to
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 { //50% chance of a horizontal tunnel then a veritcal one or the opposite
                        //Appply the tunnels
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        //Appply the tunnels
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }            

                map.rooms.push(new_room); //Add the succesfully created room to the vector list of rooms      
            }
        }    

        //Place the down stairs at the center of the last room
        let stairs_position = map.rooms[map.rooms.len() - 1].center();
        let stairs_idx = map.xy_idx(stairs_position.0, stairs_position.1);
        map.tiles[stairs_idx] = TileType::DownStairs;
    
        map
    }    

    //Function to apply a room to the map
    fn apply_room_to_map(&mut self, room : &Rect) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }    
    
    //Function to apply a horizontal tunnel to the map
    fn apply_horizontal_tunnel(&mut self, x1:i32, x2:i32, y:i32) {
        for x in min(x1, x2) ..= max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (self.width * self.height) as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    
    //Function to apply a vertical tunnel to the map
    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        for y in min(y1, y2) ..= max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (self.width * self.height) as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
}
