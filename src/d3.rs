use std::ops::{Add, Sub, DivAssign, Div};

const EPS: f64 = 1e-4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point3 {
    pub fn cross(&self, other: Point3) -> Point3 {
        let x = self.y*other.z - other.y*self.z;
        let y = self.z*other.x - other.z*self.x;
        let z = self.x*other.y - other.x*self.y;
        Point3 {x, y, z}
    }

    pub fn dot(self, other: Point3) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 {x, y, z}
    }
}

impl Add for Point3 {
    type Output = Point3;

    fn add(self, other: Point3) -> Point3 {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Point3 {x, y, z}
    }
}

impl Sub for Point3 {
    type Output = Point3;

    fn sub(self, other: Point3) -> Point3 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Point3 {x, y, z}
    }
}

impl Div<f64> for Point3 {
    type Output = Point3;

    fn div(self, divisor: f64) -> Point3 {
        let x = self.x / divisor;
        let y = self.y / divisor;
        let z = self.z / divisor;
        Point3 {x, y, z}
    }
}

impl DivAssign<f64> for Point3 {
    fn div_assign(&mut self, divisor: f64) {
        self.x /= divisor;
        self.y /= divisor;
        self.z /= divisor;
    }
}

#[derive(Debug, Clone)]
pub struct Edge3 {
    pub vertices: [Point3; 2],
}

impl PartialEq for Edge3 {
    // edges are symmetric
    fn eq(&self, other: &Edge3) -> bool {
        self.vertices[0] == other.vertices[0] && self.vertices[1] == other.vertices[1]
        || self.vertices[0] == other.vertices[1] && self.vertices[1] == other.vertices[0]
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Facet3 {
    pub vertices: [Point3; 3],
}

impl Facet3 {
    pub fn normal(&self) -> Point3 {
        let mut n = (self.vertices[1] - self.vertices[0]).cross(self.vertices[2] - self.vertices[0]);
        n /= n.length();
        n
        // i dont think i need it normalized
        // (self.vertices[1] - self.vertices[0]).cross(self.vertices[2] - self.vertices[0])
    }

    pub fn surface(&self) -> f64 {
        (self.vertices[1] - self.vertices[0]).cross(self.vertices[2] - self.vertices[0]).length()/2.
    }

    pub fn mid(&self) -> Point3 {
        (self.vertices[0] + self.vertices[1] + self.vertices[2])/3.
    }

    pub fn visible_from(&self, q: &Point3) -> bool {
        (*q - self.vertices[0]).dot(self.normal()) > EPS
    }
}

pub fn surface(facets: &[Facet3]) -> f64 {
    facets.iter().map(|f| f.surface()).sum()
}

/// check that all points are behind every facet (or on)
pub fn is_convex(facets: &[Facet3], points: &[Point3]) -> bool {
    for f in facets {
        for p in points {
            if f.visible_from(p) {
                return false
            }
        }
    }

    true
}

#[cfg(feature = "visual")] use std::io;
#[cfg(feature = "visual")] use std::io::prelude::*;
#[cfg(feature = "visual")] use std::fs::File;
#[cfg(feature = "visual")] use std::path::Path;

#[cfg(feature = "visual")]
pub fn threejs(
    points: &[Point3],
    facets: &[Facet3],
    eyepoint: &Point3,
    candidates: &[Point3],
    hull_delete: &[Facet3],
    horizon: &[Edge3],
    filename: &str
) -> Result<(), io::Error> {
    let path = Path::new(filename);

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path)?;

    /* Write Header */
    write!(file,
        "<html>\n
        <head>\n
            <meta charset=\"utf-8\"/>\n
            <script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r83/three.min.js\"></script>\n
        </head>\n
        <body>\n
        <script>\n
        var camera, scene, renderer,
            geometry, material,
            hullMesh,
            hullDelete, hullDeleteMesh,
            trace, traceMesh,
            eyepoint, eyepointMesh,
            candidates, candidatesMesh,
            group;\n
        \n
        init();\n
        animate();\n
        screenshot();\n
        \n
        function cylinder(trace, x, y, z, dx, dy, dz, thickness) {{\n
            var length = Math.sqrt(dx*dx + dy*dy + dz*dz);\n
            var box = new THREE.CylinderGeometry( thickness, thickness, length );\n
            var dir = new THREE.Vector3( dx, dy, dz ).normalize();\n
            var q = new THREE.Quaternion();\n
            q.setFromUnitVectors( new THREE.Vector3(0, 1, 0), dir );\n
            var m = new THREE.Matrix4();\n
            m.makeRotationFromQuaternion(q);\n
            box.applyMatrix(m);\n
            box.translate( x, y, z );\n
            var boxMesh = new THREE.Mesh(box);\n
            boxMesh.updateMatrix();\n
            trace.merge(boxMesh.geometry, boxMesh.matrix);\n
        \n
            var sphere = new THREE.SphereGeometry( thickness * 1.2 );\n
            sphere.translate(x+dx/2, y+dy/2, z+dz/2);\n
            var sphereMesh = new THREE.Mesh(sphere);\n
            sphereMesh.updateMatrix();\n
            trace.merge(sphereMesh.geometry, sphereMesh.matrix);\n
        }}\n
        \n
        function init() {{\n
            scene = new THREE.Scene();\n
        \n
            camera = new THREE.PerspectiveCamera( 75, window.innerWidth / window.innerHeight, 1, 10000 );\n
            camera.position.z = 2;\n
        \n
            var light = new THREE.AmbientLight( 0x606060 ); // soft white light\n
            scene.add( light );\n
            var directionalLight = new THREE.DirectionalLight(0xffffff,1);\n
            directionalLight.position.set(1, 1, 1).normalize();\n
            scene.add(directionalLight);\n
            group = new THREE.Group();\n
            var trace = new THREE.Geometry();\n
            var candidates = new THREE.Geometry();\n
            var eyepoint = new THREE.Geometry();\n
            var hull = new THREE.Geometry();\n
            var hullDelete = new THREE.Geometry();\n"
    )?;

    let mut num_vertices = 0;
    let mut num_vertices_delete = 0;
    for p in points {
        write!(file, "{}", print_point(p, "trace", 1))?;
    }
    for p in candidates {
        write!(file, "{}", print_point(p, "candidates", 1))?;
    }
    write!(file, "{}", print_point(eyepoint, "eyepoint", 2))?;
    for f in facets {
        num_vertices += 3;
        write!(file, "{}", print_facet(f, "hull", num_vertices))?;
    }
    for f in hull_delete {
        num_vertices_delete += 3;
        write!(file, "{}", print_facet(f, "hullDelete", num_vertices_delete))?;
    }
    for e in horizon {
        write!(file, "{}", print_edge(e))?;
    }

    // in between we define our geometry
    write!(file,
            "hull.computeFaceNormals();\n
            hull.computeVertexNormals();\n
            // this will only work, because both have the same size\n
            // trace.normalize();\n
            // hull.normalize();\n
            material = new THREE.MeshPhongMaterial( {{ color: 0xee0000, transparent: true, opacity: 0.5, shininess: 60 }} );\n
            materialBack = new THREE.MeshPhongMaterial( {{ color: 0x00ee00, transparent: true, opacity: 0.5, shininess: 60, side: THREE.BackSide }} );\n
            materialDouble = new THREE.MeshPhongMaterial( {{ color: 0x5555ee, shininess: 60 }} );\n
            materialDelete = new THREE.MeshPhongMaterial( {{ color: 0xee5555, shininess: 60 }} );\n
            // materialDouble = new THREE.MeshPhongMaterial( {{ color: 0x5555ee, shininess: 60, side: THREE.DoubleSide }} );\n
            // materialDelete = new THREE.MeshPhongMaterial( {{ color: 0xee5555, shininess: 60, side: THREE.DoubleSide }} );\n
            // material = new THREE.MeshPhongMaterial( {{ color: 0xee0000, transparent: true, shininess: 60 }} );\n
            material.shading = THREE.FlatShading;\n
            materialBack.shading = THREE.FlatShading;\n
            materialDouble.shading = THREE.FlatShading;\n
            materialDelete.shading = THREE.FlatShading;\n
            materialTrace = new THREE.MeshPhongMaterial( {{ color: 0x444444 }} );\n
            materialTrace.shading = THREE.SmoothShading;\n
            materialEyepoint = new THREE.MeshPhongMaterial( {{ color: 0xff4444 }} );\n
            materialEyepoint.shading = THREE.SmoothShading;\n
            materialCandidates = new THREE.MeshPhongMaterial( {{ color: 0x44ff44 }} );\n
            materialCandidates.shading = THREE.SmoothShading;\n
            hullMesh = new THREE.Mesh( hull, materialDouble );\n
            hullDeleteMesh = new THREE.Mesh( hullDelete, materialDelete );\n
            traceMesh = new THREE.Mesh( trace, materialTrace );\n
            candidatesMesh = new THREE.Mesh( candidates, materialCandidates );\n
            eyepointMesh = new THREE.Mesh( eyepoint, materialEyepoint );\n
            hullMesh.scale.set(0.008, 0.008, 0.008);\n
            hullDeleteMesh.scale.set(0.008, 0.008, 0.008);\n
            traceMesh.scale.set(0.008, 0.008, 0.008);\n
            candidatesMesh.scale.set(0.008, 0.008, 0.008);\n
            eyepointMesh.scale.set(0.008, 0.008, 0.008);\n
            group.add(hullMesh);\n
            group.add(hullDeleteMesh);\n
            group.add(traceMesh);\n
            group.add(candidatesMesh);\n
            group.add(eyepointMesh);\n
            scene.add( group );\n
            renderer = new THREE.WebGLRenderer( {{ alpha: true, antialias: true }} );\n
            renderer.setSize( window.innerWidth, window.innerHeight );\n
            document.body.appendChild( renderer.domElement );\n
        }}\n
        \n
        function animate() {{\n
            requestAnimationFrame( animate );\n
            render();\n
        }}\n
        \n
        function downloadURI(uri, name) {{\n
            var link = document.createElement(\"a\");\n
            link.download = name;\n
            link.href = uri;\n
            document.body.appendChild(link);\n
            link.click();\n
            document.body.removeChild(link);\n
            delete link;\n
        }}\n
        \n
        // call this function from the debug console to create a screenshot\n
        function screenshot() {{\n
            var r = new THREE.WebGLRenderer( {{ alpha: true, antialias: true, preserveDrawingBuffer: true }} );\n
            r.setSize(4096, 4096);\n
            var c = new THREE.PerspectiveCamera( 75, 1, 1, 10000 );\n
            c.position.z = 2;\n
            r.render( scene, c );\n
            // window.open( r.domElement.toDataURL( 'image/png' ), 'screenshot' );\n
            downloadURI(r.domElement.toDataURL( 'image/png' ), '{}.png');\n
        }}\n
        \n
        function render() {{\n
            group.rotation.x += 0.003;\n
            group.rotation.y += 0.006;\n

            renderer.render( scene, camera );\n
        }}\n
        </script>\n
        </body>\n
        </html>\n",
        filename
    )?;

    Ok(())
}

#[cfg(feature = "visual")]
fn print_point(point: &Point3, part: &str, r: u32) -> String {
    format!(
        "var sphere = new THREE.SphereGeometry( {} );\n
        sphere.translate( {}, {}, {} );\n
        var sphereMesh = new THREE.Mesh(sphere);\n
        sphereMesh.updateMatrix();\n
        {}.merge(sphereMesh.geometry, sphereMesh.matrix);\n",
        r,
        point.x, point.y, point.z,
        part
    )
}

#[cfg(feature = "visual")]
fn print_facet(facet: &Facet3, part: &str, num_vertices: usize) -> String {
    let a = facet.vertices[0];
    let b = facet.vertices[1];
    let c = facet.vertices[2];
    format!("{}.vertices.push( new THREE.Vector3( {}, {}, {} ) );\n
             {}.vertices.push( new THREE.Vector3( {}, {}, {} ) );\n
             {}.vertices.push( new THREE.Vector3( {}, {}, {} ) );\n
             {}.faces.push( new THREE.Face3( {}, {}, {} ) );\n",
             part, a.x, a.y, a.z,
             part, b.x, b.y, b.z,
             part, c.x, c.y, c.z,
             part, num_vertices-3, num_vertices-2, num_vertices-1
    )
}

#[cfg(feature = "visual")]
fn print_edge(edge: &Edge3) -> String {
    let a = edge.vertices[0];
    let b = edge.vertices[1];
    let dir = a-b;
    let center = (a+b)/2.;
    format!("cylinder(trace, {}, {}, {}, {}, {}, {}, {});\n",
             center.x, center.y, center.z,
             dir.x, dir.y, dir.z,
             0.5
    )
}
