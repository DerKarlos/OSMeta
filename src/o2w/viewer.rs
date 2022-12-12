
//use bevy::prelude::*;
//bevy::render::camera::ActiveCamera;  

//use super::osm2world.js"
//use super::dataPath, ScenePos, DeviceLimit, pbf_Zoom } from "./utils.js"
use super::osmscene::*;
use super::cameraview::*;
use super::geopos::*;
use super::geoview::*;
//use super::Cars } from "./cars.js"
//use super::Tdmr } from "./tdmr.js"
//use super::Animations } from "./animations.js"
//use super::Environment } from "./environment.js"
//use super::Materials } from "./materials.js"
//use super::Tile_Name } from "./utils.js"

/**
 * Fills an BJS scene with a 3D rendering of an [[OsmScene]], a "place on Earth"
 *
 * The first class, a user of this lib will instantiate is a [[Viewer]].
 * It will "project" a 3D view of an [[OsmScene]] on a canvas on the users html page.
 * The viewer-constructor will need the ID of the canvas.
 * The user needs to create the BABYLON.Engine and the Scene itself and use it as constructor parameter.
 * The returned viewer handler will be used to define the visible [[OsmScene]] etc.
 *
 * The constructor takes some optional values: a Web-url to find the Pbf-Tiles and whether to show shadows,
 * cars and the VR-mode symbol. For debugging there is a distance limit to show OSM objects
 * and an free to use integer test switch.
 */

 pub struct Viewer {
    geo_view:  Option<GeoView>,
    osm_scene: Vec<OsmScene>,

 // camera: Camera3dBundle,
 }

 impl Viewer {

    pub fn new(
    //  camera: Camera3dBundle
    ) -> Viewer {

        Viewer{
            geo_view:  None,
            osm_scene: Vec::new(),
        //  camera,
        }

    }


    /**
     * At the given geo position on Earth, a new Scene will be created or existing will be used
     * @param geoView  gps posiiton and camera view
     */
    pub fn set_geo_view(&mut self, geo_view: Option<GeoView>) { // setgeoview

        if let Some(geo_view) = geo_view {
            self.geo_view = Some(geo_view);
        } else {
            self.geo_view = Some(GeoView::new(
                GeoPos::new() // default passau
            ));
        }

        println!("    setGeoView: {:?}", self.geo_view);

        if self.osm_scene.len() > 0 
        //if let Some(osm_scene) = self.osm_scene
        { // if the scene exists
            let camera_view = self.geo_view.unwrap().to_camera_view(&self.osm_scene[0]);
            self.set_camera_view(camera_view); // just set the camera
        } else { // create the scene. it will set the camera to.
            self.osm_scene.push( OsmScene::new(
                geo_view.unwrap(), self)
            );
        }
    }


    /**
     * set the actual cameras position and direction
     * @param view  instance with all values
     */
    pub fn set_camera_view(&self, _view: CameraView) { // startPosition,startRotation
        //self.camera.transform        (  -1450.028,      4.807,     -0758.637    );

        //self.camera.target = view.scene_pos;
        //self.camera.beta = view.beta; // x view      beta  (radians) the latitudinal  rotation  0=down 90=horizotal 180=up
        //self.camera.alpha = view.alpha; // y direction alpha (radians) the longitudinal rotation  -90=Nord   left/right
        //self.camera.radius = view.radius; // z radius the distance from the target position
        //self.camera.fov = view.fov;
    }

/*

    / **
      * Scene showing an acual part of the OSM-Map, rendered in 3D, visible in the canvas.
      * Multible Scenes may be created and cashed. This is the active scene
      * /
    public osmScene: OsmScene | undefined;

    / ** Opions set by the lib user * /
    public parameter: OsmParameter;

    / ** the cars manager * /
    public cars: Cars;

    / ** the 3D model manger manager * /
    public tdmr: Tdmr;

    / ** the animations manager * /
    public animations: Animations;

    / ** the used control for the moves of the user * /
    public control: any;

    / ** instance of the Environment with Sky etc. * /
    public env: Environment;

    / ** instance of the materials and texture cashe * /
    public materials: Materials;

    / ** Type (node/way/rel.) of the selected osm object * /
    private selectedType: string = "";
    / ** Number of the selected osm object * /
    private selectedNr: string = "";

    / ** list of URL parameters and values * /
    private HTTP_GET_VARS: any = {};

    / ** show more or less view tiles to stay abowe 15 fps i.e. * /
    private distanceFactor: number = 1;

    / ** position of the camera to check, what tiles should be visible and need load * /
    private cameraPosition: BABYLON.Vector3 = BABYLON.Vector3.Zero();
    / ** position of the cameras view target to select the nearest tiles * /
    public cameraTarget: BABYLON.Vector3 = BABYLON.Vector3.Zero();
    / ** canvas hight-width relation to place the compass * /
    public canvasAspect: number = window.innerWidth / window.innerHeight  // >1 if landscape
    / ** Babylon loading screan to show load states * /
    public loadingScreen: BABYLON.DefaultLoadingScreen | undefined;

    / ** Mesh for 2D ground before 2D tiles. Not visible used at the moment * /
    public m2dGround: BABYLON.Mesh;
    / ** Mesh 2D Material of the [[m2dGround]] * /
    public m2dMaterial: BABYLON.StandardMaterial;
    / ** Counter of pbf-file bytes to limit the loading memory * /
    public pbfFileByte: number = 0;
    / ** Loading limit of pbf file bytes * /
    public pbfFileMax: number = 1000 * 1000 * 1000;// passau = 6'x00'000 ?
    / ** Counter of vertex positions to limit the visible objects * /
    public viewPositions: number = 0;
    / ** Loading limit of positions * /
    public viewPositionsMax: number = 5000 * 1000 * 1000;
    / ** Time integrator to limit the tile loading * /
    private dSecTiles: number = 0;
    / ** Limit of the acutal device * /
    public deviceLimit: number = DeviceLimit.NONE;
    / ** No user action: show more tiles * /
    public passive: boolean = false;

    / ** Radius of the visible world * /
    public envRadius: number = 5 * 1000; // 5*1000 radius of the visible scene / "world" / sky dome
    / ** Visible "rings" of view-tiles. May be limited by a test parameter * /
    public viewRings: number;


    / ** OSM zoom unit of the view-tiles  * /
    public viewZoom: number = 16;
    / ** Calculated factor between pbf zoom and view zoom. * /
    public factZoom: number;
    / ** Calculated size of a view-tile, about 400 meter = pbf-tile size 3200 meter / 8 * /
    public aboutViewSize: number;
    / ** Array of distance limits for LoD tiles * /
    public maxDistLod: number[];

    / ** This string contains an array of existing stripes of pbf-tiles in OSM index units * /
    private scannedX: string = "";
    / ** Array of strings, containing existing pbf-tiles in OSM index units * /
    private scannedY: string[] = [];
    / ** Last set browser url * /
    public tileUrl: string = "";

    / ** State-counter for loading ui * /
    public uiStepDone: number = 0;

    / ** For diagnose only: number of draw calls, needed for the meshes of this pbf-tile * /
    public countDrawCalls: number = 0; // wind startindex: 1476 => 25xx  submeshes 4074 => 1140

    / **
     * Viewer constructor
     * @param scene    defines the hosting scene
     * @param options  Options and settings the viewer may get
     * /
    constructor(scene: BABYLON.Scene, parameter?: OsmParameter) {

        this.readUrlParameter();
        if (typeof (parameter) === "undefined") parameter = {
            useUrl: 3, control: true, xrMode: 0, shadow: 1, water: 1, cars: 1,
            distanceMax: 888 * 1000, fpsMin: 15, viewRings: 1, selected: this.defaultObjectSelected.bind(this), test: 1
        };
        // set default and remember the lib users parameter settings
        if (typeof (parameter.useUrl) === "undefined") parameter.useUrl = 3;
        if (typeof (parameter.selected) === "undefined") parameter.selected = this.defaultObjectSelected.bind(this);
        if (typeof (parameter.control) === "undefined") parameter.control = true;
        if (typeof (parameter.xrMode) === "undefined") parameter.xrMode = 0;
        if (typeof (parameter.shadow) === "undefined") parameter.shadow = 1;
        if (typeof (parameter.water) === "undefined") parameter.water = 1;
        if (typeof (parameter.cars) === "undefined") parameter.cars = 0.4;
        if (typeof (parameter.distanceMax) === "undefined") parameter.distanceMax = 888 * 1000; //  888*1000;
        if (typeof (parameter.fpsMin) === "undefined") parameter.fpsMin = 15;
        if (typeof (parameter.viewRings) === "undefined") parameter.viewRings = -1;
        if (typeof (parameter.test) === "undefined") parameter.test = 1;
        this.parameter = parameter;

        if (parameter.test == 1)
            parameter.test = this.getUrlParameter("t", "1") * 1;

        console.log("navigator: ", navigator);

        if (parameter.test == 4
            || navigator.platform == "iPhone"
            || navigator.platform == "MacIntel" && navigator.maxTouchPoints > 0  // not "iPad", "MacIntel" is also for macOS Safari!
        ) {
            console.log("iOS ipadOS iOS ipadOS iOS ipadOS iOS")
            this.deviceLimit = DeviceLimit.IOS;
        }

        if (parameter.test == 5
            || navigator.userAgent.includes("Oculus") // Oculus Quest - Default Browser (Chrome)
            || navigator.userAgent.includes("Mobile VR") // Oculus Quest - Morzilla VR Browser !!! DOES NOT WORK WELL
        ) {
            console.log("WbXR WbXR WbXR WbXR WbXR ")
            this.deviceLimit = DeviceLimit.WEBXR;
            this.tileUrl = / * relativ: * / "./tiles/13/"
        }

        if (parameter.test == 2) {
            parameter.water = 0;
            parameter.shadow = 0;
        }

        if (this.parameter.test == 20) this.parameter.shadow = 0;
        if (this.parameter.test == 21) this.parameter.shadow = 1;
        if (this.parameter.test == 22) this.parameter.shadow = 2;
        if (this.parameter.test == 23) this.parameter.water = 0;
        if (this.parameter.test == 24) {
            this.parameter.shadow = 0;
            this.parameter.water = 0;
        }


        this.tileUrl = / * relativ: * / "./tiles/13/" // NO https, "local" host
        if ((document.location.hostname == "www.osmgo.org" || document.location.hostname == "osmgo.org")
            && this.deviceLimit != DeviceLimit.WEBXR // WebXR needs httpS! Local tiles are used at the moment. Todo: Wait for server with httpS
            || parameter.test == 33) {
            this.tileUrl = "http://95.216.25.217/tiles/13/"
        }
        else console.log("    local HTTPs")

        this.factZoom = Math.pow(2, this.viewZoom - pbf_Zoom);  // 3^2=8   (in zoom/zoom)
        this.aboutViewSize = 26578947 / Math.pow(2, this.viewZoom); // about osmScene.viewSize

        const dist = this.aboutViewSize // envRadius/10  // aboutViewSize; //100 // ca. osmScene.viewSize .x    -- 16=>400 17=200 18=100
            * 1.0//ddd  to debug, default = 1?   0.3-0.28

        // PBF-Data object structur contains:
        //           maxLod: 4 Detail more detail, min distance
        //           minLod: 2        less detail, max distance

        // Lod N is visible up to this distance, not more fare away
        this.maxDistLod = [
            999 * dist, // Lod 0: unlimited far visible,  (trees)
            4.0 * dist, // Lod 1: not used
            2.0 * dist, // Lod 2: about like buildings    (h o m e)
            1.5 * dist, // Lod 3: about like roads, areas (flat)
            1.0 * dist, // Lod 4: about like lamps, banks (near)
            0.0 * dist, // Lod U: unlimited near visible
        ];
        // Used Lod-Blocks:  (Mesh-Index/Name = max * 10 + min // i.e 4-2 = 42 = lamps-buildings
        //  4_0 allways      . 4 _ _ _ 0   (h o m e)
        //  4_2 tight        . 4 _ 2       (flat)
        //  4_3 close        . 4 3         (near)
        //  2_0 far          .     2 _ 0   (Tree)
        //  Level of Detail: 5 4 3 2 1 0   There are load requests for the first 3(near) together and for the last(far) separate



        this.viewRings = Math.floor(this.envRadius / this.aboutViewSize) + 1; //  Load x rings of view tiels around the camera spot        //   11   limited to 5 because of missing LoD
        console.log("### OSM2World start ###    test parameter:", parameter.test, "  viewRings:", this.viewRings, "  aboutViewSize:", this.aboutViewSize);


        switch (this.deviceLimit) {
            case DeviceLimit.NONE:
                // default values
                break;
            case DeviceLimit.IOS:
                alert("Device in iOS-Mode")
                this.pbfFileMax = 1000 * 1000 * 1000; // 1 * 1000 * 1000
                this.viewPositionsMax = 8 * 1000 * 1000;
                this.envRadius /= 2;
                parameter.water = 0;
                parameter.shadow = 0;
                parameter.fpsMin = -15; // don't add more ViewTiles

                // Todo: Remember memory use in cookie and limit after tab-reboot ???
                break;
            case DeviceLimit.WEBXR:
                alert("Device in WebXR-Mode.")
                //this.viewZoom = 17: 3200/16 = 200 meter
                //this.pbfFileMax = 1 * 1000 * 1000;
                //this.viewPositionsMax = 1 * 1000 * 1000;
                //this.envRadius == 1000;
                parameter.water = 0;
                parameter.shadow = 0;
                parameter.distanceMax = 1111;
                parameter.viewRings = 1;
                //parameter.fpsMin = 15; // don't add more ViewTiles
                break;
        }


        this.scanPbfTilesX();

        this.m2dMaterial = new BABYLON.StandardMaterial('', scene);
        this.m2dMaterial.alpha = 0;
        this.m2dGround = BABYLON.MeshBuilder.CreateGround('', { width: this.aboutViewSize, height: this.aboutViewSize }, scene);
        this.m2dGround.material = this.m2dMaterial;

        this.env = new Environment(this);
        this.materials = new Materials(this);

        // preloadTextures(scene);  //--  This does fasten loading, but will load to much for countryside areas

        if (parameter.test != 5)
            this.setLogo();

        this.cars = new Cars(this);
        this.tdmr = new Tdmr(this, './');
        this.animations = new Animations(this);


        if (this.parameter.control) {
            this.control = new OSM2WORLD.ControlO2W(this, "NOwalk"); // creating the camera(s) and its key/mouse/touch/XR control

            switch (this.parameter.xrMode) {
                //   -1: off
                case +0: if (this.deviceLimit == DeviceLimit.WEBXR) this.control.activateWebXrAsync(true); break; // conditional
                case +1: this.control.activateWebXrAsync(true); break; // always
                case +2: this.control.activateWebXrAsync(true, {
                    uiOptions: {
                        sessionMode: "immersive-ar",
                        // optionalFeatures: true,
                        // optionalFeatures: ["hit-test", "anchors"],
                    }
                });  // ask for an ar-session
                    // https://doc.babylonjs.com/divingDeeper/webXR/webXRARFeatures
                    //this.envRadius = this.aboutViewSize; // ddd???
                    this.pbfFileMax = 1; // límit 1 byte => limit 1 pbf-tile
                    if (this.parameter.distanceMax > this.aboutViewSize * 1)
                        this.parameter.distanceMax = this.aboutViewSize * 1;
                    break;

            }

        }

        // Hide view tiles if far away
        this.scene.customLODSelector = this.customLOD.bind(this);


        // Set the resize-handler of the canvas/window size
        window.addEventListener('resize', () => {
            console.log("DONE: resize");
            this.canvasAspect = window.innerWidth / window.innerHeight  // >1 if landscape
            this.engine.resize();
        });

        window.onbeforeunload = () => { //  evt: any
            console.log("||| Dismiss O2W Web-Worker");
            if (this.osmScene)
                this.osmScene.dismiss();
        }
        // The onunload event occurs once a page has unloaded (or the browser window has been closed).
        // The reload() method is used to reload the current document.
        // The onbeforeunload event occurs when the document is about to be unloaded.
        // this.onbeforeunload = function(){myScript};


        //this.createRenderLoop();
        //this.engine.runRenderLoop(() => { this.renderLoop() });
        this.engine.runRenderLoop(this.renderLoop.bind(this));

    }


    / **
     * Scan the pbf-tile server for existing lines of pbf-tiles.
     *
     * The send url requests the root directiry of the server.
     * Each responded line is a directory, containing pbf-tiles.
     * The name of the directory is the y indice of an OSM tile name.
     * /
    scanPbfTilesX(): void {
        let scanxReq = new XMLHttpRequest();
        scanxReq.open("GET", this.tileUrl, true);
        scanxReq.onerror = () => { alert("scanX") };
        scanxReq.onload = () => {

            let lines = scanxReq.response.split('</td><td><a href="')
            for (var i = 2; i < lines.length; i++) {
                let tileY = lines[i].split("/")[0]
                if (tileY != "0") {
                    this.scannedX += (" " + tileY);
                }
            }
            //console.log("scannedX:", this.scannedX);

        };//onload
        scanxReq.send(null);
    }


    / **
     * Scan the pbf-tile server for existing  pbf-tiles.
     *
     * The send url requests a y directiry of the server.
     * Each responded line is an existing pbf-tile.
     * The name of the tile is the x indice of an OSM tile name.
     * /
    scanPbfTilesY(pbfTile_Name: Tile_Name, callback?: Function | undefined): void {

        if (!this.scannedX.includes("" + pbfTile_Name.x)) {
            if (callback) callback(false);
            return;
        }

        if (this.scannedY[pbfTile_Name.x]) {
            let pbfExists = this.scannedY[pbfTile_Name.x].includes("" + pbfTile_Name.y);
            if (callback) callback(pbfExists);
            return;
        }

        this.scannedY[pbfTile_Name.x] = "";

        let scanxReq = new XMLHttpRequest();
        scanxReq.open("GET", this.tileUrl + pbfTile_Name.x + "/", true);
        scanxReq.onerror = () => { if (callback) callback(false); };
        scanxReq.onload = () => {

            let lines = scanxReq.response.split('</td><td><a href="')
            for (var i = 2; i < lines.length; i++) {
                let tileY = lines[i].split(".")[0]
                if (tileY != "0") {
                    this.scannedY[pbfTile_Name.x] += (" " + tileY);
                }
            }
            //console.log("scannedY",pbfTile_Name.x,":", this.scannedY[pbfTile_Name.x]);
            let pbfExists = this.scannedY[pbfTile_Name.x].includes("" + pbfTile_Name.y);
            if (callback) callback(pbfExists);

        };//onload
        scanxReq.send(null);

    }



    / **
     * Renders the [[OsmScene]] and the [[Environment]].
     * Calculates the distance-limit of the visual [[ViewTile]]s
     * Called by the Babylon render loop
     * /
    private renderLoop(): void {

        let dSec = this.engine.getDeltaTime() / 1000;
        let fps = this.engine.getFps();
        if (dSec > 1) dSec = 1; // after breaktpoints etc.

        // integer and limited fps
        // let fps = dSec > 0 ? Math.floor(1 / dSec) : 0;
        fps = Math.floor(fps)

        if (this.osmScene) {
            this.osmScene.render(dSec);
            this.env.render(dSec);
        }

        let cycle = 1 / 10; // check only with 10 fps
        this.dSecTiles += dSec;
        if (this.dSecTiles > cycle && this.scannedX) {
            this.rampDistance(dSec, fps);  // actual time to go below limit if passive
            this.dSecTiles = 0;
        }

        this.scene.render();
    }


    / **
     * Calculates the distance-limit of the visual [[ViewTile]]s
     * by ramping up/down the distance according to the actual frames-per-second
     * @param dSec  Time delta to last render cycle
     * @param fps Actual frames per second
     * /
    rampDistance(dSec: number, fps: number): void {

        // check if camera was moved
        let camera = this.scene.activeCamera as BABYLON.ArcRotateCamera;
        this.passive = camera.position.equals(this.cameraPosition);
        this.cameraPosition = camera.position.clone();
        this.cameraTarget = camera.target.clone();

        if (!this.osmScene) return

        let min = Math.abs(this.parameter.fpsMin);

        if (this.parameter.fpsMin > 0) { // move enabeling control active: if moved reduce visibility
            if (this.passive) //??? && this.osmScene.requestCount <= 0
                min = 2;
            else { // aktive: limit to 1 if > 1 and fps is slow
                let less = 2;
                if (this.distanceFactor > less && fps < min)
                    this.distanceFactor = less;
            }
        }

        if ((min != 0)) { // if set, regulate the fps by changing the visibility
            if ((fps < min)) { // slow
                if (this.distanceFactor > 1.00) this.distanceFactor -= dSec * 1.; // show less tiles
            } else {                                                       // x=schrauben
                if (this.distanceFactor < 50.0) this.distanceFactor += dSec * 20; // show more tiles  - viewRings: 25
                if (this.osmScene && this.viewPositions < this.viewPositionsMax) this.osmScene.requestTiles();
            }
        } else { // pfs = 0: no control, only tile load
            this.osmScene.requestTiles();
        }

        //dd- if (fpsLog) console.log("fps:", Math.floor(fps), Math.floor(this.distanceFactor * 100), this.passive ? "p0" : "p1"
        //dd-     ,"d" + this.osmScene.drawCount, "r" + this.osmScene.requestCount
        //dd- )

    }


    / **
     * The BJS engine will request what to to with all meshes, each render cycle!
     * Hide view tiles if far away
     * @param mesh  BABYLON.Mesh, the Lod is requested for
     * @param camera  BABYLON.Camera, the actual used camera
     * @return BABYLON.Mesh the mesh to be shown at last. Or null if none
     * /
    private customLOD(mesh: BABYLON.AbstractMesh, camera: BABYLON.Camera): BABYLON.AbstractMesh | null {
        if (!mesh.maxDist) { //   || true dddar  Dont check minDist, it may be 0!
            return mesh;
        }

        //do this once per render cycle only: todo
        //let targetCamera = camera as BABYLON.ArcRotateCamera;
        let target = (this.cameraTarget) ? this.cameraTarget : this.cameraPosition;
        let focusPosUp = camera.position.add(target).divide(new ScenePos(2, 2, 2));  // caly only in render loop!!! todo ???
        focusPosUp.y = camera.position.y;  //ok???

        let position = mesh.position;
        if (mesh.name == "anim" && mesh.parent) {
            let parent = mesh.parent as BABYLON.Mesh;
            position = parent.position;
        }

        let distance = Math.abs(BABYLON.Vector3.Distance(position, focusPosUp));

        if (
            distance < (this.envRadius) && // in sky range (world)
            (
                (distance < this.maxDistLod[4] * this.distanceFactor) //  ||?    in dynamic range to be visible (fog/fps)
                &&                // 200m * >=50 = 10'000
                (distance <= mesh.maxDist && distance > mesh.minDist) // in LoD range
            )
        ) {
            mesh.isVisible = true;
            return mesh;
        };

        mesh.isVisible = false;
        return null;
    }




    setCameraView(view: CameraView): void { // startPosition,startRotation

        let camera = this.scene.activeCamera as BABYLON.ArcRotateCamera;

        if (!camera)
            return alert("!!!-!!! NO CAMERA DEFINED - viewer.setCameraView")

        camera.target = view.scenePos;
        camera.beta = view.beta; // x view      beta  (radians) the latitudinal  rotation  0=down 90=horizotal 180=up
        camera.alpha = view.alpha; // y direction alpha (radians) the longitudinal rotation  -90=Nord   left/right
        camera.radius = view.radius; // z radius the distance from the target position
        camera.fov = view.fov;
        // camera.updateProjectionMatrix();

    }


    / **
     * get the actual cameras scene position and direction
     * @return CameraView instance with all values
     * /
    getCameraView(): CameraView {
        let camera = this.scene.activeCamera as BABYLON.ArcRotateCamera;

        let alpha = camera.rotation.y;
        let beta = camera.rotation.x;
        if (typeof (camera.alpha) !== "undefined") {
            alpha = camera.alpha;
            beta = camera.beta;
        }

        return new CameraView(
            camera.target,
            alpha,
            beta,
            camera.radius,
            camera.fov
        );
    }





    / **
     * Set the BabylonJS loading screen with the OSM2World logo and the version of this lib
     * /
    private setLogo(): void {

        if (!this.canvas) return;

        // Set user defined Babylon loading screen // Todo:BJS String is not always below the logo
        // https://forum.babylonjs.com/t/defaultlogourl-gets-an-elipse/13824
        // https://www.babylonjs-playground.com/#WTEX40#7
        // https://github.com/BabylonJS/Babylon.js/issues/8845
        BABYLON.DefaultLoadingScreen.DefaultLogoUrl = dataPath + "textures/osm2world_logo.png";
        this.loadingScreen = new BABYLON.DefaultLoadingScreen(this.canvas, "  ");// "Please wait a minute  -  Please wait a minute  -  Please wait a minute"
        this.engine.loadingScreen = this.loadingScreen;
        if (this.parameter.test <= 1) this.engine.displayLoadingUI();

    }


    / **
     * get the actual cameras geo position and direction
     * @return GeoView instance with all values
     * /
    getGeoViewAtCamera(): GeoView {
        if (!this.osmScene) return new GeoView(new GeoPos());
        let cameraView = this.getCameraView();
        return cameraView.toGeoView(this.osmScene);
    }


    / **
     * Restore the start position and view, stored by the scene automatically
     * /
    restoreStart(): void {
        let view = this.restoreGeoView("start");
        if (!view) return;

        this.setGeoView(view);
        if (this.osmScene) this.osmScene.webARroot.scaling = new BABYLON.Vector3(1, 1, 1);
    }



    / **
     * restore this geo pos from browser cookie
     * @param id  "name" of the cookie to restore it
     * @return restored geo view
     * /
    restoreGeoView(id: string): GeoView | undefined {

        // todo: far jump calcualtes wrong pbf-tile ? (wind=>passau)
        // https://192.168.3.141:8080/o2w/tiles/13/4385/2827.o2w.pbf = wind
        // https://192.168.3.141:8080/o2w/tiles/13/4385/2829.o2w.pbf = BAD 9!
        // https://192.168.3.141:8080/o2w/tiles/13/4402/2828.o2w.pbf = default passau

        if (document.cookie.indexOf('OSM2World_GeoView_' + id) == -1) return undefined;   // cookie does not exists

        // store actual position before jump, may be you like to jump back to it.
        if (id != "last")
            this.getGeoViewAtCamera().storeCookie("last");

        let cookie = this._getCookie('OSM2World_GeoView_' + id);
        console.log("<<< geo: ", id, cookie);

        let floats = cookie.split(' ');

        let geoPos = new GeoPos(
            parseFloat(floats[0]),
            parseFloat(floats[1])
        );

        return new GeoView(
            geoPos,
            parseFloat(floats[2]), // height
            parseFloat(floats[3]), // alpha
            parseFloat(floats[4]), // beta
            parseFloat(floats[5]), // radius
            parseFloat(floats[6]), // fow
        )

    }



    / **
     * Sets the loading UI step and text
     * @param message  new visible text
     * @param step  just a number, count up
     * @param nr  appendet to the text
     * /
    uiStep(message: string, step: number, nr?: number): void {
        if (this.uiStepDone < step || nr) {
            this.uiStepDone = step;
            if (!this.loadingScreen) return;
            let text = message + " - " + message + " - " + message;
            if (nr) text = text + " " + nr;
            this.loadingScreen.loadingUIText = text;
            if (!nr) console.log(text);
        }
    }





    / **
     * read a browser cookie
     * @param cookie "name"
     * @return the cookie string
     * /
    private _getCookie(cname: string): string {
        let name = cname + "=";
        let ca = document.cookie.split(';');
        for (let i = 0; i < ca.length; i++) {
            let c = ca[i];
            while (c.charAt(0) == ' ') c = c.substring(1);
            if (c.indexOf(name) != -1) return c.substring(name.length, c.length);
        }
        return "";
    }



    / **
     * OSM xml request loaded.
     * The xml-coded data of a selected OSM object was responded
     * Show popup and load OSM object page if the user responded it.
     * @param event  loaded event. its respones is the xml string
     * /
    onXml(event: any): void {

        let xml: string = event.currentTarget.response;

        // https://goessner.net/download/prj/jsonxml/
        let tag = xml.indexOf("<tag ") + 5;
        xml = xml.substr(tag);
        let end = xml.indexOf("/>");
        xml = xml.substr(0, end);
        console.log(xml);


        let r = confirm("Show OSM-Info of " + this.selectedType + this.selectedNr + "? (" + xml + ")");
        if (r != true)
            return;

        if (this.selectedType == "w") window.open("https://www.openstreetmap.org/way/" + this.selectedNr, "_blank");
        if (this.selectedType == "n") window.open("https://www.openstreetmap.org/node/" + this.selectedNr, "_blank");
        if (this.selectedType == "r") window.open("https://www.openstreetmap.org/relation/" + this.selectedNr, "_blank");
    }


    / **
     * Default reaction when an OSM object is selected by shift mouse-click.
     * request the object data from "osm.org".
     * @param type  node, way or relation
     * @param nr  OSM id number
     * /
    defaultObjectSelected(type: string, nr: string): void {
        this.selectedType = type;
        this.selectedNr = nr;

        let t: string = "?";
        if (type == "w") t = "way";
        if (type == "n") t = "node";
        if (type == "r") t = "rel";

        //         https://www.openstreetmap.org/api/0.6/way/373814659
        let url = "https://www.openstreetmap.org/api/0.6/" + t + "/" + nr;
        let tileReq = new XMLHttpRequest();
        tileReq.open("GET", url, true);
        tileReq.responseType = "text"; // xml ??
        tileReq.onload = this.onXml.bind(this);
        tileReq.send(null);
    }




    / **
     * Find url parameter-name and return the value. Or the defaut value or an empty string
     * @param name  name of the requested parameter
     * @param defaultVal  Default value if the parameter does not exist
     * @return found value or default
     * /
    getUrlParameter(name: string, defaultVal?: any): string | any {
        if (!defaultVal) defaultVal = '';
        let value = this.HTTP_GET_VARS[name];
        if (!value) { return defaultVal; }   // no name as index? return default value
        return value;                // or return the value of the named parameter
    }


    / **
     * Store URL parameter in an array and a function to read them
     * /
    readUrlParameter(): void {

        let domLocation = document.location.search;
        let location = decodeURI(domLocation.substr(1, Math.min(domLocation.length, 1000))); // No unlimited string

        if (location != '') {
            let urlParts: string[] = location.split('&');
            for (let i in urlParts)// for all HTTP parameter
            {
                let value = '';
                let parameters: string[] = urlParts[i].split('=');  // split name and value
                if (parameters.length > 1) { value = parameters[1]; }   // walue exists? remember
                let name: any = parameters[0];
                this.HTTP_GET_VARS[name] = value;  // name-index in array
            }
        }
    }





}


*****/

}