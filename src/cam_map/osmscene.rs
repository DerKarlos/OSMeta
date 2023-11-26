// OSM-Scene: A part of the  MAP,  visible in 3D

use super::geopos::{ GeoPos };
use super::geoview::*;
use super::utils::
{
    TileName,
    ScenePos, PI, LAT_FAKT, PBF_ZOOM, FACT_ZOOM
};

/**
 * Handler of a 3D rendered scene with OSM objects at a given geo position on Earth
 *
 * To make the OSM object visible, some actions have to be managed:
 * Check, what pbf-tiles are needed and start loading.
 * Check, what viewtiles should be visible.
 * Check, what LoD-Level of the view-tiles should be visible or hidden.
 *
 * The null point of the scene in the GPU is equal to the center of the first loaded pbf-file.
 * The osm scene shows an arrea around the given geo position
 *
 * A large distance from the scene center would cause
 * large f32s in the GPU, inaccurade calculations and an ugly wrong 3D view.
 * To show i.e. a place in London and New York,
 * each place has to be done by an extra instance of OsmScene
 *
 * Not supported yet:
 * A) Moving far away from the center should shift the center point???
 * B) Moving around a lot would load many tiles, and render OSM objects.
 * To avoild overload of the system, far away invisieble tiels and data should be dismissed.
 */

#[derive(Debug)]
pub struct OsmScene {
    /** Tile-name(x/y) of the first loaded pbf-tile */
    pub first_pbf_tile_name:  TileName,

    /** Tile-name(x/y) of the nord west view-tile of the first loaded pbf-tile */
    pub first_view_tile_name: TileName,

    pub fst_geo_pos: GeoPos,
    pub one_geo_pos: GeoPos,
    
    /** Size of a pbf-tile in meter (only x/z used). Depends on the latitude. */
    pub pbf_size: glam::Vec3,

    /** Calculates the pbf-tile corner-to-center offset */
    pub pbf_corner_to_center: glam::Vec3, // BABYLON.Vector3;

    /** Geo-location of the center of the first loaded pbf-tile, and the center of the scene */
    // Merkator center of pbf-tile = pbf zoom+1 tile corner = pbf name*2+1 and pbf zoom+1
    pub null_geo_pos: GeoPos,
    /** geo-location of the nord west corner of the first loaded pbf-tile */
    // lat: 48.545707582202596 lon: 13.491210938407548
    pub null_corner_geo_pos: GeoPos,

    /** count of loading workers to limit the running workers */
    pub pbf_count: u32, // = 0,
}


impl OsmScene {
    /**
     * OsmScene constructor: Start loading the pbf file/tile.
     * @param geoView  Geo position and camea view to start the scene with
     * @param viewer  OSM scene handler
     */
    pub fn new (geo_view: GeoView) -> OsmScene {

        // geo_view.store("start".to_string());


        // calculate tile-names(x/y), containing the CPU-Scene 0/0 in its center
        let first_pbf_tile_name  = geo_view.geo_pos.calc_tile_name(PBF_ZOOM);
        // pbf-tile 1/2 scaled would be 8/16 and added 12/20 i.e.  Adding is to get the first tiel next to the GPU 0 point
        let first_view_tile_name = first_pbf_tile_name 
                                 * TileName{x: FACT_ZOOM,   y: FACT_ZOOM  }
                                 + TileName{x: FACT_ZOOM/2, y: FACT_ZOOM/2};

        // Get the first loaded pbf/view-tile(corner) geoPos
        // and the next (+1x/y) pbf/view-tile(corner) geoPos -- The next pbf-tile is the end of the first one
        let fst_geo_pos = OsmScene::calc_corner_geo_pos_from_name(           first_pbf_tile_name,                PBF_ZOOM);
        let one_geo_pos = OsmScene::calc_corner_geo_pos_from_name(first_pbf_tile_name + TileName::ONE, PBF_ZOOM);
        //println!("+111: {:?}",osm_scene.first_pbf_tile_name + 1.);

        let null_geo_pos = OsmScene::calc_corner_geo_pos_from_name(
                                                                      first_pbf_tile_name * TileName::TWO + TileName::ONE,
                                                                      PBF_ZOOM + 1
                                                                  );

        // calcs the geoPos delta (degrees) and trans-calcs it to meters:
        let mut pbf_size = one_geo_pos.calc_meters_to_other_geo_pos(fst_geo_pos);
        // _x: 3232.2079333866873   >  _z: 3231.278959942741  should be equal? calculaton not exactly???

        // _Name add One => +x/+y => +x/-z meter  because:
        // _Name y+1 means more south
        //           means less degrees
        //           means more to the 3D-camera
        //           means less z (because z to the eye is negative and to the back is positive in BJS/bevy)
        // to correct self:
        pbf_size.z *= -1.; // todo?: let it negative as needed for ..toCenter

        // From the nord-west / upper-left corner (+z / -x) ...
        // ... to the center (0 / 0) by adding self delta (-z / +x)
        // Example First pbfTile: 0/0 -16xx/+16zz = -16xx/+16zz
        let pbf_corner_to_center = glam::Vec3::new (
             -pbf_size.x / 2., 0.,
             pbf_size.z / 2. );  // ??? - for BEVY ???

        let null_corner_geo_pos = OsmScene::calc_corner_geo_pos_from_name(first_pbf_tile_name, PBF_ZOOM);

        OsmScene{

        first_pbf_tile_name,
        first_view_tile_name,
        fst_geo_pos,
        one_geo_pos,
        pbf_size,

        pbf_corner_to_center,
        null_geo_pos,        // todo: remove and use null_corner_geo_pos
        null_corner_geo_pos,
        pbf_count: 0,
        }

    } // new / constructor

    /**
     * Calculate the geo-location of a tile (nord west edge) by self tile-name(x/y)
     * More x means more lon. More y means less lat!
     * @param tile_Name  tile-name(x/y) of a tile to calc the geo-location from
     * @param zoom  Zoom level on the OSM tile-name(x/y) system
     * @return a lat,lon geo position (GPS)
     */
    pub fn calc_corner_geo_pos_from_name(tile_name: TileName, zoom: u32) -> GeoPos {
        let n = PI - 2. * PI * tile_name.y as f32 / 2_u32.pow(zoom) as f32;
        let lat = 180. / PI * (0.5 * ((n).exp() - (-n).exp() )).atan();
        let lon = tile_name.x as f32 / 2_u32.pow(zoom) as f32 * 360. - 180.;
        GeoPos{lat, lon}
    }

    /**
     * calculate the GPS position from a position in the scene
     * @param scenePos position in the scene
     * @return GeoPos position on Earth
     **/
    pub fn calc_geo_pos_from_scene_pos(&self, scene_pos: ScenePos) -> GeoPos {
        let lat = -scene_pos.z /  LAT_FAKT + self.null_geo_pos.lat;  // -z   to nord = more z =
        let lon =  scene_pos.x / (LAT_FAKT * ((lat / 180. * PI).abs() ).cos()) + self.null_geo_pos.lon;
        GeoPos{lat, lon}
    }
}
