use super::geoview::*;
use super::utils::{ScenePos,PI};
use super::osmscene::*;

/**
 * Camera position and view angles at/abowe a [[OsmScene]]
 * (internal! uses rad. For the user API see [[GeoView]] )
 */
#[derive(Debug,Clone,Copy)]
pub struct CameraView {
     pub scene_pos: ScenePos, // (meter)   Position in the virtual 3D Babylon world.
     pub alpha:    f32,      // (radians) The longitudinal/horizontal rotation of the camera. Default is -25 degr
     pub beta:     f32,      // (radians) The latitudinal/ up down    rotation of the camera. Default is -13 degr
     pub radius:   f32,      // (meter)   The camera distance from the target position. Defaut is 450 getUrlParameter
     pub fov:      f32,      // (rad)     The camera view angle / zoom
}

impl CameraView {

    /**
     * View constructor
     * @param scenePos (meter)   Position in the virtual 3D Babylon world.
     * @param alpha    (radians) The longitudinal/horizontal rotation of the camera. Default is -25 degr
     * @param beta     (radians) The latitudinal/ up down    rotation of the camera. Default is -13 degr
     * @param radius   (meter)   The camera distance from the target position. Defaut is 450 getUrlParameter
     * @param fov      (rad)     The camera view angle / zoom
     */
    pub fn _new(
        scene_pos: ScenePos,
        alpha: f32,
        beta: f32,
        radius: f32,
        fov: f32
    ) -> CameraView
    {
        CameraView{
        scene_pos,
        alpha,
        beta,
        radius,
        fov,
        }
    }


    /**
     * Create a new [[GeoView]] of self CameraView
     * @param osmScene  The scene, the camera view is used in
     * @return Containing the actual camera position and view on Earth
     */
    fn _to_geo_view(mut self, osm_scene: OsmScene) -> GeoView {

        // keep the direction value between +/- 180 degrees
        if self.alpha >  PI { self.alpha -= 2. * PI };
        if self.alpha < -PI { self.alpha += 2. * PI };

        GeoView{
            geo_pos: osm_scene._calc_geo_pos_from_scene_pos(self.scene_pos),
            height:  self.scene_pos.y,
            dir:     (self.alpha).to_radians(), // + 90., //  BJS alpha -90 rad = nord        becomes  API dir  0 degrees = nord
            view:    (self.beta ).to_radians(), // - 90., //  BJS beta  +90 rad = horizontal  becomes  API view 0 degrees = horizontal
            radius:  self.radius,
            fov:     (self.fov).to_degrees(),
        }
    }


}
