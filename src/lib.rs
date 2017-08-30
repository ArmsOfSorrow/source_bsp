#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn load_file_by_path() {
        let path = Path::new("testfiles/water_v2.bsp");
        let file = load_file(path).unwrap();

        let header = file.header;
        let string = str::from_utf8(&header.magic).unwrap();
        assert_eq!(string, "VBSP");
        assert_eq!(header.version, 20);

        assert_eq!(header.map_revision, 1555);
    }

    #[test]
    fn plane_lumpdir_entry() {
        let path = Path::new("testfiles/water_v2.bsp");
        let file = load_file(path).unwrap();
        let header = file.header;
        let plane_lump = header.lumps[0];
        assert_eq!(434240, plane_lump.length);
    }
}

extern crate byteorder;

use std::fs::File;
use std::path::Path;
use std::io;
use std::io::{BufReader, SeekFrom};
use std::io::prelude::*;
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

const HEADER_LUMPS: usize = 64;

pub fn load_file(path: &Path) -> io::Result<BspFile> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let header = load_header(&mut reader);
    let bsp_file = BspFile { header: header };

    Ok(bsp_file)
}

fn load_header<R: BufRead>(reader: &mut R) -> BspHeader {
    let mut header = BspHeader {
        magic: [0; 4],
        version: -1,
        lumps: [LumpDirEntry {
            offset: -1,
            length: -1,
            version: -1,
            four_cc: [0; 4],
        }; HEADER_LUMPS],
        map_revision: -1,
    };

    let mut magic = [0u8; 4];
    reader.read(&mut magic).expect("couldn't read magic");

    //check if it's VBSP or PSBV (little or big endian), then use byteorder
    //to read shit
    header.magic = magic;
    let magic_str = String::from_utf8_lossy(&magic);

    if magic_str == "VBSP" {
        //call generic method with LittleEndian param
    } else if magic_str == "PSBV" {
        //call generic method with BigEndian param
    }

    header.version = reader.read_i32::<LittleEndian>().unwrap();

    load_lump_directory::<_, LittleEndian>(reader, &mut header);

    header.map_revision = reader.read_i32::<LittleEndian>().unwrap();

    header
}

fn load_lump_directory<R: BufRead, O: ByteOrder>(reader: &mut R, header: &mut BspHeader) {
    for index in 0..HEADER_LUMPS {
        let offset = reader.read_i32::<O>().unwrap();
        let length = reader.read_i32::<O>().unwrap();
        let version = reader.read_i32::<O>().unwrap();
        let mut four_cc = [0; 4];
        reader
            .read(&mut four_cc)
            .expect(&format!("failed to read four_cc at lump index {}", index));

        let lump = &mut header.lumps[index];
        lump.offset = offset;
        lump.length = length;
        lump.version = version;
        lump.four_cc = four_cc;
    }
}

pub struct BspFile {
    pub header: BspHeader,
}

impl BspFile {
    //loads lump 1 (planes)
    pub fn load_planes<R: BufRead + Seek, O: ByteOrder>(
        &self,
        reader: &mut R,
    ) -> io::Result<Vec<Plane>> {
        //get lump directory entry
        let lump = self.header.lumps[1];
        let offset = lump.offset;
        let len = lump.length;

        //seek to right file location
        reader.seek(SeekFrom::Start(offset as u64))?;

        let plane_size = std::mem::size_of::<Plane>();
        let count = len as usize / plane_size;
        let mut v: Vec<Plane> = Vec::with_capacity(count);

        //read data (4 f32, 1 i32) until we're done and push into vec
        for i in 0..count {
            let x = reader.read_f32::<O>()?;
            let y = reader.read_f32::<O>()?;
            let z = reader.read_f32::<O>()?;
            let dist = reader.read_f32::<O>()?;
            let id = reader.read_i32::<O>()?;

            let vector = Vector::new(x, y, z);
            let plane = Plane::new(vector, dist, id);

            v[i] = plane;
        }

        Ok(v)
    }

    //what if we had load_lump<T> where we pass the lump type?
    //it wouldn't have a return value though...or would return a lumptype
    //struct with associated data?
    pub fn load_lump<R: BufRead + Seek, O: ByteOrder>(
        &self,
        reader: &mut R,
        lump: Lump
    ) {
        let index = lump.get_index();
        let dir_entry = self.header.lumps[index];
        let offset = dir_entry.offset;
        let len = dir_entry.length;

        if offset != -1 && len != -1 {
            //some enum variant...but how do we decide on loading logic?
            //should a lump variant load itself after matching?
        } else {
            //none
        }
    }

    pub fn load_vertices<R: BufRead + Seek, O: ByteOrder>(
        &self,
        reader: &mut R,
    ) -> Option<Vec<Vector>> {
        let lump = self.header.lumps[3];
        let offset = lump.offset;
        let len = lump.length;

        if offset != -1 && len != -1 {
            let vert_size = std::mem::size_of::<Vector>();
            let count = len as usize / vert_size;
            let mut v: Vec<Vector> = Vec::with_capacity(count);

            for i in 0..count {
                let x = reader.read_f32::<O>().unwrap();
                let y = reader.read_f32::<O>().unwrap();
                let z = reader.read_f32::<O>().unwrap();

                let vertex = Vector::new(x, y, z);
                v[i] = vertex;
            }
            Some(v)
        } else {
            None
        }
    }
}

pub struct BspHeader {
    pub magic: [u8; 4],
    pub version: i32,
    pub lumps: [LumpDirEntry; HEADER_LUMPS],
    pub map_revision: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct LumpDirEntry {
    pub offset: i32,
    pub length: i32,
    pub version: i32,
    pub four_cc: [u8; 4],
}

pub enum Lump {
    Entity,
    Plane(Vec<Plane>),
    Vertex(Vec<Vector>)
}

impl Lump {
    fn get_index(&self) -> usize {
        match self {
            &Lump::Entity => 0,
            &Lump::Plane(ref planes) => 1, //ref prevents pattern matching from taking ownership
            &Lump::Vertex(ref verts) => 3
        }
    }
}

pub struct Plane {
    normal: Vector, //normal vector
    distance: f32,  // distance from origin
    type_id: i32,   // plane axis identifier
}

impl Plane {
    fn new(n: Vector, dist: f32, id: i32) -> Plane {
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
    fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }
}
