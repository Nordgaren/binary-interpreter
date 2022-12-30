extern crate core;

pub mod binary_reader;
pub mod error;
mod util;

#[cfg(test)]
mod tests {
    use std::io::SeekFrom::End;
    use crate::binary_reader::{BinaryReader, Endian};

    #[test]
    fn read_c_string() {
        let hello_world: Vec<u8> = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut br = BinaryReader::new(hello_world.as_slice()).unwrap();

        let string = br.read_cstr().unwrap();

        assert_eq!(string, "Hello, World!");
        assert_eq!(br.length - br.position(), hello_world.len() - br.position());
    }

    #[test]
    fn read_wide_string() {
        let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();

        let result = br.read_wcstr().unwrap();

        assert_eq!(result, "Hello, World!");
        assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    }

    #[test]
    fn read_byte() {
        let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
        let byte = br.read_u8().unwrap();
        assert_eq!(byte, 0x48);
    }

    #[test]
    fn read_u16() {
        let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();

        let ushort = br.read_u16().unwrap();
        assert_eq!(ushort, 0x48);
        assert_eq!(br.length - br.position(), w_hello_world.len() - br.position());
    }

    #[test]
    fn read_u32() {
        let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
        let uint = br.read_u32().unwrap();
        println!("{:02x}",uint);
        assert_eq!(uint, 0x650048);
    }

    #[test]
    fn read_u64() {
        let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
        let ulong = br.read_u64().unwrap();
        println!("{:02x}", ulong);
        assert_eq!(ulong, 0x6C006C00650048);
    }

    #[test]
    fn read_u128() {
        let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
        let udouble_quad = br.read_u128().unwrap();
        println!("{:02x}", udouble_quad);
        assert_eq!(udouble_quad, 0x570020002C006F006C006C00650048);
    }

    #[test]
    fn read_sbyte() {
        let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
        let sbyte = br.read_i8().unwrap();
        assert_eq!(sbyte, 0x48);
    }

    #[test]
    fn read_i16() {
        let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();

        let short = br.read_i16().unwrap();
        assert_eq!(short, 0x48);
    }

    #[test]
    fn read_i32() {
        let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
        let int = br.read_i32().unwrap();
        println!("{:02x}", int);
        assert_eq!(int, 0x650048);
    }

    #[test]
    fn read_i64() {
        let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
        let long = br.read_i64().unwrap();
        println!("{:02x}", long);
        assert_eq!(long, 0x6C006C00650048);
    }

    #[test]
    fn read_i128() {
        let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
        let double_quad = br.read_i128().unwrap();
        println!("{:02x}", double_quad);
        assert_eq!(double_quad, 0x570020002C006F006C006C00650048);
    }

    #[test]
    fn read_f32() {
        let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
        let float = br.read_f32().unwrap();
        assert_eq!(float, 9.275486e-39);
    }

    #[test]
    fn read_f64() {
        let w_hello_world = vec![0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x2c, 0x00, 0x20, 0x00, 0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x21, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut br = BinaryReader::new(w_hello_world.as_slice()).unwrap();
        let double = br.read_f64().unwrap();
        assert_eq!(double, 1.246114697985072e-306);
    }
}