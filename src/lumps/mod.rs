use std::io::prelude::*;
use std::marker::Sized;
use byteorder::{ByteOrder, ReadBytesExt};

pub trait LumpData {
     fn load<R: BufRead, O: ByteOrder>(reader: &mut R) -> ::io::Result<Self> where Self: Sized;
     fn get_index() -> usize;
}

pub struct Plane {
    pub normal: Vector, //normal vector
    pub distance: f32,  // distance from origin
    pub type_id: i32,   // plane axis identifier
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

impl LumpData for Plane {
    fn load<R: BufRead, O: ByteOrder>(reader: &mut R) -> ::io::Result<Self> where Self: Sized {

        let vector = Vector::load::<R, O>(reader)?;
        let dist = reader.read_f32::<O>()?;
        let id = reader.read_i32::<O>()?;
        let plane = Plane::new(vector, dist, id);

        Ok(plane)
    }

    fn get_index() -> usize {
        1
    }
}

pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }
}

impl LumpData for Vector {
    fn load<R: BufRead, O: ByteOrder>(reader: &mut R) -> ::io::Result<Self> where Self: Sized {

        let x = reader.read_f32::<O>()?;
        let y = reader.read_f32::<O>()?;
        let z = reader.read_f32::<O>()?;
        let vector = Vector::new(x,y,z);

        Ok(vector)
    }

    fn get_index() -> usize {
        3
    }
}
