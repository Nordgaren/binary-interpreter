use std::io::{Read, Seek, SeekFrom};
use byteorder::{ReadBytesExt, NativeEndian, ByteOrder};
use crate::{peek_type};
use paste::paste;

pub trait BinaryReader: ReadBytesExt {

    fn read_bytes(&mut self, size: usize) -> std::io::Result<Vec<u8>> {
        let mut buf = vec![0u8; size];
        self.read_exact(&mut buf[..])?;
        Ok(buf)
    }

    fn read_cstr(&mut self) -> std::io::Result<String> {
        let mut chrs = Vec::new();
        while let chr = self.read_u8()? {
            if chr == 0 {
                break;
            }
            chrs.push(chr);
        }
        Ok(String::from_utf8(chrs).unwrap())
    }

    fn read_wcstr(&mut self) -> std::io::Result<String> {
        let mut chrs = Vec::new();
        while let chr = self.read_u16::<NativeEndian>()? {
            if chr == 0 {
                break;
            }
            chrs.push(chr);
        }
        Ok(String::from_utf16(chrs.as_slice()).unwrap())
    }

    fn read_fixed_cstr(&mut self, size: usize) -> std::io::Result<String> {
        let mut chrs = vec![0u8; size];
        self.read(&mut chrs[..])?;
        Ok(String::from_utf8(chrs).unwrap())
    }

    fn read_fixed_wcstr(&mut self, size: usize) -> std::io::Result<String> {
        let mut chrs = Vec::with_capacity(size);
        for _ in 0..size {
            chrs.push(self.read_u16::<NativeEndian>()?);
        }
        Ok(String::from_utf16(chrs.as_slice()).unwrap())
    }
}

impl<R: ReadBytesExt + ?Sized> BinaryReader for R {}

pub trait BinaryPeeker: ReadBytesExt + Seek {

    fn peek_bytes(&mut self, position: u64, size: usize) -> std::io::Result<Vec<u8>> {
        let start = self.stream_position()?;
        self.seek(SeekFrom::Start(position))?;
        let bytes = self.read_bytes(size);
        self.seek(SeekFrom::Start(start))?;
        return bytes;
    }

    fn peek_cstr(&mut self, position: u64) -> std::io::Result<String> {
        let start = self.stream_position()?;
        self.seek(SeekFrom::Start(position))?;
        let cstr = self.read_cstr();
        self.seek(SeekFrom::Start(start))?;
        return cstr;
    }

    fn peek_wcstr(&mut self, position: u64) -> std::io::Result<String> {
        let start = self.stream_position()?;
        self.seek(SeekFrom::Start(position))?;
        let wcstr = self.read_wcstr();
        self.seek(SeekFrom::Start(start))?;
        return wcstr;
    }

    fn peek_fixed_cstr(&mut self, position: u64, size: usize) -> std::io::Result<String> {
        let start = self.stream_position()?;
        self.seek(SeekFrom::Start(position))?;
        let cstr = self.read_fixed_cstr(size);
        self.seek(SeekFrom::Start(start))?;
        return cstr;
    }

    fn peek_fixed_wcstr(&mut self, position: u64, size: usize) -> std::io::Result<String> {
        let start = self.stream_position()?;
        self.seek(SeekFrom::Start(position))?;
        let wcstr = self.read_fixed_wcstr(size);
        self.seek(SeekFrom::Start(start))?;
        return wcstr;
    }

    fn peek_u8(&mut self, position: u64) -> std::io::Result<u8> {
        let start = self.stream_position()?;
        self.seek(SeekFrom::Start(position))?;
        let byte = self.read_u8();
        self.seek(SeekFrom::Start(start))?;
        return byte;
    }

    fn peek_i8(&mut self, position: u64) -> std::io::Result<i8> {
        let start = self.stream_position()?;
        self.seek(SeekFrom::Start(position))?;
        let byte = self.read_i8();
        self.seek(SeekFrom::Start(start))?;
        return byte;
    }
    peek_type!(u16);
    peek_type!(i16);
    peek_type!(u32);
    peek_type!(i32);
    peek_type!(u64);
    peek_type!(i64);
    peek_type!(u128);
    peek_type!(i128);
}

impl<R: ReadBytesExt + Seek> BinaryPeeker for R {}



//
// pub struct BinaryReader {
//     cursor: Cursor<Vec<u8>>,
//     pub length: usize,
//     endianness: Endian,
// }
//
// impl BinaryReader {
//     pub fn new(bytes: &[u8]) -> Result<BinaryReader, InterpreterError> {
//         Ok(BinaryReader {
//             length: bytes.len() as usize,
//             cursor: Cursor::new(bytes.to_vec()),
//             endianness: NativeEndian,
//         })
//     }
//
//     pub fn from_vec(vec: Vec<u8>) -> Result<BinaryReader, InterpreterError> {
//         BinaryReader::new(vec.as_slice())
//     }
//
//     pub fn from_path(path: PathBuf) -> Result<BinaryReader, InterpreterError> {
//         let file = fs::read(path)?;
//         BinaryReader::new(file.as_slice())
//     }
//
//     pub fn set_endian(&mut self ,endian: Endian) {
//         self.endianness = endian
//         self.cursor.stream_len();
//     }
//
//     pub fn read_cstr(&mut self) -> Result<String, InterpreterError> {
//         let mut buf = Vec::new();
//         self.cursor.read_until(0, &mut buf)?;
//         buf.pop(); //remove the null byte
//         Ok(String::from_utf8(buf)?)
//     }
//
//     pub fn read_wcstr(&mut self) -> Result<String, InterpreterError> {
//         let mut chrs = Vec::new();
//         while let chr = self.cursor.read_u16::<NativeEndian>()? {
//             if chr == 0 {
//                 break;
//             }
//             chrs.push(chr);
//         }
//
//         Ok(String::from_utf16(chrs.as_slice())?)
//     }
//
//     pub fn position(&self) -> usize {
//         self.cursor.position() as usize
//     }
//
//     pub fn read_u8(&mut self) -> std::io::Result<u8> {
//         self.cursor.read_u8()
//     }
//
//     pub fn read_i8(&mut self) -> std::io::Result<i8> {
//         self.cursor.read_i8()
//     }
//
//     pub fn peek_byte(&mut self, position: usize) -> std::io::Result<u8> {
//         let start = self.cursor.position();
//         self.cursor.set_position(position as u64);
//         let byte = self.read_u8();
//         self.cursor.set_position(start);
//         return byte;
//     }
//
//     read_type!(u16);
//     read_type!(u32);
//     read_type!(u64);
//     read_type!(u128);
//
//     read_type!(i16);
//     read_type!(i32);
//     read_type!(i64);
//     read_type!(i128);
//
//     read_type!(f32);
//     read_type!(f64);
//
//     // pub fn read<T>(&mut self) -> std::io::Result<u16> {
//     //         let mut data = vec![0u8, size_of::<u16>() as u8];
//     //         self.cursor.read(&mut data[..])?;
//     //         let mut data = data.as_slice();
//     //         paste! {
//     //             match self.endianness {
//     //             BigEndian | NetworkEndian => data.read_u16::<BigEndian>(),
//     //             LittleEndian => data.read_u16::<LittleEndian>(),
//     //             NativeEndian => data.read_u16::<NativeEndian>(),
//     //         }
//     //     }
//     // }
// }
