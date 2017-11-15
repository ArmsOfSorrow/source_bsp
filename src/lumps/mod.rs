use std::io::prelude::*;
use std::marker::Sized;
use byteorder::ByteOrder;

pub mod vector;
pub mod plane;
pub mod edge;
pub mod surfedge;
pub mod face;
pub mod brush;

pub trait LumpData {
    //maybe we should call this load_item or something, since it loads a single struct
    //(except for entities :/)
    fn load<R: Read, O: ByteOrder>(reader: &mut R) -> ::io::Result<Self> where Self: Sized;
    fn get_index() -> usize;
}
