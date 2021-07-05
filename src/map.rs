#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

#[derive(Default, Clone)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub width : i32,
    pub height : i32
}

impl Map { 
    //Constructor function for the map
    pub fn new(width: i32, height: i32) -> Map {
        //Calculate the total number of tiles in the map based on its width and height
        let map_tile_count = (width*height) as usize;

        //Create the map struct
        let mut map = Map {
            tiles : vec![TileType::Floor; map_tile_count],
            width,
            height
        };

        //Generate a random map
        map.generate();

        //Return the newly generated map
        map
    }

    //Function to get a unique index for each map tile from its position
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    //Function to generate a random map layout
    fn generate(&mut self) {
        for x in 0..self.width {
            let mut idx = self.xy_idx(x, 0);
            self.tiles[idx] = TileType::Wall;
            idx = self.xy_idx(x, self.height - 1);
            self.tiles[idx] = TileType::Wall;
        }
        for y in 0..self.height {
            let mut idx = self.xy_idx(0, y);
            self.tiles[idx] = TileType::Wall;
            idx = self.xy_idx(self.width - 1, y);
            self.tiles[idx] = TileType::Wall;
        }

        //Randomly place 400 walls
        let mut rng = rltk::RandomNumberGenerator::new(); //Obtain the thread-local RNG
        for _i in 0..400 {
            let x = rng.roll_dice(1, self.width - 1);
            let y = rng.roll_dice(1, self.height - 1);
            let idx = self.xy_idx(x, y); //Calculate the new walls location in the tile vector from its position
            if idx != self.xy_idx(self.width / 2, self.height / 2) {
                self.tiles[idx] = TileType::Wall;
            }
        }
    }
}
