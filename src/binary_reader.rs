use std::fs;
use std::io::{BufRead, Cursor, Read};
use std::mem::size_of;
use std::path::PathBuf;
use crate::error::*;
use byteorder::{ReadBytesExt, BE, LE, NetworkEndian, NativeEndian, BigEndian, LittleEndian, WriteBytesExt};
use crate::binary_reader::Endian::*;
use crate::read_type;
use paste::paste;

pub enum Endian {
    BigEndian,
    NetworkEndian,
    LittleEndian,
    NativeEndian,
}

pub struct BinaryReader {
    cursor: Cursor<Vec<u8>>,
    pub length: usize,
    endianness: Endian,
}

impl BinaryReader {
    pub fn new(bytes: &[u8]) -> Result<BinaryReader, InterpreterError> {
        Ok(BinaryReader {
            length: bytes.len() as usize,
            cursor: Cursor::new(bytes.to_vec()),
            endianness: NativeEndian,
        })
    }

    pub fn from_vec(vec: Vec<u8>) -> Result<BinaryReader, InterpreterError> {
        BinaryReader::new(vec.as_slice())
    }

    pub fn from_path(path: PathBuf) -> Result<BinaryReader, InterpreterError> {
        let file = fs::read(path)?;
        BinaryReader::new(file.as_slice())
    }

    pub fn set_endian(&mut self ,endian: Endian) {
        self.endianness = endian
    }

    pub fn read_cstr(&mut self) -> Result<String, InterpreterError> {
        let mut buf = Vec::new();
        self.cursor.read_until(0, &mut buf)?;
        buf.pop(); //remove the null byte
        Ok(String::from_utf8(buf)?)
    }

    pub fn read_wcstr(&mut self) -> Result<String, InterpreterError> {
        let mut chrs = Vec::new();
        while let chr = self.cursor.read_u16::<NativeEndian>()? {
            if chr == 0 {
                break;
            }
            chrs.push(chr);
        }

        Ok(String::from_utf16(chrs.as_slice())?)
    }

    pub fn position(&self) -> usize {
        self.cursor.position() as usize
    }

    pub fn read_u8(&mut self) -> std::io::Result<u8> {
        self.cursor.read_u8()
    }

    pub fn read_i8(&mut self) -> std::io::Result<i8> {
        self.cursor.read_i8()
    }

    pub fn peek_byte(&mut self, position: usize) -> Result<u8, InterpreterError> {
        let start = self.cursor.position();
        self.cursor.set_position(position as u64);
        let byte = self.read_u8()?;
        self.cursor.set_position(start);
        Ok(byte)
    }

    read_type!(u16);
    read_type!(u32);
    read_type!(u64);
    read_type!(u128);

    read_type!(i16);
    read_type!(i32);
    read_type!(i64);
    read_type!(i128);

    read_type!(f32);
    read_type!(f64);

    // pub fn read<T>(&mut self) -> std::io::Result<u16> {
    //         let mut data = vec![0u8, size_of::<u16>() as u8];
    //         self.cursor.read(&mut data[..])?;
    //         let mut data = data.as_slice();
    //         paste! {
    //             match self.endianness {
    //             BigEndian | NetworkEndian => data.read_u16::<BigEndian>(),
    //             LittleEndian => data.read_u16::<LittleEndian>(),
    //             NativeEndian => data.read_u16::<NativeEndian>(),
    //         }
    //     }
    // }
}
