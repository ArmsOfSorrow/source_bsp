use std::io::Read;
use byteorder::{ByteOrder, ReadBytesExt};
use lumps::LumpData;

#[derive(Debug, PartialEq)]
#[repr(C)]
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
    fn load<R: Read, O: ByteOrder>(reader: &mut R) -> ::io::Result<Self> where Self: Sized {

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