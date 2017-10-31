extern crate byteorder;

use std::io;
use std::io::prelude::*;
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
    // header: BspHeader,
    //a data source we can read from and seek
    reader: T
}

impl<T> BspFile<T> where T: Read + Seek {
    pub fn new(r: T) -> BspFile<T> {
        //maybe we should create a custom error type for that (or is there one
        //for invalid format already?)
        
        //and read_header should be BspHeader's new() function,
        //since we can't access self from here.
        BspFile { reader: r }
    }

    // fn read_header(&mut self) -> io::Result<BspHeader> {
    //     let mut header = BspHeader {
    //         magic: [0; 4],
    //         version: -1,
    //         lumps: [LumpDirEntry {
    //             offset: -1,
    //             length: -1,
    //             version: -1,
    //             four_cc: [0; 4],
    //         }; HEADER_LUMPS],
    //         map_revision: -1,
    //     };

    //     //TODO: seek to beginning before reading

    //     let mut magic = [0u8; 4];
    //     self.reader.read(&mut magic)?;

    //     //check if it's VBSP or PSBV (little or big endian), then use byteorder
    //     //to read shit
    //     header.magic = magic;
    //     let magic_str = String::from_utf8_lossy(&magic);

    //     if magic_str == "VBSP" {
    //         header.version = self.reader.read_i32::<LittleEndian>()?;
    //         Self::read_lump_directory::<LittleEndian>(self, &mut header);
    //         header.map_revision = self.reader.read_i32::<LittleEndian>()?;
    //     } else if magic_str == "PSBV" {
    //         header.version = self.reader.read_i32::<BigEndian>()?;
    //         Self::read_lump_directory::<BigEndian>(self, &mut header);
    //         header.map_revision = self.reader.read_i32::<BigEndian>()?;
    //     } /*else {
    //         Err("The specified file is not a valid source BSP file")
    //     }*/

    //     Ok(header)
    // }

//hm, this requires the header to be part of BspFile again. How should we deal with it?
//implementing Default for BspHeader would be easiest, I think. Pr just call read_header
//from the constructor and make it private. It might be "overkill" but default wouldn't
//work unless we box the LumpDirEntry array.
    // pub fn read_lump<O: ByteOrder, L: LumpData>(&self) -> Option<Vec<L>> {

    //     let i = L::get_index();
    //     let lump = self.header.lumps[i];
    //     let offset = lump.offset;
    //     let len = lump.length;

    //     if offset != -1 && len != -1 {
    //         //these differ by struct size
    //         let size = std::mem::size_of::<L>();
    //         let count = len as usize / size;
    //         let mut v: Vec<L> = Vec::with_capacity(count);

    //         for i in 0..count {
    //             let elem = L::load::<_, O>(self.reader).unwrap();
    //             v[i] = elem;
    //         }
    //         Some(v)
    //     } else {
    //         None
    //     }
    // }

    // fn read_lump_directory<O: ByteOrder>(&mut self, header: &mut BspHeader) {
    //     for index in 0..HEADER_LUMPS {
    //         let offset = self.reader.read_i32::<O>().unwrap();
    //         let length = self.reader.read_i32::<O>().unwrap();
    //         let version = self.reader.read_i32::<O>().unwrap();
    //         let mut four_cc = [0; 4];
    //         self.reader
    //             .read(&mut four_cc)
    //             .expect(&format!("failed to read four_cc at lump index {}", index));

    //         let lump = &mut header.lumps[index];
    //         lump.offset = offset;
    //         lump.length = length;
    //         lump.version = version;
    //         lump.four_cc = four_cc;
    //     }
    // }
}

// impl BspFile {

//     pub fn load_lump<R: BufRead + Seek, O: ByteOrder, L: LumpData>(
//         &self,
//         reader: &mut R,
//     ) -> Option<Vec<L>> {

//         let i = L::get_index();
//         let lump = self.header.lumps[i];
//         let offset = lump.offset;
//         let len = lump.length;

//         if offset != -1 && len != -1 {
//             //these differ by struct size
//             let size = std::mem::size_of::<L>();
//             let count = len as usize / size;
//             let mut v: Vec<L> = Vec::with_capacity(count);

//             for i in 0..count {
//                 let elem = L::load::<R, O>(reader).unwrap();
//                 v[i] = elem;
//             }
//             Some(v)
//         } else {
//             None
//         }
//     }
// }

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
        for index in 0..HEADER_LUMPS {
            let offset = reader.read_i32::<O>().unwrap();
            let length = reader.read_i32::<O>().unwrap();
            let version = reader.read_i32::<O>().unwrap();
            let mut four_cc = [0; 4];
            reader
                .read(&mut four_cc)
                .expect(&format!("failed to read four_cc at lump index {}", index));

            let mut lump = self.lumps[index];
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

    #[test]
    fn load_header() {
        let f = File::open("testfiles/water_v2.bsp").unwrap();
        let reader = BufReader::new(f);

        let mut bsp_file = BspFile::new(reader);
        let header = bsp_file.read_header().unwrap();

        let string = str::from_utf8(&header.magic).unwrap();
        assert_eq!(string, "VBSP");
        assert_eq!(header.version, 20);
        assert_eq!(header.map_revision, 1555);
    }

    // #[test]
    // fn load_file_by_path() {
    //     let path = Path::new("testfiles/water_v2.bsp");
    //     let file = load_file(path).unwrap();

    //     let header = file.header;
    //     let string = str::from_utf8(&header.magic).unwrap();
    //     assert_eq!(string, "VBSP");
    //     assert_eq!(header.version, 20);

    //     assert_eq!(header.map_revision, 1555);
    // }

    // #[test]
    // fn plane_lumpdir_entry() {
    //     let path = Path::new("testfiles/water_v2.bsp");
    //     let file = load_file(path).unwrap();
    //     let header = file.header;
    //     let plane_lump = header.lumps[0];
    //     assert_eq!(434240, plane_lump.length);
    // }

    // #[test]
    // fn load_lump_data() {
    //     let path = Path::new("testfiles/water_v2.bsp");
    //     let file = load_file(path).unwrap();

    //     //TODO: load lump data for index/variant
        
    // }
}
