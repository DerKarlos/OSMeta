// extern and system uses
use glam::{Vec2, Vec3};
//use protobuf::EnumOrUnknown;
use rand::prelude::*;  // random

// module uses
use super::frontend::*;
use super::viewtile::*;
use super::instance_parameter::*;
use super::materialobject::*;
use super::utils::*;
use super::textures::*;
use super::cars::*;
use super::print::*;


/**
 * The pbf-tile data is processed in steps
 */
#[derive(Debug)]
pub enum PbfState {
    /** Load is requested */
    Idle,
    /** Loading on the way */
    Loading,
    /** Loaded, ready to use */
    Loaded,
}



#[derive(Debug)]
pub struct PbfTile {
    state: PbfState,
    pbf_url: String,
    start_pos: Vec3,

    coords2d: Vec<i64>,
    coords3d: Vec<i64>,
    pbf_materials: Vec<Material>,
    material_map: Vec<Vec<usize>>,
    strings: Vec<String>,
    shapes: Vec<Shape>,
    models: Vec<WorldObject>,
    objects: Vec<WorldObject>, // pbrfile objects

    type_tree_row: u32,
    type_tree: u32,
    type_forest: u32,

    material_objects: Vec<MaterialObject>,
}

impl PbfTile {
    pub fn new(x: u32, y: u32, start_pos: Vec3) -> PbfTile {
        // logs(format!("  PbfTile OSM tile: {}/{}", x, y));

        let mut material_map = Vec::<Vec::<usize>>::new(); // NO! vec![..]
        material_map.push(           Vec::<usize>::new() );
        material_map.push(           Vec::<usize>::new() ); // This code syntax is not Working:  vec![<usize>;0]

        PbfTile {
            state: PbfState::Idle,

            pbf_url: pbf_tile_path(x, y),
            start_pos,
            coords2d: Vec::new(),
            coords3d: Vec::new(),
            pbf_materials: Vec::new(),
            material_map,
            strings: Vec::new(),
            shapes: Vec::new(),
            models: Vec::new(),
            objects: Vec::new(),

            type_tree_row: 999,
            type_tree: 999,
            type_forest: 999,

            material_objects: Vec::new(),
        }
    }

    pub fn load(&mut self, renderer: &mut super::Renderer, textures: &mut Textures, cars: &mut Cars) -> ViewTile {
        logs(format!("  loading OSM tile: {} (takes some seconds or up to 3 minutes)", self.pbf_url));

        self.state = PbfState::Loading;

        let bytes = super::load_pbr_bytes(self.pbf_url.clone());
        let tile: Tile = protobuf::Message::parse_from_bytes(&bytes).unwrap();

        self.coords2d = tile.vector2dBlock.coords.to_vec();
        self.coords3d = tile.vector3dBlock.coords.to_vec();
        self.pbf_materials = tile.materialBlock.materials.to_vec();
        self.shapes = tile.shapeBlock.shapes.to_vec();
        self.models = tile.modelBlock.models.to_vec();
        self.objects = tile.objects.to_vec();
        self.strings = tile.stringBlock.strings.to_vec();

        for (string_index, string) in self.strings.iter().enumerate() {
            match string.as_str() {
                "TreeRow" => self.type_tree_row = string_index as u32,
                "Tree" => self.type_tree = string_index as u32,
                "Forest" => self.type_forest = string_index as u32,
                _ => (),
            };
        }

        print_materials(&self.pbf_materials, PRINT_LIMIT);

        let view_tile = ViewTile::new(11,22);

        for (pbf_material_index, pbf_material) in self.pbf_materials.iter_mut().enumerate() {
            // println!("PBR material {}", pbf_material_index);

            //// check for textures and prepare loading ////
            let texture_layers = &pbf_material.textureLayer;

            if !texture_layers.is_empty() {
                let texture_layer = &texture_layers[0];

                if let Some(url) = texture_layer.baseColorTextureURI.clone() { textures.add(url); }
                if let Some(url) = texture_layer.ormTextureURI.clone()       { textures.add(url); }
                if let Some(url) = texture_layer.normalTextureURI.clone()    { textures.add(url); }

                // println!("{}#4 url: {}  orm: {}  nor:{}", pbf_material_index, url, orm, nor);

            };

            //// Create first MaterialObject. May have a Texture ////
            let material_object = MaterialObject::new(pbf_material, pbf_material_index, 0);
            self.material_map[0].push(self.material_objects.len());
            self.material_objects.push(material_object);

            if texture_layers.len() > 1 {
                let texture_layer = &texture_layers[1];

                if let Some(url) = texture_layer.baseColorTextureURI.clone() { textures.add(url); }
                if let Some(url) = texture_layer.ormTextureURI.clone()       { textures.add(url); }
                if let Some(url) = texture_layer.normalTextureURI.clone()    { textures.add(url); }

                //// Create second MaterialObject. Will have a Texture ////
                let material_object = MaterialObject::new(pbf_material, pbf_material_index, 1);
                self.material_map[1].push(self.material_objects.len());
                self.material_objects.push(material_object);
            } else {
                self.material_map[1].push(999); // push dummy
            };
        }

        // println!("134 material_map: {:?}", self.material_map);

        logs(format!( // !!! CPU->GPU ist faster with --release  why???
            "Tile loaded. Bytes:{}  materials: {}  textures: {}>{}  osm-objects: {}.",
            bytes.len(),
            self.material_objects.len(),
            &textures.adds,
            &textures.len(),
            self.objects.len(),
        ));
        logs(format!("Loading all textures in CPU and GPU (that will take some secounds)"));

        textures.load(renderer);

        // next "step"
        self.tile_objects(renderer, textures, cars);

        //cars.instantiate(renderer);

        view_tile // return
    }

    fn tile_objects(&mut self, renderer: &mut super::Renderer, textures: &mut Textures, cars: &mut Cars) {
        // println!("tile_objects");

        /////// objects //////////////////
        self.proccess_objects(cars);

        /////// material_objects //////////////////
        //let shadow_objects =
        //  &pbf_tile.create_objects(&mut loaded, renderer: &rend3::Renderer);
        self.create_objects(renderer, textures);

        self.state = PbfState::Loaded;
        log(""); // new line after "......"
    }

    fn proccess_objects(&mut self, cars: &mut Cars) {
        //let start = Local::now();

        // get and draw all objects    ttt  iiiall
        for (obejct_index, object) in self.objects.to_vec().iter().enumerate() {
            //if obejct_index != 1930  // 1930: -963.299, 1.0, -1295.737,
            //&& obejct_index != 11722 {continue}
            //if obejct_index != 8 {continue}

            let osm_id =
            if let Some(id) = object.osmId.clone() { 
                id
            } else {
                "".to_string()
            };

            //if object.get_osmId() != "w336274097" {continue}   // w336274097 = Radständer

            // if object.get_osmId() != "n1709339190" {continue};

            if
            //true ||
                osm_id ==  "w42082584"  ||  // !!
                osm_id == "w797605537"  ||
                osm_id == "w797605530"  ||
                osm_id == "w797605524"
            {
                /****
                let tn = object.get_typeName() as usize;
                let n = &self.strings[tn];
                println!("\nObject #{}={:?} type: #{:?}={:?}", obejct_index, _osm_id, tn,n );
                let tgs = object.get_triangleGeometries();
                for g in tgs.iter() {  // Glass_Normal.jpg
                    let mi = g.get_material() as usize;
                    let material = &self.pbf_materials[mi];
                    println!("   tg material: #{}={:?}", mi, material ); // ääää
                    let tls = &material.get_textureLayer();
                    for l in tls.iter() {
                        println!(" * textureLayer: {:?}", l );
                    }

                }
                let egs = object.get_extrusionGeometries();
                for g in egs.iter() {
                    let mi = g.get_material() as usize;
                    let m = &self.pbf_materials[mi];
                    println!("   eg material: #{}={:?}", mi, m );
                }
                ****/
                let igs = &object.instanceGeometries;
                for g in igs.iter() {
                    println!("   instance: {:?}", g);
                }
            };

            // n4750768485 = Bank vor Akropolis  index 1608  vertex 19238*3=57714: -1491.816​/500/721.101

            // log(&self.strings[ object.get_typeName() as usize ]);

            // 0=indefinite .. 4=near
            if let Some(max) = object.maxLod.clone() {  // n1788460026 = 2 ????
                if max < 4 {
                    continue;
                }    
            }
//            let max_lod = object.maxLod.unwrap(); // n1788460026 = 2 ????
//            if max_lod < 4 {
//                continue;
//            }

            //println!("{} id: {}", obejct_index, object.get_osmId());

            let mut type_name_index = 0;
            if let Some(index) = object.typeName {
                type_name_index = index
            }
            self.proccess_object(
                obejct_index,
                object,
                &InstanceParameter::nop(&osm_id, type_name_index),
                cars,
            );
        } // objects

        //println!("::: proccessing time {:?}", Local::now() - start);
    } // proccess_objects

    fn proccess_object(
        &mut self,
        _obejct_index: usize,
        object: &WorldObject,
        instance_parameter: &InstanceParameter,
        cars: &mut Cars,
    )
    {
        //println!("object: {:?}", object);

        // if _obejct_index > 2000 {return;} //ddd

        //let mut _test = 0;

        for triangle_geometry in object.triangleGeometries.to_vec().iter() {
            self.proccess_triangle_geometry(triangle_geometry, instance_parameter, _obejct_index);
        }

        for instance_geometry in object.instanceGeometries.to_vec().iter() {
            //println!("instance_geometry");
            self.proccess_instance_geometry(instance_geometry, instance_parameter, cars);
        }

        //  in list.iter() for line {  // looks more readable
        for extrusion_geometry in object.extrusionGeometries.to_vec().iter() {
            //println!("{} extrusion {:?}",_obejct_index,object.get_osmId());
            self.proccess_extrusion_geometry(extrusion_geometry, instance_parameter);
        }
    } //proccess_object




    fn calc_positions(
        &mut self,
        vertices: &Vec<u64>,
        instance_parameter: &InstanceParameter,
    ) -> Vec<ScenePos>
    {

        let mut positions: Vec<ScenePos> = Vec::new();
        for vertice in vertices.iter() {
            let mut meter = ScenePos::new(
                self.coords3d[*vertice as usize * 3    ] as f32,
                self.coords3d[*vertice as usize * 3 + 1] as f32,
                self.coords3d[*vertice as usize * 3 + 2] as f32,
            ) / 1000.0; // 1000: mm to meter

            //// instance code here ///
            // Rotate model instance
            let angle = -instance_parameter.direction;
            let x_rotated = meter.x * angle.cos() - meter.z * angle.sin(); // no Vec.rotate_y(angle) ???
            let z_rotated = meter.z * angle.cos() + meter.x * angle.sin();
            meter.x = x_rotated;
            meter.z = z_rotated;
            // scale and place model in the scene
            meter = meter * instance_parameter.scale + instance_parameter.pos_offset;

            //println!("position:: {} {} {}",x_meter,y_meter,-z_meter);
            positions.push(meter);
        }

        positions // return
    }



    fn calc_uvdummies(&mut self, count: usize) -> Vec<super::Uv> {
        let mut uvs: Vec<super::Uv> = Vec::new();
        for _number in 0..(count as u32) {
            uvs.push(super::shape_uv(0.0, 0.0)); //Vec2::new(0.0, 0.0));
        }
        uvs // return
    }

    fn calc_uvs(&mut self, tex_coords: &[u64], count: usize) -> Vec<super::Uv> {
        let mut uvs: Vec<super::Uv> = Vec::new();
        for tex_coord in tex_coords.iter().take(count) {  //  OR: layer-renderer
            uvs.push(super::shape_uv(
                self.coords2d[*tex_coord as usize * 2    ] as f32 / 1000.0,
                self.coords2d[*tex_coord as usize * 2 + 1] as f32 / -1000.0, // 1000: mm to meter
            ));
        }
        uvs // return
    }

    fn calc_vertices_uvs(&mut self, vertices: &[u64], texture_layer: &material::TextureLayer) -> Vec<super::Uv> {

        let width  = texture_layer.textureWidth()  as f32 / 1000.0; // 1000: mm to meter
        let height = texture_layer.textureHeight() as f32 / 1000.0;

        let mut uvs: Vec<super::Uv> = Vec::new();
        for vertice in vertices.iter() {
            uvs.push(super::shape_uv(
                // width/height the texture shall be used in meter, independend of the pixsel size.
                self.coords3d[*vertice as usize * 3    ] as f32 / 1000.0 / width, // 1000: mm to meter / pixsel = [1]
                self.coords3d[*vertice as usize * 3 + 2] as f32 / 1000.0 / height,
            ));
        }
        uvs // return
    }



    fn proccess_triangle_geometry(
        &mut self,
        triangle_geometry: &TriangleGeometry,
        instance_parameter: &InstanceParameter,
        _obejct_index: usize,
    ) {
        let vertices = &triangle_geometry.vertices; // 3 = face
        let pbf_material_index = triangle_geometry.material.unwrap() as usize;
        let pbf_material = &self.pbf_materials[pbf_material_index].clone(); // CLONE!
        let texture_layers = &pbf_material.textureLayer;

        if vertices.len() > 0 {
            let vertice = vertices[0];  // vertices: &[u64],
            let first_pos = ScenePos::new(
                self.coords3d[vertice as usize * 3    ] as f32,
                self.coords3d[vertice as usize * 3 + 1] as f32,
               -self.coords3d[vertice as usize * 3 + 2] as f32,
            ) / 1000.0; // 1000: mm to meter
            let distance = first_pos.distance(self.start_pos);
            //println!("{:?} {:?} {:?} {:?} {:?} ",vertices,vertice,first_pos,self.start_pos,distance);
            if distance > 10000.0 {return};
        }

        let mut next_text_coords: usize = 0; // start index of the next set of UV-values

        if texture_layers.is_empty() // if true
        {
            // tile-material does not use textures: onyl DUMMY - UV/texturCoords

            let positions = self.calc_positions(&vertices, instance_parameter);
            let mut uvs = self.calc_uvdummies(vertices.len());
            let material_objects_index0 = self.material_map[0][pbf_material_index];
            // println!("texture empty: {},{}",material_objects_index0,pbf_material_index);
            let material_object0 = &mut self.material_objects[material_objects_index0];
            material_object0.push_object(
                &positions,
                &mut uvs,
                false,
            );
        } else {
            for (texture_index, texture_layer) in texture_layers.iter().enumerate() {
                // one set of XYZ vertices UV texture coordinates and per texture
                // println!("tex/mat {}/{}",texture_index,pbf_material_index);

                let positions = self.calc_positions(&vertices, instance_parameter);

                let mut xz_to_uv = false;

                if let Some(tcf_box) = texture_layer.texCoordFunction {                                 // println!("tcf: {:#?}",tcf_box);
                    if tcf_box.unwrap() ==  material::texture_layer::TexCoordFunction::GLOBAL_X_Z {     // ??? TexCoordFunction is pub !!!
                        xz_to_uv = true;                                                                // println!("true");
                }   }

                if !xz_to_uv && triangle_geometry.texCoords.len() < vertices.len() {
                    xz_to_uv = true;
                }

                let mut uvs =
                if xz_to_uv {
                    self.calc_vertices_uvs(&vertices, texture_layer)
                } else {
                    let tex_coords = &triangle_geometry.texCoords; // -> &[u64]
                    let int_slice = &tex_coords[next_text_coords..];
                    next_text_coords += vertices.len();
                    self.calc_uvs( int_slice, vertices.len() )
                };

                let material_objects_index = self.material_map[texture_index][pbf_material_index];
                // println!("texture index: {},{},{}",material_objects_index,texture_index,pbf_material_index);
                let material_object = &mut self.material_objects[material_objects_index];
                material_object.push_object(
                    &positions,
                    &mut uvs,
                    false,
                );

                /****
                let material_objects_index = self.material_map[texture_index][pbf_material_index];
                self.draw_vertices_xyz(vertices, instance_parameter, material_objects_index); // faces

                let material_object = &mut self.material_objects[self.material_map[texture_index][pbf_material_index]];

                let mut xz_to_uv =
                    texture_layer.get_texCoordFunction() == Material_TextureLayer_TexCoordFunction::GLOBAL_X_Z; // 2;

                if !xz_to_uv && triangle_geometry.get_texCoords().len() < vertices.len() {
                    xz_to_uv = true;
                }

                if xz_to_uv {
                    material_object.draw_vertices_xz_to_uv(&self.coords3d, vertices);
                } else {
                    let tex_coords = triangle_geometry.get_texCoords(); // -> &[u64]
                    let int_slice = &tex_coords[next_text_coords..];
                    material_object.draw_vertices_uv(&self.coords2d, int_slice, vertices.len());
                    next_text_coords += vertices.len();
                };
                ****/
            } // for
        } // else

    } // proccess_triangle_geometry



    fn proccess_instance_geometry(
        &mut self,
        instance_geometry: &InstanceGeometry,
        instance_parameter: &InstanceParameter,
        cars: &mut Cars,
    ) {
        // Not yet used: instance_parameter: &InstanceParameter

        //println!("instance_geometry {:?}",instance_geometry);

        let count = instance_geometry.position.len() / 3;
        for position_index in 0..count {
            //    if position_index > 2000 && position_index % 1000 == 0 {
            //        println!("instance_geometry {}/{}", position_index,count);
            //    }

            //// draw one instance/model

            let mut instance_parameter = InstanceParameter::new(
                instance_parameter.osm_id,
                instance_parameter.type_name_index,
                instance_geometry,
                position_index,
            );

            if instance_parameter.type_name_index == self.type_tree_row
                || instance_parameter.type_name_index == self.type_tree
                || instance_parameter.type_name_index == self.type_forest
            {
                // if (object.typeName == this.strings.Forest && Math.random() > 0.25) continue; // only 25% forest will get instanced -- d d d forest
                instance_parameter.scale *= rand::random::<f32>() / 5.0 + 0.9;
                if instance_parameter.direction == 0.0 {
                    instance_parameter.direction = random::<f32>() * std::f32::consts::FRAC_PI_2;
                }
                if instance_parameter.type_name_index == self.type_tree_row {
                    let dif = instance_parameter.scale * 0.05; // 5% of the height as position variation
                    instance_parameter.pos_offset.x += random::<f32>() * 2.0 * dif - dif;
                    instance_parameter.pos_offset.z += random::<f32>() * 2.0 * dif - dif;
                }
            }

            // outside tile? no draw
            let pbf_tile_position_max = 1620.0; // 1620.0
            if instance_parameter.pos_offset.x > pbf_tile_position_max
                || instance_parameter.pos_offset.x < -pbf_tile_position_max
                || instance_parameter.pos_offset.z > pbf_tile_position_max
                || instance_parameter.pos_offset.z < -pbf_tile_position_max
            {
                continue;
            };

            // let _position = ScenePos::new(instance_parameter.pos_offset.x, 0.0, instance_parameter.pos_offset.z);
            // let viewTile = this.calcViewTile(position, object.typeName == this.strings.Forest);
            // if (!viewTile)
            //     continue;  // do not draw (test limit, outside tile)
            //let layer = this.lodLayer(object, viewTile) as Layer;

            /******** car & ani *************/

            if let Some(rid) = instance_geometry.resourceIdentifier.clone() {
                if  rid == "car" {
                    instance_parameter.pos_offset.x += 0.0; //this.pos.x;
                    instance_parameter.pos_offset.z += 0.0; //this.pos.z;
                    //cars.add(instance_parameter.pos_offset, instance_parameter.direction);
                    continue; // drawn
                }
            }
            
//            if instance_geometry.resourceIdentifier.unwrap() == "car" {
//            }

            /******** ani ************* /

            if (instances_Geometry.animation) { // Animation //
                //if (layer.viewTile && layer.viewTile.pbfTile.checkMaterial <= 0) // only one material anyway whish
                this.animation(instances_Geometry, viewTile, instanceParameter, model);
                continue;
            }

            *********************/

            // triangleGeometries is an array of structures,
            // each structure with parameters and arrays of parameters for models to be placed

            //println!("t1: {:#?}",instance_geometry.model);
            let model = &self.models.to_vec()[instance_geometry.model.unwrap() as usize]; // why does .to_vec() prevent that error? Is it a hidden "copy"?

            //println!("iii instance_parameter {:?}",instance_parameter);
            self.proccess_object(position_index, model, &instance_parameter, cars);
        }
    } // proccess_instance_geometry

    fn proccess_extrusion_geometry(
        &mut self,
        extrusion_geometry: &ExtrusionGeometry,
        instance_parameter: &InstanceParameter,
    ) {
        //println!("extrusion_geometry {} - {:?}", instance_parameter.osm_id ,extrusion_geometry);

        let shape_index = extrusion_geometry.shape.unwrap();
        let scales = &extrusion_geometry.scaleFactors; // 1/1000
        let path_indices = &extrusion_geometry.path; // path-3d-indices und path-ScenePos
        let material_index = extrusion_geometry.material.unwrap();
        let coords3d_offset = self.coords3d.len() as u64 / 3;

        let mut start = false;
        if let Some(g_start) = extrusion_geometry.startCap {
            start = g_start;
        }
        let mut end = false;
        if let Some(g_end) = extrusion_geometry.endCap {
            end = g_end;
        }

        // calculate shape 2D Points, still im mm!
        let pbf_shape = &self.shapes[shape_index as usize];

        let field_type = pbf_shape.type_.unwrap().unwrap(); // Option<Shape_ShapeType> CIRCLE = 1,POLYGON = 2,POLYLINE = 3,
        let parameters = &pbf_shape.parameters; // Vec<i64>

        //// calculate shape as Vec3 points (not ScenePos) ////
        let mut shape_points = Vec::new();
        //println!("field_type {:?}",field_type);
        match field_type {
            shape::ShapeType::CIRCLE => {
                let center = parameters[0];
                let radius = parameters[1] as f32;

                // match as integer in milimeters
                let corners = match (radius * 1000.) as i32 {
                    0..=14 => 3,
                    15..=24 => 4,
                    25..=39 => 6,
                    40..=100 => 8,
                    _ => 12,
                };

                let center = Vec2::new(
                    self.coords2d[center as usize * 2    ] as f32, // 0.0,
                    self.coords2d[center as usize * 2 + 1] as f32,
                );

                let mut angle = 0.0;
                while angle <= (360.01_f32).to_radians() {
                    // (degrees(90.0)
                    shape_points.push(
                        Vec2::new(
                            radius * angle.sin(),
                            radius * angle.cos(), // Vec2 with y only
                        ) + center,
                    );
                    angle += (360_f32).to_radians() / corners as f32;
                }
            } // CIRCLE

            _ => {
                let mut i = 0;
                while i < parameters.len() {
                    shape_points.push(Vec2::new(
                         parameters[i    ] as f32,  // Vec2 XY only
                        -parameters[i + 1] as f32, // minus!  Tobias???
                    ));
                    i += 2;
                }
                if field_type == shape::ShapeType::POLYGON {
                    shape_points.push(shape_points[0]); // Add the first vertex also as the last to close the volume
                }
            }
        } // field_type
          //println!("shape_points {:?}",shape_points); // not scaled yet

        let mut path_positions = Vec::<ScenePos>::new();
        for path_index in path_indices.iter() {
            path_positions.push(ScenePos::new(
                // shapeVertices.push(new BABYLON.Vector3(-v.x, v.z)); //  -x? -v.x, v.z
                self.coords3d[*path_index as usize * 3    ] as f32,
                self.coords3d[*path_index as usize * 3 + 1] as f32,
                self.coords3d[*path_index as usize * 3 + 2] as f32,
            ));
        }
        //println!("path_positions: {:?}",path_positions);

        //// procces path and create a new TriangleGeoemtry ////
      //let mut extrusion_coords3d = Vec::<i64>::new();
        let mut extrusion_vertices = Vec::<u64>::new();

        let vertices_end = path_indices.len() * shape_points.len(); // place to put the center of start and end
        let point_start = path_positions[0];
        let point_end = path_positions[path_indices.len() - 1];

        // calculate up vextors
        let mut up_vectors = Vec::<Vec3>::new();
        let up_vector_indices = &extrusion_geometry.upVectors;
        if up_vector_indices.len() != path_indices.len() {
            for _ in 0..path_indices.len() {
                up_vectors.push(ScenePos::new(0., 0., 1.));
            }
        } else {
            for up_vector_index in up_vector_indices.iter() {
                let mut up = ScenePos::new(
                    self.coords3d[*up_vector_index as usize * 3    ] as f32,
                    self.coords3d[*up_vector_index as usize * 3 + 1] as f32,
                    self.coords3d[*up_vector_index as usize * 3 + 2] as f32,
                ) / 1000.0;
                up.y = -up.y;
                up_vectors.push(up);
            }
        }

        // calculate the forward direction of the shape from the path.
        let mut forward_vectors = Vec::<ScenePos>::new(); // NO! vec![..];
        forward_vectors.push((path_positions[1] - path_positions[0]).normalize());
        // check all positions but the outer
        for path_index in 1..path_positions.len() - 1 {
            let forward_vector = path_positions[path_index + 1] - path_positions[path_index - 1];
            forward_vectors.push(forward_vector.normalize());
        }
        let last = path_positions.len() - 1;
        forward_vectors.push((path_positions[last] - path_positions[last - 1]).normalize());

        // todo?:  block from typeScript code  "texCoordLists" not used?

        for (path_index, path_position) in path_positions.iter().enumerate() {
            // prepare the matrix for the next shape
            let up = up_vectors[path_index];
            let forward = forward_vectors[path_index];
            let right = up.cross(forward).normalize(); // division by 0 not handled ???
            let m = [
                //rotation matrix   //final double[][]
                [  right.x,   right.y,   right.z],
                [     up.x,      up.y,      up.z],
                [forward.x, forward.y, forward.z],
            ];
            //println!("ufr: {:?} {:?} {:?}",up,forward,right);
            //println!("  m: {:?}",m);

            for (shape_point_index, shape_point) in shape_points.iter().enumerate() {
                // calculate the scene pos of the shape point
                let mut scene_pos = ScenePos::new(
                    // Vec2 xy nach Vec3 xy+0: The shape is "standing up" and gets rotated to flat by the matrix
                    shape_point.x,
                    shape_point.y,
                    0.0,
                );

                if scales.len() == path_positions.len() {
                    scene_pos = scene_pos * (scales[path_index] as f32) / 1000.0;
                }

                // Rotate the shape:     rrr
                // Transform only one pos of one shape here. The matrix for each shape is prepared abowe.
                let v = scene_pos; // shape[i];
                let v2 = ScenePos::new(
                    m[0][0] * v.x + m[1][0] * v.y + m[2][0] * v.z,
                    m[0][1] * v.x + m[1][1] * v.y + m[2][1] * v.z,
                    m[0][2] * v.x + m[1][2] * v.y + m[2][2] * v.z,
                );

                //// ??? Todo: ScenePos = Vec3 should have an Add !!! Se: https://docs.rs/glam/0.11.2/glam/struct.Vec3.html
                //  let mut t1 = glam::Vec3::new(1.,2.,3.);
                //  let mut t2 = glam::Vec3::new(1.,2.,3.);
                //  let mut _t = glam::Vec3::new(1.,2.,3.);
                //  _t = t1.Add(t2);

                // println!("{}.{}: {:?} ## {:?}",path_index,shape_point_index,v,v2);
                // scene_pos = v2.Add(path_position);  // todo scene_pos = v2 + path_position;
                scene_pos = ScenePos::new(v2.x + path_position.x, v2.y + path_position.y, v2.z + path_position.z);

                // A/C) push the shape center
                self.coords3d.push(scene_pos.x as i64); // There is no way to push multible times ???
                self.coords3d.push(scene_pos.y as i64); // is there a vec3.to_vecI() ???
                self.coords3d.push(scene_pos.z as i64); // make a mehthode !!

                // if shape_point_index > shape_points.len()-3 {continue} //dd-zwirbel

                // B/C) push the outer tube
                // Not first point: process face ring
                if path_index > 0 && shape_point_index > 0 {
                    // two faces are a square
                    extrusion_vertices.push( coords3d_offset + ((path_index - 1) * shape_points.len() + (shape_point_index - 1)) as u64); // 1
                    extrusion_vertices.push( coords3d_offset + ((path_index - 1) * shape_points.len() +  shape_point_index     ) as u64);
                    extrusion_vertices.push( coords3d_offset + ( path_index      * shape_points.len() + (shape_point_index - 1)) as u64);
                    extrusion_vertices.push( coords3d_offset + ( path_index      * shape_points.len() + (shape_point_index - 1)) as u64); // 2
                    extrusion_vertices.push( coords3d_offset + ((path_index - 1) * shape_points.len() +  shape_point_index     ) as u64);
                    extrusion_vertices.push( coords3d_offset + ( path_index      * shape_points.len() +  shape_point_index     ) as u64);
                }

                // C/C) push the start/end
                if start && path_index == 0 && shape_point_index > 0 {
                    extrusion_vertices.push( coords3d_offset + (path_index * shape_points.len() + (shape_point_index - 1)) as u64);
                    extrusion_vertices.push( coords3d_offset + (path_index * shape_points.len() +  shape_point_index     ) as u64);
                    extrusion_vertices.push( coords3d_offset + vertices_end as u64);
                }
                if end && path_index == path_positions.len() - 1 && shape_point_index > 0 {
                    extrusion_vertices.push( coords3d_offset + (path_index * shape_points.len() + (shape_point_index - 1)) as u64);
                    extrusion_vertices.push( coords3d_offset + (path_index * shape_points.len() +  shape_point_index     ) as u64);
                    extrusion_vertices.push( coords3d_offset + (vertices_end + 1) as u64);
                }
            } // corners
        } //points

        // ad the center points of start and end
        self.coords3d.push((point_start.x) as i64);
        self.coords3d.push((point_start.y) as i64);
        self.coords3d.push((point_start.z) as i64);
        self.coords3d.push((point_end.x) as i64);
        self.coords3d.push((point_end.y) as i64);
        self.coords3d.push((point_end.z) as i64);

        //println!("points done");

        let mut triangle_geometry = TriangleGeometry::new();
        triangle_geometry.set_material(material_index);
        triangle_geometry.vertices = extrusion_vertices;
        triangle_geometry.texCoords = Vec::<u64>::new(); // default anyway???   pub vertices: ::std::vec::Vec<u64>,

      //self.proccess_extrusion_triangle_geometry(&triangle_geometry, &extrusion_coords3d, instance_parameter, 0);
      //self.proccess_extrusion_triangle_geometry(&triangle_geometry, &extrusion_coords3d, instance_parameter, 1);
        self.proccess_triangle_geometry(&triangle_geometry, instance_parameter, 0);
        self.proccess_triangle_geometry(&triangle_geometry, instance_parameter, 1);

        //println!("triangles done");
    } // proccess_extrusion_geometry


    fn create_objects(&mut self, renderer: &mut super::Renderer, textures: &mut Textures) {
        let mut _positions: usize = 0;
        let mut _indices: usize = 0;
      //let mut _test = 0;
        let mut full_count = 0;
        for (mut _test, (index, material_object)) in self.material_objects.iter_mut().enumerate().enumerate() {
      //for (        index, material_object)  in self.material_objects.iter_mut().enumerate() {
            // (liefert) yields references

            // if index > 4 {return}; // ddd

            _test += 1;
            //if _test-1 == 999999999 {continue}
            let full = material_object.create_object(textures, renderer, index);
            if full {full_count += 1};

            let (positions, indices) = material_object.get_lens();
            _positions += positions;
            _indices   += indices;
        }
        logs(format!("  full objects: {}", full_count));

        //logs(format!("Positions:{:?} Indices:{:?}", _positions, _indices));
    } // create_objects
} // pbfTile
