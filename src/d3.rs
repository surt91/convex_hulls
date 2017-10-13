use std::ops::{Sub, DivAssign};

use itertools::Itertools;

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

impl Sub for Point3 {
    type Output = Point3;

    fn sub(self, other: Point3) -> Point3 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
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

#[derive(Debug, PartialEq)]
pub struct Facet3 {
    pub vertices: [Point3; 3],
}

impl Facet3 {
    pub fn normal(&self) -> Point3 {
        // let mut n = (self.vertices[1] - self.vertices[0]).cross(self.vertices[2] - self.vertices[0]);
        // n /= n.length();
        // n
        // i dont think i need it normalized
        (self.vertices[1] - self.vertices[0]).cross(self.vertices[2] - self.vertices[0])
    }

    pub fn surface(&self) -> f64 {
        (self.vertices[1] - self.vertices[0]).cross(self.vertices[2] - self.vertices[0]).length()/2.
    }
}

pub fn surface(facets: &[Facet3]) -> f64 {
    facets.iter().map(|f| f.surface()).sum()
}

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn threejs(points: &[Point3], facets: &[Facet3], filename: &str) -> Result<(), io::Error> {
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
        var camera, scene, renderer,\n
        geometry, material, hullMesh, trace, traceMesh, group;\n
        \n
        init();\n
        animate();\n
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
            var hull = new THREE.Geometry();\n"
    )?;

    let mut num_vertices = 0;
    for p in points {
        write!(file, "{}", print_point(p))?;
    }
    for f in facets {
        num_vertices += 3;
        write!(file, "{}", print_facet(f, num_vertices))?;
    }

    // in between we define our geometry
    write!(file,
            "hull.computeFaceNormals();\n
            hull.computeVertexNormals();\n
            // this will only work, because both have the same size\n
            hull.normalize();\n
            trace.normalize();\n
            material = new THREE.MeshPhongMaterial( {{ color: 0xee0000, transparent: true, opacity: 0.5, shininess: 60 }} );\n
            material.shading = THREE.FlatShading;\n
            materialTrace = new THREE.MeshPhongMaterial( {{ color: 0x444444 }} );\n
            materialTrace.shading = THREE.SmoothShading;\n
            hullMesh = new THREE.Mesh( hull, material );\n
            traceMesh = new THREE.Mesh( trace, materialTrace );\n
            group.add(hullMesh);\n
            group.add(traceMesh);\n
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
        // call this function from the debug console to create a screenshot\n
        function screenshot() {{\n
            var r = new THREE.WebGLRenderer( {{ alpha: true, antialias: true, preserveDrawingBuffer: true }} );\n
            r.setSize(4096, 4096);\n
            var c = new THREE.PerspectiveCamera( 75, w / h, 1, 10000 );\n
            c.position.z = 2;\n
            r.render( scene, c );\n
            window.open( r.domElement.toDataURL( 'image/png' ), 'screenshot' );\n
        }}\n
        \n
        function render() {{\n
            group.rotation.x += 0.003;\n
            group.rotation.y += 0.006;\n

            renderer.render( scene, camera );\n
        }}\n
        </script>\n
        </body>\n
        </html>\n"
    )?;

    Ok(())
}

fn print_point(point: &Point3) -> String {
    format!(
        "var sphere = new THREE.SphereGeometry( 1 );\n
        sphere.translate( {}, {}, {} );\n
        var sphereMesh = new THREE.Mesh(sphere);\n
        sphereMesh.updateMatrix();\n
        trace.merge(sphereMesh.geometry, sphereMesh.matrix);\n",
        point.x, point.y, point.z,
    )
}

fn print_facet(facet: &Facet3, num_vertices: usize) -> String {
    let a = facet.vertices[0];
    let b = facet.vertices[1];
    let c = facet.vertices[2];
    format!("hull.vertices.push( new THREE.Vector3( {}, {}, {} ) );\n
             hull.vertices.push( new THREE.Vector3( {}, {}, {} ) );\n
             hull.vertices.push( new THREE.Vector3( {}, {}, {} ) );\n
             hull.faces.push( new THREE.Face3( {}, {}, {} ) );\n",
             a.x, a.y, a.z,
             b.x, b.y, b.z,
             c.x, c.y, c.z,
             num_vertices-3, num_vertices-2, num_vertices-1
    )
}
