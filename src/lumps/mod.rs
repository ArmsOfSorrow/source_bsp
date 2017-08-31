use ::std::io::prelude::*;
use ::std::mem;
use BspHeader;

pub enum Lump {
    Plane(Vec<Plane>),
    Vertex(Vec<Vector>)
}

impl Lump {
    pub fn get_index(&self) -> usize {
        match self {
            &Lump::Plane(ref planes) => 1, //ref prevents pattern matching from taking ownership
            &Lump::Vertex(ref verts) => 3
        }
    }

    //returns the size of a single element of the associated data
    pub fn get_data_size(&self) -> usize {
        match self {
            &Lump::Plane(ref planes) => mem::size_of::<Plane>(),
            &Lump::Vertex(ref verts) => mem::size_of::<Plane>(),
        }
    }

    fn load<R: BufRead>(&self, header: &BspHeader, reader: &R) {
        
        //match on ourselves and load data with reader
        match *self {
            Lump::Plane(ref planes) => {},
            Lump::Vertex(ref verts) => {}
        }
    }
}

pub struct Plane {
    normal: Vector, //normal vector
    distance: f32,  // distance from origin
    type_id: i32,   // plane axis identifier
}

impl Plane {
    pub fn new(n: Vector, dist: f32, id: i32) -> Plane {
        Plane {
            normal: n,
            distance: dist,
            type_id: id,
        }
    }
}

pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }
}