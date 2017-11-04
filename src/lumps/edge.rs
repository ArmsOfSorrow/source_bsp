use std::io::Read;
use byteorder::{ByteOrder, ReadBytesExt};
use lumps::LumpData;

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Edge {
    pub v: [u16; 2]
}

impl Edge {
    pub fn new(v: [u16; 2]) -> Self {
        Edge { v }
    }
}

impl LumpData for Edge {
    fn load<R: Read, O: ByteOrder>(reader: &mut R) -> ::io::Result<Self> where Self: Sized {
        let mut buf = [0u16; 2];
        buf[0] = reader.read_u16::<O>()?;
        buf[1] = reader.read_u16::<O>()?;

        let edge = Edge::new(buf);
        Ok(edge)
    }

    fn get_index() -> usize {
        12
    }
}