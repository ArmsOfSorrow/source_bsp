use std::io::prelude::*;
use std::mem;
use std::marker::Sized;
use byteorder::{ByteOrder, ReadBytesExt};
use LumpDirEntry;

pub trait LumpData {
     fn load<R: BufRead, O: ByteOrder>(reader: &mut R) -> ::io::Result<Self> where Self: Sized;
}

pub enum Lump {
    Plane = 1,
    Vertex = 3,
}

impl Lump {
    pub fn get_index(&self) -> usize {
        match self {
            &Lump::Plane => 1, //ref prevents pattern matching from taking ownership
            &Lump::Vertex => 3,
        }
    }

    //returns the size of a single element of the associated data
    pub fn get_data_size(&self) -> usize {
        match self {
            &Lump::Plane => mem::size_of::<Plane>(),
            &Lump::Vertex => mem::size_of::<Plane>(),
        }
    }

    //we could also have a loadable trait that every lump type implements...but does that
    //make it better? since every lump is an array of elements, we could just provide
    //a load method with a type param and implement that
    fn load_vertex<O:ByteOrder, R: BufRead>(reader: &mut R) -> ::io::Result<Vector> { //get enum and lumpdirentry as param, return a vec or a slice here
        let x = reader.read_f32::<O>()?;
        let y = reader.read_f32::<O>()?;
        let z = reader.read_f32::<O>()?;

        Ok(Vector::new(x,y,z))
    }

    fn load_plane<O: ByteOrder, R: BufRead>(reader: &mut R) -> ::io::Result<Plane> {
        let vector = Self::load_vertex::<O, R>(reader)?;
        let dist = reader.read_f32::<O>()?;
        let id = reader.read_i32::<O>()?;

        Ok(Plane::new(vector,dist,id))
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

    pub fn load<R: BufRead, O: ByteOrder>(reader: &mut R) -> ::io::Result<Vector> {
        let x = reader.read_f32::<O>()?;
        let y = reader.read_f32::<O>()?;
        let z = reader.read_f32::<O>()?;
        let dist = reader.read_f32::<O>()?;
        let id = reader.read_i32::<O>()?;

        Ok(Vector::new(x, y, z))
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
}

impl LumpData for Plane {
    fn load<R: BufRead, O: ByteOrder>(reader: &mut R) -> ::io::Result<Self> where Self: Sized {

        let vector = Vector::load::<R, O>(reader)?;
        let dist = reader.read_f32::<O>()?;
        let id = reader.read_i32::<O>()?;
        let plane = Plane::new(vector, dist, id);

        Ok(plane)
    }
}
