use lumps::LumpData;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io::prelude::*;

#[derive(Debug)]
#[repr(C)]
pub struct Brush {
    pub first_side: i32,
    pub num_sides: i32,
    pub contents: i32 //this could use bitflags/enum
}

impl Brush {
    fn new(first_side: i32, num_sides: i32, contents: i32) -> Brush {
        Brush { first_side, num_sides, contents }
    }
}

impl LumpData for Brush {
    fn load<R: Read, O: ByteOrder>(reader: &mut R) -> ::io::Result<Self> where Self: Sized {
        let first_side = reader.read_i32::<O>()?;
        let num_sides = reader.read_i32::<O>()?;
        let contents = reader.read_i32::<O>()?;

        let brush = Brush::new(first_side, num_sides, contents);
        Ok(brush)
    }

    fn get_index() -> usize {
        18
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct BrushSide {
    pub plane_num: u16,
    pub tex_info: i16,
    pub disp_info: i16,
    pub bevel: i16
}

impl BrushSide {
    fn new(plane_num: u16, tex_info: i16, disp_info: i16, bevel: i16) -> BrushSide {
        BrushSide { plane_num, tex_info, disp_info, bevel }
    }
}

impl LumpData for BrushSide {
    fn load<R: Read, O: ByteOrder>(reader: &mut R) -> ::io::Result<Self> where Self: Sized {
        let plane_num = reader.read_u16::<O>()?;
        let tex_info = reader.read_i16::<O>()?;
        let disp_info = reader.read_i16::<O>()?;
        let bevel = reader.read_i16::<O>()?;

        let brush_side = BrushSide::new(plane_num, tex_info, disp_info, bevel);
        Ok(brush_side)
    }

    fn get_index() -> usize {
        19
    }
}