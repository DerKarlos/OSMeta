// o2w utils.rs
// pub fn don't need to be used
#![allow(dead_code)]

pub type ScenePos = glam::Vec3; //type Vec3 = Vector3<f32>;   with x,y,z   https://docs.rs/cgmath/0.17.0/cgmath/struct.Vector3.html

pub static PI: f32 = std::f32::consts::PI;
pub static TAU: f32 = std::f32::consts::PI * 2.0;
pub static PI15: f32 = std::f32::consts::PI * 1.5;
pub static PI_2: f32 = std::f32::consts::FRAC_PI_2;

pub static SHADOW_DARKNESS: f32 = 0.5; // 4.00  0.0
pub static AMBIENT_LIGHT: f32 = 0.7; // 0.02  0.7 ohne schatten: 1.0 ist schon voll hell
pub static LOCAL: bool = true;

pub static PRINT_LIMIT: u8 = 0;
pub static PRINT_INDEX: usize = 5;

pub static DEFAULT_LAT: f32 = 48.565837; //  x? 4402 35216..23 48.57478991x 1111?0.0 1615.6 795.652 - 820.450
pub static DEFAULT_LON: f32 = 13.453166; //  z? 2828 22624..31 13.447265625  73512.4 1616.1 305.071 -1310.565

/** Camera view direction to nord (camera alpha-90) degrees */
pub static DEFAULT_DIR: f32 = -32.;

/** Camera view slightly down (camera beta) degrees */
pub static DEFAULT_VIEW: f32 = -12.;

/** Camera distance to the view/rotating point in meters */
pub static DEFAULT_RADIUS: f32 = 375.;  // 547 -- F4 zoom 17/18/19 is about 1500/750/375 meter radius

/** Height of the view point (eyes height of a small person) in meters */
pub static DEFAULT_HEIGHT: f32 = 1.6;

/** Camera field of view angle (FoV) in degrees */
pub static DEFAULT_FOV: f32 = 23.; // F4=    OSMgo=23?


/** Factor to calculate meters from gps geo.decimals (latitude, Nort/South position) */
pub static LAT_FAKT: f32 = 111100.0; // 111285; // exactly enough  111120 = 1.852 * 1000.0 * 60  // 1 NM je Bogenminute: 1 Grad Lat = 60 NM = 111 km, 0.001 Grad = 111 m
// todo: pbf-tiles are about 1 meter overlapping witt 111100!
// Tested: 111285 is ok but not perfect



/**
 * OSM x/y tile "name" - parts
 *
 * We use the OSM naming for tiles:
 * https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames.
 * x and y relate to a lonitude and latitude position on Earth.
 * In OSM he two values are only used as part of the filename of a tile.
 * So we named this TypeScript variable/class-type as "Tile_Name",
 * even if it is an x/y coordinate in numbers.
 */
pub type TileName = glam::Vec2; // BABYLON.Vector2


/**
 * x/z dimentions of a tile: nord, latitude / east, Longitude  (y is not used)
 * The Earth nord direction is shown into the backround of the BabylonJS 3D space.
 * Positiv Longitude will be a negative z value!
 */
pub type TileSize = glam::Vec2; // BABYLON.Vector3


pub fn pythagoras(a: f32, b: f32) -> f32 {
    (a.powf(2.) + b.powf(2.)).sqrt()
}

pub fn server_path(file: String) -> String {
    format!("https://osmgo.org/o2w/{}", &file)
    //format!( "http://95.216.25.217/{}", &file) // if it is the tobias-server
}


pub fn pbf_tile_path(x: i32, y: i32) -> String {
    if LOCAL {
        format!("./assets/{}/{}.o2w.pbf", x, y)
    } else {
        let path = format!("tiles/13/{}/{}.o2w.pbf", x, y);
        server_path(path)
    }
}

pub fn textures_path(texture: String) -> String {  // The pbr file includes already "texture/"
    if LOCAL {
        format!("../o2w/{}", texture)
    } else {
        server_path(texture)
    }
}


// https://stackoverflow.com/questions/14154753/how-do-i-make-an-http-request-from-rust
pub fn load_texture(uri: &str) -> Vec<u8> {
    let uri = textures_path(uri.to_string());
    //println!("URI: {}",uri);
    load_data(&uri)
}




pub fn load_data(uri: &str) -> Vec<u8> {
    //println!("load_data: {}",uri);

    if LOCAL {
        std::fs::read(uri).unwrap()
    } else {
        /*
        let response = reqwest::blocking::get(uri).unwrap();
        response.bytes().unwrap().to_vec()
        */
        Vec::new()
    } // return
}


pub fn models_path(model: &str) -> String {
    if LOCAL {
        format!(                     "./examples/osm2world/assets/{}", &model)
        //  "/Users/karlos/Public/rend3/examples/osm2world/assets/car2.glb"
        //ncat!(                   env!("CARGO_MANIFEST_DIR"),      "/assets/").to_string()
        //   "/Users/karlos/Public/rend3/examples/osm2world" concat "/assets/"
    } else {
        server_path(format!("models/{}", &model))
    }
}

// USE: logs(format!("Text {:?}", any));
pub fn logs(text: String) {
    use log::info;
    info!("{}", text);
    //println!("{}", text);
}

pub fn log(text: &str) {
    use log::info;
    info!("{}", text);
    //println!("{}", text);
}

pub fn logn(value: f32) {
    use log::info;
    info!("{}", value);
    //println!("{}", value);
}

pub struct LogFps {
    time: f32,
    cycles: u32,
}

//impl LogFps {

pub fn vertex(pos: [f32; 3]) -> glam::Vec3 {
    glam::Vec3::from(pos)
}

pub fn uv(pos: [f32; 2]) -> glam::Vec2 {
    glam::Vec2::from(pos)
}
