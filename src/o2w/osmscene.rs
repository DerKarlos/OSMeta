use super::viewer::*;
// import { PbfClient, ViewTile } from "./pbfclient.js"
use super::geopos::{ GeoPos };
use super::geoview::*;
use super::cameraview::*;
//use super::geoview::*;
use super::utils::
{
    //TileName, TileSize, 
    ScenePos, PI, LAT_FAKT // pbf_Zoom,
    //LayerStep, LoadStep, ViewStep, rad, degr, phytagoras,
    //DeviceLimit, default_height
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

// #[derive(Clone)]
pub struct OsmScene {

    /** The BabylonJS scene handler */
    //pub scene: BABYLON.Scene,

    /** OSM viewer showing self scene */
    //pub viewer: Viewer,

    /** Tile-name(x/y) of the first loaded pbf-tile */
    //pub firstPbfTile_Name: TileName,

    /** Tile-name(x/y) of the nord west view-tile of the first loaded pbf-tile */
    //pub firstViewTile_Name: TileName,

    /** Size of a pbf-tile in meter (only x/z used). Depends on the latitude. */
    //pub pbfSize: TileSize,

    /** Size of a view tile in meter (x is about equal to z). Depends on the latitude. */
    //pub viewSize: f32,

    /** Calculates the pbf-tile corner-to-center offset */
    pub pbf_corner_to_center: glam::Vec3, // BABYLON.Vector3;

    /** Geo-location of the center of the first loaded pbf-tile, and the center of the scene */
    pub null_geo_pos: GeoPos,
    /** geo-location of the nord west corner of the first loaded pbf-tile */
    pub null_corner_geo_pos: GeoPos,

    /** 2D Array of pbf-tiles, "the Map" */
    //private _viewTiles: ViewTile[][],

    /** the root-mesh of the osmScene to scale it i.e. on a table in a VR scene */
    // pub webARroot: BABYLON.Mesh,
    /** The scale of the scene */
    //pub webARscale = 1./36.,

    /** the root-mesh of the ground to move it down if the camera is far abowe */
    // pub groundRoot: BABYLON.Mesh,

    /** last string, that was set to the browser URL */
    //  lastUrl: string = "",

    /** count of loading workers to limit the running workers */
    //pub pbfCount: f32, // = 0,
    /** count of requested view tiles to limit the requests */
    //pub requestCount: f32, // = 0,  // todo: how to check if the load into the GPU is done? Ask BJS forum
    /** count of drawing view tiles, actually limited to 1 */
    //pub drawCount: f32, // = 0,

    /** part of a second gone by, used to limit the settings of the browser URL */
    //    secPart: f32, // = 0,

    camera_view: Vec<CameraView>,


    /* * First camera position, used as debug center in the worker / [[PbfTile]] */
    //pub startScenePos: ScenePos,

    /* * User interface step state. 0: init/done 1:requesting pbf file 2:loading 4:rendering */
    //pub ui_step: f32, // = 0,

    /* * Array of all pbf-tiles/-clients */
    //private pbfClients: PbfClient[] = [];
    /* * List of all Y/nord-south directories on the pbf-tile server */
    //pub scannedY: string[] = [];
}


impl OsmScene {



    /**
     * OsmScene constructor: Start loading the pbf file/tile.
     * @param geoView  Geo position and camea view to start the scene with
     * @param viewer  OSM scene handler
     */
    pub fn new (geo_view: GeoView, _viewer: &Viewer) -> OsmScene {

        let mut osm_scene = OsmScene{

    /*****
        self.viewer = viewer;
        self.scene = viewer.scene;
      //self.webARroot = new BABYLON.Mesh("webARroot", self.scene);
        self._viewTiles = [];


        // use URL parameter? try it
        if (self.viewer.parameter.useUrl == 1 || self.viewer.parameter.useUrl == 3) {
            self.readUrl(geoView); // parameter geoView is a reference and writable!
            if (viewer.deviceLimit == DeviceLimit.WEBXR) {
                console.log("DeviceLimit.WEBXR")
                geoView.height = default_height;
                geoView.view = 0; // horizontal
                geoView.radius = 0.2;
            }
        }

        geoView.storeCookie("start");

        // calculate tile-names(x/y), containing the CPU-Scene 0/0 in its center
        self.firstPbfTile_Name = geoView.geoPos.calcTile_Name(pbf_Zoom);
        self.firstViewTile_Name = self.firstPbfTile_Name.scale(self.viewer.factZoom).add(new BABYLON.Vector2(self.viewer.factZoom / 2, self.viewer.factZoom / 2));
        // pbf-tile 1/2 scaled would be 8/16 and added 12/20 i.e.  Adding is to get the first tiel next to the GPU 0 point

        // Get the first loaded pbf/view-tile(corner) geoPos
        // and the next (+1x/y) pbf/view-tile(corner) geoPos -- The next pbf-tile is the end of the first one
        let fstGeoPos = self.calcCornerGeoPosFrom_Name(self.firstPbfTile_Name, pbf_Zoom);
        let oneGeoPos = self.calcCornerGeoPosFrom_Name(self.firstPbfTile_Name.add(BABYLON.Vector2.One()), pbf_Zoom);
        // calcs the geoPos delta (degrees) and trans-calcs it to meters:
        self.pbfSize = oneGeoPos.calcMetersToOtherGeoPos(fstGeoPos);
        // _x: 3232.2079333866873   >  _z: 3231.278959942741  should be equal? calculaton not exactly???

        // _Name add One => +x/+y => +x/-z meter  because:
        // _Name y+1 means more south
        //           means less degrees
        //           means more to the eye
        //           means less z (because z to the eye is negative and to the back is positive in BJS)
        // to correct self:
        self.pbfSize.z *= -1; // todo: let it negative as needed for ..toCenter

        // raw view tile size for some calculations
        self.viewSize = self.pbfSize.x / self.viewer.factZoom;
        */

        pbf_corner_to_center: glam::Vec3::ZERO,

        // Merkator center of pbf-tile = pbf zoom+1 tile corner = pbf name*2+1 and pbf zoom+1
        null_geo_pos: GeoPos::new(),
        // todo: remove and use null_corner_geo_pos

        camera_view: Vec::new(), // geoView.to_camera_view(self),

        // lat: 48.545707582202596 lon: 13.491210938407548
        null_corner_geo_pos: GeoPos::new(),


        /*
        self.groundRoot = new BABYLON.Mesh("groundRoot", self.scene); // ddd scene needed ???
        self.groundRoot.parent = self.webARroot;

        self.startScenePos = cameraView.scenePos;
        self.viewer.setCameraView(cameraView);
        //self.scene.getEngine().onBeginFrameObservable.add(self.render().bind(self));
*/

        };

        // From the nord-west / upper-left corner (+z / -x) ...
        // ... to the center (0 / 0) by adding self delta (-z / +x)
        // Example First pbfTile: 0/0 -16xx/+16zz = -16xx/+16zz
        osm_scene.pbf_corner_to_center = glam::Vec3::new (
            /*-osm_scene.pbfSize.x / . */ 2., 0.,
            /* osm_scene.pbfSize.z / . */ 2.);

        // self.null_corner_geo_pos = fstGeoPos;
        

        //osm_scene.null_geo_pos = osm_scene.calcCornerGeoPosFrom_Name(osm_scene.firstPbfTile_Name.scale(2).add(BABYLON.Vector2.One()), pbf_Zoom + 1);
        osm_scene.camera_view.push(geo_view.to_camera_view(&osm_scene));

        osm_scene

    } // new / constructor




    /**
     * If there are parameters in the URL, overwrite the geo view, given by the user code
     * @param  geoView reference, writable! to the lib users geo view
     * . /
    readUrl(geoView: GeoView): void {

        // overwrite position and view - if set in the url
        let lat = self.viewer.getUrlParameter('lat');
        let lon = self.viewer.getUrlParameter('lon');
        if (lat && lon) {

            // calculate the camera position in the scene by lat/lon and ele
            geoView.geoPos = new GeoPos(lat * 1, lon * 1);

            let ele = self.viewer.getUrlParameter('ele');
            if (ele) geoView.height = ele * 1;

            // if a parameter is in the url, set the rotation and radius
            let dirr = self.viewer.getUrlParameter('dir');
            let view = self.viewer.getUrlParameter('view');
            let dist = self.viewer.getUrlParameter('dist');
            let fov = self.viewer.getUrlParameter('fov');
            if (dirr.length > 0) geoView.dir    = dirr * 1; // alpha
            if (view.length > 0) geoView.view   = view * 1; // beta
            if (dist.length > 0) geoView.radius = dist * 1;
            if ( fov.length > 0) geoView.fov    = fov * 1;

            // todo: to viewer or control?
            // if(fly   = GET_ParD("fly",  4)) document.getElementById('fly'  ).value   = fly
            // if(opt   = GET_ParD("opt",  2)) document.getElementById('opt'  ).value   = opt
            // if(tiles = GET_ParD("tiles",0)) document.getElementById('tiles').value   = tiles
            // if(sha   = GET_ParD("sha",  0)) document.getElementById('sha'  ).checked = true
            // if(card  = GET_ParD("card", 0)) document.getElementById('card' ).checked = true
            // if(filt  = GET_ParD("f",    0)) document.getElementById('f'    ).value   = filt
            // if(user  = GET_ParD("user", 0)) document.getElementById('user' ).value   = user

        }

    }


    / **
     * TODO: Starts the WebXR mode: no sky dome, change to a smal ground mesh
     * /



    / . **
     * Update the visible OSM-objects / LoD-Level / tiles:
     * @param dSec Time delta in secounds
     * . /
    render(dSec: f32): void {
        // console.log("OsmScene.render");
        self.viewer.animations.render(dSec);

        if (self.viewer.parameter.useUrl == 3 || self.viewer.parameter.useUrl == 3) {
            // update the url any second (if changes exist)
            self.secPart += dSec;  //??? only if user is active
            if (self.secPart > 1.0) {
                self.secPart -= 1.0
                self.updateUrl();
            }
        }


    }


    / . **
     * Create a new view tile and add it to the 2D array of view tiles (client side)
     * at the given OSM x/y tile position (Name)
     * @param x  west/east osm-coordinate of the tile
     * @param y  nord/south osm-coordinate of the tile
     * @pbfClient PbfClient  if the "parent" is already existing
     * @returns the new ViewTile
     * . /
    addViewTile(x: f32, y: f32, pbfClient?: PbfClient): ViewTile {  // in self case, x y is better than tile_Name

        if (!pbfClient) pbfClient = undefined;

        let viewTile = new ViewTile(new Tile_Name(x, y), pbfClient, self);

        if (!self._viewTiles[x])
            self._viewTiles[x] = [];
        self._viewTiles[x][y] = viewTile;

        return viewTile
    }


    / **
     * Get the view tile by a given OSM x/y tile position (Name)  (Clinet side)
     * @param tile_Name  x/y positin of the tile
     * @return existing view tile or undefined
     * /
    getViewTile(tile_Name: Tile_Name): ViewTile | undefined {
        if (!self._viewTiles[tile_Name.x]) return undefined;
        if (!self._viewTiles[tile_Name.x][tile_Name.y]) return undefined;
        return self._viewTiles[tile_Name.x][tile_Name.y];
    }



    / **
     * Calculate the geo-location of a tile (nord west edge) by self tile-name(x/y)
     * @param tile_Name  tile-name(x/y) of a tile to calc the geo-location from
     * @param zoom  Zoom level on the OSM tile-name(x/y) system
     * @return a lat,lon geo position (GPS)
     * /
    pub calcCornerGeoPosFrom_Name(tile_Name: Tile_Name, zoom: f32): GeoPos {
        let n = Math.PI - 2 * Math.PI * tile_Name.y / Math.pow(2, zoom);
        let lat = 180 / Math.PI * Math.atan(0.5 * (Math.exp(n) - Math.exp(-n)));
        let lon = tile_Name.x / Math.pow(2, zoom) * 360 - 180;
        return new GeoPos(lat, lon);
    }

    **/

    /**
     * calculate the GPS position from a position in the scene
     * @param scenePos position in the scene
     * @return GeoPos position on Earth
     **/
    pub fn calc_geo_pos_from_scene_pos(&self, scene_pos: ScenePos) -> GeoPos {
        let lat = scene_pos.z /  LAT_FAKT + self.null_geo_pos.lat;  // -z   to nord = more z =
        let lon = scene_pos.x / (LAT_FAKT * ((lat / 180. * PI).abs() ).cos()) + self.null_geo_pos.lon;
        GeoPos{lat, lon}
    }


    /******

    / **
     * take the cameras position and orientation,
     * create url parameters and set them in the browsers url input line
     * if the values are different and some time has gone since the last update.
     * /
    private updateUrl(): void {

        let cameraView = self.viewer.getCameraView();
        let geoView = cameraView.toGeoView(self);

        //      let camera = self.scene.activeCamera as BABYLON.ArcRotateCamera;
        //      if (!camera.target) return;

        //      if (camera.alpha > +Math.PI) camera.alpha -= (2 * Math.PI)
        //      if (camera.alpha < -Math.PI) camera.alpha += (2 * Math.PI)

        //      let geoPos = self.calcGeoPosFromScenePos(camera.target)

        let dist: string = geoView.radius.toFixed(geoView.radius < 10 ? 1 : 0);
        let camera = self.scene.activeCamera as BABYLON.ArcRotateCamera;

        let newUrl = '' / *"http://www.OSMgo.org/o2w/"* /   // only relative URL!
            + '?lat=' + geoView.geoPos.lat.toFixed(7)
            + '&lon=' + geoView.geoPos.lon.toFixed(7)
            + '&ele=' + geoView.height.toFixed(1)
            + '&dir=' + geoView.dir.toFixed(0)
            + '&view=' + geoView.view.toFixed(0)  // parseInt(
            + '&dist=' + dist // distance camera to view point
            + '&fov=' + Math.floor(degr(camera.fov * 10) / 10) // feeld of view (camera zoom)
        if (self.viewer.parameter.test) // if test is used, use it in the URL to
            newUrl += '&t=' + self.viewer.parameter.test
        let stateObj = { bar: "OSM2WORLDposbar" };
        if (self.lastUrl != newUrl) {
            self.lastUrl = newUrl;
            window.history.replaceState(stateObj, "title", newUrl)
        }
    }



    / **
     * Calculates the view tile name, containing the given scene pos
     * @param scenePos  position inside the to be calcualted view tile
     * @return the name of the calcualted view tile
     * /
    calcViewTileNameAtPos(scenePos: ScenePos): Tile_Name {

        let geoPos = self.calcGeoPosFromScenePos(scenePos); // including -z
        let viewTile_Name = geoPos.calcTile_Name(self.viewer.viewZoom);

        / ** / , equator?:boolean
        let x = self.view-Size.x;
        let z = self.view-Size.z;
        if(equator) {
            x = self.view-SizeEquator.x / factZoom;
            z = self.view-SizeEquator.z / factZoom;
        }

        // Calc Sub-Tile from Meters
        // First calc position, relativ to pbf grid corner. Divided by PbfTileSize makes the 012.. index-offset for viewTile
        let partXindex = Math.floor((self.pbf-Size.x / 2 + scenePos.x) / x);
        let partYindex = Math.floor((self.pbf-Size.z / 2 - scenePos.z) / z); // NOT: +y=-z
        // worldIndex = 0/0 startIndex + subTileOffest
        let alt = self.firstViewTile_Name.add(new Tile_Name(partXindex, partYindex));
        // console.log("equal?:", alt,viewTile_Name);
        / ** /
        return viewTile_Name;

    }


    / **
     * Called by a message from the server/worker: All materials are processed and all textures are loaded.
     *
     * The last / actual loading pbf-tile is calculated and gets done. A next load may start now.
     * /
    materialsDone(): void {
        let last = self.pbfClients.length-1;
        let pbfClient = self.pbfClients[last];
        pbfClient.materialsDone();
    }


    / **
     * Dismiss scene and all its pbf-tiles inluding the worker. Called if the HTML page is unloaded.
     * /
    dismiss() {
        self.pbfClients.forEach(function(pbfClient: PbfClient) {
            pbfClient.dismiss(3);
        })
    }

    / **
     * creates a new [[PbfTile]] instance and requests the data loading
     * @param viewTile_Name  The view tile, containing the pbf-tile
     * /
    requestPbfTile(viewTile: ViewTile): void {
        if (self.viewer.pbfFileByte >= self.viewer.pbfFileMax) return;
        if (self.pbfCount > 0) return; // a worker is still loading, don't do it twice the same time???
        self.pbfCount++;
        // view and pbfTile are not yet requested to load: start worker
        let pbfTile_Name = viewTile.tile_Name.scale(1 / self.viewer.factZoom).floor();
        self.pbfClients.push(new PbfClient(&self, pbfTile_Name));
        return;
    };


    / **
     * Check if a view tile is ready to be requested for visualisation
     * and how importend the draw is. Importend is:
     * * if the tile is close to the spot, the camera is pointing to
     * * if the tile is in the direction, the camera is looking to
     * * if the importand layers of the tile are not done.
     * @param x  offest to the view tile name of the camera
     * @param y  offest to the view tile name of the camera
     * @param target  the position in the scene, the camera ist focussing on
     * @param camAngle  direction, the camera is looking to
     * @param focusTile_Name  The X/Y of the focus name, to calc the neighbours
     * @return drawing importance score the tile got, the more the better
     * / 
     getViewTileScore(x: f32, y: f32, focusPos: ScenePos, camAngle: f32, focusViewTile_Name: Tile_Name): f32 {

            // cam-first=viewAndPbfCorner + viewSize/2 = viewCenterAndPbfCorner - factZoom.. = viewAndPbfCenter
            let tilePosX = (focusViewTile_Name.x - self.firstViewTile_Name.x + 0.5 + x) * +self.viewSize;
            let tilePosZ = (focusViewTile_Name.y - self.firstViewTile_Name.y + 0.5 + y) * -self.viewSize; // +y=-z    ATTENTION!  A positive/South index-delta is a negative/Front GPU-position!
            let distX = (focusPos.x - tilePosX);
            let distZ = (focusPos.z - tilePosZ);
            // todo:? calc pos via geoPos?

            // calc distance score
            let dist = phytagoras(distX, distZ);
            let distScore = 1 / (dist / self.viewSize)

            // calc angle and angle-score
            let tileAngle = Math.atan2(distZ, distX);
            let angle = -camAngle + tileAngle;

            //console.log( "-",
            //    Math.floor(degr(camAngle)),"+",
            //    Math.floor(degr(tileAngle)),"=",
            //    Math.floor(degr(angle)),
            //    x,y,distX,distZ,tilePosX,tilePosZ,
            //    " ");

            let max1 = rad(180);
            let max2 = rad(360);
            while (angle > +max1) angle -= max2;
            while (angle < -max1) angle += max2;

            angle = Math.abs(angle);
            // isInFrustum or self:  ???
            // if (angle > rad(23)/2*1.5 && distScore < 0.5) // camera.fov/2? * high-width-ratio
            //     return -1; // out of view angle: return negative score
            let angleScore = (1 - Math.abs(angle / max1)) / 2; // 1/2 at 0 degr   -- / 2 to wheigt it less
            //todo: angleScore *= Math.sin(beta)

            // the less is to show, the more relevant is the tile
            let stepScore = 0;

            let viewTile_Name = new Tile_Name(focusViewTile_Name.x + x, focusViewTile_Name.y + y);
            let viewTile = self.getViewTile(viewTile_Name);
            if (viewTile) {
                if (viewTile.viewStep == ViewStep.error) return -5;
                let pbfClient = viewTile.pbfClient;

                if (viewTile.mesh2d && !viewTile.mesh2d.isInFrustum(self.scene.frustumPlanes)) {
                    / ** //mmm load all at last and end the worker: not much memory gets free * /
                    if (pbfClient && pbfClient.loadStep >= LoadStep.loaded)
                        angleScore = 0; // Load only if no others are relevant
                    else
                        / ** /
                        return -7;
                }

                if (!pbfClient) return (angleScore + distScore); // not loaded started
                if (pbfClient.loadStep < LoadStep.loaded) return -2.1; // not loaded dont score

                // If near prefare buildings, if far prefare forests
                if (viewTile.viewStep < ViewStep.near) {
                    if (dist < self.viewer.maxDistLod[2]) // Todo: Far before Near as step ???
                        stepScore = +0.5; // schrauben : prefare to draw near before far, more or less
                }



                if (viewTile.viewStep > ViewStep.far) return -4; // done, dont score
            } else {
                // create view tile before pbf load to calculate if it isInFrustrum (and show the 2D map)
                // and to calculate what pbf load has the hight score!
                viewTile = self.addViewTile(focusViewTile_Name.x + x, focusViewTile_Name.y + y);
                return (angleScore + distScore); // not loaded started
            }

            //  console.log(dx, dy, angle, dist, angleScore, distScore, stepScore);
            return angleScore + distScore + stepScore; // schrauben
        }


        / **
         * Find the most load-relevant view tile
         * @param camera  actual active camea
         * @return  The most relevant view tile, if a tile was found
         * /
        findHighscore(camera: BABYLON.ArcRotateCamera): ViewTile | undefined {

            // the focus, the view tiles get visible first is between the camear focus and its position
            /// focusPos = camera.position.add(camera.position).add(camera.target).divide(new ScenePos(3, 3, 3));
            let focusPos = camera.position.add(camera.target).divide(new ScenePos(2, 2, 2));
            let focusTile_Name = self.calcViewTileNameAtPos(focusPos/ *, true???* /);
            let viewTile = self.getViewTile(focusTile_Name);
            if (!viewTile) viewTile = self.addViewTile(focusTile_Name.x, focusTile_Name.y);  //222

            if (!viewTile.pbfClient) {
                self.requestPbfTile(viewTile); // request tile at camera
                return undefined;
            }

            // scan all visible tiles around the camera
            let scoreView: ViewTile | undefined = undefined;
            let scorePbf: ViewTile | undefined = undefined;
            let scoreMax = 0;
            let pbfMax = 0;

            let angle = (typeof (camera.alpha) === "undefined") ? camera.rotation.y : camera.alpha; // WebXR camera: no alpha

            let max = self.viewer.viewRings;
            if (self.viewer.parameter.viewRings >= 0)
                max = self.viewer.parameter.viewRings;

            for (let x = -max; x <= +max; x++) {
                for (let y = -max; y <= +max; y++) {
                    let score = self.getViewTileScore(x, y, focusPos, angle + rad(0), focusTile_Name);
                    // if the tile is not ready to be requested, the scope will be negative
                    // if the score is the new highscore
                    let name = new Tile_Name(focusTile_Name.x + x, focusTile_Name.y + y);
                    let view = self.getViewTile(name);
                    if (view && view.pbfClient) {
                        if (scoreMax < score) { // remember the tile
                            scoreMax = score;
                            scoreView = view;
                        }
                    } else {
                        if (pbfMax < score) { // remember the tile
                            pbfMax = score;
                            scorePbf = view;
                        }
                    }
                }
            }

            if (scorePbf)
                self.requestPbfTile(scorePbf);

            return scoreView;
        }


        / **
         * Cyclically check if a view tile may be loaded
         * and request the draw data from the service worker.
         * self is done by tree state mashines. Yes tree!
         * * 1) The service worker loads the pbf-tile and prepares the view tiles (self requestTiles)
         * * 2) The view tile is requested form the worker in two steps: "near" and "far" (self requestTiles)
         *      Requesting the "near" layers will replay with some the near- and flat-layers (pbfClinet/Serfer)
         * * 3) A layer request respones to 5 messages: layer parameters and 4 mesh data arrays (pbfClinet/Server)
         *
         * The good thing is: the layer requests are autonoumusly send to the GPU as a BabylonJS mesh.
         * /
        requestTiles(): void {

            if (self.drawCount > 0) // self is visible (during load) not usefull
                return;

            let camera = self.scene.activeCamera as BABYLON.ArcRotateCamera;
            let viewTile = self.findHighscore(camera);
            if (!viewTile)
                return; // no loadable tile found


            let pbfClient = viewTile.pbfClient;
            if (!pbfClient) {
                alert("no pbfClient???")
                // self view tile is not even requested to draw by a worker 222
                //self.requestPbfTile(viewTile);
                return;
            };

            if (pbfClient.layerStep != LayerStep.headerExpected)
                return; // worker is bussy, not ready to use

            let step = viewTile.viewStep; // is the tile (partly) visible?

            if (pbfClient.loadStep >= LoadStep.loaded) { // worker prepared the "near" layers
                if (step == ViewStep.idle) { // the tile is not visible yet
                    //console.log("near",next.x,next.y)
                    pbfClient.requestLodNear(viewTile); // request the "near" layers
                    viewTile.mesh2d.dispose();
                }
            }

            if (pbfClient.loadStep >= LoadStep.loaded) { // worker prepared the "far" layers
                if (step == ViewStep.near) { // the tile is partly visible
                    //console.log("tree ",next.x,next.y,scoreAngle)
                    pbfClient.requestLodFar(viewTile); // request the far layer
                }
                if (step == ViewStep.far && pbfClient.layerStep == LayerStep.headerExpected) { // the tile is totaly loaded
                    //console.log("DONE ",next.x,next.y,scoreAngle)
                    viewTile.viewStep = ViewStep.done; // mark layer as done
                }
            }


        }


    }


******/

}