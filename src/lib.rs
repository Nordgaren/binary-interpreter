extern crate core;

use byteorder::{BigEndian, LittleEndian, NativeEndian, NetworkEndian};

pub mod binary_reader;
pub mod error;
mod util;

pub enum Endian {
    BigEndian(BigEndian),
    NetworkEndian(NetworkEndian),
    LittleEndian(LittleEndian),
    NativeEndian(NativeEndian),
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::io::SeekFrom::End;
    use byteorder::{BE, LE,NativeEndian, ReadBytesExt};
    use crate::binary_reader::{BinaryPeeker, BinaryReader};

    #[test]
    fn read_c_string() {
        let hello_world: Vec<u8> = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let expected = hello_world.as_slice().read_cstr().unwrap();

        assert_eq!(expected, "Hello, World!");
        //assert_eq!(br.length - br.position(), hello_world.len() - br.position());
    }

    #[test]
    fn read_wide_string() {
        let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let expected = w_hello_world.as_slice().read_wcstr().unwrap();
        assert_eq!(expected, "Hello, World!");
        //assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    }

    #[test]
    fn read_fixed_c_string() {
        let hello_world: Vec<u8> = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let expected = hello_world.as_slice().read_fixed_cstr(4).unwrap();

        assert_eq!(expected, "Hell");
        //assert_eq!(br.length - br.position(), hello_world.len() - br.position());
    }

    #[test]
    fn read_fixed_wide_string() {
        let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let expected = w_hello_world.as_slice().read_fixed_wcstr(4).unwrap();
        assert_eq!(expected, "Hell");
        //assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    }


    #[test]
    fn peek_u8(){
        let hello_world: Vec<u8> = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut c = Cursor::new(hello_world);
        let expected = c.peek_u8(0).unwrap();
        assert_eq!(expected, 0x48);
    }

    #[test]
    fn peek_u16() {
        let hello_world: Vec<u8> = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut c = Cursor::new(hello_world);
        let expected = c.peek_u16::<LE>(0).unwrap();
        assert_eq!(expected, 0x6548);
        let expected = c.peek_u16::<BE>(0).unwrap();
        assert_eq!(expected, 0x4865);
    }

    #[test]
    fn peek_u32() {
        let hello_world: Vec<u8> = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut c = Cursor::new(hello_world);
        let expected = c.peek_u32::<LE>(0).unwrap();
        assert_eq!(expected, 0x6C6C6548);
        let expected = c.peek_u32::<BE>(0).unwrap();
        assert_eq!(expected, 0x48656C6C);
    }

    #[test]
    fn peek_u64() {
        let hello_world: Vec<u8> = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut c = Cursor::new(hello_world);
        let expected = c.peek_u64::<LE>(0).unwrap();
        assert_eq!(expected, 0x57202C6F6C6C6548);
        let expected = c.peek_u64::<BE>(0).unwrap();
        assert_eq!(expected, 0x48656C6C6F2C2057);
    }

    #[test]
    fn peek_u128() {
        let hello_world: Vec<u8> = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut c = Cursor::new(hello_world);
        let expected = c.peek_u128::<LE>(0).unwrap();
        assert_eq!(expected, 0x2010021646C726F57202C6F6C6C6548);
        let expected = c.peek_u128::<BE>(0).unwrap();
        assert_eq!(expected, 0x48656C6C6F2C20576F726C6421000102);
    }

    #[test]
    fn peek_i8(){
        let hello_world: Vec<u8> = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut c = Cursor::new(hello_world);
        let expected = c.peek_i8(0).unwrap();
        assert_eq!(expected, 0x48);
    }

    #[test]
    fn peek_i16() {
        let hello_world: Vec<u8> = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut c = Cursor::new(hello_world);
        let expected = c.peek_i16::<LE>(0).unwrap();
        assert_eq!(expected, 0x6548);
        let expected = c.peek_i16::<BE>(0).unwrap();
        assert_eq!(expected, 0x4865);
    }

    #[test]
    fn peek_i32() {
        let hello_world: Vec<u8> = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut c = Cursor::new(hello_world);
        let expected = c.peek_i32::<LE>(0).unwrap();
        assert_eq!(expected, 0x6C6C6548);
        let expected = c.peek_i32::<BE>(0).unwrap();
        assert_eq!(expected, 0x48656C6C);
    }

    #[test]
    fn peek_i64() {
        let hello_world: Vec<u8> = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut c = Cursor::new(hello_world);
        let expected = c.peek_i64::<LE>(0).unwrap();
        assert_eq!(expected, 0x57202C6F6C6C6548);
        let expected = c.peek_i64::<BE>(0).unwrap();
        assert_eq!(expected, 0x48656C6C6F2C2057);
    }

    #[test]
    fn peek_i128() {
        let hello_world: Vec<u8> = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut c = Cursor::new(hello_world);
        let expected = c.peek_i128::<LE>(0).unwrap();
        assert_eq!(expected, 0x2010021646C726F57202C6F6C6C6548);
        let expected = c.peek_i128::<BE>(0).unwrap();
        assert_eq!(expected, 0x48656C6C6F2C20576F726C6421000102);
    }
    // #[test]
    // fn read_byte() {
    //     let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    //     let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
    //     let byte = br.read_u8().unwrap();
    //     assert_eq!(byte, 0x48);
    //     assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    // }
    //
    // #[test]
    // fn read_u16() {
    //     let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    //     let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
    //     let ushort = br.read_u16().unwrap();
    //     assert_eq!(ushort, 0x48);
    //     assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    // }
    //
    // #[test]
    // fn read_u32() {
    //     let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    //     let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
    //     let uint = br.read_u32().unwrap();
    //     assert_eq!(uint, 0x650048);
    //     assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    //
    // }
    //
    // #[test]
    // fn read_u64() {
    //     let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    //     let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
    //     let ulong = br.read_u64().unwrap();
    //     assert_eq!(ulong, 0x6C006C00650048);
    //     assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    // }
    //
    // #[test]
    // fn read_u128() {
    //     let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    //     let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
    //     let udouble_quad = br.read_u128().unwrap();
    //     assert_eq!(udouble_quad, 0x570020002C006F006C006C00650048);
    //     assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    // }
    //
    // #[test]
    // fn read_sbyte() {
    //     let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    //     let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
    //     let sbyte = br.read_i8().unwrap();
    //     assert_eq!(sbyte, 0x48);
    //     assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    // }
    //
    // #[test]
    // fn read_i16() {
    //     let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    //     let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
    //     let short = br.read_i16().unwrap();
    //     assert_eq!(short, 0x48);
    //     assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    // }
    //
    // #[test]
    // fn read_i32() {
    //     let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    //     let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
    //     let int = br.read_i32().unwrap();
    //     assert_eq!(int, 0x650048);
    //     assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    // }
    //
    // #[test]
    // fn read_i64() {
    //     let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    //     let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
    //     let long = br.read_i64().unwrap();
    //     assert_eq!(long, 0x6C006C00650048);
    //     assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    // }
    //
    // #[test]
    // fn read_i128() {
    //     let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    //     let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
    //     let double_quad = br.read_i128().unwrap();
    //     assert_eq!(double_quad, 0x570020002C006F006C006C00650048);
    //     assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    // }
    //
    // #[test]
    // fn read_f32() {
    //     let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    //     let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
    //     let float = br.read_f32().unwrap();
    //     assert_eq!(float, 9.275486e-39);
    //     assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    // }
    //
    // #[test]
    // fn read_f64() {
    //     let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    //     let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
    //     let double = br.read_f64().unwrap();
    //     assert_eq!(double, 1.246114697985072e-306);
    //     assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    // }
}
