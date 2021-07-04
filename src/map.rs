use rltk::{RGB, Rltk};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

//Function to get a unique index for each map tile from its position
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

//Constructor function for the map
pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];

    //Make the boundaries of the map walls
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    //Randomly place 400 walls
    //Obtain the thread-local RNG
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y); //Calculate the new walls location in the tile vector from its position
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

//Function to draw the map
pub fn draw_map(map: &[TileType], ctx : &mut Rltk) {
    let mut y = 0;
    let mut x = 0;

    for tile in map.iter() {
        //Render the tile based on its type
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_u8(80, 55, 10), RGB::from_u8(0, 0, 0), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_u8(160, 110, 20), RGB::from_u8(0, 0, 0), rltk::to_cp437('#'));
            }
        }

        //Move to the next tile
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
