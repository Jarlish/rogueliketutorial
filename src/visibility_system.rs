use specs::prelude::*;
use super::{Viewshed, Position, Map, Player};
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (WriteExpect<'a, Map>, Entities<'a>, WriteStorage<'a, Viewshed>, WriteStorage<'a, Position>, ReadStorage<'a, Player>);

    fn run(&mut self, data : Self::SystemData) {
        //Get the necessary data from the ECS
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() { //All entities with a viewshed and a position
            if viewshed.dirty { //Viewshed needs to be updated
                viewshed.dirty = false; //Mark viewshed as up to date
                viewshed.visible_tiles.clear(); //Clear the list of visible tiles
                viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map); //Use RLTK's fov function to calculate the visibility of the tiles in a given map from a given position within a given range
                viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height); //Retain only tiles within the bounds of the map

                //If the current entity is the player, reveal the tiles they can see
                let _p : Option<&Player> = player.get(ent); //Check if the entity has a Player component
                if let Some(_p) = _p { //If there was a Playe component
                    for t in map.visible_tiles.iter_mut() { *t = false }; //Mark all tiles in the map not visible
                    //Mark all tiles on the map that the player can see as visible
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}
