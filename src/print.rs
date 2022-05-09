// pub fn don't need to be used
#![allow(dead_code)]

use crate::frontend::*;
//use crate::o2w_utils::*;

pub fn print_materials(pbf_materials: &[Material], limit: u8) {
    if limit == 0 {
        return;
    };
    for (index, material) in pbf_materials.iter().enumerate() {
        print_material(index, material, limit);
    }
}

pub fn print_material(index: usize, material: &Material, limit: u8) {
    println!(
        "   material: #{} rgb[{:?},{:?},{:?}] {} {} {}",
        index,
        material.get_baseColorR(),
        material.get_baseColorG(),
        material.get_baseColorB(),
        match material.get_transparency() {
            Material_Transparency::TRUE => "transparency:Cutout",
            Material_Transparency::BINARY => "transparency:BINARY",
            _ => "",
        },
        if material.get_castShadow() { "castShadow" } else { "" },
        if material.get_doubleSided() { "doulbeSide" } else { "" },
    );

    print_texture_layers(material.get_textureLayer(), limit - 1);
}

pub fn print_texture_layers(texture_layers: &[Material_TextureLayer], limit: u8) {
    if limit == 0 {
        return;
    };
    for (index, texture_layer) in texture_layers.iter().enumerate() {
        print_texture_layer(index, texture_layer, limit);
    }
}

pub fn print_texture_layer(index: usize, texture_layer: &Material_TextureLayer, _limit: u8) {
    //    normalTextureURI: ::protobuf::SingularField<::std::string::String>,
    //    displacementTextureURI: ::protobuf::SingularField<::std::string::String>,
    //    emissiveTextureURI: ::protobuf::SingularField<::std::string::String>,
    //    reflectivityTextureURI: ::protobuf::SingularField<::std::string::String>,
    //    wrap: ::std::option::Option<Material_TextureLayer_Wrap>,
    //    colorable: ::std::option::Option<bool>,
    //    texCoordFunction: ::std::option::Option<Material_TextureLayer_TexCoordFunction>,

    let mut base = texture_layer.get_baseColorTextureURI().to_string();
    base.truncate(80);
    let mut orm = texture_layer.get_ormTextureURI().to_string();
    orm.truncate(80);
    let width = texture_layer.get_textureWidth().to_string();
    let height = texture_layer.get_textureHeight().to_string();

    println!(
        "   `  textureLayer: #{} width/height:{}/{} baseColor:{} orm:{}",
        index, width, height, base, orm,
    );
}

fn trim_uri(uri: &mut String) -> String {
    uri.truncate(80);
    uri.to_string()
}

/**************

material: #0 rgb[77,128,102]  castShadow
material: #1 rgb[110,182,145]  castShadow
material: #2 rgb[200,200,200]  castShadow
`  textureLayer: #1500 width/height:1500/0 baseColor:textures/Metal002_Color.jpg orm:textures/Metal002_ORM.jpg
material: #3 rgb[0,128,0]  castShadow doulbeSide
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/Fabric032_Color.jpg orm:textures/Fabric032_ORM.jpg
material: #4 rgb[77,77,77]
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/Asphalt010_Color.png orm:textures/Asphalt010_ORM.jpg
material: #5 rgb[102,102,102]  castShadow
`  textureLayer: #1600 width/height:200/0 baseColor:textures/KnerrKerb0001.png orm:
material: #6 rgb[77,77,77]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/Asphalt010_Color.png orm:textures/Asphalt010_ORM.jpg
`  textureLayer: #1000 width/height:2000/1 baseColor:textures/road_marking_crossing.png orm:
material: #7 rgb[77,77,77]
`  textureLayer: #1500 width/height:1500/0 baseColor:textures/PavingStones033_Color.jpg orm:textures/PavingStones033_ORM.jpg
material: #8 rgb[230,230,230]
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/Asphalt010_Color.png orm:textures/Asphalt010_ORM.jpg
`  textureLayer: #9000 width/height:300/1 baseColor:textures/road_marking_dash.png orm:
material: #9 rgb[204,0,0]  castShadow
material: #10 rgb[204,0,0]  castShadow
material: #11 rgb[140,140,140]  castShadow
`  textureLayer: #1200 width/height:600/0 baseColor:textures/Concrete034_Color.jpg orm:textures/Concrete034_ORM.jpg
material: #12 rgb[46,82,36]  castShadow
material: #13 rgb[99,38,28]  castShadow
material: #14 rgb[200,200,200]  castShadow
`  textureLayer: #1500 width/height:1500/0 baseColor:textures/Metal002_Color.jpg orm:textures/Metal002_ORM.jpg
material: #15 rgb[255,255,255]  castShadow
material: #16 rgb[77,0,0]
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/Ground042_Color.jpg orm:textures/Ground042_ORM.jpg
material: #17 rgb[102,102,102]
`  textureLayer: #600 width/height:600/0 baseColor:textures/Gravel004_Color.jpg orm:textures/Gravel004_ORM.jpg
material: #18 rgb[77,51,51]  castShadow
`  textureLayer: #500 width/height:500/0 baseColor:textures/Wood026_Color.jpg orm:textures/Wood026_ORM.jpg
material: #19 rgb[250,230,13]  castShadow
material: #20 rgb[255,204,0]  castShadow
material: #21 rgb[140,140,140]  castShadow
`  textureLayer: #1200 width/height:600/0 baseColor:textures/Concrete034_Color.jpg orm:textures/Concrete034_ORM.jpg
material: #22 rgb[230,230,230]  castShadow
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/Glass_Color.jpg orm:textures/Glass_ORM.jpg
material: #23 rgb[225,0,116]  castShadow
material: #24 rgb[77,51,51]  castShadow
`  textureLayer: #500 width/height:500/0 baseColor:textures/Wood026_Color.jpg orm:textures/Wood026_ORM.jpg
material: #25 rgb[165,42,42]  castShadow
`  textureLayer: #500 width/height:500/0 baseColor:textures/Wood026_Color.jpg orm:textures/Wood026_ORM.jpg
material: #26 rgb[255,255,0]  castShadow
`  textureLayer: #500 width/height:500/0 baseColor:textures/Wood026_Color.jpg orm:textures/Wood026_ORM.jpg
material: #27 rgb[102,102,102]
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/PavingStones072_Color.jpg orm:textures/PavingStones072_ORM.jpg
material: #28 rgb[0,128,0]  castShadow
`  textureLayer: #500 width/height:500/0 baseColor:textures/Wood026_Color.jpg orm:textures/Wood026_ORM.jpg
material: #29 rgb[179,179,179] transparency:BINARY castShadow doulbeSide
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/powertower_vertical.png orm:
material: #30 rgb[179,179,179] transparency:BINARY castShadow doulbeSide
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/powertower_horizontal.png orm:
material: #31 rgb[255,255,204]  castShadow
`  textureLayer: #2565 width/height:1465/0 baseColor:textures/OSM2World_Domposter.png orm:textures/Paper001_ORM.jpg
material: #32 rgb[77,77,77]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/Asphalt010_Color.png orm:textures/Asphalt010_ORM.jpg
`  textureLayer: #1000 width/height:1000/1 baseColor:textures/road_marking_dash.png orm:
material: #33 rgb[160,166,155]  castShadow
`  textureLayer: #2000 width/height:2000/0 baseColor:textures/Bricks008_Color.jpg orm:textures/Bricks008_ORM.jpg
material: #34 rgb[255,136,136]
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/Asphalt010_Color.png orm:textures/Asphalt010_ORM.jpg
material: #35 rgb[206,109,90]
`  textureLayer: #500 width/height:500/0 baseColor:textures/Fabi2-putz1.png orm:
material: #36 rgb[0,204,0]
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/Ground003_Color.jpg orm:textures/Ground003_ORM.jpg
material: #37 rgb[204,186,128]  castShadow
material: #38 rgb[255,255,255]  castShadow
`  textureLayer: #1500 width/height:1500/0 baseColor:textures/Plastic010_Color.jpg orm:textures/Plastic010_ORM.jpg
material: #39 rgb[0,0,0]  castShadow doulbeSide
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/Fabric032_Color.jpg orm:textures/Fabric032_ORM.jpg
material: #40 rgb[255,207,0]  castShadow doulbeSide
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/Fabric032_Color.jpg orm:textures/Fabric032_ORM.jpg
material: #41 rgb[222,0,0]  castShadow doulbeSide
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/Fabric032_Color.jpg orm:textures/Fabric032_ORM.jpg
material: #42 rgb[255,255,204]  castShadow
`  textureLayer: #2565 width/height:1465/0 baseColor:textures/OSM2World_Domposter.png orm:textures/Paper001_ORM.jpg
material: #43 rgb[0,0,255]  castShadow
material: #44 rgb[0,0,0]  castShadow
material: #45 rgb[64,64,64]
`  textureLayer: #8600 width/height:2969/0 baseColor:textures/Railway_Color.png orm:
material: #46 rgb[188,198,204]  castShadow
material: #47 rgb[192,192,192]  castShadow
material: #48 rgb[0,128,0]  castShadow
`  textureLayer: #1500 width/height:1500/0 baseColor:textures/Grass-2048.JPG orm:
material: #49 rgb[192,192,192]  castShadow
material: #50 rgb[0,255,0]
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/Ground003_Color.jpg orm:textures/Ground003_ORM.jpg
material: #51 rgb[77,77,77]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/Asphalt010_Color.png orm:textures/Asphalt010_ORM.jpg
`  textureLayer: #5000 width/height:1200/1 baseColor:textures/road_arrow_right.png orm:
material: #52 rgb[77,77,77]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/Asphalt010_Color.png orm:textures/Asphalt010_ORM.jpg
`  textureLayer: #5000 width/height:480/1 baseColor:textures/road_arrow_through.png orm:
material: #53 rgb[128,128,128]  castShadow
material: #54 rgb[128,128,128]  castShadow
material: #55 rgb[188,198,204]  castShadow
material: #56 rgb[77,77,77]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/Asphalt010_Color.png orm:textures/Asphalt010_ORM.jpg
`  textureLayer: #5000 width/height:1200/1 baseColor:textures/road_arrow_right_left.png orm:
material: #57 rgb[188,198,204] transparency:Cutout castShadow doulbeSide
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/Fence006_Color.png orm:textures/Fence006_ORM.jpg
material: #58 rgb[255,255,255]  castShadow
`  textureLayer: #1200 width/height:600/0 baseColor:textures/Concrete034_Color.jpg orm:textures/Concrete034_ORM.jpg
material: #59 rgb[77,77,77]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/Asphalt010_Color.png orm:textures/Asphalt010_ORM.jpg
`  textureLayer: #5000 width/height:960/1 baseColor:textures/road_arrow_through_right.png orm:
material: #60 rgb[255,0,0]  castShadow
`  textureLayer: #2000 width/height:2000/0 baseColor:textures/Bricks008_Color.jpg orm:textures/Bricks008_ORM.jpg
material: #61 rgb[255,255,0]  castShadow
`  textureLayer: #2000 width/height:2000/0 baseColor:textures/Bricks008_Color.jpg orm:textures/Bricks008_ORM.jpg
material: #62 rgb[77,77,77]  castShadow
`  textureLayer: #1500 width/height:1500/0 baseColor:textures/PavingStones033_Color.jpg orm:textures/PavingStones033_ORM.jpg
`  textureLayer: #5000 width/height:1200/1 baseColor:textures/road_arrow_right.png orm:
material: #63 rgb[255,255,255]
material: #64 rgb[77,128,77]
`  textureLayer: #1600 width/height:1600/0 baseColor:textures/PavingStones053_Color.jpg orm:textures/PavingStones053_ORM.jpg
material: #65 rgb[51,0,0]  castShadow
`  textureLayer: #800 width/height:2100/0 baseColor:textures/DE19F1FreisingDoor00005_small.png orm:
material: #66 rgb[230,230,230]  castShadow
`  textureLayer: #72000 width/height:72000/0 baseColor:textures/Facade005_Color.jpg orm:textures/Facade005_ORM.jpg
material: #67 rgb[240,240,240]  castShadow
`  textureLayer: #1200 width/height:600/0 baseColor:textures/Concrete034_Color.jpg orm:textures/Concrete034_ORM.jpg
material: #68 rgb[100,100,100]  castShadow
`  textureLayer: #1200 width/height:600/0 baseColor:textures/Concrete034_Color.jpg orm:textures/Concrete034_ORM.jpg
material: #69 rgb[240,240,240]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #70 rgb[204,0,0]
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/RoofingTiles010_Color.jpg orm:textures/RoofingTiles010_ORM.jpg
material: #71 rgb[240,240,240]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #72 rgb[255,230,140]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #73 rgb[255,230,140]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #74 rgb[100,100,100]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #75 rgb[0,0,255]
`  textureLayer: #100000 width/height:100000/0 baseColor:textures/P1140080E_Water.jpg orm:
material: #76 rgb[255,225,150]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #77 rgb[255,225,150]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #78 rgb[100,50,200]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/RoofingTiles010_Color.jpg orm:textures/RoofingTiles010_ORM.jpg
material: #79 rgb[204,0,0]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/RoofingTiles010_Color.jpg orm:textures/RoofingTiles010_ORM.jpg
material: #80 rgb[76,76,76]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/RoofingTiles010_Color.jpg orm:textures/RoofingTiles010_ORM.jpg
material: #81 rgb[190,255,190]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #82 rgb[76,76,76]  castShadow
`  textureLayer: #1200 width/height:600/0 baseColor:textures/Concrete034_Color.jpg orm:textures/Concrete034_ORM.jpg
material: #83 rgb[140,140,140]  castShadow
`  textureLayer: #1200 width/height:600/0 baseColor:textures/Concrete034_Color.jpg orm:textures/Concrete034_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #84 rgb[120,110,110]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/RoofingTiles010_Color.jpg orm:textures/RoofingTiles010_ORM.jpg
material: #85 rgb[170,130,80]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #86 rgb[170,130,80]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #87 rgb[255,255,175]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #88 rgb[255,255,175]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #89 rgb[100,100,100]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/RoofingTiles010_Color.jpg orm:textures/RoofingTiles010_ORM.jpg
material: #90 rgb[255,255,255]  castShadow
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/DE20F1GarageDoor00001.jpg orm:
material: #91 rgb[190,190,255]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #92 rgb[100,100,100]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #93 rgb[255,190,190]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #94 rgb[0,0,255]  castShadow
material: #95 rgb[204,0,0]  castShadow
`  textureLayer: #1200 width/height:600/0 baseColor:textures/Concrete034_Color.jpg orm:textures/Concrete034_ORM.jpg
material: #96 rgb[100,50,200]  castShadow
`  textureLayer: #1200 width/height:600/0 baseColor:textures/Concrete034_Color.jpg orm:textures/Concrete034_ORM.jpg
material: #97 rgb[76,76,76]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #98 rgb[255,190,190]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #99 rgb[150,200,130]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/RoofingTiles010_Color.jpg orm:textures/RoofingTiles010_ORM.jpg
material: #100 rgb[225,175,225]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #101 rgb[150,200,130]  castShadow
`  textureLayer: #1200 width/height:600/0 baseColor:textures/Concrete034_Color.jpg orm:textures/Concrete034_ORM.jpg
material: #102 rgb[240,240,240]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/RoofingTiles010_Color.jpg orm:textures/RoofingTiles010_ORM.jpg
material: #103 rgb[225,175,225]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #104 rgb[195,219,185]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/RoofingTiles010_Color.jpg orm:textures/RoofingTiles010_ORM.jpg
material: #105 rgb[249,252,241]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #106 rgb[162,98,70]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/RoofingTiles010_Color.jpg orm:textures/RoofingTiles010_ORM.jpg
material: #107 rgb[184,184,184]  castShadow
material: #108 rgb[204,0,0]
`  textureLayer: #500 width/height:500/0 baseColor:textures/clay.jpg orm:
`  textureLayer: #1000 width/height:1000/1 baseColor:textures/pitchTennis.png orm:
material: #109 rgb[255,255,255] transparency:Cutout castShadow doulbeSide
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/tennis_net.png orm:
material: #110 rgb[230,230,230]  castShadow
`  textureLayer: #72000 width/height:72000/0 baseColor:textures/Facade005_Color.jpg orm:textures/Facade005_ORM.jpg
material: #111 rgb[190,255,190]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #112 rgb[245,245,220]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #113 rgb[245,245,220]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #114 rgb[120,110,110]  castShadow
`  textureLayer: #1200 width/height:600/0 baseColor:textures/Concrete034_Color.jpg orm:textures/Concrete034_ORM.jpg
material: #115 rgb[255,255,175]  castShadow
`  textureLayer: #1200 width/height:600/0 baseColor:textures/Concrete034_Color.jpg orm:textures/Concrete034_ORM.jpg
material: #116 rgb[255,255,255]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #117 rgb[255,255,255]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #118 rgb[76,76,76]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #119 rgb[255,128,64]  castShadow
`  textureLayer: #1400 width/height:1400/0 baseColor:textures/Bricks029_Color.jpg orm:textures/Bricks029_ORM.jpg
material: #120 rgb[190,190,255]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #121 rgb[255,170,170]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #122 rgb[255,170,170]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #123 rgb[144,238,144]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #124 rgb[144,238,144]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #125 rgb[250,250,210]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #126 rgb[176,196,222]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/RoofingTiles010_Color.jpg orm:textures/RoofingTiles010_ORM.jpg
material: #127 rgb[0,0,255]
`  textureLayer: #1800 width/height:2000/0 baseColor:textures/SolarPanel001_Color.jpg orm:textures/SolarPanel001_ORM.jpg
material: #128 rgb[241,233,80]
`  textureLayer: #500 width/height:500/0 baseColor:textures/Ground033_Color.jpg orm:textures/Ground033_ORM.jpg
material: #129 rgb[0,128,0]  castShadow
`  textureLayer: #800 width/height:2100/0 baseColor:textures/DE19F1FreisingDoor00005_small.png orm:
material: #130 rgb[173,216,230]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #131 rgb[255,225,150]  castShadow
`  textureLayer: #1200 width/height:600/0 baseColor:textures/Concrete034_Color.jpg orm:textures/Concrete034_ORM.jpg
material: #132 rgb[190,255,190]  castShadow
`  textureLayer: #1200 width/height:600/0 baseColor:textures/Concrete034_Color.jpg orm:textures/Concrete034_ORM.jpg
material: #133 rgb[166,216,237]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #134 rgb[166,216,237]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #135 rgb[0,204,0]
`  textureLayer: #1500 width/height:1500/0 baseColor:textures/Grass-2048.JPG orm:
`  textureLayer: #1000 width/height:1000/1 baseColor:textures/pitchSoccer.png orm:
material: #136 rgb[255,213,243]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #137 rgb[158,129,115]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/RoofingTiles010_Color.jpg orm:textures/RoofingTiles010_ORM.jpg
material: #138 rgb[167,225,255]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #139 rgb[180,118,69]  castShadow
`  textureLayer: #1200 width/height:1200/0 baseColor:textures/RoofingTiles010_Color.jpg orm:textures/RoofingTiles010_ORM.jpg
material: #140 rgb[128,0,128]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
`  textureLayer: #2500 width/height:2500/1 baseColor:textures/MarekCompositeWall00001_transparent.png orm:
material: #141 rgb[128,0,128]  castShadow
`  textureLayer: #2500 width/height:2500/0 baseColor:textures/Plaster002_Color.jpg orm:textures/Plaster002_ORM.jpg
material: #142 rgb[241,233,80]
`  textureLayer: #500 width/height:500/0 baseColor:textures/Ground033_Color.jpg orm:textures/Ground033_ORM.jpg
`  textureLayer: #1000 width/height:1000/1 baseColor:textures/pitchBeachvolleyball.png orm:
material: #143 rgb[0,128,0] transparency:BINARY castShadow doulbeSide
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/arbaro_tree_coniferous.png orm:
material: #144 rgb[0,128,0] transparency:BINARY castShadow doulbeSide
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/arbaro_tree_broad_leaved.png orm:
material: #145 rgb[255,255,255] transparency:BINARY castShadow
`  textureLayer: #840 width/height:840/0 baseColor:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAlgAAAJYCAYAAAC+ZpjcAACAAElEQVR4Xm orm:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAlgAAAJYCAYAAAC+ZpjcAACAAElEQVR4Xo
material: #146 rgb[255,255,255] transparency:BINARY castShadow
`  textureLayer: #840 width/height:840/0 baseColor:textures/306.svg orm:
material: #147 rgb[255,255,255] transparency:BINARY castShadow
`  textureLayer: #840 width/height:840/0 baseColor:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAA08AAALoCAYAAACgUM74AACAAElEQVR4Xl orm:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAA08AAALoCAYAAACgUM74AACAAElEQVR4Xn
material: #148 rgb[255,255,255] transparency:BINARY castShadow
`  textureLayer: #840 width/height:840/0 baseColor:textures/205.svg orm:
material: #149 rgb[255,255,255] transparency:BINARY castShadow
`  textureLayer: #840 width/height:840/0 baseColor:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAlkAAAJZCAYAAACa+CBHAACAAElEQVR4Xm orm:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAlkAAAJZCAYAAACa+CBHAACAAElEQVR4Xo
material: #150 rgb[255,255,255] transparency:BINARY castShadow
`  textureLayer: #840 width/height:840/0 baseColor:textures/283-31.svg orm:
material: #151 rgb[255,255,255] transparency:BINARY castShadow
`  textureLayer: #840 width/height:840/0 baseColor:textures/283-30.svg orm:
material: #152 rgb[255,255,255] transparency:BINARY castShadow
`  textureLayer: #840 width/height:840/0 baseColor:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAlkAAAJZCAYAAACa+CBHAACAAElEQVR4Xm orm:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAlkAAAJZCAYAAACa+CBHAACAAElEQVR4Xo
material: #153 rgb[255,255,255]  castShadow
material: #154 rgb[255,255,255]  castShadow
material: #155 rgb[255,255,255] transparency:BINARY castShadow
`  textureLayer: #900 width/height:600/0 baseColor:textures/310.svg orm:
`  textureLayer: #900 width/height:600/1 baseColor:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAbYAAAEkCAYAAABDpxQ4AAANFklEQVR4Xu orm:
material: #156 rgb[255,255,255] transparency:BINARY castShadow
`  textureLayer: #900 width/height:600/0 baseColor:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAA4QAAAJYCAYAAAA6xSjbAACAAElEQVR4Xm orm:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAA4QAAAJYCAYAAAA6xSjbAACAAElEQVR4Xn
material: #157 rgb[255,255,255]  castShadow
material: #158 rgb[255,255,255]  castShadow
material: #159 rgb[255,255,255] transparency:BINARY castShadow
`  textureLayer: #900 width/height:900/0 baseColor:textures/206.svg orm:
material: #160 rgb[255,255,255] transparency:BINARY castShadow
`  textureLayer: #900 width/height:900/0 baseColor:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAA4UAAAOFCAYAAADd0EegAACAAElEQVR4Xm orm:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAA4UAAAOFCAYAAADd0EegAACAAElEQVR4Xn
material: #161 rgb[255,255,255] transparency:BINARY castShadow
`  textureLayer: #840 width/height:840/0 baseColor:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAlkAAAJZCAYAAACa+CBHAACAAElEQVR4Xm orm:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAlkAAAJZCAYAAACa+CBHAACAAElEQVR4Xo
material: #162 rgb[255,255,255] transparency:BINARY castShadow
`  textureLayer: #840 width/height:840/0 baseColor:textures/283-10.svg orm:
material: #163 rgb[0,128,0] transparency:Cutout castShadow doulbeSide
`  textureLayer: #1000 width/height:1000/0 baseColor:textures/apfelbaum_alpha.png orm:


**************/
