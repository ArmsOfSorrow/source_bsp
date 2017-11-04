extern crate byteorder;

use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};
use lumps::LumpData;

const HEADER_LUMPS: usize = 64;

mod lumps;

//API design and impl details:
//- load header (from read/bufread + seek? take ownership?)
//- then let the user load lumps
//- everything should return a result as read_datatype always does that
//- option<T> for the case that a lump doesn't exist
//- we need to save byteorder somewhere but byteorder crate types can't be used here
//- no unwrap in lib code

pub struct BspFile<T> where T: Read + Seek {
    header: BspHeader,
    //a data source we can read from and seek
    reader: T
}

impl<T> BspFile<T> where T: Read + Seek {
    pub fn new(mut r: T) -> io::Result<BspFile<T>> {
        //maybe we should create a custom error type for that (or is there one
        //for invalid format already?)
        
        let bsp_header = BspHeader::new(&mut r)?;
        let bsp_file = BspFile { header: bsp_header, reader: r };
        Ok(bsp_file)
    }

//hm, this requires the header to be part of BspFile again. How should we deal with it?
//implementing Default for BspHeader would be easiest, I think. Pr just call read_header
//from the constructor and make it private. It might be "overkill" but default wouldn't
//work unless we box the LumpDirEntry array.
    pub fn read_lump<O: ByteOrder, L: LumpData>(&mut self) -> Option<Vec<L>> {

        let i = L::get_index();
        let lump = self.header.lumps[i];
        let offset = lump.offset;
        let len = lump.length;

        if offset != -1 && len != -1 {
            let size = std::mem::size_of::<L>();
            let count = len as usize / size;
            let mut v: Vec<L> = Vec::with_capacity(count);
            self.reader.seek(SeekFrom::Start(offset as u64)).unwrap();

            for _ in 0..count {
                let elem = L::load::<_, O>(&mut self.reader).unwrap();
                v.push(elem);
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

impl BspHeader {
    fn new<T: Read + Seek>(reader: &mut T) -> io::Result<BspHeader> {
        //we could return result<BspHeader, std::io::Error with ErrorKind InvalidData
        //and later define our own error type if desired
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

        //TODO: seek to beginning before reading?

        let mut magic = [0u8; 4];
        reader.read(&mut magic)?;

        //check if it's VBSP or PSBV (little or big endian), then use byteorder
        //to read shit
        header.magic = magic;
        let magic_str = String::from_utf8_lossy(&magic);

        if magic_str == "VBSP" {
            header.version = reader.read_i32::<LittleEndian>()?;
            header.read_lump_directory::<LittleEndian, T>(reader);
            header.map_revision = reader.read_i32::<LittleEndian>()?;
        } else if magic_str == "PSBV" {
            header.version = reader.read_i32::<BigEndian>()?;
            header.read_lump_directory::<BigEndian, T>(reader);
            header.map_revision = reader.read_i32::<BigEndian>()?;
        } /*else {
            Err("The specified file is not a valid source BSP file")
        }*/

        Ok(header)
    }

    fn read_lump_directory<O: ByteOrder, T: Read + Seek>(&mut self, reader: &mut T) {
        
        //TODO: remove unwrap calls
        for index in 0..HEADER_LUMPS {
            let offset = reader.read_i32::<O>().unwrap();
            let length = reader.read_i32::<O>().unwrap();
            let version = reader.read_i32::<O>().unwrap();
            let mut four_cc = [0; 4];
            reader
                .read(&mut four_cc)
                .expect(&format!("failed to read four_cc at lump index {}", index));

            let lump = &mut self.lumps[index];
            lump.offset = offset;
            lump.length = length;
            lump.version = version;
            lump.four_cc = four_cc;
        }
    }
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
    use std::fs::File;
    use std::io::BufReader;
    use lumps::vector::Vector;
    use lumps::face::Face;

    #[test]
    fn load_header() {
        let f = File::open("testfiles/water_v2.bsp").unwrap();
        let reader = BufReader::new(f);

        let bsp_file = BspFile::new(reader).unwrap();
        let header = bsp_file.header;
        let string = str::from_utf8(&header.magic).unwrap();

        assert_eq!(string, "VBSP");
        assert_eq!(header.version, 20);
        assert_eq!(header.map_revision, 1555);
    }

    #[test]
    fn load_vertex_lump() {
        let f = File::open("testfiles/water_v2.bsp").unwrap();
        let reader = BufReader::new(f);

        let mut bsp_file = BspFile::new(reader).unwrap();
        let vec_lump = bsp_file.read_lump::<LittleEndian, Vector>();

        if let Some(v) = vec_lump {
            assert_eq!(v[0], Vector::new(0.0,0.0,0.0));
            assert_eq!(v[1], Vector::new(4854.69,13350.1,-861.289));
        } else {
            panic!("ain't got no vertices!");
        }
    }

    #[test]
    fn load_face_lump() {
        //TODO: change test map to be something from HL2 or portal
        //and assert some values from there
        let f = File::open("testfiles/water_v2.bsp").unwrap();
        let reader = BufReader::new(f);

        let mut bsp_file = BspFile::new(reader).unwrap();
        let lump = bsp_file.read_lump::<LittleEndian, Face>();

        if let Some(faces) = lump {
            println!("{:?}", faces[0]);
        } else {
            panic!("ain't got no faces!");
        }
    }
}
