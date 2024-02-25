## OSMeta
**OpenStreetMap-Metaverse**


* Programming langue: Rust / wasm
* Render engine: Bevy and others (also wgpu)
* Runtimes: Native desktop. Intended: iOS, Android and Web-App (using wasm)


The main goal of this project is, to offer a Rust crate to render the OSM data in 3D.
An OSM vector map like maplibre-rs could add it like Googles street-view.

Actually the new GLB 3D tiles from OSM2World are used.
There are several other OSM 3D renderer and their ideas could be integrated too.

Not only a bird view but also a street-level view is provided.
So the crate could be used to create a wheel-chair or a hot air balloon simulation.
As Bevy is a game engine, even gamifications like car races are easy to build.

The “Meta” in “OSMeta” is partly a joke against the intended “free and open” Metaverse of “Facebook".
But a multiuser mode will get added. Also the Metaverse features, as far as Bevy engine offers them, to immerse OSM as a virtual world.


*This texts are written in English by a German, so please gratefully correct my errors and nonsense. -karlos-*




## The long History:

After the Big Bang, hydrogen collapsed to ignite the first generation of stars.
They soon ended in supernovae to breed “metallic" matter for planet building.
After cooling down live and mankind raised and build roads.

A student in London could not find map data and started to collect them and made OpenStreetMap.
Pokémon used the data and generated 3D views.
After creating Science Fiction 3D things, I (karlos) thought:
That’s not wizarding, I could do it too and have some gamification for OSM.
So I started “OSM-GO”. See www.osm.org/wiki/osm_go and https://www.openstreetmap.org/user/-karlos-/diary 
Just mimicking Pokémon extended to show all OSM tagging, but not a realistic visualisation like F4map.com.

I got in contact with Tobias and we started a Web-Frontend for https://wiki.openstreetmap.org/wiki/OSM2World.
The render Engine changed from Three-JS to Babylon-JS, JavaScript to TypeScript.
The frontend is not gone public yet, because of some missing features, errors and providing more but D-A-CH.
Reasendly we changed from a propertarry tile file format ot standard GLB.

Meanwhile I got enthusiastic about WASM and Rust too. Just to challenge myself, I transcoded parts of the frontend.
The used render engine changed from “tree-d” to “rend3” and now Bevy it is.
As there is a Maplibre-RS now, this project could be used there to.

The future idea is, to make a render engine agnostic core crate.
You call it for a GPS coordinate and by a wrapper for the actually used renderer,
it will load GLB tiles, needed to fill the camera view (frustrum).
This will be done by a Rust module and later, the core-crate. The “mod.rs” serves as the wrapper.
Next there may be crates for different renderer.
Only the control or the gamification is added by the user to get an application.

--------------------------

There is an not up to date http://www.osmgo.org/OSMeta/demo.mov
You may be able to run it here: https://derkarlos.github.io/OSMeta/

--------------------------

## Concepts:

Todo
