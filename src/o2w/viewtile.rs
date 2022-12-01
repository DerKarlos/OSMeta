use super::materialobject::*;


//#[derive(Debug)]
pub struct ViewTile {
    _material_objects: Vec<MaterialObject>,
}

impl ViewTile {

    pub fn new(_x: i32, _y: i32) -> ViewTile {

        ViewTile {
            _material_objects: Vec::new(),
        } // return
    }

}
