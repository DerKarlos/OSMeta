use crate::frontend::*;
use crate::o2w_utils::*;

/**
 * structure for parameters of one instance
 * it contains the position, rotation and scale of a model-instance
 */
#[derive(Debug)]
pub struct InstanceParameter<'l> {
    // OSM ID
    pub osm_id: &'l str,
    // type name index in the string map
    pub type_name_index: u32,
    // Position in Scene
    pub pos_offset: ScenePos,
    // compas/view direction
    pub direction: f32,
    // size scalling of the model
    pub scale: f32,
}

impl<'l> InstanceParameter<'l> {
    /**
     * @param instances_Geometry  data structure with position-, rotation-, size-arrays
     * @param index  index into the array
     */
    pub fn new<'a>(
        osm_id: &'a str,
        type_name_index: u32,
        instance_geometry: &InstanceGeometry,
        index: usize,
    ) -> InstanceParameter<'a> {
        let pos_offset = ScenePos::new(
            instance_geometry.position[index * 3    ] as f32,
            instance_geometry.position[index * 3 + 1] as f32,
            instance_geometry.position[index * 3 + 2] as f32,
        ) / 1000.0; // mm to meter

        let direction = if index < instance_geometry.direction.len() {
            instance_geometry.direction[index] as f32 / 1000.
        } else {
            0.
        };
        let scale = if index < instance_geometry.scale.len() {
            instance_geometry.scale[index] as f32 / 1000.
        } else {
            1.
        };

        // return
        InstanceParameter {
            osm_id,
            type_name_index,
            pos_offset,
            direction,
            scale,
        }
    }

    pub fn nop(osm_id: &str, type_name_index: u32) -> InstanceParameter {
        InstanceParameter {
            osm_id,
            type_name_index,
            pos_offset: ScenePos::new(0., 0., 0.),
            direction: 0.,
            scale: 1.,
        }
    }
}
