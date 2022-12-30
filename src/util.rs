use std::mem::size_of;
use byteorder::*;
use paste::paste;

// #[macro_export]
#[macro_use]
    /// generates read_type functions.
    macro_rules! read_type {
    ($ty:ty) => {
        paste! {
            #[doc = "Reads a `" $ty "` type from the stream and advances the cursor by the size of the type" ]
            pub fn [<read_ $ty>](&mut self) -> std::io::Result<$ty> {
                let mut data = vec![0u8; size_of::<$ty>()];
                self.cursor.read(&mut data[..])?;
                let mut data = data.as_slice();
                match self.endianness {
                    LittleEndian => data.[<read_ $ty>]::<LittleEndian>(),
                    NativeEndian => data.[<read_ $ty>]::<NativeEndian>(),
                    _ => data.[<read_ $ty>]::<BigEndian>(),
                }
            }
        }
    };
}

#[macro_export]
/// generates read_type functions.
macro_rules! peek_type {
    ($ty:ty) => {
        paste! {
            #[doc = "Reads a `" $ty "` type from the stream at the position then returns to where it started" ]
            fn [<peek_ $ty>]<T: ByteOrder>(&mut self, position: usize) -> std::io::Result<$ty> {
                let start = self.stream_position()?;
                self.seek(SeekFrom::Start(position as u64))?;
                let byte = self.[<read_ $ty>]::<T>();
                self.seek(SeekFrom::Start(start))?;
                return byte;
            }
        }
    };
}


