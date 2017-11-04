use std::io::Read;
use byteorder::{ByteOrder, ReadBytesExt};
use lumps::LumpData;

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct Face {
    pub plane_num: u16,
    pub side: u8,
    pub on_node: u8,
    pub first_edge: i32,
    pub num_edges: i16,
    pub tex_info: i16,
    pub disp_info: i16,
    pub surface_fog_volume_id: i16,
    pub styles: [u8; 4],
    pub light_ofs: i32,
    pub area: f32,
    pub lightmap_texture_mins_in_luxels: [i32; 2],
    pub lightmap_texture_size_in_luxels: [i32; 2],
    pub orig_face: i32,
    pub num_prims: u16,
    pub first_prim_id: u16,
    pub smoothing_groups: u32
}

impl LumpData for Face {
    fn load<R: Read, O: ByteOrder>(reader: &mut R) -> ::io::Result<Self> where Self: Sized {
        let plane_num = reader.read_u16::<O>()?;
        let side = reader.read_u8()?;
        let on_node = reader.read_u8()?;
        let first_edge = reader.read_i32::<O>()?;
        let num_edges = reader.read_i16::<O>()?;
        let tex_info = reader.read_i16::<O>()?;
        let disp_info = reader.read_i16::<O>()?;
        let surface_fog_volume_id = reader.read_i16::<O>()?;
        
        let mut styles = [0u8; 4];
        reader.read(&mut styles)?;

        let light_ofs = reader.read_i32::<O>()?;
        let area = reader.read_f32::<O>()?;

        let mut lightmap_texture_mins_in_luxels = [0i32; 2];
        let mut l1 = reader.read_i32::<O>()?;
        let mut l2 = reader.read_i32::<O>()?;
        lightmap_texture_mins_in_luxels[0] = l1;
        lightmap_texture_mins_in_luxels[1] = l2;

        let mut lightmap_texture_size_in_luxels = [0i32; 2];
        l1 = reader.read_i32::<O>()?;
        l2 = reader.read_i32::<O>()?;
        lightmap_texture_size_in_luxels[0] = l1;
        lightmap_texture_size_in_luxels[1] = l2;
        
        let orig_face = reader.read_i32::<O>()?;
        let num_prims = reader.read_u16::<O>()?;
        let first_prim_id = reader.read_u16::<O>()?;
        let smoothing_groups = reader.read_u32::<O>()?;

        let face = Face { plane_num, side, on_node, first_edge, num_edges, tex_info, disp_info, surface_fog_volume_id,
        styles, light_ofs, area, lightmap_texture_mins_in_luxels, lightmap_texture_size_in_luxels, orig_face,
        num_prims, first_prim_id, smoothing_groups };

        Ok(face)
    }

    fn get_index() -> usize {
        7
    }
}