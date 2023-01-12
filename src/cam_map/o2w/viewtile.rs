//use bevy::asset::LoadState;

use super::materialobject::*;
use crate::cam_map::utils::TileName;
//e crate::cam_map::o2w::PbfTile;

/* *
 * The view tile data is processed in steps
 * The view tile is send to the GPU in layers
 * /
#[derive(Debug)]
pub enum ViewState { // last requested layer
    /** An error was detected, tile is not usable */
    Error,
    /** Visualisation is requested but not done yet */
    Idle,
    /** Tile is on the way to the GPU to get visible. (buildings etc.) */
    Near,
    /** Tile is on the way to the GPU to get visible. (trees etc.) */
    Far,
    /** tile is visible */
    Done,
}

#[derive(Debug)]
pub struct PbfClient { // dummy only!
    //name: TileName,
    //state: LoadState,
}
*/

#[derive(Debug)]
pub struct ViewTile {
    pub name: TileName,    // x/y OSM-coordinates of this tile
    //pub pbf_client: Option<PbfClient>,
    //state: ViewState,
    _material_objects: Vec<MaterialObject>,
}

impl ViewTile {

    pub fn new(x: u32, y: u32) -> ViewTile {

        ViewTile {
            name: TileName{ x, y }, // TODO  * fakt,  no constants!
            //pbf_client: None,
            //state: ViewState::Idle,
            _material_objects: Vec::new(),
        } // return
    }

}
