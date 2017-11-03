use std::io::prelude::*;
use std::marker::Sized;
use byteorder::ByteOrder;

pub mod vector;
pub mod plane;
pub mod edge;

pub trait LumpData {
     fn load<R: Read, O: ByteOrder>(reader: &mut R) -> ::io::Result<Self> where Self: Sized;
     fn get_index() -> usize;
}
