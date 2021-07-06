  use rltk::{RandomNumberGenerator, BaseMap, Point };
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
    pub depth : i32,
    pub starting_position_x : i32,
    pub starting_position_y : i32
}

impl BaseMap for Map {
    //Implements the BaseMap trait function to check if a tile at the given index can be seen through
    fn is_opaque(&self, idx:usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    //Implements the BaseMap trait function to determine the distance between two points in pathfinding
    fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    //Implements the BaseMap trait function to return a vector of tile indexes which can be entered from the tile at the given index 
    fn get_available_exits(&self, idx:usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new(); //Create a vector to store the valid exits
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        //Add valid exits in the cardinal directions
        if self.is_exit_valid(x - 1, y) { exits.push((idx - 1, 1.0)) };
        if self.is_exit_valid(x + 1, y) { exits.push((idx + 1, 1.0)) };
        if self.is_exit_valid(x, y - 1) { exits.push((idx-w, 1.0)) };
        if self.is_exit_valid(x, y + 1) { exits.push((idx+w, 1.0)) };

        //Add valid exits in the diagonal directions
        if self.is_exit_valid(x - 1, y - 1) { exits.push(((idx-w) - 1, 1.45)); }
        if self.is_exit_valid(x + 1, y - 1) { exits.push(((idx-w) + 1, 1.45)); }
        if self.is_exit_valid(x - 1, y + 1) { exits.push(((idx+w) - 1, 1.45)); }
        if self.is_exit_valid(x + 1, y + 1) { exits.push(((idx+w) + 1, 1.45)); }

        //Return the list of valid exits
        exits
    }
}

impl Map { 
    //Function to get a unique index for each map tile from its position
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    //Function to calculate if the given position can be entered
    fn is_exit_valid(&self, x:i32, y:i32) -> bool {
        if x < 1 || x > self.width-1 || y < 1 || y > self.height-1 { return false; }
        let idx = self.xy_idx(x, y);
        !(self.tiles[idx] == TileType::Wall)
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
            depth: new_depth,
            starting_position_x: 0,
            starting_position_y: 0  
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

        //Set the starting position to the center of the first room
        map.starting_position_x = map.rooms[0].center().0;
        map.starting_position_y = map.rooms[0].center().1;

        //Place the down stairs at the center of the last room
        let stairs_position = map.rooms[map.rooms.len() - 1].center();
        let stairs_idx = map.xy_idx(stairs_position.0, stairs_position.1);
        map.tiles[stairs_idx] = TileType::DownStairs;
    
        //Return the newly generated map
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

    //Function to create a new map with cellular automata
    pub fn new_map_cellular_automata(new_depth : i32, width: i32, height: i32) -> Map {
        //Calculate the total number of tiles in the map based on its width and height
        let map_tile_count = (width * height) as usize;

        //Create the map struct
        let mut map = Map {
            tiles : vec![TileType::Wall; map_tile_count],
            rooms : Vec::new(),
            width,
            height,
            depth: new_depth,
            starting_position_x: 0,
            starting_position_y: 0
        };
    
        //Generate the map
        map.generate_cellular_automata();

        //Return the newly generated map
        map
    }

    //Function to apply cellular automata to the map
    fn generate_cellular_automata(&mut self) {
        //Set the map's generation properties
        const ITERATIONS : i32 = 10;
        const BIRTH_LIMIT : i32 = 4;
        const DEATH_LIMIT : i32 = 3;
        const INITIAL_CHANCE : i32 = 35;
        
        //Create a new random number generator
        let mut rng = RandomNumberGenerator::new();

        //Completely randomize the map
        for y in 1..self.height-1 {
            for x in 1..self.width-1 {
                let roll = rng.roll_dice(1, 100);
                let idx = self.xy_idx(x, y);
                if roll > INITIAL_CHANCE {
                    self.tiles[idx] = TileType::Floor 
                }else { 
                    self.tiles[idx] = TileType::Wall
                }
            }
        }

        //Iteratively apply cellular automata rules
        for _i in 0..ITERATIONS { //Iterate the algorithm
            let mut newtiles = self.tiles.clone();

            //Iterate through the tile map (excluding the borders)
            for y in 1..self.height-1 {
                for x in 1..self.width-1 {
                    let idx = self.xy_idx(x, y); //Get the index of this tile

                    //Calculate the number of wall neighbors to this tile
                    let mut neighbors = 0;
                    if self.tiles[idx - 1] == TileType::Wall { neighbors += 1; }
                    if self.tiles[idx + 1] == TileType::Wall { neighbors += 1; }
                    if self.tiles[idx - self.width as usize] == TileType::Wall { neighbors += 1; }
                    if self.tiles[idx + self.width as usize] == TileType::Wall { neighbors += 1; }
                    if self.tiles[idx - (self.width as usize - 1)] == TileType::Wall { neighbors += 1; }
                    if self.tiles[idx - (self.width as usize + 1)] == TileType::Wall { neighbors += 1; }
                    if self.tiles[idx + (self.width as usize - 1)] == TileType::Wall { neighbors += 1; }
                    if self.tiles[idx + (self.width as usize + 1)] == TileType::Wall { neighbors += 1; }

                    if self.tiles[idx] == TileType::Wall {
                        if neighbors < DEATH_LIMIT {
                            newtiles[idx] = TileType::Floor; //Wall didn't have enough neighbors, remove it
                        }else {
                            newtiles[idx] = TileType::Wall;
                        }
                    }else {
                        if neighbors > BIRTH_LIMIT {
                            newtiles[idx] = TileType::Wall; //Floor had enough neighbors, make it a wall
                        }else {
                            newtiles[idx] = TileType::Floor;
                        }
                    }
                }
            }

            //Update the map's tiles
            self.tiles = newtiles.clone();
        }

        //Find a starting point; start at the middle and walk left until we find an open tile
        self.starting_position_x = self.width / 2;
        self.starting_position_y = self.height / 2;
        let mut start_idx = self.xy_idx(self.starting_position_x, self.starting_position_y);
        while self.tiles[start_idx] != TileType::Floor {
            self.starting_position_x -= 1;
            start_idx = self.xy_idx(self.starting_position_x, self.starting_position_y);
        }

        //Use a dijkstra map to find all the tiles we cannot reach from the starting point and fill them + find a viable location for the exit stairs
        let map_starts : Vec<usize> = vec![start_idx]; //Set the starting position for the dijkstra map
        let dijkstra_map = rltk::DijkstraMap::new(self.width, self.height, &map_starts , self, 200.0); //Create the dijkstra map
        let mut exit_tile = (0, 0.0f32); //Create a tuple representing the exit position that will be searched for (tile index of the exit, distance from the start tile to the exit tile)
        for (i, tile) in self.tiles.iter_mut().enumerate() { //Iterate through all the tiles in the map
            if *tile == TileType::Floor {
                let distance_to_start = dijkstra_map.map[i]; //Get the distance from this tile to the start position from the dijkstra map
                if distance_to_start == std::f32::MAX { //This tile is inaccessible (Flagged by a max value f32) so turn it into a wall
                    *tile = TileType::Wall;
                } else {
                    //If this tile is further from the start position than our current exit candidate, set the exit to this tile's position
                    if distance_to_start > exit_tile.1 {
                        exit_tile.0 = i;
                        exit_tile.1 = distance_to_start;
                    }
                }
            }
        }

        //Place the down stairs at the viable exit that was found
        self.tiles[exit_tile.0] = TileType::DownStairs;
    }
}
