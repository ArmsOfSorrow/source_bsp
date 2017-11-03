use std::io::Read;
use byteorder::{ByteOrder, ReadBytesExt};
use lumps::LumpData;
use lumps::vector::Vector;

#[derive(Debug, PartialEq)]
#[repr(C)]
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
    fn load<R: Read, O: ByteOrder>(reader: &mut R) -> ::io::Result<Self> where Self: Sized {

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