use std::io::Read;
use byteorder::{ByteOrder, ReadBytesExt};
use lumps::LumpData;

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct SurfEdge {
    pub i: i32
}

impl SurfEdge {
    pub fn new(i: i32) -> Self {
        SurfEdge { i }
    }
}

impl LumpData for SurfEdge {
    fn load<R: Read, O: ByteOrder>(reader: &mut R) -> ::io::Result<Self> where Self: Sized {
        let i = reader.read_i32::<O>()?;
        let s = SurfEdge::new(i);
        Ok(s)
    }

    fn get_index() -> usize {
        13
    }
}