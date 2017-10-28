extern crate byteorder;

use std::fs::File;
use std::path::Path;
use std::io;
use std::io::{BufReader};
use std::io::prelude::*;
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};
use lumps::{Plane, Vector, LumpData};

const HEADER_LUMPS: usize = 64;

mod lumps;

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
        header.version = reader.read_i32::<LittleEndian>().unwrap();
        load_lump_directory::<_, LittleEndian>(reader, &mut header);
        header.map_revision = reader.read_i32::<LittleEndian>().unwrap();
    } else if magic_str == "PSBV" {
        header.version = reader.read_i32::<BigEndian>().unwrap();
        load_lump_directory::<_, BigEndian>(reader, &mut header);
        header.map_revision = reader.read_i32::<BigEndian>().unwrap();
    }

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

    pub fn load_lump<R: BufRead + Seek, O: ByteOrder, L: LumpData>(
        &self,
        reader: &mut R,
    ) -> Option<Vec<L>> {

        
        let i = L::get_index();
        let lump = self.header.lumps[i];
        let offset = lump.offset;
        let len = lump.length;

        if offset != -1 && len != -1 {
            //these differ by struct size
            let size = std::mem::size_of::<L>();
            let count = len as usize / size;
            let mut v: Vec<L> = Vec::with_capacity(count);

            for i in 0..count {
                let elem = L::load::<R, O>(reader).unwrap();
                v[i] = elem;
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

    #[test]
    fn load_lump_data() {
        let path = Path::new("testfiles/water_v2.bsp");
        let file = load_file(path).unwrap();

        //TODO: load lump data for index/variant
    }
}
