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

pub fn rad(degr: f32) -> f32 {
    degr / 180. * PI
}

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
